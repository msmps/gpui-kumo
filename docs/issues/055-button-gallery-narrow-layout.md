# KUMO-055: Button gallery narrow composition

Status: Resolved

GitHub issue: https://github.com/msmps/gpui-kumo/issues/40

Actual #39 owner captures showed existing counter and fixed variant-column overflow at520px. Final variant review also exposed the long-button size example sharing an unwrapped row. Pinned Kumo Button explicitly uses `w-max shrink-0`; the library's content width must remain authoritative. Fix caller/gallery composition, not Button sizing or truncation.

Acceptance:

- [x] Activation actions/counter remain visible while loading and restored, including narrow wraps.
- [x] Preserve wide comparison layout; narrow variants show labelled Enabled/Disabled/Loading rows. Control IDs, state, focus and callbacks remain stable.
- [x] Size/icon/long-label rows wrap with full source-sized controls; actual native long-control bounds fit the panel.
- [x] Use Kumo Text for readable typed counter feedback; pointer/Space/Enter update once and unavailable pointer does not update.
- [x] Required Rust/example gates; actual native owner/bounds/captures in light/dark1040/520; independent alignment/corner/layer review.

Implementation uses existing dimensions/spacing,920px threshold derived from the808px wide row and112px combined padding, and existing160px caption width. No library Button/API/style change. [Before captures](../evidence/linux-busy-state/before-narrow-dark.png) and [joint acceptance/results](../linux-busy-state-validation.md) accompany the #39 checkpoint. Continue #26 Pagination parts afterward.

Resolved in the joint Busy/narrow-gallery checkpoint; [validation and evidence](../linux-busy-state-validation.md).166 workspace tests/nine doctests plus required Rust/vendor gates and actual four-combination native probes passed. No OS speech/other-platform claim.

Published joint checkpoint: `e1c38b832030605601a256065264dfd087b53a4b`; remote work ref/tree verified. No CI workflow/runs.
