#!/usr/bin/env bash
# Run from the repository root. Rust/toolchain/dependencies stay pinned in Cargo.
set -euo pipefail
rustc --version --verbose
cargo --version
cargo fmt --all -- --check
cargo test --workspace --all-features --locked
# Public documentation and normal/compile-fail examples are part of the API gate.
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo build --workspace --all-targets --all-features --locked
cargo build -p kumo-gallery --locked
# Exact standalone adapter is excluded from the workspace. Its own lock is committed.
manifest=vendor/accesskit_atspi_common-0.19.1/Cargo.toml
cargo fmt --manifest-path "$manifest" -- --check
cargo test --manifest-path "$manifest" --locked
cargo test --manifest-path "$manifest" --all-features --locked
cargo clippy --manifest-path "$manifest" --all-targets --all-features --locked -- -D warnings
