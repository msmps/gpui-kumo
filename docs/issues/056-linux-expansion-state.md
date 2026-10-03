# KUMO-056: Authored Linux expansion state

Status: Resolved

GitHub issue: https://github.com/msmps/gpui-kumo/issues/41

Native PageSize opens its popup but Expanded is absent. Pinned atspi_common0.19.1 has no Expanded/Expandable mapping despite authored AccessKit expansion. Inspect actual disclosure roles, authored absence/false/true semantics and installed APIs before the smallest exact-version correction. Preserve Busy/disabled/readonly behavior. Actual adapter full state sets and transition events plus native opening/Escape/Tab/owner-disable in both themes1040/520 and required Rust/vendor gates/review are acceptance. No OS speech claim.

Selected after PageSize checkpoint before dropdown Controls; shared Select/Popover/Collapsible disclosure metadata has higher value than continuing on an incorrect native expansion claim. ComboBox string values lack Text/Value interfaces unless text ranges/numeric values exist; keep separate in #27, never invent a numeric value from string content.

Active after PageSize `f9d592edf2d2ae82340c49b842c202566bccf8f3`. [Matrix/source APIs](../linux-expansion-state-validation.md). Exact Adapter None/false/true/clear regression failed before the six-line mapping and passes after it.12default/14all-feature vendor tests,169workspace tests/nine doctests, required Rust/vendor gates and strict native Light/Dark1040/520 probes pass. [Recorded evidence](../evidence/linux-expansion-state/README.md). No speech/other-platform claim.
