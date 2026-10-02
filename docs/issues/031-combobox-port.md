# KUMO-031: Port Combobox with Kumo fidelity over GPUI Base

Status: Open

GitHub issue: https://github.com/msmps/gpui-kumo/issues/16

## Goal and status

Port the supported **Combobox** Kumo family to Rust/GPUI. This family is currently unported; the issue covers its supported parts and public API, not just a static default-state demo.

## GPUI Kit/Base foundation

**Match: Direct counterpart.**

Use/evaluate: Combobox + input::Input/InputState + Positioner/Popup; reuse existing retained Input and Popover patterns.

Limits: Base owns root semantics, open/dismiss hooks and focus transfer; application owns searchable collection, selection and popup presentation. It is not a full option-list implementation.

- [gpui-base 0.7.0: combobox.rs](https://docs.rs/crate/gpui-base/0.7.0/source/src/combobox.rs)
- [gpui-base 0.7.0: input/mod.rs](https://docs.rs/crate/gpui-base/0.7.0/source/src/input/mod.rs)
- [gpui-base 0.7.0: positioner.rs](https://docs.rs/crate/gpui-base/0.7.0/source/src/positioner.rs)
- [gpui-base 0.7.0: popup.rs](https://docs.rs/crate/gpui-base/0.7.0/source/src/popup.rs)

## Dependencies and scope

Build on: Input, Popover, Button. These are dependencies, not a claim that every dependency is already complete. Preserve the existing project and API patterns.

[Full coverage/backlog](https://github.com/msmps/gpui-kumo/blob/work/docs/component-coverage.md); [progress and selected milestone](https://github.com/msmps/gpui-kumo/blob/work/docs/port-progress.md).

## Family acceptance criteria

- [ ] Port supported source parts, item rendering, query/selection, clear/empty/loading/disabled contracts with explicit controlled ownership.
- [ ] Implement source keyboard highlight/typeahead/search behavior, selection once, Escape/outside dismissal and restoration; test reorder, long options, nested overlays and IME composition.

## Shared fidelity and validation gate

- [ ] Before coding, inspect the complete pinned Kumo source, styles, matching documentation/demos/tests and installed Base APIs; define a parts/variants/sizes/states/keyboard/focus/semantics parity matrix, explicitly marking N/A.
- [ ] Keep Kumo semantic tokens, presentation and light/dark behavior authoritative. Use suitable Base behavior; avoid gpui-component default styling, duplicate state/listeners and entities recreated during render.
- [ ] Add a realistic gallery example, observable regression tests and skeptical diff/render review. Run required formatting, tests, warning-denied workspace linting, builds and affected examples.
- [ ] Inspect both themes at wide/narrow sizes: actual icon/control centres, text baselines, spacing/padding, clipping, rounded edges and agreement of fills/borders/rings/shadows across layers.
- [ ] Record passed/failed/not-run evidence, native adaptations and unsupported semantics. Browser comparisons (#10), OS IME (#1) and screen-reader checks (#2–#4) are separate validation gates; no compilation-only full-fidelity claim.

## Pinned reference

- [Kumo Combobox source](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/combobox/combobox.tsx) — revision `3fd5b648df578cb1ba214dedd30f475009f6a668`.
- GPUI Kit/Base `0.7.0`; GPUI family `0.3.7` (local documented tab-registration patch retained). Do not silently chase upstream APIs.
- Repository checkpoint `f9bb821cf2523951765e443867ca42c2cdd5a444`; component counts: 43 scoped /18 implemented /25 unported. Deprecated components, charts, Flow, Sidebar/app shells, branding and blocks remain excluded.
