# KUMO-048: Port TagInput with Kumo fidelity over GPUI Base

Status: Open

GitHub issue: https://github.com/msmps/gpui-kumo/issues/33

## Goal and status

Port the supported **TagInput** Kumo family to Rust/GPUI. This family is currently unported; the issue covers its supported parts and public API, not just a static default-state demo.

## GPUI Kit/Base foundation

**Match: Partial composition.**

Use/evaluate: input::Input/InputState + Button; public input::InlineToken/InputContent and token presentation APIs are candidates after compatibility inspection.

Limits: No Base TagInput family. Kumo renders removable values around an input; native inline tokens have document/history semantics and are optional candidates, not an automatic substitute. Combobox is only relevant if the pinned source actually calls for suggestions.

- [gpui-base 0.7.0: input/mod.rs](https://docs.rs/crate/gpui-base/0.7.0/source/src/input/mod.rs)
- [gpui-base 0.7.0: input/base/inline_tokens.rs](https://docs.rs/crate/gpui-base/0.7.0/source/src/input/base/inline_tokens.rs)
- [gpui-base 0.7.0: input/base/token_presentation.rs](https://docs.rs/crate/gpui-base/0.7.0/source/src/input/base/token_presentation.rs)
- [gpui-base 0.7.0: button.rs](https://docs.rs/crate/gpui-base/0.7.0/source/src/button.rs)

## Dependencies and scope

Build on: Input, Field, Button. These are dependencies, not a claim that every dependency is already complete. Preserve the existing project and API patterns.
- InputGroup composition follow-up: [#9](https://github.com/msmps/gpui-kumo/issues/9).

[Full coverage/backlog](https://github.com/msmps/gpui-kumo/blob/work/docs/component-coverage.md); [progress and selected milestone](https://github.com/msmps/gpui-kumo/blob/work/docs/port-progress.md).

## Family acceptance criteria

- [ ] Port source values, delimiters/paste, validation, duplicate policy, maxValues, localized remove/error labels and disabled/read-only ownership.
- [ ] Verify add/remove/backspace/focus behavior, IME commit isolation, clipboard parsing, controlled rejection and once-only callbacks; preserve retained editor and unknown values per source policy.
- [ ] Inspect wrapped chips/field/helper/error composition in both themes and narrow layouts; test long Unicode values and unavailable remove controls.

## Shared fidelity and validation gate

- [ ] Before coding, inspect the complete pinned Kumo source, styles, matching documentation/demos/tests and installed Base APIs; define a parts/variants/sizes/states/keyboard/focus/semantics parity matrix, explicitly marking N/A.
- [ ] Keep Kumo semantic tokens, presentation and light/dark behavior authoritative. Use suitable Base behavior; avoid gpui-component default styling, duplicate state/listeners and entities recreated during render.
- [ ] Add a realistic gallery example, observable regression tests and skeptical diff/render review. Run required formatting, tests, warning-denied workspace linting, builds and affected examples.
- [ ] Inspect both themes at wide/narrow sizes: actual icon/control centres, text baselines, spacing/padding, clipping, rounded edges and agreement of fills/borders/rings/shadows across layers.
- [ ] Record passed/failed/not-run evidence, native adaptations and unsupported semantics. Browser comparisons (#10), OS IME (#1) and screen-reader checks (#2–#4) are separate validation gates; no compilation-only full-fidelity claim.

## Pinned reference

- [Kumo TagInput source](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/tag-input/tag-input.tsx) — revision `3fd5b648df578cb1ba214dedd30f475009f6a668`.
- GPUI Kit/Base `0.7.0`; GPUI family `0.3.7` (local documented tab-registration patch retained). Do not silently chase upstream APIs.
- Repository checkpoint `f9bb821cf2523951765e443867ca42c2cdd5a444`; component counts: 43 scoped /18 implemented /25 unported. Deprecated components, charts, Flow, Sidebar/app shells, branding and blocks remain excluded.
