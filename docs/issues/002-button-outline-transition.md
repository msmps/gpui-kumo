# KUMO-002: Button: implement the authored 100ms outline color transition

Status: Open

GitHub issue: Pending publication

## Problem

Outline color changes are immediate in the native component. The pinned Kumo source specifies a 100ms transition. This is a documented incomplete behavior rather than a failing activation path.

## Evidence

- [Native Button contract](../kumo-component-recipes.md#native-button-contract)
- [Button implementation](../../crates/gpui-kumo/src/button.rs)
- [Button tests](../../crates/gpui-kumo/src/button_tests.rs)

## Acceptance criteria

- [ ] Identify the exact properties and easing from generated CSS at the pinned reference.
- [ ] Implement the native transition without changing layout or focus/availability precedence.
- [ ] Ensure reduced motion renders the final state without unnecessary frame requests.
- [ ] Test interrupted transitions and rendered intermediate/final colors.

Baseline: commit `38c2ea8`, GPUI Kit/Base 0.7.0 and GPUI 0.3.7. Where a browser comparison is needed, use Kumo commit `3fd5b648df578cb1ba214dedd30f475009f6a668`.
