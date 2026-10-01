# Source baseline and refresh procedure

Read when selecting a dependency, checking a version-sensitive API, or updating these references.

## Research baseline

These docs were researched on **2026-10-01** against Zed upstream commit [`95cd535a5fad96d649513f96c5784ceefd379e47`](https://github.com/zed-industries/zed/commit/95cd535a5fad96d649513f96c5784ceefd379e47). Source links pin that commit so the described behavior remains inspectable.

This is a research baseline, not a dependency selection. There is no Cargo manifest or lockfile yet. GPUI is pre-1.0; the upstream README explicitly warns of breaking changes. A published crate may differ from this source snapshot. Use the source corresponding to the project's eventual selected version as the authority for executable APIs. [Upstream README](https://github.com/zed-industries/zed/blob/95cd535a5fad96d649513f96c5784ceefd379e47/crates/gpui/README.md)

Code snippets in these docs illustrate source-checked API shapes. They have not been compiled or launched. Product guidance is labeled separately from upstream facts.

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
