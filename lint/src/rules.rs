mod domain_no_mut_self;
mod domain_no_try_from;
mod test_name;

use syn::{ImplItem, Item, TraitItem};

use crate::context::Context;

pub trait Rule {
    fn check_item(&self, _cx: &mut Context<'_>, _item: &Item) {}
    fn check_impl_item(&self, _cx: &mut Context<'_>, _item: &ImplItem) {}
    fn check_trait_item(&self, _cx: &mut Context<'_>, _item: &TraitItem) {}
}

pub fn all() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(domain_no_mut_self::DomainNoMutSelf),
        Box::new(domain_no_try_from::DomainNoTryFrom),
        Box::new(test_name::TestName),
    ]
}
