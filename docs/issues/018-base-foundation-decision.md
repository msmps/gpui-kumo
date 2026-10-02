# KUMO-018: Components: record the Base dependency decision after the first slice

Status: Resolved — decision and measurement evidence recorded 2026-10-02.

GitHub issue: Pending publication

## Problem

The Base assessment still describes a proposed approach. The first Button/Input/Popover slice now has evidence showing useful reuse and specific accessibility/motion constraints. Implementation-plan step 5 asks for build/dependency cost and an explicit retain/adapt/replace decision before expanding the catalog.

## Evidence

- [Proposed Base assessment](../gpui-base-assessment.md)
- [Evaluation acceptance criteria](../implementation-plan.md)
- [Measured Popover findings](../popover-validation.md)
- [Workspace manifest](../../Cargo.toml)

## Acceptance criteria

- [x] Record which Base behaviors are retained, adapted or replaced for each implemented component and why.
- [x] Measure representative clean/incremental build cost and the selected normal dependency graph; state toolchain/platform/resources.
- [x] Identify the maintained patch/bridge/upgrade policy for the documented accessibility and motion limitations.
- [x] Update the assessment to an explicit dependency decision and link remaining issues without claiming unverified platform acceptance.

Baseline: commit `38c2ea8`, GPUI Kit/Base 0.7.0 and GPUI 0.3.7. Where a browser comparison is needed, use Kumo commit `3fd5b648df578cb1ba214dedd30f475009f6a668`.

Resolution: [explicit decision](../gpui-base-assessment.md#decision-after-the-first-slice), [measured build/graph costs](../port-progress.md#measured-baseline). Retain Base selectively; private hooks repair state metadata without a fork; fix Popover parent reassignment locally. Linux adapters, incoming reversed selection, motion and platform acceptance remain separately tracked. The related continuation commit can be located in this file's Git history.
