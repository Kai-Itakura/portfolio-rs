use syn::{Item, Visibility};

pub fn upper_camel_case(snake: &str) -> String {
    snake
        .split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect(),
                None => String::new(),
            }
        })
        .collect()
}

pub fn is_public(visibility: &Visibility) -> bool {
    !matches!(visibility, Visibility::Inherited)
}

pub fn public_type_names(items: &[Item]) -> Vec<&syn::Ident> {
    items
        .iter()
        .filter_map(|item| match item {
            Item::Struct(item) if is_public(&item.vis) => Some(&item.ident),
            Item::Enum(item) if is_public(&item.vis) => Some(&item.ident),
            _ => None,
        })
        .collect()
}

pub fn public_fns(items: &[Item]) -> Vec<&syn::ItemFn> {
    items
        .iter()
        .filter_map(|item| match item {
            Item::Fn(item) if is_public(&item.vis) => Some(item),
            _ => None,
        })
        .collect()
}
