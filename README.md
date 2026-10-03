# GPUI Kumo

A native Kumo design system built on GPUI, with GPUI Base as the initial behavior foundation.

## Workspace

- `crates/gpui-kumo`: typed Kumo theme and native design-system library. See the [component catalog](docs/component-coverage.md) for coverage and [Select acceptance/API](docs/select-validation.md) for retained single/multiple selection.
- `apps/gallery`: native component examples and foundation previews with light/dark switching. See the [coverage inventory](docs/component-coverage.md) and [GitHub issues](https://github.com/msmps/gpui-kumo/issues) for outstanding work.

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

For the announcement workflow, run `cargo run -p kumo-gallery --example announcement --locked`. It composes document actions, editing/confirmation and Toast feedback; [preview evidence](docs/validation-fixtures.md) records the Linux rehearsal and remaining platform acceptance.

Read the [maintenance index](docs/README.md) before implementing components.

The retained Textarea uses a narrow [gpui-base0.7.0 growth patch](docs/gpui-base-textarea-growth-patch.md). Base also carries a narrow [Select confirmation focus repair](docs/gpui-base-select-focus-patch.md). Downstream Cargo roots must override both GPUI and Base as documented to reproduce this workspace behavior.

For the full locked workspace and exact AccessKit adapter checks, run `bash scripts/check-rust.sh`. The same commands run in `.github/workflows/validation.yml`; native visual/input/OS acceptance remains separately tracked. Outstanding work and acceptance criteria are tracked in [GitHub issues](https://github.com/msmps/gpui-kumo/issues).
