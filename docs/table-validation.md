# Table contract and validation

Table ports the supported Kumo family from `3fd5b648df578cb1ba214dedd30f475009f6a668` onto GPUI 0.3.7 and Kit/Base 0.7.0. It composes Base table semantics with library-owned shared column layout and sticky painting. No dependency patch was added for Table. [Issue #30](https://github.com/msmps/gpui-kumo/issues/30) remains the acceptance tracker.

## Composition and ownership

`Table::new(id)` accepts a typed `TableHeader`, one or more `TableBody` sections, an optional `TableFooter` and readable `TableCaption`. Header rows contain `TableHead`; body/footer rows contain `TableCell`. Checkbox parts convert only into their corresponding cell kind. Parts are configured before insertion; only the completed Table renders independently. Stable section/row/cell identities survive reordering. Row and column indices/counts are computed, including column spans, rather than authored separately.

Use `TableHead::text` and `TableCell::text` for readable text that inherits the cell recipe. `child` accepts rich content; its author supplies readable metadata and names for nested controls. Raw GPUI strings paint text but do not establish the equivalent native readable-node contract. `Table::accessibility_label` names the table separately from its caption. Caption is a native adapter requested by the issue; the pinned web Table does not export a Caption part.

Selection, sort order, filtering, pagination and resize values remain application-owned. `RowVariant::Selected` controls tint independently of checkbox values. `TableCheckHead`/`TableCheckCell` require an authored control name, accept `checkbox::State`, `disabled`, `track_focus` and the existing Checkbox `on_change` proposal shape. Pointer activation anywhere inside their padded cell reaches the same checkbox, including Space/Enter after pointer focus. The deprecated web `onValueChange` alias is omitted.

`TableHead::resize_handle(TableResizeHandle)` is a typed slot for the trailing resize affordance. Its width range is finite/positive and inclusive. Dragging proposes widths even after the pointer leaves the strip; Left/Right propose eight-pixel changes. Update the owner's `Table::columns` and notify. Disabled handles are inert. The handle appears on header hover or keyboard focus. It does not resize columns autonomously.

## Layout and presentation

| Capability | Native contract |
| --- | --- |
| Typography/padding | Inherited family, 14px/21px copy; 12px cell padding; semibold headers and bottom fill border |
| Compact headers | Elevated fill, strong text and 8px vertical padding; checkbox remains 16px |
| Row states | Alternating base/elevated fills restart within each section; selected tint; no body borders |
| Automatic layout | Mounted cells contribute max-content widths, including padding; remaining viewport width is shared among automatic columns |
| Fixed layout | Remaining width is shared among automatic columns, with a 24px minimum; exact pixel widths stay exact |
| Check columns | Automatic check columns reserve 40px, including padding |
| Narrow layouts | Viewport is independent of `min_table_width`; the grid authors its measured width so the native scroll range includes wide columns; fixed text wraps and row height equalises |
| Scroll gestures | Lock to the gesture axis and consume gestures the table handles. Horizontal swipes do not also move an enclosing vertical page. Vertical gestures can reach the page at a table boundary |
| Sticky columns | Left/right columns stack in column order, with opaque row/header fills and 24px edge fades; hit targets stay at their painted locations |
| Sticky header | Header rows remain above scrolling body cells, including combined sticky columns; normal body drawing/hit masks exclude the pinned bands |
| Semantics | Table/row/header/cell roles, one-based indices, counts and column spans; sticky cells register in their original row ancestry before only their painting is deferred |
| Focus after removal | A non-tab-stop Table scope restores focus when an owner removes its focused descendant, allowing host Tab navigation to continue |

First mount and changes to content/viewport may need a second frame to settle shared widths. Widths are cached by table identity, and stable widths do not request further settling frames. Style refinements follow recipes; use `columns` for cell widths to preserve shared alignment. A caller overriding geometry owns the resulting composition.

This is a native table, not an HTML layout engine or virtualised data grid. Browser min/max-content negotiation, row spans, arbitrary DOM props and semantic relationships inside arbitrary rich content are not emulated. All rows are mounted. Pixel-only columns can leave spare viewport space. A viewport narrower than the combined pinned columns can produce overlapping edge regions; supply a usable minimum viewport or avoid pinning both edges in that composition.

Table has square row surfaces and does not claim rounded arbitrary-descendant clipping. When placing it in a rounded LayerCard, follow [LayerCard's supported clipping contract](layer-card-validation.md); `rounded` plus rectangular overflow alone does not establish corner clipping for custom/deferred descendants. Native accessibility metadata does not establish screen-reader speech or Linux/Windows adapter acceptance.

## Reproduction

```sh
cargo test -p gpui-kumo table:: --locked
cargo run -p kumo-gallery --example table --locked -- --width=1040
cargo run -p kumo-gallery --example table --locked -- --width=420 --dark
bash scripts/check-rust.sh
python3 scripts/check-unpatched.py --output-dir /tmp/kumo-table-unpatched
```

The focused example includes compact/default headers, select-all mixed state, controlled selection, stable row reordering, disabled controls, nested actions, a spanning footer, both sticky edges, a resizable header, wrapping Unicode paths and an empty body/caption. Cmd+L switches themes; Cmd+Q quits. Review control centres, baselines, padding, wrapping, border edges and focus rings in both themes and at both widths. Exercise horizontal/vertical scroll, padded checkbox corners then Space/Enter, resize drag then arrows, and Tab out of the table. Keep screenshots/recordings outside Git.

Rendered regressions live beside the implementation in `crates/gpui-kumo/src/table/tests.rs`. They cover roles/counts/spans, shared intrinsic/fixed sizing, owner text updates and stable geometry, wrapping/readable text, compact metrics, painted selected fills, full-cell pointer/keyboard activation, rejected proposals, reordering/removal and subsequent Tab exit, sticky hit targets, and resize proposals/disabled guards. Rustdoc includes normal and compile-fail composition examples. Platform results belong to the measured validation record; unsupported capabilities above are explicit boundaries, not inferred passes.

## Measured validation

Validated on macOS with the selected workspace and separately with registry dependencies. Eight Table library regressions pass, plus the narrow gallery integration regression. The latter reproduces a mixed horizontal/vertical swipe inside a vertically scrolling page and checks horizontal movement without page drift, vertical containment while the table can scroll, and vertical handoff at its boundary.

The full Rust gate passes: 254 library tests, four gallery tests, normal/compile-fail rustdoc examples, warnings-denied documentation and Clippy, all-target/all-feature builds, and both standalone adapter test configurations. The extracted unpatched package reports 236 passing tests and the same ten named dependency failures; every new Table test passes, with no unexpected failures or missing baseline tests.

Native review covered 1040px and 420px in light/dark, compact/default headers, mixed/selected/disabled checkboxes, pointer then Space/Enter, owner reordering, keyboard resize and dragging beyond the handle. Readable text and cell ancestry appear in the macOS accessibility tree. Alignment review checked checkbox and action centres, header/data offsets, wrapped text, sibling gaps and 12px edge padding. Layer review checked header borders, selected/striped fills, checkbox corners and resize focus borders; the surrounding rounded panel has inset table surfaces. Arbitrary descendant corner clipping, Linux/Windows adapter behavior and screen-reader speech remain the explicit limits above. No image evidence was added to Git.
