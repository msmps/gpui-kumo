# Authored Linux busy-state export

Tracker [#39/KUMO-054](issues/054-linux-busy-state.md), selected after #36 published4144c60. Exact adapter source/archive/checksum/license/root consumer/removal policy remains [the pinned patch](linux-disabled-state-patch.md). No dependency version, Kumo presentation, role, Base activation, event or focus ownership change.

Installed AccessKit0.24.1 has is_busy/set_busy/clear_busy; consumer0.38.0 forwards node flags. Pinned atspi_common0.19.1 had no Busy mapping. Native #36 evidence measured seven Loading-description Buttons with Busy false; their disabled availability and Click guards were independently correct. A new actual Adapter regression fails before the mapping on enabled Button Busy.

| Area | Acceptance / verification |
| --- | --- |
| Busy flag | Map authored flag to AT-SPI Busy and clear when owner clears it; actual Adapter and native queries |
| Availability | Busy alone leaves Button/Switch enabled; busy+disabled preserves existing unavailable/ReadOnly policy. Compare entire state sets, only Busy may change |
| Notifications | Actual Adapter emits exactly one Busy true/false event on flag updates; preserve #36 Enabled/Sensitive notifications |
| Native | Seven loading Buttons Busy while Enabled/Sensitive/Click absent, both themes1040/520. Real gallery Start/Stop loading owner updates expose/clear Busy and restore enabled Click; captures verify actual themes/paint |
| Appearance | Inspect centres/baselines/spacing, corners/fills/borders/rings/layers on loading/restored samples in both themes and widths; reuse verified source styling |
| Input | Existing disabled/busy pointer/Space/Enter and focus/Tab policy unaffected; rendered Button regression and strict native Pagination interaction/edit/clipboard checks |
| N/A | New variants/sizes, public control API, duplicate state/entities, IME machinery, live announcements or platform speech; no semantic fabrication |
| Gate | Failing baseline, upstream default/all-feature tests, workspace formatting/tests/doctests/warning-denied Clippy/build/native build, explicit vendor fmt/Clippy, actual native probes, independent review |

Implementation: three production lines map is_busy to State::Busy separately from availability. One Button/Switch enabled/disabled set/clear state/event regression. Private shared callback fixture extracted only because existing availability and new busy tests now duplicate concrete event capture. No broader adapter rewrite. Independent source review found no blocker. Final native/required results recorded after gates finish; Linux metadata does not establish Orca speech/Wayland/macOS/Windows workflows.

Next planned #26 Pagination dropdown/PageSize, with #38 rich highlight font resolution still in the backlog. Existing working coverage remains27/43,16 unported, with Pagination partial.

Native review found an existing narrow gallery defect, tracked jointly as #40/KUMO-055: activation counter, fixed variant columns and the long-label size row overflowed right. Inspected pinned Button source `w-max shrink-0`; preserve source intrinsic width. Gallery now wraps activation and size rows; below the derived920px wide-table threshold it stacks explicitly labelled state rows with the same control IDs and160px caption width. Counter uses existing Kumo Text Secondary/Base so its complete native feedback can verify actual activation once. No design-system styling/API/state-owner change. Before narrow captures are preserved alongside final results.

The committed owner probe scopes AT-SPI and X11 lookup to its launched PID (installed get_process_id and pinned _NET_WM_PID verified), checks complete counter feedback for pointer/Space/Enter and disabled-pointer rejection, and measures all three long variant controls and the long-label size example against actual panel bounds. Captures include loading/restored, long variants and sizes in all four combinations. This is observable native output, not an assertion of the responsive branch condition.

Final gate:166 workspace tests/nine doctests, formatting, warning-denied all-target/all-feature Clippy/build and default native gallery build passed. Exact vendored adapter11default/13all-feature tests, explicit changed-vendor-file formatting and standalone warning-denied all-target/all-feature Clippy passed. Baseline failed before the three-line mapping; only Busy changes in complete state sets and actual transition events. Logs `/tmp/busy-state-*` and `/tmp/busy-gallery-complete-*`; existing GPUI profiler deprecation and AT-SPI signature warnings remain visible. No CI workflow is configured.

Both committed native probes passed Light/Dark1040/520: strict Pagination `--require-busy-state` plus actual owner Start/Stop loading, complete readable activation count, once-only pointer/Space/Enter, unavailable pointer rejection and actual long-control bounds. [Raw results and before/final captures](evidence/linux-busy-state/). Primary visual review confirms source-sized Button rows fit with unchanged wide layout; checked control centres/baselines, spacing, outer corners and fill/border/ring layers. Separate pre-existing foundation swatch-column overflow remains a gallery follow-up; it does not affect these Button bounds and is not represented as fixed.

Independent final source/capture review found no Button blocker: fresh light520 confirms wrapping, stable IDs/state and clean centres/baselines/corners/layers. It independently confirmed the separate narrow foundation swatch overflow.
