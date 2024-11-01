use proc_macro2::{Ident, Span, TokenTree};

use crate::parse::{Parse, ParseStream};

const PREFIX: &str = "__expandable";
const PREFIX_: &str = "__expandable_";

pub struct FragmentExpr {
    pub span: Span,
    pub payload: usize,
}

impl Parse for FragmentExpr {
    fn parse(input: ParseStream) -> crate::Result<FragmentExpr> {
        input.step(|cursor| {
            cursor
                .expr_fragment()
                .ok_or_else(|| todo!("error message for FragmentExpr"))
        })
    }
}

impl From<FragmentExpr> for TokenTree {
    fn from(fragment_expr: FragmentExpr) -> TokenTree {
        let span = fragment_expr.span;
        let payload = fragment_expr.payload;
        let label = FragmentExpr::LABEL;

        let ident = format!("{PREFIX}_{label}_{payload}");
        let ident = Ident::new(ident.as_str(), span);

        ident.into()
    }
}

impl FragmentExpr {
    const LABEL: &str = "expr";
    const LABEL_: &str = "expr_";
}

#[derive(Clone)]
pub enum Fragment {
    Expr(FragmentExpr),
}

impl From<FragmentExpr> for Fragment {
    fn from(v: FragmentExpr) -> Fragment {
        Fragment::Expr(v)
    }
}

impl Fragment {
    pub(crate) fn span(&self) -> Span {
        match self {
            Fragment::Expr(expr_fragment) => expr_fragment.span,
        }
    }

    pub fn as_expr(&self) -> Option<&FragmentExpr> {
        match self {
            Self::Expr(v) => Some(v),
            #[expect(unreachable_patterns)]
            _ => None,
        }
    }
}

impl From<Fragment> for TokenTree {
    fn from(fragment: Fragment) -> TokenTree {
        let (label, payload, span) = match fragment {
            Fragment::Expr(expr_fragment) => (
                FragmentExpr::LABEL,
                expr_fragment.payload,
                expr_fragment.span,
            ),
        };

        let ident = format!("{PREFIX}_{label}_{payload}");
        let ident = Ident::new(ident.as_str(), span);

        ident.into()
    }
}

impl TryFrom<Ident> for Fragment {
    type Error = Ident;

    fn try_from(ident: Ident) -> Result<Fragment, Ident> {
        let span = ident.span();
        let ident_ = ident.to_string();

        let after_prefix = match ident_.strip_prefix(PREFIX_) {
            Some(after_prefix) => after_prefix,
            None => return Err(ident),
        };

        let fragment = if let Some(after_label) = after_prefix.strip_prefix(FragmentExpr::LABEL_) {
            let payload = match after_label.parse() {
                Ok(payload) => payload,
                Err(_) => return Err(ident),
            };

            FragmentExpr { span, payload }.into()
        } else {
            return Err(ident);
        };

        Ok(fragment)
    }
}

#[cfg(feature = "printing")]
mod printing {
    use std::iter;

    use proc_macro2::{Ident, Span, TokenStream, TokenTree};
    use quote::ToTokens;

    use super::{FragmentExpr, PREFIX};

    #[cfg_attr(docsrs, doc(cfg(feature = "printing")))]
    impl ToTokens for FragmentExpr {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let ident = fragment_ident(FragmentExpr::LABEL, self.payload, self.span);
            let tree = TokenTree::Ident(ident);

            tokens.extend(iter::once(tree));
        }
    }

    fn fragment_ident(label: &str, payload: usize, span: Span) -> Ident {
        let ident = format!("{PREFIX}_{label}_{payload}");
        Ident::new(ident.as_str(), span)
    }
}
