# KUMO-054: Export authored busy state on Linux

Status: Resolved

GitHub issue: https://github.com/msmps/gpui-kumo/issues/39

The six loading Button samples author AccessKit busy plus disabled and a Loading description. Pinned atspi_common0.19.1 does not map busy in NodeWrapper::state; actual native Busy-based fixture discovery fails. Use existing Loading descriptions to validate #36 independently. Switch transitions also author busy; busy alone must not imply disabled.

Acceptance: verify actual flags versus exported Busy; smallest pinned mapping; observable Busy set/clear state/event regressions for enabled and disabled Button/Switch; native loading metadata/action guard both themes1040/520; required workspace/vendor gates, appearance review and independent review. Preserve readonly/disabled mapping and all Kumo/Base behavior. Speech/other-platform workflows remain separate.

Next selected after #36, then #26 Pagination parts. Exact source/checksum/root consumer policy follows [the disabled-state patch](../linux-disabled-state-patch.md).

Baseline actual Adapter regression fails for enabled Button Busy after set_busy. Added role matrix covers Button/Switch, enabled/disabled, state set/clear and emitted notifications; only Busy is allowed to change. Shared private state-event fixture extracted from existing concrete test duplication. Production candidate maps is_busy to State::Busy using installed getter.

Resolved in the joint Busy/narrow-gallery checkpoint; [validation and evidence](../linux-busy-state-validation.md).166 workspace tests/nine doctests plus required Rust/vendor gates and actual four-combination native probes passed. No OS speech/other-platform claim.

Published joint checkpoint: `e1c38b832030605601a256065264dfd087b53a4b`; remote work ref/tree verified. No CI workflow/runs.
