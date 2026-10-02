# Kumo native port progress

Updated 2026-10-02. Task branch: `work`. This is the continuation checkpoint and dependency-aware backlog; individual first-slice issues remain in [issues](issues/README.md).

## Baseline and scope

Kumo source `3fd5b648df578cb1ba214dedd30f475009f6a668`; GPUI Kit/Base 0.7.0; GPUI snapshot family 0.3.7; Rust 1.99.0. Keep these pinned. Public APIs own Kumo semantics and presentation; Base supplies suitable behavior. Styled Component and bundled assets are excluded. The selected GPUI snapshot derives from Zed `1a28cff4b409169bac058bca40dfbfeb7621d19b`.

Continue the component library beyond the first Button/Input/Popover evaluation. Inventory comes from the pinned Kumo `packages/kumo/src/components` tree, fetched with GitHub's recursive tree API on 2026-10-02. Charts, Flow, Sidebar/application shells, Cloudflare branding and blocks are excluded by task scope. Native equivalents require explicit contracts rather than importing browser validity, routing or DOM behavior.

## Coverage and ordered backlog

| Priority | Components or work | Status / dependencies |
| --- | --- | --- |
| 0 | Foundation decision; locally repairable accessibility and nested-overlay defects | Selected milestone; existing theme, Button/Input/Popover |
| 1 | Button/Input browser comparison; Popover geometry stress; transparent shadows and outline motion | First-slice fidelity work, tracked individually |
| 2 | Text, Label/Field, Loader, Surface, LayerCard, Badge, Banner, Empty, Code, Link | Presentation and compositions over tokens; Field portions currently inside Input; Loader currently private to Button |
| 3 | Checkbox, Radio, Switch; ButtonGroup, InputGroup, SensitiveInput, ClipboardText, InlineCopyText | Common form/composition controls; typed values, availability and labels |
| 4 | Tooltip, Collapsible, Tabs, Meter, Pagination | Focus, motion, selection or presentation foundations |
| 5 | Dialog, LayerDialog, Dropdown, Menubar, Toolbar, Select | Overlay and composite navigation; Button/Popover/focus foundations |
| 6 | Combobox, Autocomplete, CommandPalette, TagInput | Retained Input and composite selection/navigation |
| 7 | DatePicker, DateRangePicker, Toast, Breadcrumbs, Grid, Table, TableOfContents | Calendar/state, notifications, native navigation and data-layout contracts |
| Final | Cross-component integration, public docs/examples, complete supported-platform validation | After scoped coverage; platform limitations remain explicit |

Checkboxes in an issue are evidence, not a coverage guarantee. Button, Input and Popover are implemented but still have documented parity gaps. Their initial metadata, activation, editing, nesting, geometry and theme behavior have automated tests; native/browser evidence is narrower and linked from component recipes and issues.

## Selected milestone: retain/adapt decision and foundation repair

Why now: the implementation plan requires evaluation before expanding the catalog; a read-only independent review found locally repairable metadata and parent-registration gaps. Fix those while retaining proven Base editing/activation behavior.

Acceptance:

- Explicitly record retained, adapted and replaced behaviors and the upgrade/patch boundary.
- Measure clean and incremental native debug builds plus the selected normal dependency graph, with environment and measurement limits.
- Button exports disabled for unavailable states and busy for loading; Input exports disabled/read-only/invalid on the actual Base control node, without duplicate controls or an editing engine.
- Reparented or detached Popovers are not dismissed by an old parent; ancestry cycles are rejected before changing valid registration.
- Existing behavior and new regressions pass; formatting, workspace tests, warning-denied Clippy and gallery/example builds pass.
- Inspect native accessibility output for the state changes. Separate AccessKit metadata, adapter exposure and spoken-reader acceptance.

Review findings: high, Button/Input descriptions do not expose state flags, although the installed `a11y_synthetic_children` parent-node hook can do so. Medium, Popover reparenting leaves an old parent's child registration and longer ancestry cycles are not rejected. Fix locally, then re-review the diff and rendered/input behavior. Linux EditableText, directed incoming selection and general subtree transforms remain separate dependency limits.

## Measured baseline

On macOS 26.6.2 / aarch64-apple-darwin, MacBookPro18,1, 10 CPUs, 16 GiB RAM, Rust 1.99.0:

- Before foundation changes: workspace tests passed (25 during initial checkout; 30 after remote Input accessibility continuation), formatting and warning-denied Clippy passed.
- Fresh isolated target directory, cached dependencies, default debug gallery build: Cargo reports 53.64 seconds; `/usr/bin/time` reports 53.69 seconds wall, 251.66 user, 25.67 system. Cargo succeeded. The timer's later `kern.clockrate` query was denied by the sandbox, so no peak-memory result is claimed. This is not a cold download or release-build measurement.
- Same target directory after changing Button/Input metadata: successful incremental rebuild, 2.769 seconds wall (Python monotonic timer), Cargo reports 2.71 seconds.
- `cargo tree --locked --workspace --edges normal --prefix none --format '{p}' --offline`: 374 unique package/version entries, including the two workspace packages. No `gpui-component` or bundled component assets in this active host graph. This is host normal dependencies, not every target or the larger lockfile inventory.

## Next exact action

Foundation repair checkpoint: 33 workspace tests pass, including three new rendered Popover parent regressions; formatting and warning-denied workspace/all-target Clippy pass. Native macOS gallery and state fixture confirm disabled/re-enabled controls, loading disabling, changing read-only/error help, and restoration. Actual debug-tree actions reject editing in read-only mode and restore it on reset. The debug serializer omits disabled/busy/read-only/invalid bits: busy/read-only/invalid platform export is not claimed from those descriptions. KUMO-001/005 remain In progress. KUMO-018 is resolved by the explicit decision and cost measurement. The Actions API returned no workflow runs; repository inspection found no CI configuration.

Next milestone: Popover geometry stress (KUMO-011), highest-value unresolved shared overlay behavior before more overlay controls. It depends on the existing Positioner and corrected nesting lifecycle. Verify resize, scrolling, oversized/long content, corners and neither-side-fits placement with actual rendered geometry/input tests and native captures; compare fitting policy to pinned browser source. Then proceed to first-slice visual comparisons and presentation catalog expansion. Keep this document updated at coherent checkpoints; do not treat pending checks as passed.

### Geometry continuation checkpoint

KUMO-011 remains selected. Three new rendered/input regressions pass: fitting includes the requested gap, long content's last action remains reachable across four open-window sizes, and an open popup follows a trigger scrolled by its ancestor. The gap regression demonstrated a Base side-selection defect before its private adapter repair. Current total: 36 passing workspace tests; formatting, all-target/all-feature warning-denied Clippy and example builds pass.

Native long-content/theme/resize activation was observed, but stale dismissal pixels and unreliable input updates remain unresolved; see [KUMO-011](issues/011-popover-geometry-stress.md). Exact next action: isolate native input delivery versus frame presentation, then validate corners, neither-side-fits policy against the pinned browser, arrows and nested routing. Do not resolve the geometry milestone from the automated fitting checks alone.

Apollo's `rust-best-practices` skill was installed at the user's request. Apply its relevant reference chapters during Rust work: intentional ownership, explicit fallible error handling, meaningful observable tests, documented public contracts and all-target/all-feature locked Clippy. Gallery/example startup failures now report errors and quit; accessibility evidence writes report errors rather than panic. Test fixtures include explicit Quit actions. Close test applications and verify their processes have stopped after computer-use checks.
