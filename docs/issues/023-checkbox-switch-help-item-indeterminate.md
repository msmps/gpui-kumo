# KUMO-023: Checkbox/Switch: complete contextual help and Checkbox.Item mixed presentation

GitHub issue: https://github.com/msmps/gpui-kumo/issues/8

Status: Open

## Problem

Single Checkbox rich labels and cancellable Checkbox.Item callbacks are implemented. Remaining form-control capabilities include Checkbox/Switch labelTooltip composition and explicit Checkbox.Item indeterminate presentation. Item labels are string-only in the pinned source; group selected values remain authoritative.

Category: **Missing API/state**.

## Acceptance criteria

- [ ] Inspect pinned Checkbox/Switch contextual-help composition and matching Base UI Item checked/indeterminate behavior; do not invent rich Item labels or an independent checked override.
- [ ] Add independently focusable help through existing Label/Tooltip, outside decorative control content; label click still activates the control once, help activation does not toggle it.
- [ ] Keep help independent of disabled editing where the pinned contract requires it; close old disclosure on hide/remove/replace and verify forward/reverse Tab and Escape.
- [ ] Implement Item indeterminate presentation without duplicating group state. Specify toggle outcome, controlled proposal and item-before-group cancellation behavior against source.
- [ ] Test pointer/Space/Enter, rejected proposals, disabled retained-focus paths, aggregate/external updates and repeated group instances.
- [ ] Review both themes, long/narrow labels, control-first/label-first, optional decoration, actual info glyph/ring paint, icon centres and layered corners. Document platform semantic limits.

## Evidence and baseline

- [checkbox-validation.md](https://github.com/msmps/gpui-kumo/blob/work/docs/checkbox-validation.md)
- [switch-validation.md](https://github.com/msmps/gpui-kumo/blob/work/docs/switch-validation.md)
- [field-validation.md](https://github.com/msmps/gpui-kumo/blob/work/docs/field-validation.md)
- [Pinned Kumo source](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/checkbox/checkbox.tsx)

Baseline: Kumo `3fd5b648df578cb1ba214dedd30f475009f6a668`; GPUI Kit/Base 0.7.0; GPUI snapshot family 0.3.7. Current implementation checkpoint: `d07646f8d24acf72248f954a03c8da056d84848d`. Keep dependency versions pinned; do not claim full parity from compilation or screenshots alone.

