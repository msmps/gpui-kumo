# Kumo native port progress

Updated 2026-10-02. Task branch: `work`. This is the continuation checkpoint and dependency-aware backlog; individual first-slice issues remain in [issues](issues/README.md).

## Baseline and scope

Kumo source `3fd5b648df578cb1ba214dedd30f475009f6a668`; GPUI Kit/Base 0.7.0; GPUI snapshot family 0.3.7; Rust 1.99.0. Keep these pinned. Public APIs own Kumo semantics and presentation; Base supplies suitable behavior. Styled Component and bundled assets are excluded. The selected GPUI snapshot derives from Zed `1a28cff4b409169bac058bca40dfbfeb7621d19b`.

Continue the component library beyond the first Button/Input/Popover evaluation. Inventory comes from the pinned Kumo `packages/kumo/src/components` tree, fetched with GitHub's recursive tree API on 2026-10-02. Charts, Flow, Sidebar/application shells, Cloudflare branding and blocks are excluded by task scope. Native equivalents require explicit contracts rather than importing browser validity, routing or DOM behavior.

## Coverage and ordered backlog

| Priority | Components or work | Status / dependencies |
| --- | --- | --- |
| 0 | Foundation decision; locally repairable accessibility and nested-overlay defects | Repair checkpoint complete; platform export gaps remain |
| 1 | Button/Input browser comparison; Popover geometry stress; transparent shadows and outline motion | First-slice fidelity work, tracked individually |
| 2 | Text, Label/Field, Loader, LayerCard, Badge, Banner, Empty, Code, Link | Text/Loader implemented with documented platform/comparison gaps; LayerCard implemented with rounded subtree/platform gaps; Badge next; Field portions currently inside Input |
| 3 | Checkbox, Radio, Switch; ButtonGroup, InputGroup, SensitiveInput, ClipboardText, InlineCopyText | Common form/composition controls; typed values, availability and labels |
| 4 | Tooltip, Collapsible, Tabs, Meter, Pagination | Focus, motion, selection or presentation foundations |
| 5 | Dialog, LayerDialog, Dropdown, Menubar, Toolbar, Select | Overlay and composite navigation; Button/Popover/focus foundations |
| 6 | Combobox, Autocomplete, CommandPalette, TagInput | Retained Input and composite selection/navigation |
| 7 | DatePicker, DateRangePicker, Toast, Breadcrumbs, Grid, Table, TableOfContents | Calendar/state, notifications, native navigation and data-layout contracts |
| Final | Cross-component integration, public docs/examples, complete supported-platform validation | After scoped coverage; platform limitations remain explicit |

Checkboxes in an issue are evidence, not a coverage guarantee. Button, Input and Popover are implemented but still have documented parity gaps. Their initial metadata, activation, editing, nesting, geometry and theme behavior have automated tests; native/browser evidence is narrower and linked from component recipes and issues.

## Measured baseline

On macOS 26.6.2 / aarch64-apple-darwin, MacBookPro18,1, 10 CPUs, 16 GiB RAM, Rust 1.99.0:

- Before foundation changes: workspace tests passed (25 during initial checkout; 30 after remote Input accessibility continuation), formatting and warning-denied Clippy passed.
- Fresh isolated target directory, cached dependencies, default debug gallery build: Cargo reports 53.64 seconds; `/usr/bin/time` reports 53.69 seconds wall, 251.66 user, 25.67 system. Cargo succeeded. The timer's later `kern.clockrate` query was denied by the sandbox, so no peak-memory result is claimed. This is not a cold download or release-build measurement.
- Same target directory after changing Button/Input metadata: successful incremental rebuild, 2.769 seconds wall (Python monotonic timer), Cargo reports 2.71 seconds.
- `cargo tree --locked --workspace --edges normal --prefix none --format '{p}' --offline`: 374 unique package/version entries, including the two workspace packages. No `gpui-component` or bundled component assets in this active host graph. This is host normal dependencies, not every target or the larger lockfile inventory.

## Current checkpoint

LayerCard implementation and retained gallery composition are ready for the validation/commit gate. Six rendered regressions cover source geometry and overrides, base line-height versus simple inheritance, nested Button/Input activation/focus/Unicode editing/disabled behavior, empty/oversized content, default top-corner paint and header/footer fills under caller radius/background overrides. The user’s screenshot finding reproduced square secondary fills over the rounded root. Solid secondary fills now intersect the actual rounded root shape with section bounds; the native rebuilt light capture shows smooth corners. Arbitrary descendant rounded clipping remains [KUMO-019](issues/019-layer-card-rounded-clipping.md).

Selected source inspection excludes deprecated Surface, Badge destructive and its legacy type alias. LayerCard supplies the supported simple surface. Apply the repository’s deprecated-feature policy to each new component before accepting its API.

Latest pushed checkpoint before this gate: `91436da`; implementation remains uncommitted until required checks finish. Native testing apps are closed and gallery process absence verified. No CI configuration/workflow runs were found at the prior pushed checkpoint.

## Validation and review evidence

- Foundation retain/adapt decision and build-cost evaluation resolved KUMO-018. Button/Input metadata hooks and Popover parent registration/cycle repair are implemented. AccessKit metadata and operating-system/spoken exposure remain distinct; KUMO-001/005 remain in progress, and Linux EditableText/directed incoming selection remain dependency limits.
- Popover gap fitting, resize/scroll reachability, corner/no-fit input boundaries, cached paint removal after Escape and nested resize/focus regressions pass. Native resized light/dark dismissal passed on repeat; an earlier stale capture was not reproduced and received no speculative workaround. Browser geometry/arrow comparison remains KUMO-011/012.
- [Text](text-validation.md) supports modern copy/heading/mono styles, rich inline composition, full accessible content and inherited consumer fonts. Native light/dark inspected; browser comparison and spoken heading semantics remain pending.
- [Loader](loader-validation.md) is shared with Button and has size/name, light/dark foreground, animated endpoint and reduced-motion regressions. Native light/dark captures show advancing arcs. Deterministic pinned SVG comparison is prepared; browser surfaces were unavailable. Exact browser motion/path and spoken status checks remain pending.
- [LayerCard](layer-card-validation.md) records the complete acceptance matrix and measured results. Skeptical review repaired inherited layered line-height and the default secondary fill defect. Follow-up geometry coverage caught padded absolute canvases shifting the mask; explicit zero insets fixed it. Nested controls retain their tested input paths.

Current automated gate: 49 workspace tests and two doctests pass. Formatting, all-target/all-feature locked Clippy with warnings denied and gallery/example builds pass. The Loader timing regression failed during the gate because its executor clock did not drive GPUI’s unsynced animation; its test now advances the actual wall time, and both workspace and isolated checks pass. The upstream block 0.1.6 future-compatibility notice remains separate from current check results.

Final rebuilt captures show smooth corners in light/dark after cmd-l theme switching. The Save counter updated once; Unicode value changes reached the native tree, but Input pixels lagged the exposed value. Native input/frame consistency remains pending. The gallery was quit and its process absence verified.

## Next exact action

Finish the LayerCard gate, recheck the final native build, update measured check results, commit and push `work`. Then inspect supported Badge source/styles/examples and define its acceptance matrix before coding. Badge depends on Theme/Text and unlocks Banner/Empty compositions; preserve the open comparison and clipping branches above. Source variant maps control the API when prose names keys that do not exist.

Close every testing application after inspection and verify process termination. Apply Apollo Rust best practices and preserve explicit state ownership/private Base boundaries at each milestone.
