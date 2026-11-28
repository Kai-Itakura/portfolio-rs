mod domain_no_mut_self;
mod domain_no_try_from;
mod query_service_entry;
mod query_service_no_application;
mod query_service_no_interior_write;
mod query_service_no_mut_param;
mod query_service_no_sibling;
mod query_service_only_from_application;
mod query_service_returns_root;
mod query_service_root_type;
mod query_service_type_prefix;
mod test_name;

use syn::{ExprMethodCall, File, ImplItem, Item, TraitItem};

use crate::context::Context;

pub trait Rule {
    fn check_file(&self, _cx: &mut Context<'_>, _file: &File) {}
    fn check_item(&self, _cx: &mut Context<'_>, _item: &Item) {}
    fn check_impl_item(&self, _cx: &mut Context<'_>, _item: &ImplItem) {}
    fn check_trait_item(&self, _cx: &mut Context<'_>, _item: &TraitItem) {}
    fn check_expr_method_call(&self, _cx: &mut Context<'_>, _expr: &ExprMethodCall) {}
}

pub fn all() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(domain_no_mut_self::DomainNoMutSelf),
        Box::new(domain_no_try_from::DomainNoTryFrom),
        Box::new(query_service_entry::QueryServiceEntry),
        Box::new(query_service_no_application::QueryServiceNoApplication),
        Box::new(query_service_no_interior_write::QueryServiceNoInteriorWrite),
        Box::new(query_service_no_mut_param::QueryServiceNoMutParam),
        Box::new(query_service_no_sibling::QueryServiceNoSibling),
        Box::new(query_service_only_from_application::QueryServiceOnlyFromApplication),
        Box::new(query_service_returns_root::QueryServiceReturnsRoot),
        Box::new(query_service_root_type::QueryServiceRootType),
        Box::new(query_service_type_prefix::QueryServiceTypePrefix),
        Box::new(test_name::TestName),
    ]
}
