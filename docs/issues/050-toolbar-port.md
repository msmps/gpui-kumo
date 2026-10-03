# KUMO-050: Port Toolbar with Kumo fidelity over GPUI Base

Status: Open — action/link core implemented; hosted editing/composition remains

GitHub issue: https://github.com/msmps/gpui-kumo/issues/35

## Goal and status

Port the supported **Toolbar** Kumo family to Rust/GPUI. The action/link core is implemented and validated; hosted Input/InputGroup, popup replacement/composition, RTL and full-gallery/platform/speech acceptance remain.

## Pushed action/link milestone — 2026-10-03

Implementation/evidence [da6ff47](https://github.com/msmps/gpui-kumo/commit/da6ff4795c00455bdee1425e9a8e45c1b3210e2d) is pushed. [Remote macOS Rust gate37128708107](https://github.com/msmps/gpui-kumo/actions/runs/37128708107) succeeds for implementation da6ff47. [Contract/parts matrix/adaptations](../toolbar-validation.md), [browser/native results](../evidence/toolbar/README.md), [retained Base Link focus patch](../gpui-base-link-focus-patch.md). Full local gate passes190library/1gallery/9doctests, adapter12/14 and fmt/lint/builds. Browser/native Light/Dark1040/520 input/semantic/geometry matrices and visual review pass. Coverage is29/43 working families,14 unported.

Next slice: Toolbar Input/InputGroup editing and caret-boundary navigation, then popup replacement. Full-family criteria below remain open for missing parts; passed action/link evidence does not establish speech, other platforms or arbitrary composition acceptance.

## GPUI Kit/Base foundation

**Match: Direct counterpart.**

Source exports Root/Button/Link/Input/InputGroup; Group and Separator are not public source parts. The action/link core uses Base Toolbar/Button/Link with retained owner navigation filling verified Base gaps.

Limits: Base supports horizontal arrow focus among rendered descendants. Its disabled flag suppresses its own navigation only; the owner must disable hosted controls. Check source orientation/loop/entry/exit rules rather than claiming full roving-tabindex behavior.

- [gpui-base 0.7.0: toolbar.rs](https://docs.rs/crate/gpui-base/0.7.0/source/src/toolbar.rs)
- [gpui-base 0.7.0: toggle.rs](https://docs.rs/crate/gpui-base/0.7.0/source/src/toggle.rs)
- [gpui-base 0.7.0: toggle_group.rs](https://docs.rs/crate/gpui-base/0.7.0/source/src/toggle_group.rs)
- [gpui-base 0.7.0: button.rs](https://docs.rs/crate/gpui-base/0.7.0/source/src/button.rs)

## Dependencies and scope

Build on: Button, Switch/Checkbox, Dropdown where menu compositions require. These are dependencies, not a claim that every dependency is already complete. Preserve the existing project and API patterns.

[Full coverage/backlog](https://github.com/msmps/gpui-kumo/blob/work/docs/component-coverage.md); [progress and selected milestone](https://github.com/msmps/gpui-kumo/blob/work/docs/port-progress.md).

## Family acceptance criteria

- [ ] Port source parts, grouping/separators, supported orientation and control composition with explicit hosted-control ownership.
- [ ] Verify keyboard entry/exit, arrows/wrapping, source Home/End policy where applicable, disabled controls and dynamic insertion/removal; hosted Input retains caret arrows.
- [ ] Inspect grouped borders/gaps, icon centres, focus layers and long/narrow wrapping in both themes; reuse primitives without importing gpui-component styling.

## Shared fidelity and validation gate

- [ ] Before coding, inspect the complete pinned Kumo source, styles, matching documentation/demos/tests and installed Base APIs; define a parts/variants/sizes/states/keyboard/focus/semantics parity matrix, explicitly marking N/A.
- [ ] Keep Kumo semantic tokens, presentation and light/dark behavior authoritative. Use suitable Base behavior; avoid gpui-component default styling, duplicate state/listeners and entities recreated during render.
- [ ] Add a realistic gallery example, observable regression tests and skeptical diff/render review. Run required formatting, tests, warning-denied workspace linting, builds and affected examples.
- [ ] Inspect both themes at wide/narrow sizes: actual icon/control centres, text baselines, spacing/padding, clipping, rounded edges and agreement of fills/borders/rings/shadows across layers.
- [ ] Record passed/failed/not-run evidence, native adaptations and unsupported semantics. Browser comparisons (#10), OS IME (#1) and screen-reader checks (#2–#4) are separate validation gates; no compilation-only full-fidelity claim.

## Pinned reference

- [Kumo Toolbar source](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/toolbar/toolbar.tsx) — revision `3fd5b648df578cb1ba214dedd30f475009f6a668`.
- GPUI Kit/Base `0.7.0`; GPUI family `0.3.7` (local documented tab-registration patch retained). Do not silently chase upstream APIs.
- Original tracker checkpoint `f9bb821cf2523951765e443867ca42c2cdd5a444` was43 scoped/18 implemented/25 unported; the current milestone above supersedes those counts. Deprecated components, charts, Flow, Sidebar/app shells, branding and blocks remain excluded.
