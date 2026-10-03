# KUMO-024: InputGroup: support leading actions, multiple editors and rich parts

GitHub issue: https://github.com/msmps/gpui-kumo/issues/9

Status: Open

## Problem

Retained editing, addons, suffix, compact actions, individual/hybrid joins and contextual help are implemented. The current builder puts one editor first, followed by direct actions; leading direct buttons, multiple-editor compositions and arbitrary rich addon children remain outside its supported contract.

Category: **Missing API/composition**.

## Acceptance criteria

- [ ] Inspect pinned Root partitioning, Input, Button, Addon and consumer demos; distinguish source individual ordering from hybrid grouping rules.
- [ ] Define a typed native part API preserving existing consumers, stable IDs and caller-owned editor entities; never recreate persistent editing entities while rendering.
- [ ] Implement relevant leading/multiple-editor and rich-addon capabilities against that source contract, with explicit availability and focus ownership.
- [ ] Test editor value/Unicode/selection retention, independent focus/Tab order, reorder stability, once-only callbacks and disabled paths through every input method.
- [ ] Verify four sizes, source seams/radii/spacing, focus border layering, long rich parts and narrow overflow in both themes; inspect actual painted bounds and middle borders.
- [ ] Preserve native selection/clipboard/IME behavior; real OS IME acceptance is tracked separately, and arbitrary rounded-descendant clipping remains a foundation limitation.

## Evidence and baseline

- [input-group-validation.md](https://github.com/msmps/gpui-kumo/blob/work/docs/input-group-validation.md)
- [Pinned Kumo source](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/input-group/input-group.tsx)

Baseline: Kumo `3fd5b648df578cb1ba214dedd30f475009f6a668`; GPUI Kit/Base 0.7.0; GPUI snapshot family 0.3.7. Current implementation checkpoint: `d07646f8d24acf72248f954a03c8da056d84848d`. Keep dependency versions pinned; do not claim full parity from compilation or screenshots alone.


Leading direct actions and fixed/native-aligned editor composition now implemented;158 tests/nine doctests and Rust gates pass; review width/order repairs and both-theme1040/520 evidence in [InputGroup acceptance](../input-group-validation.md). Multiple editors/rich parts remain open.
