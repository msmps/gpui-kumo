# KUMO-001: Button: expose disabled and loading accessibility state

Status: Resolved — consolidated into published repair and platform trackers

GitHub issues: [#36](https://github.com/msmps/gpui-kumo/issues/36), [#39](https://github.com/msmps/gpui-kumo/issues/39)

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

## Continuation — 2026-10-02

Button enriches Base's existing AccessKit node through `a11y_synthetic_children`: disabled is set for `disabled || loading`, busy for loading. No dependency fork, duplicate role or public API change. Native macOS inspection of the gallery and [state fixture](../../apps/gallery/examples/state_semantics.rs) reports `(disabled)` for disabled/loading buttons and removes it on reset. Existing rendered pointer/keyboard/traversal tests pass. Busy export and assistive activation gating across platforms remain acceptance work. GPUI's debug JSON serializer omits these state bits; its description strings are not used as proof of the flags.

## Consolidation — 2026-10-03

Authored disabled/busy metadata and exact Linux adapter exports are implemented and validated in closed #36/#39. Keep their patches/evidence. This unpublished umbrella is superseded; remaining assistive activation and spoken/cross-platform acceptance belongs to #2–#4. Historical acceptance checkboxes above are not a claim that every platform was exercised.
