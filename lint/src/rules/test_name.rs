use syn::{Attribute, Item};

use super::Rule;
use crate::context::Context;

pub struct TestName;

impl Rule for TestName {
    fn check_item(&self, cx: &mut Context<'_>, item: &Item) {
        let Item::Fn(item) = item else {
            return;
        };
        if !is_test(&item.attrs) {
            return;
        }
        let name = item.sig.ident.to_string();
        if !is_test_name(&name) {
            cx.report(
                item.sig.ident.span(),
                "test-name",
                format!("テスト名 `{name}` が `<関数>_<結果>_when_<条件>` の形になっていない"),
            );
        }
    }
}

fn is_test(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        attr.path()
            .segments
            .last()
            .is_some_and(|segment| segment.ident == "test")
    })
}

fn is_test_name(name: &str) -> bool {
    let is_snake_case = name
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_');
    let Some((before, after)) = name.split_once("_when_") else {
        return false;
    };
    is_snake_case && before.contains('_') && !after.is_empty()
}
