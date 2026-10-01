# KUMO-007: Input: validate composition through operating-system IMEs

Status: Open

GitHub issue: Pending publication

## Problem

Headless tests invoke UTF-16 marked-text/commit handlers directly; they do not prove integration with an operating-system IME. Native smoke tests covered ordinary typing, not a real composition candidate/commit/cancel workflow.

## Evidence

- [Native Input contract](../kumo-component-recipes.md#native-input-contract)
- [Input implementation](../../crates/gpui-kumo/src/input.rs)
- [Input tests](../../crates/gpui-kumo/src/input_tests.rs)

## Acceptance criteria

- [ ] Run real OS IME composition on the supported native platforms, recording OS/IME versions and activation steps.
- [ ] Exercise CJK candidate selection, commit/cancel, replacement of a selection, and composition around surrogate pairs or combining sequences.
- [ ] Verify caret/selection placement, horizontal scrolling, no duplicate Change/Submit delivery and appropriate read-only/disabled gating.
- [ ] Repeat inside Popover content, including Escape interactions, and document platform-specific findings with reproducible steps.

Baseline: commit `38c2ea8`, GPUI Kit/Base 0.7.0 and GPUI 0.3.7. Where a browser comparison is needed, use Kumo commit `3fd5b648df578cb1ba214dedd30f475009f6a668`.
