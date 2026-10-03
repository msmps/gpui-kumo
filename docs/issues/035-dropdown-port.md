# KUMO-035: Port Dropdown with Kumo fidelity over GPUI Base

Status: Action core implemented; full family open

GitHub issue: https://github.com/msmps/gpui-kumo/issues/20

## Goal and status

Port the supported **Dropdown** Kumo family to Rust/GPUI. The flat action-menu core is implemented; the issue remains open for checkbox/radio/link/submenu/group/joined and rich trigger composition, motion/RTL and full platform/speech acceptance.

## Action core checkpoint — 2026-10-03

Published implementation **e560a2d52e5ee3997c97de6e47017c7a272ecd92** passes the [remote macOS Rust gate37135561747](https://github.com/msmps/gpui-kumo/actions/runs/37135561747). Retained DropdownState/typed action parts/operation events, source keyboard-focusable disabled rows, arrow/Home/End/typeahead, once-only activation, Escape/Tab/outside lifecycle, reorder/removal/unmount recovery and bounded scrolling are implemented over Base Popup. Shared gallery Save actions and focused native preview are integrated. Six rendered regressions and browser/native Light/Dark1040/520 evidence cover the flat action core. [Contract/adaptations](../dropdown-validation.md), [evidence](../evidence/dropdown/README.md). Coverage30/43 working,13 unported is not full fidelity; #20 remains open. No version/vendor changes.

## GPUI Kit/Base foundation

**Match: Partial composition.**

Use/evaluate: Popover/PopoverState + Positioner/Popup for overlay; Button, Checkbox/Radio as suitable item building blocks.

Limits: No dedicated Base Menu/Dropdown primitive exists in 0.7.0. Select has combobox semantics and must not be substituted as a menu root merely because it opens a popup; own verified menu semantics/navigation where Base is missing.

- [gpui-base 0.7.0: popover.rs](https://docs.rs/crate/gpui-base/0.7.0/source/src/popover.rs)
- [gpui-base 0.7.0: positioner.rs](https://docs.rs/crate/gpui-base/0.7.0/source/src/positioner.rs)
- [gpui-base 0.7.0: button.rs](https://docs.rs/crate/gpui-base/0.7.0/source/src/button.rs)
- [gpui-base 0.7.0: checkbox.rs](https://docs.rs/crate/gpui-base/0.7.0/source/src/checkbox.rs)
- [gpui-base 0.7.0: radio.rs](https://docs.rs/crate/gpui-base/0.7.0/source/src/radio.rs)

## Dependencies and scope

Build on: Button, Popover, Checkbox, Radio. These are dependencies, not a claim that every dependency is already complete. Preserve the existing project and API patterns.
- Rich/native trigger composition: [#7](https://github.com/msmps/gpui-kumo/issues/7).

[Full coverage/backlog](https://github.com/msmps/gpui-kumo/blob/work/docs/component-coverage.md); [progress and selected milestone](https://github.com/msmps/gpui-kumo/blob/work/docs/port-progress.md).

## Family acceptance criteria

- [ ] Inspect and port actual supported menu parts, separators, selection/check states and nested-menu capabilities; retain action contracts and closed/open controlled ownership.
- [ ] Implement menu keyboard traversal/typeahead, pinned unavailable-item focus/activation policy, activation once, submenu opening/dismissal where supported, Escape/outside boundaries and focus restoration.
- [ ] Integrate a real Save dropdown and joined trigger appearance with ButtonGroup; do not present the existing arrow callback as a completed menu.

## Shared fidelity and validation gate

- [ ] Before coding, inspect the complete pinned Kumo source, styles, matching documentation/demos/tests and installed Base APIs; define a parts/variants/sizes/states/keyboard/focus/semantics parity matrix, explicitly marking N/A.
- [ ] Keep Kumo semantic tokens, presentation and light/dark behavior authoritative. Use suitable Base behavior; avoid gpui-component default styling, duplicate state/listeners and entities recreated during render.
- [ ] Add a realistic gallery example, observable regression tests and skeptical diff/render review. Run required formatting, tests, warning-denied workspace linting, builds and affected examples.
- [ ] Inspect both themes at wide/narrow sizes: actual icon/control centres, text baselines, spacing/padding, clipping, rounded edges and agreement of fills/borders/rings/shadows across layers.
- [ ] Record passed/failed/not-run evidence, native adaptations and unsupported semantics. Browser comparisons (#10), OS IME (#1) and screen-reader checks (#2–#4) are separate validation gates; no compilation-only full-fidelity claim.

## Pinned reference

- [Kumo Dropdown source](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/dropdown/dropdown.tsx) — revision `3fd5b648df578cb1ba214dedd30f475009f6a668`.
- GPUI Kit/Base `0.7.0`; GPUI family `0.3.7` (local documented tab-registration patch retained). Do not silently chase upstream APIs.
- Repository checkpoint `f9bb821cf2523951765e443867ca42c2cdd5a444`; component counts: 43 scoped /18 implemented /25 unported. Deprecated components, charts, Flow, Sidebar/app shells, branding and blocks remain excluded.
