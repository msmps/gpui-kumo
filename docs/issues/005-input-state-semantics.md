# KUMO-005: Input: expose disabled, read-only and invalid accessibility metadata

Status: Open

GitHub issue: Pending publication

## Problem

Input enforces editing and traversal availability and exposes error/help descriptions, but the selected GPUI fluent API does not expose disabled, read-only or invalid state flags. A screen reader should be able to distinguish these states semantically.

## Evidence

- [Native Input contract](../kumo-component-recipes.md#native-input-contract)
- [Input implementation](../../crates/gpui-kumo/src/input.rs)
- [Input tests](../../crates/gpui-kumo/src/input_tests.rs)

## Acceptance criteria

- [ ] Expose each state independently through an accessibility bridge or supported upstream API.
- [ ] Keep the application-owned validation/display contract and the current read-only selection/copy behavior.
- [ ] Associate error/help meaning with the named control and verify state changes when clearing errors or re-enabling editing.
- [ ] Test metadata and actual platform state alongside existing input gating/focus behavior.

Baseline: commit `38c2ea8`, GPUI Kit/Base 0.7.0 and GPUI 0.3.7. Where a browser comparison is needed, use Kumo commit `3fd5b648df578cb1ba214dedd30f475009f6a668`.
