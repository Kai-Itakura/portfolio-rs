mod domain_no_mut_self;
mod domain_no_query_service;
mod domain_no_try_from;
mod query_service_no_interior_write;
mod query_service_no_mut_param;
mod test_name;

use syn::{ExprMethodCall, ImplItem, Item, TraitItem};

use crate::context::Context;

pub trait Rule {
    fn check_item(&self, _cx: &mut Context<'_>, _item: &Item) {}
    fn check_impl_item(&self, _cx: &mut Context<'_>, _item: &ImplItem) {}
    fn check_trait_item(&self, _cx: &mut Context<'_>, _item: &TraitItem) {}
    fn check_expr_method_call(&self, _cx: &mut Context<'_>, _expr: &ExprMethodCall) {}
}

pub fn all() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(domain_no_mut_self::DomainNoMutSelf),
        Box::new(domain_no_query_service::DomainNoQueryService),
        Box::new(domain_no_try_from::DomainNoTryFrom),
        Box::new(query_service_no_interior_write::QueryServiceNoInteriorWrite),
        Box::new(query_service_no_mut_param::QueryServiceNoMutParam),
        Box::new(test_name::TestName),
    ]
}
