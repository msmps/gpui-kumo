# KUMO-004: Button: complete pinned browser visual and narrow-layout comparisons

Status: Open

GitHub issue: Pending publication

## Problem

Button geometry and input dispatch are tested, but browser pixel parity and narrow-container behavior are not established. CSS group opacity/compositing and native per-primitive opacity may differ, especially for disabled/loading emphasis variants.

## Evidence

- [Native Button contract](../kumo-component-recipes.md#native-button-contract)
- [Button implementation](../../crates/gpui-kumo/src/button.rs)
- [Button tests](../../crates/gpui-kumo/src/button_tests.rs)

## Acceptance criteria

- [ ] Pin browser dependency/CSS versions, viewport, scale factor and consumer font profile for reproducible comparisons.
- [ ] Compare all variants/sizes/shapes/slots in light/dark modes, with focus, hover, open, loading and disabled combinations.
- [ ] Cover long labels and narrow parents; verify and document the native one-line/content-sized layout policy and icon-only geometry.
- [ ] Measure foregrounds, rings, gradients and opacity/compositing; record differences separately from intended native policies.
- [ ] Commit reproducible comparison instructions and reference/native captures; split newly found independent defects into issues.

## Scope

The Outline transition and transparent-outline shadow have dedicated follow-ups; this issue establishes comparison coverage and does not duplicate those implementation tasks.

Baseline: commit `38c2ea8`, GPUI Kit/Base 0.7.0 and GPUI 0.3.7. Where a browser comparison is needed, use Kumo commit `3fd5b648df578cb1ba214dedd30f475009f6a668`.
