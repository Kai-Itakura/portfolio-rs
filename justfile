default:
    @just --list

fmt:
    cargo fmt --all
    topcoat fmt src lint

clippy:
    cargo clippy --workspace --all-targets -- -D warnings

test:
    cargo test --workspace

lint:
    cargo run -p lint

check: fmt clippy test lint

cov:
    cargo +nightly llvm-cov --branch --summary-only

setup:
    rustup toolchain install nightly --profile minimal -c llvm-tools-preview
    cargo install topcoat-cli@0.10.0 cargo-llvm-cov --locked
