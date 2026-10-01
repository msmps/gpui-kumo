# KUMO-006: Input: provide accessible text ranges, selection and editing actions

Status: Open

GitHub issue: Pending publication

## Problem

Live Linux AT-SPI inspection finds the named Input entry and its focus, but only Accessible/Component interfaces. Base supplies no accessible text-run subtree, so aria_value alone does not expose the Text interface. Native assistive-technology editing and selection remain incomplete.

## Evidence

- [Native Input contract](../kumo-component-recipes.md#native-input-contract)
- [Input implementation](../../crates/gpui-kumo/src/input.rs)
- [Input tests](../../crates/gpui-kumo/src/input_tests.rs)
- [Observed AT-SPI limitation](../popover-validation.md#concrete-limits)

## Acceptance criteria

- [ ] Expose an accessible text subtree with stable identity, current text, Unicode character boundaries and valid bounds.
- [ ] Expose caret/selection state and route supported accessibility editing/selection actions into the retained Base editor.
- [ ] Test updates, selection and composition with non-ASCII text without duplicate Change events or a second editing engine.
- [ ] Validate text reads and selection/editing through live AT-SPI and the target platform adapters.
- [ ] Preserve disabled/read-only restrictions and avoid introducing entity retention cycles.

Baseline: commit `38c2ea8`, GPUI Kit/Base 0.7.0 and GPUI 0.3.7. Where a browser comparison is needed, use Kumo commit `3fd5b648df578cb1ba214dedd30f475009f6a668`.
