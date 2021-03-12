use crate::utils::{is_type_diagnostic_item, match_type, paths, snippet_with_macro_callsite, span_lint_and_sugg};
use rustc_errors::Applicability;
use rustc_hir as hir;
use rustc_lint::LateContext;
use rustc_middle::ty;
use rustc_span::symbol::sym;

use super::CLONE_ON_REF_PTR;

pub(super) fn check(cx: &LateContext<'_>, expr: &hir::Expr<'_>, arg: &hir::Expr<'_>) {
    let obj_ty = cx.typeck_results().expr_ty(arg).peel_refs();

    if let ty::Adt(_, subst) = obj_ty.kind() {
        let caller_type = if is_type_diagnostic_item(cx, obj_ty, sym::Rc) {
            "Rc"
        } else if is_type_diagnostic_item(cx, obj_ty, sym::Arc) {
            "Arc"
        } else if match_type(cx, obj_ty, &paths::WEAK_RC) || match_type(cx, obj_ty, &paths::WEAK_ARC) {
            "Weak"
        } else {
            return;
        };

        let snippet = snippet_with_macro_callsite(cx, arg.span, "..");

        span_lint_and_sugg(
            cx,
            CLONE_ON_REF_PTR,
            expr.span,
            "using `.clone()` on a ref-counted pointer",
            "try this",
            format!("{}::<{}>::clone(&{})", caller_type, subst.type_at(0), snippet),
            Applicability::Unspecified, // Sometimes unnecessary ::<_> after Rc/Arc/Weak
        );
    }
}
