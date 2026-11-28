mod checker;
mod context;
mod naming;
mod rules;
mod violation;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use syn::visit::Visit;

use crate::checker::Checker;
use crate::context::Context;
use crate::rules::Rule;
use crate::violation::Violation;

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

fn query_service_module(relative: &Path) -> Option<String> {
    if !relative.starts_with("src/query_service") || relative == Path::new("src/query_service.rs") {
        return None;
    }
    relative
        .file_stem()
        .map(|stem| stem.to_string_lossy().into_owned())
}

fn check_file(rules: &[Box<dyn Rule>], root: &Path, path: &Path) -> Result<Vec<Violation>, String> {
    let source =
        fs::read_to_string(path).map_err(|error| format!("{}: {error}", path.display()))?;
    let file = syn::parse_file(&source).map_err(|error| format!("{}: {error}", path.display()))?;
    let relative = path.strip_prefix(root).unwrap_or(path);
    let cx = Context {
        path: relative,
        in_domain: relative.starts_with("src/domain") || relative == Path::new("src/domain.rs"),
        in_query_service: relative.starts_with("src/query_service")
            || relative == Path::new("src/query_service.rs"),
        query_service_module: query_service_module(relative),
        violations: Vec::new(),
    };
    let mut checker = Checker::new(rules, cx);
    checker.visit_file(&file);
    Ok(checker.into_violations())
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

    let rules = rules::all();
    let mut violations = Vec::new();
    for path in &files {
        match check_file(&rules, root, path) {
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
