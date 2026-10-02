# KUMO-011: Popover: validate scrolling, resizing and extreme collision geometry

Status: In progress

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

Selected 2026-10-02 after foundation checkpoint `d7224c0`. Start with rendered resize/corner/neither-side-fits and scrolling reachability tests; inspect native output and pinned browser fitting before resolving the issue. Long content must remain reachable through real input, not merely fit an asserted outer rectangle.

## 2026-10-02 checkpoint

Found a concrete Base 0.7.0 fitting defect: its side selection excludes the requested offset. With a 400px viewport, trigger y=220–256, popup height 132px and 8px viewport margin, the bottom side has 136px available but needs 140px including the gap. Base selected bottom and clamped the gap to 4px even though top fits. The new rendered regression failed before the repair. The private adapter now dilates the exclusion bounds by the requested gap and passes zero offset; it preserves center alignment and keeps arrows attached to the actual trigger. The regression now selects top with the full 8px gap.

Rendered/input evidence passes for four open-window sizes (640×480, 320×240, 160×120, 800×600), a 600px-wide long popup, viewport fitting, wheel scrolling to and activating its final action at each size, and retained open state. A separate real wheel event scrolls the trigger's ancestor by 40px; the popup follows with its 8px gap and the moved trigger still dismisses it. All 36 workspace tests pass, including these three regressions.

The new `popover_stress` example exercises resizing, long content, a final-action counter and theme switching. On macOS, earlier native checks reached the final action in both themes and after resizing, incrementing its counter twice. However, after resized Escape dismissal the AX dialog disappeared while a subsequent screenshot still showed the popup; an owner theme change cleared it. A repeat native session also showed unreliable pointer/keyboard updates. This is unresolved native evidence, not an established cause or a verified paint pass. Investigate input delivery and frame presentation before changing notification policy or claiming dismissal parity. All launched test processes were confirmed stopped with a process query after this session.

Formatting, warning-denied Clippy with all workspace targets/features, and gallery example builds pass. Corners, neither-side-fits browser comparison, arrow/seam/gutter checks and nested resize routing remain pending. Keep this issue In progress.

Continuation after `75cb0b7`: native resize/dismissal was repeated successfully in light and dark appearance, including scrolling to and activating the last action. The screenshots cleared the popup and showed counters 1 and 2 with the trigger's restored focus ring. The earlier stale image did not recur, so its cause remains unclassified; no speculative refresh workaround was added. A raw Escape regression now verifies surface metadata and painted quads disappear after resizing without calling the test helper's forced `refresh`. This protects cached painting, but does not reproduce the macOS frame scheduler.

Rendered checks now cover eight corner/side combinations and a central 224px-high popup where neither vertical side fits. Base chooses the larger available side and clamps across the trigger; pointer input over that covered trigger leaves the popup open, and Escape restores trigger focus. This is a tested native fitting policy, not a completed pinned-browser comparison. Nested open panels are also resized to 320×240 before checking bounded painted surfaces, retained child focus and nearest-panel Escape dismissal. Arrow paint/seam/gutter and browser policy comparisons remain pending. Native test processes were confirmed stopped after these checks.
