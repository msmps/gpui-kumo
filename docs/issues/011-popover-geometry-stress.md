# KUMO-011: Popover: validate scrolling, resizing and extreme collision geometry

Status: Open

GitHub issue: Pending publication

## Problem

All four opposite-side flips, an 8px gap, a moving trigger and nested dismissal have passing evidence. Remaining geometry acceptance work concerns resizing and scrolling, oversized/long content, extreme narrow viewports and corner-clamped arrows; these are unverified combinations, not a claim that flipping is broken.

## Evidence

- [Popover validation and limits](../popover-validation.md)
- [Popover implementation](../../crates/gpui-kumo/src/popover.rs)
- [Popover tests](../../crates/gpui-kumo/src/popover_tests.rs)
- [Runnable native edge example](../../apps/gallery/examples/popover_edges.rs)
- [Four-edge screenshots and measured bounds](../evidence/popover-flips.json)

## Acceptance criteria

- [ ] Exercise open-popup window resizing, trigger scrolling/movement, and long vertically scrolling content with native captures and input checks.
- [ ] Cover corners and cases where neither preferred nor opposite side fits; compare native fitting policy with the pinned browser.
- [ ] Check arrow direction/seam/clamping, the documented omission below 36px, outline/shadow clipping and child focus-ring gutters.
- [ ] Verify inside/outside input routing and reachable content during resizing, including nested panels.
- [ ] Determine whether the current next-frame trigger-bound synchronization is sufficient; add meaningful regression checks for any defects.

Baseline: commit `38c2ea8`, GPUI Kit/Base 0.7.0 and GPUI 0.3.7. Where a browser comparison is needed, use Kumo commit `3fd5b648df578cb1ba214dedd30f475009f6a668`.
