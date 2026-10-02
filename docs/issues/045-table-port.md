# KUMO-045: Port Table with Kumo fidelity over GPUI Base

Status: Open

GitHub issue: https://github.com/msmps/gpui-kumo/issues/30

## Goal and status

Port the supported **Table** Kumo family to Rust/GPUI. This family is currently unported; the issue covers its supported parts and public API, not just a static default-state demo.

## GPUI Kit/Base foundation

**Match: Direct counterpart.**

Use/evaluate: Table, TableHeader, TableBody, TableHead, TableRow, TableCell and TableCaption.

Limits: Base supplies composable table semantics, not Kumo presentation or a full data-grid engine. Restrict scope to source-supported table parts; do not add charts, spreadsheet editing or speculative data infrastructure.

- [gpui-base 0.7.0: table.rs](https://docs.rs/crate/gpui-base/0.7.0/source/src/table.rs)

## Dependencies and scope

Build on: Text, Checkbox/Button only where source compositions require. These are dependencies, not a claim that every dependency is already complete. Preserve the existing project and API patterns.
- Rounded descendant clipping: [#5](https://github.com/msmps/gpui-kumo/issues/5).

[Full coverage/backlog](https://github.com/msmps/gpui-kumo/blob/work/docs/component-coverage.md); [progress and selected milestone](https://github.com/msmps/gpui-kumo/blob/work/docs/port-progress.md).

## Family acceptance criteria

- [ ] Port pinned parts, supported density/row/cell states and content composition with actual header/cell semantics.
- [ ] Test headers/rows/cells, empty content, repeated rows, narrow overflow, long cells and nested controls; verify keyboard contracts of hosted controls and source border/alignment treatment.

## Shared fidelity and validation gate

- [ ] Before coding, inspect the complete pinned Kumo source, styles, matching documentation/demos/tests and installed Base APIs; define a parts/variants/sizes/states/keyboard/focus/semantics parity matrix, explicitly marking N/A.
- [ ] Keep Kumo semantic tokens, presentation and light/dark behavior authoritative. Use suitable Base behavior; avoid gpui-component default styling, duplicate state/listeners and entities recreated during render.
- [ ] Add a realistic gallery example, observable regression tests and skeptical diff/render review. Run required formatting, tests, warning-denied workspace linting, builds and affected examples.
- [ ] Inspect both themes at wide/narrow sizes: actual icon/control centres, text baselines, spacing/padding, clipping, rounded edges and agreement of fills/borders/rings/shadows across layers.
- [ ] Record passed/failed/not-run evidence, native adaptations and unsupported semantics. Browser comparisons (#10), OS IME (#1) and screen-reader checks (#2–#4) are separate validation gates; no compilation-only full-fidelity claim.

## Pinned reference

- [Kumo Table source](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/table/table.tsx) — revision `3fd5b648df578cb1ba214dedd30f475009f6a668`.
- GPUI Kit/Base `0.7.0`; GPUI family `0.3.7` (local documented tab-registration patch retained). Do not silently chase upstream APIs.
- Repository checkpoint `f9bb821cf2523951765e443867ca42c2cdd5a444`; component counts: 43 scoped /18 implemented /25 unported. Deprecated components, charts, Flow, Sidebar/app shells, branding and blocks remain excluded.
