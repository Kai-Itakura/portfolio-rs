use syn::visit::{self, Visit};
use syn::{ImplItem, Item, TraitItem};

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
}
