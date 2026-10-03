# KUMO-056: Authored Linux expansion state

Status: Open

GitHub issue: https://github.com/msmps/gpui-kumo/issues/41

Native PageSize opens its popup but Expanded is absent. Pinned atspi_common0.19.1 has no Expanded/Expandable mapping despite authored AccessKit expansion. Inspect actual disclosure roles, authored absence/false/true semantics and installed APIs before the smallest exact-version correction. Preserve Busy/disabled/readonly behavior. Actual adapter full state sets and transition events plus native opening/Escape/Tab/owner-disable in both themes1040/520 and required Rust/vendor gates/review are acceptance. No OS speech claim.

Selected after PageSize checkpoint before dropdown Controls; shared Select/Popover/Collapsible disclosure metadata has higher value than continuing on an incorrect native expansion claim. ComboBox string values lack Text/Value interfaces unless text ranges/numeric values exist; keep separate in #27, never invent a numeric value from string content.
