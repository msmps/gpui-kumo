# KUMO-033: Port DatePicker with Kumo fidelity over GPUI Base

Status: Open

GitHub issue: https://github.com/msmps/gpui-kumo/issues/18

## Goal and status

Port the supported **DatePicker** Kumo family to Rust/GPUI. This family is currently unported; the issue covers its supported parts and public API, not just a static default-state demo.

## GPUI Kit/Base foundation

**Match: Direct counterpart.**

Use/evaluate: DatePicker + Calendar/CalendarState, Date/Matcher APIs + Positioner/Popup; Button/Input as required by source.

Limits: Base DatePicker is an open/focus root, not a complete styled calendar control. Calendar selection, ranges, date boundaries, locale and source public contract must be checked explicitly.

- [gpui-base 0.7.0: date_picker.rs](https://docs.rs/crate/gpui-base/0.7.0/source/src/date_picker.rs)
- [gpui-base 0.7.0: calendar.rs](https://docs.rs/crate/gpui-base/0.7.0/source/src/calendar.rs)
- [gpui-base 0.7.0: positioner.rs](https://docs.rs/crate/gpui-base/0.7.0/source/src/positioner.rs)

## Dependencies and scope

Build on: Button, Input, Popover. These are dependencies, not a claim that every dependency is already complete. Preserve the existing project and API patterns.

[Full coverage/backlog](https://github.com/msmps/gpui-kumo/blob/work/docs/component-coverage.md); [progress and selected milestone](https://github.com/msmps/gpui-kumo/blob/work/docs/port-progress.md).

## Family acceptance criteria

- [ ] Port supported single and range modes in this family; omit deprecated standalone DateRangePicker.
- [ ] Test calendar keyboard movement, month/year boundaries, disabled dates, range endpoints, controlled rejection, clear/empty states, formatting/localization and popup focus/dismissal/restoration.

## Shared fidelity and validation gate

- [ ] Before coding, inspect the complete pinned Kumo source, styles, matching documentation/demos/tests and installed Base APIs; define a parts/variants/sizes/states/keyboard/focus/semantics parity matrix, explicitly marking N/A.
- [ ] Keep Kumo semantic tokens, presentation and light/dark behavior authoritative. Use suitable Base behavior; avoid gpui-component default styling, duplicate state/listeners and entities recreated during render.
- [ ] Add a realistic gallery example, observable regression tests and skeptical diff/render review. Run required formatting, tests, warning-denied workspace linting, builds and affected examples.
- [ ] Inspect both themes at wide/narrow sizes: actual icon/control centres, text baselines, spacing/padding, clipping, rounded edges and agreement of fills/borders/rings/shadows across layers.
- [ ] Record passed/failed/not-run evidence, native adaptations and unsupported semantics. Browser comparisons (#10), OS IME (#1) and screen-reader checks (#2–#4) are separate validation gates; no compilation-only full-fidelity claim.

## Pinned reference

- [Kumo DatePicker source](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/date-picker/date-picker.tsx) — revision `3fd5b648df578cb1ba214dedd30f475009f6a668`.
- GPUI Kit/Base `0.7.0`; GPUI family `0.3.7` (local documented tab-registration patch retained). Do not silently chase upstream APIs.
- Repository checkpoint `f9bb821cf2523951765e443867ca42c2cdd5a444`; component counts: 43 scoped /18 implemented /25 unported. Deprecated components, charts, Flow, Sidebar/app shells, branding and blocks remain excluded.
