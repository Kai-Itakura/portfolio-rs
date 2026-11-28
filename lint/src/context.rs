use std::path::Path;

use proc_macro2::Span;

use crate::violation::Violation;

pub struct Context<'a> {
    pub path: &'a Path,
    pub in_domain: bool,
    pub in_query_service: bool,
    pub in_application: bool,
    pub query_service_module: Option<String>,
    pub violations: Vec<Violation>,
}

impl Context<'_> {
    pub fn report(&mut self, span: Span, rule: &'static str, message: String) {
        self.violations.push(Violation {
            path: self.path.to_path_buf(),
            line: span.start().line,
            rule,
            message,
        });
    }
}
