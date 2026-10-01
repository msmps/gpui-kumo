# KUMO-001: Button: expose disabled and loading accessibility state

Status: Open

GitHub issue: Pending publication

## Problem

Unavailable buttons reject activation and leave Tab traversal, but the current fluent interface cannot expose disabled/busy metadata. “Unavailable” and “Loading” descriptions are supplementary and do not establish semantic state. Preserve the existing distinction between explicit disabled and loading presentation.

## Evidence

- [Native Button contract](../kumo-component-recipes.md#native-button-contract)
- [Button implementation](../../crates/gpui-kumo/src/button.rs)
- [Button tests](../../crates/gpui-kumo/src/button_tests.rs)

## Acceptance criteria

- [ ] Expose disabled semantics for both disabled and loading buttons, and busy semantics while loading, using a bounded bridge or compatible upstream fix.
- [ ] Assert enabled → disabled/loading → enabled metadata changes and rejection of pointer, keyboard and accessible activation.
- [ ] Inspect actual platform accessibility state; retain descriptive fallbacks where a platform cannot represent busy.
- [ ] Document the upstream dependency change or local adapter boundary without exposing Base internals in the public Button API.

Baseline: commit `38c2ea8`, GPUI Kit/Base 0.7.0 and GPUI 0.3.7. Where a browser comparison is needed, use Kumo commit `3fd5b648df578cb1ba214dedd30f475009f6a668`.
