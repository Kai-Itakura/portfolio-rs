use syn::UseTree;

pub fn refers_to_crate_module(tree: &UseTree, module: &str) -> bool {
    match tree {
        UseTree::Path(path) if path.ident == "crate" => first_is(&path.tree, module),
        UseTree::Path(path) if path.ident == "super" => mentions(&path.tree, module),
        _ => first_is(tree, module),
    }
}

fn first_is(tree: &UseTree, module: &str) -> bool {
    match tree {
        UseTree::Path(path) => path.ident == module,
        UseTree::Name(name) => name.ident == module,
        UseTree::Rename(rename) => rename.ident == module,
        UseTree::Group(group) => group.items.iter().any(|item| first_is(item, module)),
        UseTree::Glob(_) => false,
    }
}

fn mentions(tree: &UseTree, module: &str) -> bool {
    match tree {
        UseTree::Path(path) => path.ident == module || mentions(&path.tree, module),
        UseTree::Name(name) => name.ident == module,
        UseTree::Rename(rename) => rename.ident == module,
        UseTree::Group(group) => group.items.iter().any(|item| mentions(item, module)),
        UseTree::Glob(_) => false,
    }
}
