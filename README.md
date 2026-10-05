# GPUI Kumo

A native Kumo design system built on GPUI, with GPUI Base as the initial behavior foundation.

## Installation

The latest published release candidate is [`0.1.0-rc.2`](https://crates.io/crates/gpui-kumo/0.1.0-rc.2), including Table components. To use the published RC, add these dependencies to your application's `Cargo.toml`:

```toml
[dependencies]
gpui-kumo = "=0.1.0-rc.2"
gpui-kit = { version = "=0.7.0", default-features = false }
```

Use Rust 1.99 or later. On macOS, install full Xcode and select its developer directory. Other targets need the native prerequisites of the selected [GPUI Kit release](https://github.com/longbridge/gpui-kit/tree/v0.7.0); this RC does not establish complete cross-platform accessibility parity.

Cargo downloads and builds dependencies for your project. You do not install separate global copies of GPUI or GPUI Kit. The direct `gpui-kit` dependency lets your application import its window, entity and rendering APIs; Kumo uses the same dependency. Kit re-exports GPUI, so a separate `gpui` dependency is unnecessary for this setup.

### Minimal application

Put this in `src/main.rs` and run `cargo run`:

```rust
use gpui_kit::{
    App, AppContext, Context, IntoElement, ParentElement, Render, Styled,
    Window, WindowOptions, div, px,
};
use gpui_kumo::{Button, assets::Assets};

struct Demo;

impl Render for Demo {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().p(px(24.)).child(Button::new("hello", "Hello from Kumo"))
    }
}

fn main() {
    gpui_kit::application().with_assets(Assets).run(|cx: &mut App| {
        gpui_kumo::init(cx);
        cx.on_window_closed(|cx, _| cx.quit()).detach();
        gpui_kit::open_window(WindowOptions::default(), cx, |_, cx| {
            cx.new(|_| Demo)
        })
        .expect("open Kumo window");
        cx.activate(true);
    });
}
```

Call `gpui_kumo::init` before opening windows; it initializes Kit/Base and Kumo interaction behavior. `gpui_kit::open_window` installs the Base Root used by overlays. `Assets` provides named Kumo icons; internal component icons are embedded directly. If you already have an application `AssetSource`, delegate unknown paths to `gpui_kumo::assets::Assets` instead of replacing your assets.

### Compatibility and release status

This RC targets GPUI Kit **0.7.0**, GPUI Pre **0.3.7** and Base **0.7.0**. Import GPUI APIs through `gpui_kit`; different GPUI packages or Git sources may have incompatible types.

Normal installation uses published dependencies and needs no vendor patches. Known limitations affect reverse-Tab focus, Textarea automatic growth and Linux accessibility state. See [dependency compatibility, limitations and optional fixes](https://github.com/msmps/gpui-kumo/blob/main/docs/dependencies.md) before adopting the RC.

## Workspace

- `crates/gpui-kumo`: typed Kumo theme and native design-system library. See the [component catalog](https://github.com/msmps/gpui-kumo/blob/main/docs/component-coverage.md) for coverage and [Select acceptance/API](https://github.com/msmps/gpui-kumo/blob/main/docs/select-validation.md) for retained single/multiple selection.
- `apps/gallery`: native component examples and foundation previews with light/dark switching. See the [coverage inventory](https://github.com/msmps/gpui-kumo/blob/main/docs/component-coverage.md) and [GitHub issues](https://github.com/msmps/gpui-kumo/issues) for outstanding work.

Component implementations and their tests/helpers live together under `crates/gpui-kumo/src/<component>/`; standalone modules can remain single files. The gallery binary and focused examples share the `kumo_gallery` library target. `GalleryAssets` composes application assets with `gpui_kumo::assets::Assets`, so consumers do not need filesystem paths into the library's source tree.

Rust 1.99.0 is pinned in `rust-toolchain.toml`. GPUI Kit 0.7.0 is selected with its default features disabled, excluding the styled Component layer and bundled icons. Commit `Cargo.lock` when changing dependencies. The development workspace uses the [optional dependency corrections](https://github.com/msmps/gpui-kumo/blob/main/docs/dependencies.md#optional-dependency-corrections).

## Development

On macOS, install full Xcode and select its developer directory. Run from the repository root:

```sh
cargo run -p kumo-gallery
cargo test -p gpui-kumo
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

Choose Light or Dark, or press Command-L, to switch appearance. Close the window or press Command-Q to quit. Linux headless gallery checks are recorded per component; platform accessibility and macOS checks remain separately tracked.

For the announcement workflow, run `cargo run -p kumo-gallery --example announcement --locked`. It composes document actions, editing/confirmation and Toast feedback; [preview evidence](https://github.com/msmps/gpui-kumo/blob/main/docs/validation-fixtures.md) records the Linux rehearsal and remaining platform acceptance.

Read the [maintenance index](https://github.com/msmps/gpui-kumo/blob/main/docs/README.md) before implementing components.

For the full locked workspace and exact AccessKit adapter checks, run `bash scripts/check-rust.sh`. The same commands run in `.github/workflows/validation.yml`; native visual/input/OS acceptance remains separately tracked. Outstanding work and acceptance criteria are tracked in [GitHub issues](https://github.com/msmps/gpui-kumo/issues).

For manual native visual and input review, follow the [verification guidance](https://github.com/msmps/gpui-kumo/blob/main/docs/gpui-verification.md). Historical browser/native fixtures are [archived in Git history](https://github.com/msmps/gpui-kumo/blob/main/docs/validation-fixtures.md).

Standalone form controls display their label by default. Prefer `Field::control` for associated layout: it carries label help, optional indicators and feedback into the wrapper and forwards errors to the control. Disable the retained control state once; its Field label follows automatically. Use `optional_indicator(true)` for presentation-only “(optional)” text. See the [public API contract](https://github.com/msmps/gpui-kumo/blob/main/docs/design-system-components.md#public-api-contract) for composition and migration rules.
