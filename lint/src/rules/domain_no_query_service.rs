use syn::{Item, UseTree};

use super::Rule;
use crate::context::Context;

pub struct DomainNoQueryService;

impl Rule for DomainNoQueryService {
    fn check_item(&self, cx: &mut Context<'_>, item: &Item) {
        if !cx.in_domain {
            return;
        }
        let Item::Use(item) = item else {
            return;
        };
        if mentions_query_service(&item.tree) {
            cx.report(
                item.use_token.span,
                "domain-no-query-service",
                "ドメインがクエリサービスを `use` している。表示用の型はドメインの外に置き、ドメインからは使わない"
                    .to_string(),
            );
        }
    }
}

fn mentions_query_service(tree: &UseTree) -> bool {
    match tree {
        UseTree::Path(path) => path.ident == "query_service" || mentions_query_service(&path.tree),
        UseTree::Name(name) => name.ident == "query_service",
        UseTree::Rename(rename) => rename.ident == "query_service",
        UseTree::Group(group) => group.items.iter().any(mentions_query_service),
        UseTree::Glob(_) => false,
    }
}
