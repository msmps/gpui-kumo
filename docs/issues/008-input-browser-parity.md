# KUMO-008: Input: complete pinned browser and Field visual comparisons

Status: Open

GitHub issue: Pending publication

## Problem

Input/Field native geometry and editing are tested, but browser paint parity remains unmeasured. The disabled foreground uses the documented native subtle role because the inspected upstream disabled text token is undefined; selection/caret are also native policies.

## Evidence

- [Native Input contract](../kumo-component-recipes.md#native-input-contract)
- [Input implementation](../../crates/gpui-kumo/src/input.rs)
- [Input tests](../../crates/gpui-kumo/src/input_tests.rs)

## Acceptance criteria

- [ ] Compare all four sizes, label/help/error layouts, placeholder/entered text and focus/invalid/read-only/disabled combinations in light/dark modes.
- [ ] Check the generated-CSS invalid/focus ring precedence and 1px/1.5px geometry against native paint.
- [ ] Control font, device scale, width and long text; compare baseline, scrolling, selection and caret behavior.
- [ ] Record the undefined upstream disabled-foreground token and native policy rather than attributing an invented token to Kumo.
- [ ] Commit reproducible evidence and address discovered defects or document approved differences.

Baseline: commit `38c2ea8`, GPUI Kit/Base 0.7.0 and GPUI 0.3.7. Where a browser comparison is needed, use Kumo commit `3fd5b648df578cb1ba214dedd30f475009f6a668`.
