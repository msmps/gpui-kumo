# GPUI Kumo

A native Kumo design system built on GPUI, with GPUI Base as the initial behavior foundation.

## Workspace

- `crates/gpui-kumo`: design-system library and initialization entry point.
- `apps/gallery`: native foundation gallery with a light/dark switcher and previews for semantic colors, typography, gradients and shadows. Button, Input and Popover components are the next milestone.

Rust 1.99.0 is pinned in `rust-toolchain.toml`. GPUI Kit 0.7.0 is selected with its default features disabled, excluding the styled Component layer and bundled icons. Commit `Cargo.lock` when changing dependencies.

## Development

On macOS, install full Xcode and select its developer directory. Run from the repository root:

```sh
cargo run -p kumo-gallery
cargo test -p gpui-kumo
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

Choose Light or Dark, or press Command-L, to switch appearance. Close the window or press Command-Q to quit. Other platforms have not been verified.

Read the [implementation plan](docs/implementation-plan.md) for milestones and the [docs index](docs/README.md) before implementing components.
