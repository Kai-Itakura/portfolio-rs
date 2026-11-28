use syn::visit::{self, Visit};
use syn::{ExprMethodCall, File, ImplItem, Item, TraitItem};

use crate::context::Context;
use crate::rules::Rule;
use crate::violation::Violation;

pub struct Checker<'a> {
    rules: &'a [Box<dyn Rule>],
    cx: Context<'a>,
}

impl<'a> Checker<'a> {
    pub fn new(rules: &'a [Box<dyn Rule>], cx: Context<'a>) -> Self {
        Self { rules, cx }
    }

    pub fn into_violations(self) -> Vec<Violation> {
        self.cx.violations
    }
}

impl<'ast> Visit<'ast> for Checker<'_> {
    fn visit_file(&mut self, file: &'ast File) {
        for rule in self.rules {
            rule.check_file(&mut self.cx, file);
        }
        visit::visit_file(self, file);
    }

    fn visit_item(&mut self, item: &'ast Item) {
        for rule in self.rules {
            rule.check_item(&mut self.cx, item);
        }
        visit::visit_item(self, item);
    }

    fn visit_impl_item(&mut self, item: &'ast ImplItem) {
        for rule in self.rules {
            rule.check_impl_item(&mut self.cx, item);
        }
        visit::visit_impl_item(self, item);
    }

    fn visit_trait_item(&mut self, item: &'ast TraitItem) {
        for rule in self.rules {
            rule.check_trait_item(&mut self.cx, item);
        }
        visit::visit_trait_item(self, item);
    }

    fn visit_expr_method_call(&mut self, expr: &'ast ExprMethodCall) {
        for rule in self.rules {
            rule.check_expr_method_call(&mut self.cx, expr);
        }
        visit::visit_expr_method_call(self, expr);
    }
}
