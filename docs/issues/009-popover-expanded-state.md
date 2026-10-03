# KUMO-009: Popover: export expanded/collapsed state through Linux AT-SPI

Status: Resolved by #41

GitHub issue: Pending publication

## Problem

GPUI aria_expanded metadata is set and tested, but accesskit_atspi_common 0.19.1 NodeWrapper::state omits expanded/collapsed mapping. Live AT-SPI checks confirm the missing state flag. The updated expanded/collapsed dialog description is a verified fallback, not the flag.

## Evidence

- [Popover validation and limits](../popover-validation.md)
- [Popover implementation](../../crates/gpui-kumo/src/popover.rs)
- [Popover tests](../../crates/gpui-kumo/src/popover_tests.rs)

## Acceptance criteria

- [ ] Provide or adopt an upstream adapter fix, or a narrowly maintained dependency patch, for expanded/collapsed mapping.
- [ ] Inspect live AT-SPI state across pointer, keyboard, accessible and programmatic opening/closing.
- [ ] Assert state updates on disabled dismissal and nested-panel dismissal without breaking single activation or focus restoration.
- [ ] Document versions and the remaining role of the descriptive fallback.

Baseline: commit `38c2ea8`, GPUI Kit/Base 0.7.0 and GPUI 0.3.7. Where a browser comparison is needed, use Kumo commit `3fd5b648df578cb1ba214dedd30f475009f6a668`.

Published33ab766 repairs authored optional expansion in the exact adapter. See [acceptance and evidence](../linux-expansion-state-validation.md); speech and control association are separate.
