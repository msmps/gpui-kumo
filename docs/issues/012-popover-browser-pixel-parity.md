# KUMO-012: Popover: complete browser/native pixel comparisons

Status: Open

GitHub issue: Pending publication

## Problem

The pinned comparison establishes geometry and behavior, not whole-image pixel equivalence. Native arrow seams were inspected, but font rendering, shadow rasterization, colors and corner collision presentation still need controlled visual comparisons. The selected GPUI 0.3.7 supports spread and the implementation authors both negative-spread layers; the previous missing-spread claim was corrected on 2026-10-02 after checking the installed source against its crate archive.

## Evidence

- [Popover validation and limits](../popover-validation.md)
- [Popover implementation](../../crates/gpui-kumo/src/popover.rs)
- [Popover tests](../../crates/gpui-kumo/src/popover_tests.rs)
- [Arrow geometry implementation](../../crates/gpui-kumo/src/popover_arrow.rs)

## Acceptance criteria

- [ ] Capture the same Popup content, font profile, dimensions, viewport and device scale in native and pinned browser fixtures.
- [ ] Compare light/dark outline placement, layered shadow appearance, text, all arrow orientations and corner clamping.
- [ ] Measure and classify paint differences; implement reasonable fixes or explicitly accept native deviations with evidence.
- [ ] Preserve source attribution and make the capture/comparison workflow reproducible.

## Scope

Scale/exit motion and stress geometry have separate follow-ups. Existing side-flipping evidence remains valid.

Baseline: commit `38c2ea8`, GPUI Kit/Base 0.7.0 and GPUI 0.3.7. Where a browser comparison is needed, use Kumo commit `3fd5b648df578cb1ba214dedd30f475009f6a668`.
