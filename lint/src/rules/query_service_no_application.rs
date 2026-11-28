use syn::Item;

use super::Rule;
use crate::context::Context;
use crate::use_tree::refers_to_crate_module;

pub struct QueryServiceNoApplication;

impl Rule for QueryServiceNoApplication {
    fn check_item(&self, cx: &mut Context<'_>, item: &Item) {
        if !cx.in_query_service {
            return;
        }
        let Item::Use(item) = item else {
            return;
        };
        if refers_to_crate_module(&item.tree, "application") {
            cx.report(
                item.use_token.span,
                "query-service-no-application",
                "クエリサービスがアプリケーション層を `use` している。依存の向きはアプリケーション層 → クエリサービス"
                    .to_string(),
            );
        }
    }
}
