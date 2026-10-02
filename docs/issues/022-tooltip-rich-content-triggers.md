# KUMO-022: Tooltip: support rich content and native trigger composition

GitHub issue: https://github.com/msmps/gpui-kumo/issues/7

Status: Open

## Problem

Tooltip currently accepts text and a Kumo Button factory. Kumo supports richer noninteractive content and trigger composition. Label/Field/Input/InputGroup help is already implemented and should not be reopened as missing work.

Category: **Missing API/composition**.

## Acceptance criteria

- [ ] Inspect supported pinned Tooltip content/trigger APIs; exclude deprecated asChild and define idiomatic native equivalents.
- [ ] Add decorative rich-content composition preserving source typography, padding and complete accessible content/name without focusable popup children.
- [ ] Support suitable native triggers using installed Base behavior where available; retain consumer focus, bounds, activation and availability rather than adding duplicate listeners.
- [ ] Test moving/resized triggers, pointer passage, focus/Escape, callbacks exactly once, disabled triggers, repeated instances and cleanup.
- [ ] Cover long Unicode content, empty/oversized content and nested overlays; inspect light/dark/narrow alignment, clipping and continuous arrow/body join.
- [ ] Document native trigger-to-tooltip semantic limitations; screen-reader acceptance belongs to the platform issues.

## Evidence and baseline

- [tooltip-validation.md](https://github.com/msmps/gpui-kumo/blob/work/docs/tooltip-validation.md)
- [field-validation.md](https://github.com/msmps/gpui-kumo/blob/work/docs/field-validation.md)
- [Pinned Kumo source](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/tooltip/tooltip.tsx)

Baseline: Kumo `3fd5b648df578cb1ba214dedd30f475009f6a668`; GPUI Kit/Base 0.7.0; GPUI snapshot family 0.3.7. Current implementation checkpoint: `d07646f8d24acf72248f954a03c8da056d84848d`. Keep dependency versions pinned; do not claim full parity from compilation or screenshots alone.

