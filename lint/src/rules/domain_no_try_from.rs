use syn::Item;
use syn::spanned::Spanned;

use super::Rule;
use crate::context::Context;

pub struct DomainNoTryFrom;

impl Rule for DomainNoTryFrom {
    fn check_item(&self, cx: &mut Context<'_>, item: &Item) {
        if cx.in_domain
            && let Item::Impl(item) = item
            && let Some((path, _)) = &item.trait_
            && path
                .segments
                .last()
                .is_some_and(|segment| segment.ident == "TryFrom")
        {
            cx.report(
                path.span(),
                "domain-no-try-from",
                "ドメインで `TryFrom` を実装している。検査する生成関数は `Period::from_dates` のような名前付きの関数にする"
                    .to_string(),
            );
        }
    }
}
