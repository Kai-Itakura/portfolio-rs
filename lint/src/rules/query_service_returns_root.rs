use syn::visit::{self, Visit};
use syn::{File, ReturnType, TypePath};

use super::Rule;
use crate::context::Context;
use crate::naming::{public_fns, upper_camel_case};

pub struct QueryServiceReturnsRoot;

impl Rule for QueryServiceReturnsRoot {
    fn check_file(&self, cx: &mut Context<'_>, file: &File) {
        let Some(module) = cx.query_service_module.clone() else {
            return;
        };
        let root = upper_camel_case(&module);
        for entry in public_fns(&file.items) {
            if entry.sig.ident != module {
                continue;
            }
            let mentions_root = match &entry.sig.output {
                ReturnType::Default => false,
                ReturnType::Type(_, ty) => {
                    let mut finder = TypeNameFinder {
                        name: &root,
                        found: false,
                    };
                    finder.visit_type(ty);
                    finder.found
                }
            };
            if !mentions_root {
                cx.report(
                    entry.sig.ident.span(),
                    "query-service-returns-root",
                    format!("`{module}` の戻り値に、表示用のルートの型 `{root}` が無い"),
                );
            }
        }
    }
}

struct TypeNameFinder<'a> {
    name: &'a str,
    found: bool,
}

impl<'ast> Visit<'ast> for TypeNameFinder<'_> {
    fn visit_type_path(&mut self, path: &'ast TypePath) {
        if path
            .path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == self.name)
        {
            self.found = true;
        }
        visit::visit_type_path(self, path);
    }
}
