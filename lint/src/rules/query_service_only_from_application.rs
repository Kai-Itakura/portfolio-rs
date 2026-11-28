use syn::Item;

use super::Rule;
use crate::context::Context;
use crate::use_tree::refers_to_crate_module;

pub struct QueryServiceOnlyFromApplication;

impl Rule for QueryServiceOnlyFromApplication {
    fn check_item(&self, cx: &mut Context<'_>, item: &Item) {
        if cx.in_application || cx.in_query_service {
            return;
        }
        let Item::Use(item) = item else {
            return;
        };
        if refers_to_crate_module(&item.tree, "query_service") {
            cx.report(
                item.use_token.span,
                "query-service-only-from-application",
                "クエリサービスを `use` できるのはアプリケーション層だけ。表示用の型はアプリケーション層から使う"
                    .to_string(),
            );
        }
    }
}
