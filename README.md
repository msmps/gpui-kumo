# GPUI Kumo

A native Kumo design system built on GPUI, with GPUI Base as the initial behavior foundation.

## Workspace

- `crates/gpui-kumo`: typed Kumo theme, Text, Button, Input, Popover, Loader, LayerCard, Badge, Link, Banner, Empty, Label, Field, Checkbox, Radio, Switch, ButtonGroup, InputGroup and Tooltip components.
- `apps/gallery`: native component examples and foundation previews with light/dark switching. See the [port backlog](docs/port-progress.md) for current coverage and validation gaps.

Rust 1.99.0 is pinned in `rust-toolchain.toml`. GPUI Kit 0.7.0 is selected with its default features disabled, excluding the styled Component layer and bundled icons. Commit `Cargo.lock` when changing dependencies. The workspace applies an exact-version GPUI0.3.7 [tab-registration correction](docs/gpui-tab-registration-patch.md), preserving reverse Tab and Input accessibility focus. Downstream Cargo workspaces must apply the same root override; dependency patches do not propagate. See the linked setup and limitations before integrating this unreleased library.

## Development

On macOS, install full Xcode and select its developer directory. Run from the repository root:

```sh
cargo run -p kumo-gallery
cargo test -p gpui-kumo
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

Choose Light or Dark, or press Command-L, to switch appearance. Close the window or press Command-Q to quit. Linux headless gallery checks are recorded per component; platform accessibility and macOS checks remain separately tracked.

Read the [implementation plan](docs/implementation-plan.md) for milestones and the [docs index](docs/README.md) before implementing components.
