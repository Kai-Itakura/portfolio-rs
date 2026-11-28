use syn::{File, Item, UseTree};

use super::Rule;
use crate::context::Context;

pub struct QueryServiceNoSibling;

impl Rule for QueryServiceNoSibling {
    fn check_file(&self, cx: &mut Context<'_>, file: &File) {
        if cx.query_service_module.is_none() {
            return;
        }
        for item in &file.items {
            let Item::Use(item) = item else {
                continue;
            };
            if refers_to_sibling(&item.tree) {
                cx.report(
                    item.use_token.span,
                    "query-service-no-sibling",
                    "クエリサービスがほかのクエリサービスを `use` している。組み合わせはアプリケーション層でやる"
                        .to_string(),
                );
            }
        }
    }
}

fn refers_to_sibling(tree: &UseTree) -> bool {
    let UseTree::Path(path) = tree else {
        return false;
    };
    if path.ident == "super" {
        return true;
    }
    path.ident == "crate"
        && matches!(&*path.tree, UseTree::Path(next) if next.ident == "query_service")
}
