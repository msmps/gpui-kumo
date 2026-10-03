# Kumo native port progress

## Tabs implementation checkpoint — 2026-10-03

Tabs #32 now has a working retained native core: segmented/underline, base/sm, typed local/controlled selection, manual/automatic keyboard activation, disabled skipping, stable reordered focus and removal recovery, gallery and shared focused preview. Coverage advances to **28/43 working families,15 unported**. [Contract, native evidence and explicit remaining work](tabs-validation.md). #32 stays open for overflow edge controls/drag, sliding indicator/motion, replacement/link composition and full fidelity/platform acceptance; complete that source capability slice next. Select repairs are pushed as4c5fa69, with partial Pagination evidence and popup-width gap retained. Prior checkpoints below are historical.


## Current migration checkpoint — 2026-10-03

Foundations #42 and the reported Text glyph discrepancy #38 are resolved and pushed (f03da05 / 9a2a402); both remote CI gates pass. [Text evidence](evidence/text-fonts/README.md). Two further Select repairs address open-menu Tab/ShiftTab dismissal with consuming host bindings and hovered/open trigger paint. The pinned Rust gate passes177library/1gallery/9doctests and adapter12/14. [Browser and partial native evidence](evidence/pagination/native-browser/README.md) records remaining Pagination matrix and numbered Select popup-width gaps explicitly; #26/#27 remain open.

**Next implementation: Tabs #32.** Coverage remains27/43 working,16 unported until an implemented new family is validated. Preserve the preceding repairs as their own coherent checkpoint; commit and non-force push new implementation milestones regularly. Historical continuation entries below retain provenance and are superseded by this status.


## Managed native continuation — 2026-10-03

Actual latest `work`3eee132 was fetched and the clean checkout fast-forwarded; managed Git/Cargo/HTTPS now work. Full pinned Linux gate passes175library/1gallery/9doctests, workspace/adapter fmt and warning-denied lint, locked all-target/all-feature/default gallery builds and adapter12default/14all-feature tests. Exact pins/patches unchanged. [Raw results](evidence/cloud-native-2026-10-03/README.md).

#42's bounded gallery repair now passes native focused/full-gallery Light/Dark1040/520, painted text/shadow/corner/alignment review and real737/738 resize transitions. [Evidence/scripts](evidence/foundation-layout/linux/README.md). No further recipe change required. Published evidence checkpointf03da05 passes remote CI37119510684; #42 checklist is reconciled and the issue is closed through the GitHub connector. CLI API remains blocked, but connector issue writes are available. #26 native Dropdown/PageSize and pinned browser validation remain next, followed by#38 glyph reproduction. Coverage stays27/43 working,16unported; platform/browser/speech/IME acceptance remains separate.

## Current dropdown checkpoint — 2026-10-03

#26's missing dropdown candidate is recreated and published: implementation 46f919e, test corrections dfbbdc06/d3bfbe96, final source shadow correction 033ccfde78c5fdf1f925fc3e03ba8a9671f45278. Full remote macOS gates [37117625758](https://github.com/msmps/gpui-kumo/actions/runs/37117625758) and [37117805585](https://github.com/msmps/gpui-kumo/actions/runs/37117805585) pass 175 library / 1 gallery / 9 doctests, workspace/adapter formatting and warning-denied lint, locked builds/default gallery, and 12 default / 14 all-feature adapter tests. [Contract and evidence](pagination-dropdown-checkpoint.md).

Typed Input/Dropdown controls, retained controlled page Select, Full+Known+Dropdown-only allocation, activation-time owner revisions, changed totals/options, availability/focus/removal/unmount, and both-theme geometry/border regressions are implemented. Source-derived 42×36 navigation, 214px input / 83px simple, trigger gap 6, 33px numeric rows and the middle Select's retained shadow are reconciled. No versions/vendor patches changed. Coverage remains 27/43 working, 16 unported; implementation is not full fidelity.

Linux tests still stop at missing hdrhistogram metadata; configured proxy:8080 refuses connections despite enforced policy. Native/browser execution remains blocked. #26/#42/#38 stay open. Next: restore managed connectivity, run local locked gate, finish #42 native foundation matrix, then #26 native Light/Dark1040/520 and PageSize/browser matrix, followed by #38 glyph reproduction. Do not repeat the missing-candidate search/recreation or reuse its older native/browser evidence.


## Current checkpoint — 2026-10-03

Branch `work`; implementation85871b7689f987d3cdb035572e9f4b17687523f9 over clock correction9ef600b. Documentation/evidence checkpoint5ea81bb7f8ff6a26adb1e02b0228d3fdac2865a8; this final status update follows it. **#43/#45 resolved:** complete remote runs37115645935 and37116002411 both pass170 library tests,1gallery,9doctests, workspace/exact adapter formatting, warnings-denied Clippy/locked all-target/all-feature/default-gallery builds, adapter12default/14all-feature tests. [Raw outcomes, initial failure and repair](evidence/animation-clock/README.md). Preserve the new narrow GPUI executor-clock patch and its downstream/removal obligations alongside existing patches and exact pins. No assertions weakened. Coverage remains27/43 working families with fidelity gaps;16 unported.

**Blocked native continuation:** managed network policy is now reported enforced, but fresh Git/Cargo/HTTPS probes still cannot connect to `proxy:8080`. Rust1.99/native tools exist under `/workspace/.kumo-setup/activate.sh`; absent cached hdrhistogram metadata and adapter endi1.1.0 block Linux tests before execution. Original checkout stays clean at31d2c54; verified current source snapshot is separate at `/workspace/gpui-kumo-current`. [Runtime/source-recovery evidence](evidence/cloud-continuation-2026-10-03/README.md). No Linux/native/speech/OS IME acceptance claimed.

Next: restore managed proxy connectivity; fetch actual latest work and activate setup; run `bash scripts/check-rust.sh`; finish #42 final Light/Dark1040/520 text/shadow/full-gallery/resize native gate; recover/recreate #26 retained controlled Pagination dropdown and its allocation/owner/focus/lifetime/source-paint acceptance; then #38 rich font discrepancy. Those issues remain open. No recovered Dropdown candidate or new component completion is claimed.

### Earlier repeat failure and candidate (historical)

Repeat CI run [37115097659](https://github.com/msmps/gpui-kumo/actions/runs/37115097659) at documentation-onlye06c866 failed:168 passes/one Switch animation intermediate-color failure. **#43 reopened; #45/KUMO-060 in progress.** Ordinary pinned GPUI duration animations bypassed the executor clock; a40ms host sleep could resume after the150ms transition. Narrow candidate changes only AnimationElement initialization/elapsed/chained restart to the existing executor clock, retains every Switch/Loader paint assertion, adds host-stall and actual chained-rendered-width regressions, and documents root patch obligations. Local workspace/adapter formatting passes; Linux dependency/proxy blocker remains. Remote Rust acceptance and repeat run are pending. #42/#26/#38 remain open.

### Earlier CI acceptance (superseded by the repeat failure)

Cloud continuation inspected actual remote `work` at `c7fd5541cf8627303936eaae07c4e94165c5c870`. **#43 accepted:** successor remote run [37114390570](https://github.com/msmps/gpui-kumo/actions/runs/37114390570) passed the complete pinned macOS gate; the first run was cancelled. See [remote evidence and exact Linux blocker](evidence/cloud-continuation-2026-10-03/README.md). #42/#26/#38 remain open. The cloud host has pinned Rust and native tools under `/workspace/.kumo-setup/activate.sh`; the shell did not activate them automatically. A separate source snapshot restored 636 source/configuration/document blobs, each verified against the remote Git SHA; the stale original checkout remains clean at `31d2c54`. Workspace and adapter formatting pass. Linux tests are blocked before execution by missing cached dependency metadata/package downloads (`hdrhistogram`, adapter `endi 1.1.0`); ordinary Cargo fetch and Git fetch both fail because the configured `proxy:8080` is unreachable. No Linux/native acceptance is claimed. Restore managed proxy connectivity, fetch latest `work`, activate the setup, run `bash scripts/check-rust.sh`, then finish #42 native acceptance and recover/recreate #26.

### Earlier published handoff (historical)

Branch `work`; published code checkpoint `744354a1f4c222920f6bc74dfd4cc70202623c73`, independently confirmed on remote, over baseline33ab766. Subsequent handoff documentation commit reconciles this status. Starting tree was clean, with no other worktrees, stashes or validation processes. The #26 candidate reported174 tests in a separate session is missing here; that evidence is recorded, not published or independently reproduced. User authorizes recreating it after recovery search found no checkpoint branch. Preserve published Input/PageSize controls while rebuilding Dropdown.

Handoff candidates: #44 resolved test-only native InputArea modifiers; #42 foundation stacking/focused preview with incomplete native review; #43 locked CI workflow/commands. Full final script gate passes169 library tests, one gallery regression, nine doctests, workspace fmt/warning-denied Clippy/all-target/all-feature/default-gallery builds; exact adapter fmt/12default/14all-feature tests/warning-denied Clippy. [Raw outcomes](evidence/resume-checkpoint/README.md). Existing dependency warnings remain visible. #42 only has saved final Light1040 evidence; complete both-theme/narrow shadow/text review is pending. #43 remote CI must pass after push. User requested checkpoint/push/cloud handoff; local migration is paused. [Self-contained cloud prompt](cloud-handoff-prompt.md).

Audit: [current issue classifications](issue-audit-2026-10-03.md). Five originally closed issues retain valid published resolution evidence; stale checklists reconciled. Family issues remain open when APIs, motion, browser or platform acceptance remains. Coverage **27/43 working families,16 unported**, distinct from fidelity completion. #41 Linux authored expansion repair exists; GPUI/Base/AccessKit exact patches and downstream-root obligations remain.

Active #26: recreate retained full-known page Dropdown using the source measured42px navigation/input214px/simple83px, three dropdown overlaps, gap6 and33px rows as reported leads, independently verify against pinned source rendering. Define ownership/revision/allocation/mode/focus/unmount/paint acceptance first. Finish #42 and #44 independently while reconstructing; then #38 font discrepancy and dependency-aware remaining families. #43 CI must show a successful remote run before closure. OS IME/speech/other-platform acceptance remains #1–#4; Linux AT-SPI evidence is not speech proof.

[Archived historical checkpoints](history/port-progress-before-resume-2026-10-03.md) retain provenance; their “next” instructions are historical.

## Baseline and scope

Kumo source `3fd5b648df578cb1ba214dedd30f475009f6a668`; GPUI Kit/Base 0.7.0; GPUI snapshot family 0.3.7; Rust 1.99.0. Keep these pinned. Public APIs own Kumo semantics and presentation; Base supplies suitable behavior. Styled Component and bundled assets are excluded. The selected GPUI snapshot derives from Zed `1a28cff4b409169bac058bca40dfbfeb7621d19b`.

Continue the component library beyond the first Button/Input/Popover evaluation. Inventory comes from the pinned Kumo `packages/kumo/src/components` tree, fetched with GitHub's recursive tree API on 2026-10-02. Charts, Flow, Sidebar/application shells, Cloudflare branding and blocks are excluded by task scope. Native equivalents require explicit contracts rather than importing browser validity, routing or DOM behavior.

## Coverage and ordered backlog

Pinned catalog inventory: **43 in-scope component families,28 implemented with documented gaps,15 unported**. [Complete family list and counting rules](component-coverage.md). This is coverage, not a full-fidelity completion claim. Deprecated MenuBar/DateRangePicker/Surface are excluded; use segmented Tabs/DatePicker range/LayerCard. InputArea and SkeletonLine are supported catalog entries omitted by the earlier coarse backlog and now tracked explicitly.

| Priority | Components or work | Status / dependencies |
| --- | --- | --- |
| 0 | Foundation decision; locally repairable accessibility and nested-overlay defects | Repair checkpoint complete; platform export gaps remain |
| 1 | Button/Input browser comparison; Popover geometry stress; transparent shadows and outline motion | First-slice fidelity work, tracked individually |
| 2 | Text, Label/Field, Loader, LayerCard, Badge, Banner, Empty, Code, Link | Text/Loader/LayerCard/Badge/Link implemented with documented comparison/platform gaps; Banner/Empty implemented with native/comparison gaps; Label/Field core/help implemented with association/rich-help gaps; deprecated Code/CodeBlock excluded; recommended CodeHighlighted deferred |
| 3 | Checkbox, Radio, Switch; ButtonGroup, InputGroup, InputArea, SensitiveInput, ClipboardText, InlineCopyText | Checkbox single/group, Radio, Switch and ButtonGroup core implemented with documented remaining gaps. Common form/composition controls; typed values, availability and labels |
| 4 | Tooltip, SkeletonLine, Collapsible, Tabs, Meter, Pagination | Focus, motion, selection or presentation foundations |
| 5 | Dialog, LayerDialog, Dropdown, Toolbar, Select | Overlay and composite navigation; Button/Popover/focus foundations |
| 6 | Combobox, Autocomplete, CommandPalette, TagInput | Retained Input and composite selection/navigation |
| 7 | DatePicker (including range), Toast, Breadcrumbs, Grid, Table, TableOfContents | Calendar/state, notifications, native navigation and data-layout contracts |
| Final | Cross-component integration, public docs/examples, complete supported-platform validation | After scoped coverage; platform limitations remain explicit |

Checkboxes in an issue are evidence, not a coverage guarantee. Button, Input and Popover are implemented but still have documented parity gaps. Their initial metadata, activation, editing, nesting, geometry and theme behavior have automated tests; native/browser evidence is narrower and linked from component recipes and issues.

Earlier remote CI acceptance: [37114390570](https://github.com/msmps/gpui-kumo/actions/runs/37114390570) passed at c7fd554; first run37114282490 was cancelled. Repeat run37115097659 subsequently exposed the Switch timing regression; #43 is reopened pending #45 and repeat validation. #44 closed with744354a evidence; #42/#26/#38 remain open.
