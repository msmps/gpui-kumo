# Source baseline and refresh procedure

Read when selecting a dependency, checking a version-sensitive API, or updating these references.

## Research baseline

These docs were researched on **2026-10-01** against Zed upstream commit [`95cd535a5fad96d649513f96c5784ceefd379e47`](https://github.com/zed-industries/zed/commit/95cd535a5fad96d649513f96c5784ceefd379e47). Source links pin that commit so the described behavior remains inspectable.

This research baseline differs from the selected implementation dependencies below. GPUI is pre-1.0; the upstream README explicitly warns of breaking changes. Use the source corresponding to the selected dependency as the authority for executable APIs. [Upstream README](https://github.com/zed-industries/zed/blob/95cd535a5fad96d649513f96c5784ceefd379e47/crates/gpui/README.md)

Code snippets in these docs illustrate source-checked API shapes. They have not been compiled or launched. Product guidance is labeled separately from upstream facts.

## Implementation baseline

The initial workspace selects GPUI Kit **0.7.0** with default features disabled. The resolved dependency set includes GPUI Base **0.7.0** and the GPUI/platform **0.3.7** snapshot family; the styled Component layer and Kit's bundled assets are excluded. Consult [Cargo.toml](../Cargo.toml) and [Cargo.lock](../Cargo.lock) for exact configuration and resolutions. Startup was checked against the [0.7.0 facade source](https://github.com/longbridge/gpui-kit/blob/v0.7.0/crates/kit/src/lib.rs).

The repository enables retained root patches for development. A normal `gpui-kumo` consumer resolves published dependencies and supplies its own lockfile; Kit accepts compatible Base 0.7 releases while pinning the GPUI 0.3.7 family. The workspace prepares `0.1.0-rc.2`; `0.1.0-rc.1` remains the published RC. The [installation guide](../README.md#installation) defines the validated baseline, unpatched limitations and optional consumer-root overrides. Do not treat a passing patched workspace gate as unpatched consumer acceptance.

Rust is pinned in [rust-toolchain.toml](../rust-toolchain.toml). The first validation target is `aarch64-apple-darwin` with full Xcode. The gallery supplies its own embedded SVG asset. See the [repository README](../README.md) for run and check commands and the [Base assessment](gpui-base-assessment.md) for the snapshot's upstream Zed revision.

Bootstrap verification on 2026-10-01: workspace build, formatting and Clippy with warnings denied passed. Native window creation, visible text, embedded SVG rendering and the native Quit menu action were verified. Other platforms and component behavior remain unverified. Cargo reports a future-compatibility warning in the upstream `block` 0.1.6 dependency; the current toolchain builds successfully.

## Dependency selection

1. Choose a released version or a git revision; record the exact resolved dependencies in project configuration and lockfile. If using git, keep GPUI and platform crates on a compatible revision.
2. Check platform features and toolchain requirements in that revision's manifests and README. A standalone application needs actual windowing/text backends, not merely element APIs.
3. Compile and launch a minimal window with visible text and an asset. Finish when bootstrapping, glyphs, and resources work on the intended host.
4. Recheck all version-sensitive mechanisms used by the first component. Finish when signatures and behavior have been verified for that component's contract.

## Locate the owning source

All paths below are in the pinned [Zed tree](https://github.com/zed-industries/zed/tree/95cd535a5fad96d649513f96c5784ceefd379e47).

| Question | Owning source |
| --- | --- |
| What is exported and which traits need imports? | `crates/gpui/src/gpui.rs`, `prelude.rs`, `elements/mod.rs` |
| What is a view/component/custom element? | `crates/gpui/src/element.rs` |
| How do styles resolve? | `crates/gpui/src/style.rs`, `styled.rs`, `elements/div.rs` |
| Where are numbered utility methods defined? | `crates/gpui_macros/src/styles.rs` |
| What do length units mean? | `crates/gpui/src/geometry.rs`, `window.rs` |
| How do input, IDs, roles, and state styles work? | `crates/gpui/src/elements/div.rs`, `_accessibility.rs` |
| How does text obtain identity and typography? | `crates/gpui/src/elements/text.rs`, `window.rs` |
| How do state and theme consumers redraw? | `crates/gpui/src/app/context.rs`, `app.rs`, `subscription.rs` |
| What does real editable text require? | `crates/gpui/examples/input.rs` |
| Which primitive supports a specialized feature? | `crates/gpui/src/elements/` |
| How are framework tests driven? | `crates/gpui/src/app/test_context.rs` and colocated primitive tests |
| How is a higher-level component composed? | `crates/ui/src/components/button/button_like.rs`, `button.rs` |
| How does standalone startup select backends? | `crates/gpui/README.md`, `crates/gpui_platform/Cargo.toml`, `src/gpui_platform.rs` |

## Refresh without mixing versions

When upgrading GPUI, compare the selected source to the baseline for startup, trait signatures, ID requirements, state refinement order, text/accessibility APIs, click synthesis, and reduced-motion behavior. Update the affected references and their source links together. Compile their snippets as examples once an executable crate exists, then rerun the relevant component checks.

The [GPUI website](https://gpui.rs/) is a discovery entry point. Explanatory guides and Zed's higher-level UI examples can lag or add their own extension traits. Follow a questionable method to the source that owns it before treating it as a GPUI primitive.

## Packaged RC consumer validation

On 2026-10-04, `gpui-kumo 0.1.0-rc.1` was packaged with `cargo package -p gpui-kumo --no-verify --locked --offline --allow-dirty`. The resulting `.crate` archive was extracted outside this workspace. A separate application with its own `[workspace]` depended on that extracted package and crates.io `gpui-kit = "=0.7.0"` with default features disabled. Neither manifest contained patches; Cargo metadata confirmed one registry-sourced Kit 0.7.0, GPUI Pre 0.3.7 and Base 0.7.0. No library source was taken from the original checkout during consumer compilation.

Measured on `aarch64-apple-darwin` with Rust 1.99 and full Xcode:

- The exact packaged README application built and linked successfully.
- All 19 packaged rustdoc tests passed: 12 normal/no-run examples and seven compile-fail examples.
- Five standalone fixtures copied into the consumer built and linked: `public_api`, `state_semantics`, `popover_edges`, `popover_stress` and `native_counter_probe`. The public API fixture's application-owned SVG was copied with it. Gallery-module-dependent examples are not packaged library examples and were not included in this consumer gate.
- Two consumer asset tests passed. `Assets::list` exposed both named icons; both loaded and rasterized through the published GPUI `SvgRenderer`. Unknown paths returned no asset. All 22 packaged SVGs rasterized to images with nonzero alpha, covering internal component icons as well as public named assets.
- Consumer formatting and all-target Clippy with warnings denied passed. The archive included the README and license notices, and excluded vendor sources.

To repeat this gate, extract a freshly generated archive into a temporary directory, create the consumer as a sibling with its own `[workspace]`, and point only `gpui-kumo` at the extracted directory. Copy the README example into its `src/main.rs`, then run `cargo build`, `cargo test` and `cargo clippy --all-targets -- -D warnings`. Run `cargo test --doc` from the extracted package. Inspect `cargo metadata` to ensure Kit/GPUI/Base resolve from crates.io rather than local patches. Preserve the consumer lockfile for repeat checks.

This establishes package completeness, linking, public example compatibility and actual SVG decoding/rasterization. It does not establish native visual alignment, corner clipping, interactive behavior or Linux accessibility acceptance. The [unpatched limitations](dependencies.md#what-the-unpatched-rc-provides) remain. The upstream `block 0.1.6` future-compatibility notice still appears; it did not fail these gates. Temporary binaries and logs were kept outside Git, and nothing was published.

Publication checks on 2026-10-04: archive inspection passed for all 115 files, including all Rust source files and the README/license notices; no vendor, gallery or log files were packaged, and the normalized dependency manifest contained no path/Git dependencies or patches. `cargo publish -p gpui-kumo --dry-run --locked --allow-dirty` completed package verification and compilation, then explicitly aborted upload because it was a dry run. Nothing was published. The new unpatched CI gate measured 236 passes and all ten named known failures, with no unexpected failures or missing baseline tests; its failure-policy regression checks passed. This is local execution of the CI script, not a hosted GitHub Actions result.
