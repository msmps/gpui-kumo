# KUMO-010: Popover: evaluate scale and exit-motion parity with Kumo

Status: Open

GitHub issue: Pending publication

## Problem

The browser modern Popup animates 90% scale plus opacity over 150ms on enter/exit. Native Popover uses a tested 150ms opening fade and immediate dismissal. GPUI’s selected public transforms apply to SVG painting, not a whole interactive subtree; immediate dismissal avoids retained invisible controls.

## Evidence

- [Popover validation and limits](../popover-validation.md)
- [Popover implementation](../../crates/gpui-kumo/src/popover.rs)
- [Popover tests](../../crates/gpui-kumo/src/popover_tests.rs)

## Acceptance criteria

- [ ] Evaluate a bounded GPUI transform/animation solution for the complete popup or document a justified accepted native motion contract.
- [ ] If implementing exit motion, immediately remove logical activation/focus/accessibility targets while retaining only the necessary presentation.
- [ ] Handle rapid reversals, parent/child dismissal, interrupted opening and reduced motion without stale state or orphaned deferred registrations.
- [ ] Verify intermediate/final native paint and focus/action lifecycle; keep the independent nested-fade regression passing.

Baseline: commit `38c2ea8`, GPUI Kit/Base 0.7.0 and GPUI 0.3.7. Where a browser comparison is needed, use Kumo commit `3fd5b648df578cb1ba214dedd30f475009f6a668`.
