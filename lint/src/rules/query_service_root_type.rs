use proc_macro2::Span;
use syn::File;

use super::Rule;
use crate::context::Context;
use crate::naming::{public_type_names, upper_camel_case};

pub struct QueryServiceRootType;

impl Rule for QueryServiceRootType {
    fn check_file(&self, cx: &mut Context<'_>, file: &File) {
        let Some(module) = cx.query_service_module.clone() else {
            return;
        };
        let root = upper_camel_case(&module);
        let count = public_type_names(&file.items)
            .iter()
            .filter(|name| **name == root.as_str())
            .count();
        if count != 1 {
            cx.report(
                Span::call_site(),
                "query-service-root-type",
                format!(
                    "`{module}.rs` に、表示用のルートの型 `{root}`（公開の struct か enum）がちょうど 1 つ必要（今は {count} 個）"
                ),
            );
        }
    }
}
