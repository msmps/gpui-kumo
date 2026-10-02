# KUMO-021: Tooltip: match Kumo motion and grouped instant switching

GitHub issue: https://github.com/msmps/gpui-kumo/issues/6

Status: Open

## Problem

Tooltip currently appears/disappears without Kumo's 150ms opacity/scale motion. TooltipProvider supports Escape dismissal but not Kumo's grouped instantaneous switching. Core delay/cancellation, hover bridge, keyboard focus and continuous arrow/body join already work.

Category: **Missing behaviour**.

## Acceptance criteria

- [ ] Inspect pinned Tooltip/Provider and matching Base UI source to define animation curves, group switching/reset timing and interruption semantics before implementation.
- [ ] Implement source motion with a documented native reduced-motion policy; close/unmount cancellation must not retain entities or steal focus.
- [ ] Implement shared Provider timing without duplicating per-tooltip disclosure ownership; exercise repeated triggers and isolated providers.
- [ ] Test ordinary delayed opening versus rapid grouped switching, provider reset, Escape, disabled/unmounted triggers and rapid reversal.
- [ ] Capture meaningful motion/state evidence in both themes; preserve arrow/body continuity, placement, corner edges and layering. Re-run existing hover/focus/once-only activation regressions.

## Evidence and baseline

- [tooltip-validation.md](https://github.com/msmps/gpui-kumo/blob/work/docs/tooltip-validation.md)
- [Pinned Kumo source](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/tooltip/tooltip.tsx)

Baseline: Kumo `3fd5b648df578cb1ba214dedd30f475009f6a668`; GPUI Kit/Base 0.7.0; GPUI snapshot family 0.3.7. Current implementation checkpoint: `d07646f8d24acf72248f954a03c8da056d84848d`. Keep dependency versions pinned; do not claim full parity from compilation or screenshots alone.

