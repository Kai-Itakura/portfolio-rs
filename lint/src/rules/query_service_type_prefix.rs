use syn::File;

use super::Rule;
use crate::context::Context;
use crate::naming::{public_type_names, upper_camel_case};

pub struct QueryServiceTypePrefix;

impl Rule for QueryServiceTypePrefix {
    fn check_file(&self, cx: &mut Context<'_>, file: &File) {
        let Some(module) = cx.query_service_module.clone() else {
            return;
        };
        let root = upper_camel_case(&module);
        for name in public_type_names(&file.items) {
            if !name.to_string().starts_with(&root) {
                cx.report(
                    name.span(),
                    "query-service-type-prefix",
                    format!(
                        "公開の型 `{name}` の名前が `{root}` で始まっていない。`{module}.rs` に置けるのは、ルートの型 `{root}` とその部品だけ"
                    ),
                );
            }
        }
    }
}
