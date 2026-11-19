use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{Attribute, FnArg, ImplItemFn, ItemFn, ItemImpl, ReceiverKind, Signature, TraitItemFn};

struct Violation {
    path: PathBuf,
    line: usize,
    rule: &'static str,
    message: String,
}

impl fmt::Display for Violation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}: [{}] {}",
            self.path.display(),
            self.line,
            self.rule,
            self.message
        )
    }
}

struct Checker<'a> {
    path: &'a Path,
    in_domain: bool,
    violations: Vec<Violation>,
}

impl Checker<'_> {
    fn report(&mut self, span: Span, rule: &'static str, message: String) {
        self.violations.push(Violation {
            path: self.path.to_path_buf(),
            line: span.start().line,
            rule,
            message,
        });
    }

    fn check_receiver(&mut self, sig: &Signature) {
        if !self.in_domain {
            return;
        }
        let Some(FnArg::Receiver(receiver)) = sig.inputs.first() else {
            return;
        };
        if let ReceiverKind::Reference(_, _, Some(_)) = receiver.kind {
            self.report(
                receiver.span(),
                "domain-no-mut-self",
                format!(
                    "`{}` が `&mut self` を受け取っている。ドメインの関数は `self` を受け取って新しい値を返す",
                    sig.ident
                ),
            );
        }
    }

    fn check_test_name(&mut self, item: &ItemFn) {
        let name = item.sig.ident.to_string();
        if !is_test_name(&name) {
            self.report(
                item.sig.ident.span(),
                "test-name",
                format!("テスト名 `{name}` が `<関数>_<結果>_when_<条件>` の形になっていない"),
            );
        }
    }
}

impl<'ast> Visit<'ast> for Checker<'_> {
    fn visit_item_fn(&mut self, item: &'ast ItemFn) {
        if is_test(&item.attrs) {
            self.check_test_name(item);
        }
        visit::visit_item_fn(self, item);
    }

    fn visit_impl_item_fn(&mut self, item: &'ast ImplItemFn) {
        self.check_receiver(&item.sig);
        visit::visit_impl_item_fn(self, item);
    }

    fn visit_trait_item_fn(&mut self, item: &'ast TraitItemFn) {
        self.check_receiver(&item.sig);
        visit::visit_trait_item_fn(self, item);
    }

    fn visit_item_impl(&mut self, item: &'ast ItemImpl) {
        if self.in_domain
            && let Some((path, _)) = &item.trait_
            && path
                .segments
                .last()
                .is_some_and(|segment| segment.ident == "TryFrom")
        {
            self.report(
                path.span(),
                "domain-no-try-from",
                "ドメインで `TryFrom` を実装している。検査する生成関数は `Period::from_dates` のような名前付きの関数にする"
                    .to_string(),
            );
        }
        visit::visit_item_impl(self, item);
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

fn rust_files(dir: &Path, files: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            rust_files(&path, files)?;
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            files.push(path);
        }
    }
    Ok(())
}

fn check_file(root: &Path, path: &Path) -> Result<Vec<Violation>, String> {
    let source =
        fs::read_to_string(path).map_err(|error| format!("{}: {error}", path.display()))?;
    let file = syn::parse_file(&source).map_err(|error| format!("{}: {error}", path.display()))?;
    let relative = path.strip_prefix(root).unwrap_or(path);
    let mut checker = Checker {
        path: relative,
        in_domain: relative.starts_with("src/domain") || relative == Path::new("src/domain.rs"),
        violations: Vec::new(),
    };
    checker.visit_file(&file);
    Ok(checker.violations)
}

fn main() -> ExitCode {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("lint クレートはリポジトリの直下にある");
    let mut files = Vec::new();
    if let Err(error) = rust_files(&root.join("src"), &mut files) {
        eprintln!("src/ を読めない: {error}");
        return ExitCode::FAILURE;
    }
    files.sort();

    let mut violations = Vec::new();
    for path in &files {
        match check_file(root, path) {
            Ok(found) => violations.extend(found),
            Err(error) => {
                eprintln!("構文を読めない: {error}");
                return ExitCode::FAILURE;
            }
        }
    }

    for violation in &violations {
        println!("{violation}");
    }
    if violations.is_empty() {
        println!("lint: {} ファイルを確かめ、違反は無かった", files.len());
        ExitCode::SUCCESS
    } else {
        println!("lint: {} 件の違反", violations.len());
        ExitCode::FAILURE
    }
}
