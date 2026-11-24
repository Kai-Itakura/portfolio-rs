default:
    @just --list

fmt:
    cargo fmt --all --check
    topcoat fmt --check

clippy:
    cargo clippy --workspace --all-targets -- -D warnings

test:
    cargo test --workspace

lint:
    cargo run -p lint

check: fmt clippy test lint

cov:
    cargo +nightly llvm-cov --branch --summary-only
