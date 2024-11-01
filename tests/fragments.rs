// Run these tests with:
//
// ```sh
// $ cargo t --release --all-features --test fragments
// ```

#[macro_use]
mod macros;

use quote::{quote, ToTokens as _};
use syn::Expr;

#[test]
fn test_expr_fragment_as_expr() {
    let tokens = quote!(__expandable_expr_42);
    snapshot!(tokens as Expr, @r#"
    Expr::Fragment {
        kind: ExprFragmentKind::Expr(FragmentExpr {
            payload: 42,
        }),
    }
    "#);
}
