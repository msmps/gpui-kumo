> Historical snapshot; current status is in [the active audit](../issue-audit-2026-10-03.md) and [progress](../port-progress.md).

# Migration issue audit — 2026-10-03

Audited `work` at33ab7668f71ec36359164dc34ceed0e6ce8233b2. GitHub REST inventory used `--paginate` with `state=all&per_page=100`:42 issues,37 open/five closed. Public exports, gallery modules, tests, acceptance documents/local-ID mapping, recorded native captures/results and ancestor commits were compared. Independent reviewer examined seven implemented families and five closed repairs. Recorded historical validation is distinguished from the macOS tests reproduced during this session.

| GitHub | Local ID | Classification and remaining work |
| --- | --- | --- |
| #1 | KUMO-007 | Outstanding OS IME validation; native editing tests do not establish OS composition |
| #2–#4 | KUMO-014–016 | Outstanding spoken Linux/VoiceOver/Windows acceptance; platform metadata is separate |
| #5 | KUMO-019 | Partial LayerCard clipping, specific GPUI arbitrary-descendant rounded clipping limit |
| #6 | KUMO-021 | Partial Tooltip motion/provider behavior |
| #7 | KUMO-022 | Partial Tooltip rich content/trigger composition |
| #8 | KUMO-023 | Partial contextual help/Checkbox.Item mixed presentation |
| #9 | KUMO-024 | Leading actions published; multiple editors/rich parts remain |
| #10 | KUMO-025 | Incomplete pinned browser comparison for27 working families; historical18-family baseline |
| #11 | KUMO-026 | Unstarted Autocomplete |
| #12 | KUMO-027 | Published Breadcrumbs core; opacity motion/root composition/styling and browser/spoken acceptance remain |
| #13–#14 | KUMO-028–029 | Unstarted ClipboardText/CodeHighlighted |
| #15 | KUMO-030 | Published Collapsible core; motion/custom trigger/control association remain. Authored expansion repaired by#41 |
| #16–#21 | KUMO-031–036 | Unstarted Combobox/CommandPalette/DatePicker/Dialog/Dropdown/Grid |
| #22 | KUMO-037 | Published InlineCopyText core; copy/check transitions/live-region capability/speech and clipboard delivery remain |
| #23 | KUMO-038 | Published InputArea core; resize/scrollbar/rich slots/multiline accessible text ranges remain |
| #24 | KUMO-039 | Unstarted LayerDialog |
| #25 | KUMO-040 | Published Meter core; track/indicator customization and browser/spoken/preference-delivery acceptance remain |
| #26 | KUMO-041 | Published Input/Simple/Info/Separator/PageSize. Missing cloud candidate reported174 tests; not present in local/remote branches. User authorizes recreation; Dropdown and fidelity remain |
| #27 | KUMO-042 | Published Select groups/custom values/nested overlay/lifetime integration; placement/trigger/rich help and platform/browser fidelity remain |
| #28 | KUMO-043 | Published SensitiveInput core; rich slots/localization/hover/opacity and clipboard delivery remain |
| #29 | KUMO-044 | SkeletonLine implemented, required browser/preference-delivery validation outstanding; remain open until preserved bounded followups exist |
| #30–#35 | KUMO-045–050 | Unstarted Table/TableOfContents/Tabs/TagInput/Toast/Toolbar |
| #36 | KUMO-051 | Complete recorded disabled-state repair4144c60; exact guard and actual state/event/native evidence present |
| #37 | KUMO-052 | Complete recorded readable Label repair7c496e0;143 labels,0 unnamed/changed bounds after repair |
| #38 | KUMO-053 | Rich highlight font defect awaiting reproduction/repair |
| #39–#40 | KUMO-054–055 | Complete recorded Busy/narrow Button repairs e1c38b8; native owner/action and containment evidence present |
| #41 | KUMO-056 | Complete recorded authored expansion repair33ab766; optional state and exact events plus native transitions present |
| #42 | KUMO-057 | Narrow foundation overflow, local repair/visual gate in progress |
| #43 | KUMO-058 | Complete: #45 clock correction85871b7 and repeated full runs37115645935/37116002411 pass; see animation-clock evidence and resolution comment |
| #44 | KUMO-059 | Complete: test-only native shortcut repair published744354a; local and successor CI regressions pass |
| #45 | KUMO-060 | Complete: narrow correction85871b7,170-test full runs37115645935/37116002411, preserved assertions and documented downstream/removal obligations |

No family closure justified from rendering/count alone. No audit-supported reopening or duplicate closure found. Existing five resolution checklists reconciled against their published evidence, without claiming new platform measurements. GitHub remains the queue; local acceptance documents retain detailed contracts.

Independently reproduced: baseline169 library tests with three failures; repaired native shortcuts pass169 plus foundation gallery regression and nine doctests. Workspace formatting/Clippy/build/default-gallery gate passed; exact adapter12default/14all-feature tests and warning-denied lint passed after approved locked dependency download. Native foundation final matrix remains incomplete; code checkpoint744354a is published, #44 closed, first remote CI run37114282490 is in progress. No CI runs existed at audit baseline.

Cloud continuation subsequently inventoried all 44 issues with `state=all&per_page=100` (one full result set, below the page limit) at c7fd554. #43 now has successful remote acceptance in run37114390570, including its actual job log; the earlier run was cancelled. No other new closure is justified by that CI result. [Continuation evidence](../evidence/cloud-continuation-2026-10-03/README.md) separates the remote pass from the measured Linux dependency/network blocker.
