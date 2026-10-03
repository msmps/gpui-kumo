# Authored Linux expansion export

Tracker [#41](https://github.com/msmps/gpui-kumo/issues/41), selected after PageSize f9d592e. Existing exact0.19.1 source/archive/checksum/license/root consumer/removal policy remains [the pinned patch](linux-disabled-state-patch.md). No dependency version chase or Kumo/Base presentation/state/focus ownership change.

Source inspected: AccessKit0.24.1 Node is_expanded/set_expanded/clear_expanded; consumer0.38.0 public data() exposes actual node; atspi-common State::Expandable/Expanded. Pinned NodeWrapper::state has neither mapping. Actual PageSize popup opens and selected options are readable but Expanded is absent. Installed Base Select authors aria_expanded; Kumo Button/Popover and Collapsible also author expansion. Ordinary Buttons have no expansion property.

| Area | Acceptance before coding |
| --- | --- |
| Tri-state meaning | None exports neither flag; Some(false) Expandable only; Some(true) both; clear restores original state set |
| Independence | ComboBox/Button/TreeItem enabled/disabled cases preserve entire state sets except expansion bits. Authored Busy and availability remain independent; do not fabricate expansion for unmarked nodes |
| Events | Actual Adapter emits precisely changed Expandable/Expanded true/false notifications; false→true→false and removal; no redundant event on equal update |
| Behavior | Actual native Select open/commit/Escape/Tab/disabled-owner close/restoration in both themes1040/520; ordinary Button unmarked; Collapsible/Popover authored expansion queries where possible. No new activation behavior |
| Presentation | Existing PageSize gallery/captures both themes/widths; explicitly check centres/baselines/offsets/gaps/padding/corners/fills/borders/rings/layers, no source styling mutation |
| Gate | Failing actual adapter baseline, upstream default/all-feature regressions, workspace fmt/tests/doctests/warning-denied lint/build/default example, vendor fmt/Clippy, strict native probe, skeptical review, commit/push |
| N/A | New public control API, duplicate entities/listeners, motion, focus traps, live announcements or speech/IME; string-value interfaces remain #27/other-platform gate |

Next after checkpoint: #26 retained dropdown Controls. Coverage27/43 working,16 unported; Pagination full family remains open.

Baseline actual adapter test fails after successfully compiling: collapsed Button lacks Expandable (Busy/Enabled/Focusable/Sensitive/Showing/Visible otherwise identical). Six production lines map the optional property independently.12default/14all-feature upstream tests pass, with whole-state equality and exactly one transition/no equal-update duplicate across enabled/disabled Button/ComboBox/TreeItem. Required workspace169tests/nine doctests, fmt, warning-denied all-target/all-feature lint/build and default native build plus changed-vendor2024-edition fmt/standalone Clippy pass (`/tmp/expansion-state-*`).

Strict native probe now refreshes each queried node and ensures both retained trigger and owner availability Button are painted before open/disable; confirms actual popup stays open immediately before owner action. This distinguishes source owner-disable cleanup from outside-click/scroll/offscreen-action effects. Earlier strict probe completed Light1040 then exited1 in Dark1040; its overwritten trace is unavailable, so no specific failure cause is claimed. Corrected final run/evidence is recorded independently. Existing AT-SPI signature/dependency deprecation warnings remain visible.

A second early strict run failed native Collapsible false→true with a mismatch between the retained accessible bounds and the painted scrolled disclosure; `/tmp/expansion-state-native.log` preserves the actual assertion and its capture shows the requested disclosure offscreen. The final probe reacquires named/role identities after scrolling, refreshes state, and rechecks settled bounds before pointer activation. The final public `--require-expansion-state` run passes Light/Dark at1040/520: Select plus Collapsible/disabled-Collapsible/Popover expansion/focus, ordinary Button absence and owner-close checks. All24 fresh captures and raw state queries are in [evidence](validation-fixtures.md); no GPUI production cache rewrite or weakened expansion assertion was made.

Final visual review inspected both themes and widths for alignment, wrapped labels, check/caret placement, corners, focus rings and overlay layers. Source presentation remains unchanged. The pre-existing rich-highlight font discrepancy remains #38; narrow foundation examples remain #42. Next #26 dropdown Controls. No native speech, macOS/Windows or IME validation is claimed.

Independent final review found no blocker across all four combinations and the repaired owner-disable fixture. Primary review independently inspected final narrow disclosure and Popover captures. No source styling workaround was introduced.
