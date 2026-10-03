# KUMO-042: Port Select with Kumo fidelity over GPUI Base

Status: In progress

GitHub issue: https://github.com/msmps/gpui-kumo/issues/27

## Goal and status

Port the supported **Select** Kumo family to Rust/GPUI. A validated core is implemented; groups/value composition and integration remain in progress; the issue covers its supported parts and public API, not just a static default-state demo.

## GPUI Kit/Base foundation

**Match: Direct counterpart.**

Use/evaluate: Select + Positioner/Popup; Button and suitable option elements.

Limits: Base supplies combobox semantics, opening/dismissal and focus transfer; caller owns option collection, highlighted option and selected value. Verify actual selected/active metadata rather than assuming list behavior is complete.

- [gpui-base 0.7.0: select.rs](https://docs.rs/crate/gpui-base/0.7.0/source/src/select.rs)
- [gpui-base 0.7.0: positioner.rs](https://docs.rs/crate/gpui-base/0.7.0/source/src/positioner.rs)
- [gpui-base 0.7.0: popup.rs](https://docs.rs/crate/gpui-base/0.7.0/source/src/popup.rs)

## Dependencies and scope

Build on: Button, Popover. These are dependencies, not a claim that every dependency is already complete. Preserve the existing project and API patterns.

[Full coverage/backlog](https://github.com/msmps/gpui-kumo/blob/work/docs/component-coverage.md); [progress and selected milestone](https://github.com/msmps/gpui-kumo/blob/work/docs/port-progress.md).

## Family acceptance criteria

- [ ] Port pinned trigger/value/list/item/group/separator capabilities and supported selection/placeholder/clear contracts.
- [ ] Implement source arrow/Home/End/typeahead/activation policy, unavailable-option skipping, controlled rejection and once-only callbacks; verify popup focus, collision, Escape/outside dismissal and restoration.

## Shared fidelity and validation gate

- [x] Before coding, inspect the complete pinned Kumo source, styles, matching documentation/demos/tests and installed Base APIs; define a parts/variants/sizes/states/keyboard/focus/semantics parity matrix, explicitly marking N/A.
- [ ] Keep Kumo semantic tokens, presentation and light/dark behavior authoritative. Use suitable Base behavior; avoid gpui-component default styling, duplicate state/listeners and entities recreated during render.
- [ ] Add a realistic gallery example, observable regression tests and skeptical diff/render review. Run required formatting, tests, warning-denied workspace linting, builds and affected examples.
- [ ] Inspect both themes at wide/narrow sizes: actual icon/control centres, text baselines, spacing/padding, clipping, rounded edges and agreement of fills/borders/rings/shadows across layers.
- [ ] Record passed/failed/not-run evidence, native adaptations and unsupported semantics. Browser comparisons (#10), OS IME (#1) and screen-reader checks (#2–#4) are separate validation gates; no compilation-only full-fidelity claim.

## Pinned reference

- [Kumo Select source](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/select/select.tsx) — revision `3fd5b648df578cb1ba214dedd30f475009f6a668`.
- GPUI Kit/Base `0.7.0`; GPUI family `0.3.7` (local documented tab-registration patch retained). Do not silently chase upstream APIs.
- Repository checkpoint `f9bb821cf2523951765e443867ca42c2cdd5a444`; component counts: 43 scoped /18 implemented /25 unported. Deprecated components, charts, Flow, Sidebar/app shells, branding and blocks remain excluded.

## Active milestone — 2026-10-03

Selected after Breadcrumbs391113a. Complete pinned source/docs/tests/demos inspected; matrix and source/API decisions in [Select acceptance](../select-validation.md). Base Select owns semantic/keyboard disclosure and focus; caller must implement typed option collection/highlight/selection/typeahead/pointer/outside paths. Starting baseline134 tests/nine doctests. Implementation pending, no validation pass yet.


## Select foundation checkpoint — 2026-10-03

Complete pinned Select source/docs/demos/tests and installed Base APIs inspected; [acceptance matrix](select-validation.md) established. Actual rendered Base regression exposed confirmation overwriting consumer-restored focus after popup closure. Narrow [Select focus repair](gpui-base-select-focus-patch.md) retains closed-to-open transfer and preserves trigger/outside focus after confirmation. Original fails, patched passes; independent review and135 tests/nine doctests, formatting, warning-denied all-target/all-feature Clippy/build pass. Existing profiler deprecation remains visible. No Select gallery/visual acceptance yet;25/43 implemented,18 unported unchanged. #27 remains active.

Exact next action: implement typed retained option/selection/highlight ownership and Kumo-painted Base Select root/deferred list; validate actual input/callback/focus before extending rich/group/multiple compositions. Keep all source-theme/layout/platform criteria open until measured.


## Core checkpoint

144 tests/nine doctests and required Rust gates pass; native both-theme wide/narrow checks and reviewer findings/fixes are recorded in [Select acceptance](../select-validation.md). Core single/multiple selection is usable; full parts criteria remain open. Next: named groups/separators, custom value content, placements, native state metadata and parent Popover integration.26 working families/43 scoped;17 unported.

## Native semantics checkpoint

145 tests/nine doctests and required Rust gates pass. Actual Base ComboBox node enrichment and guarded accessible Option Click implemented; live Linux AT-SPI validates disclosure, commit/restoration, multiple selection, helper/read-only metadata and disabled root action exclusion. Both themes/wide/narrow reviewed. Linux exporter limitations and remaining source parts/platform/browser acceptance are recorded in [Select acceptance](../select-validation.md). Next: groups/separators/custom values/placement/nested Popover composition. Family remains in progress.

## Grouped collection checkpoint

147 tests/nine doctests and required Rust gates pass. `SelectPart`/`SelectGroup` preserve one authoritative collection and stable highlight IDs; actual nested-row bounds drive nearest8px-padded reveal without fighting manual scrolling. Both-theme narrow/deep/oversized/reorder regressions and native wide/narrow alignment/divider/layer/End review pass. Live AT-SPI group export and option actions pass; [evidence and gaps](../select-validation.md). Custom values/placement and parent Popover integration remain next; issue stays in progress.
