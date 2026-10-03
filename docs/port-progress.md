# Kumo native port progress

## Current checkpoint — 2026-10-03

Branch `work`; latest published commit `33ab7668f71ec36359164dc34ceed0e6ce8233b2`, independently matched local/remote. Starting tree was clean, with no other worktrees, stashes or validation processes. The #26 candidate reported174 tests in a separate session is missing here; that evidence is recorded, not published or independently reproduced. User authorizes recreating it after recovery search found no checkpoint branch. Preserve published Input/PageSize controls while rebuilding Dropdown.

Handoff candidates: #44 test-only native InputArea modifiers; #42 foundation stacking/focused preview with incomplete native review; #43 locked CI workflow/commands. Full final script gate passes169 library tests, one gallery regression, nine doctests, workspace fmt/warning-denied Clippy/all-target/all-feature/default-gallery builds; exact adapter fmt/12default/14all-feature tests/warning-denied Clippy. [Raw outcomes](evidence/resume-checkpoint/README.md). Existing dependency warnings remain visible. #42 only has saved final Light1040 evidence; complete both-theme/narrow shadow/text review is pending. #43 remote CI must pass after push. User requested checkpoint/push/cloud handoff; local migration is paused. [Self-contained cloud prompt](cloud-handoff-prompt.md).

Audit: [current issue classifications](issue-audit-2026-10-03.md). Six already-closed issues retain valid published resolution evidence; stale checklists reconciled. Family issues remain open when APIs, motion, browser or platform acceptance remains. Coverage **27/43 working families,16 unported**, distinct from fidelity completion. #41 Linux authored expansion repair exists; GPUI/Base/AccessKit exact patches and downstream-root obligations remain.

Active #26: recreate retained full-known page Dropdown using the source measured42px navigation/input214px/simple83px, three dropdown overlaps, gap6 and33px rows as reported leads, independently verify against pinned source rendering. Define ownership/revision/allocation/mode/focus/unmount/paint acceptance first. Finish #42 and #44 independently while reconstructing; then #38 font discrepancy and dependency-aware remaining families. #43 CI must show a successful remote run before closure. OS IME/speech/other-platform acceptance remains #1–#4; Linux AT-SPI evidence is not speech proof.

[Archived historical checkpoints](history/port-progress-before-resume-2026-10-03.md) retain provenance; their “next” instructions are historical.

## Baseline and scope

Kumo source `3fd5b648df578cb1ba214dedd30f475009f6a668`; GPUI Kit/Base 0.7.0; GPUI snapshot family 0.3.7; Rust 1.99.0. Keep these pinned. Public APIs own Kumo semantics and presentation; Base supplies suitable behavior. Styled Component and bundled assets are excluded. The selected GPUI snapshot derives from Zed `1a28cff4b409169bac058bca40dfbfeb7621d19b`.

Continue the component library beyond the first Button/Input/Popover evaluation. Inventory comes from the pinned Kumo `packages/kumo/src/components` tree, fetched with GitHub's recursive tree API on 2026-10-02. Charts, Flow, Sidebar/application shells, Cloudflare branding and blocks are excluded by task scope. Native equivalents require explicit contracts rather than importing browser validity, routing or DOM behavior.

## Coverage and ordered backlog

Pinned catalog inventory: **43 in-scope component families,27 implemented with documented gaps,16 unported**. [Complete family list and counting rules](component-coverage.md). This is coverage, not a full-fidelity completion claim. Deprecated MenuBar/DateRangePicker/Surface are excluded; use segmented Tabs/DatePicker range/LayerCard. InputArea and SkeletonLine are supported catalog entries omitted by the earlier coarse backlog and now tracked explicitly.

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
