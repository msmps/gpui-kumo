# KUMO-003: Button: evaluate faithful shadow rendering on transparent outlines

Status: Open

GitHub issue: Pending publication

## Problem

The native Outline variant deliberately omits Kumo’s shadow-xs because GPUI drop shadows paint inside transparent boxes. Track the fidelity tradeoff explicitly; this is a current native policy, not a newly reproduced regression.

## Evidence

- [Native Button contract](../kumo-component-recipes.md#native-button-contract)
- [Button implementation](../../crates/gpui-kumo/src/button.rs)
- [Button tests](../../crates/gpui-kumo/src/button_tests.rs)

## Acceptance criteria

- [ ] Capture native and pinned browser Outline buttons in light/dark modes over contrasting backgrounds.
- [ ] Evaluate an exterior-only shadow implementation that preserves the transparent interior and pointer/focus geometry.
- [ ] Implement a faithful bounded solution or record the accepted visual deviation with side-by-side evidence.
- [ ] Add a paint regression check if the shadow implementation changes.

Baseline: commit `38c2ea8`, GPUI Kit/Base 0.7.0 and GPUI 0.3.7. Where a browser comparison is needed, use Kumo commit `3fd5b648df578cb1ba214dedd30f475009f6a668`.
