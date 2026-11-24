default:
    @just --list

fmt:
    cargo fmt --all --check
    topcoat fmt --check src lint

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
    rustup component add clippy rustfmt
    rustup toolchain install nightly --profile minimal -c llvm-tools-preview
    cargo install topcoat-cli --version 0.10.0 --locked
    cargo install cargo-llvm-cov --locked
