use proc_macro2::Span;
use syn::File;

use super::Rule;
use crate::context::Context;
use crate::naming::public_fns;

pub struct QueryServiceEntry;

impl Rule for QueryServiceEntry {
    fn check_file(&self, cx: &mut Context<'_>, file: &File) {
        let Some(module) = cx.query_service_module.clone() else {
            return;
        };
        let fns = public_fns(&file.items);
        match fns.as_slice() {
            [entry] if entry.sig.ident == module => {}
            [entry] => cx.report(
                entry.sig.ident.span(),
                "query-service-entry",
                format!(
                    "公開関数 `{}` の名前がファイル名と違う。`{module}.rs` の公開関数は `{module}` にする",
                    entry.sig.ident
                ),
            ),
            _ => cx.report(
                Span::call_site(),
                "query-service-entry",
                format!(
                    "`{module}.rs` の公開関数は `{module}` の 1 つだけにする（今は {} 個）。補助の関数は非公開にする",
                    fns.len()
                ),
            ),
        }
    }
}
