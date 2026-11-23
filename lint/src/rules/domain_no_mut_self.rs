use syn::spanned::Spanned;
use syn::{FnArg, ImplItem, ReceiverKind, Signature, TraitItem};

use super::Rule;
use crate::context::Context;

pub struct DomainNoMutSelf;

impl Rule for DomainNoMutSelf {
    fn check_impl_item(&self, cx: &mut Context<'_>, item: &ImplItem) {
        if let ImplItem::Fn(item) = item {
            check_receiver(cx, &item.sig);
        }
    }

    fn check_trait_item(&self, cx: &mut Context<'_>, item: &TraitItem) {
        if let TraitItem::Fn(item) = item {
            check_receiver(cx, &item.sig);
        }
    }
}

fn check_receiver(cx: &mut Context<'_>, sig: &Signature) {
    if !cx.in_domain {
        return;
    }
    let Some(FnArg::Receiver(receiver)) = sig.inputs.first() else {
        return;
    };
    if let ReceiverKind::Reference(_, _, Some(_)) = receiver.kind {
        cx.report(
            receiver.span(),
            "domain-no-mut-self",
            format!(
                "`{}` が `&mut self` を受け取っている。ドメインの関数は `self` を受け取って新しい値を返す",
                sig.ident
            ),
        );
    }
}
