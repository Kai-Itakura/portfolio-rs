use syn::spanned::Spanned;
use syn::{FnArg, ImplItem, Item, ReceiverKind, Signature, TraitItem, Type};

use super::Rule;
use crate::context::Context;

pub struct QueryServiceNoMutParam;

impl Rule for QueryServiceNoMutParam {
    fn check_item(&self, cx: &mut Context<'_>, item: &Item) {
        if let Item::Fn(item) = item {
            check_signature(cx, &item.sig);
        }
    }

    fn check_impl_item(&self, cx: &mut Context<'_>, item: &ImplItem) {
        if let ImplItem::Fn(item) = item {
            check_signature(cx, &item.sig);
        }
    }

    fn check_trait_item(&self, cx: &mut Context<'_>, item: &TraitItem) {
        if let TraitItem::Fn(item) = item {
            check_signature(cx, &item.sig);
        }
    }
}

fn check_signature(cx: &mut Context<'_>, sig: &Signature) {
    if !cx.in_query_service {
        return;
    }
    for input in &sig.inputs {
        let is_mut = match input {
            FnArg::Receiver(receiver) => {
                matches!(receiver.kind, ReceiverKind::Reference(_, _, Some(_)))
            }
            FnArg::Typed(typed) => {
                matches!(&*typed.ty, Type::Reference(reference) if reference.mutability.is_some())
            }
        };
        if is_mut {
            cx.report(
                input.span(),
                "query-service-no-mut-param",
                format!(
                    "`{}` が `&mut` を受け取っている。クエリサービスは状態を変えないので、`&` で受け取る",
                    sig.ident
                ),
            );
        }
    }
}
