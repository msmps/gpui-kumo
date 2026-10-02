# KUMO-043: Port SensitiveInput with Kumo fidelity over GPUI Base

Status: Open

GitHub issue: https://github.com/msmps/gpui-kumo/issues/28

## Goal and status

Port the supported **SensitiveInput** Kumo family to Rust/GPUI. This family is currently unported; the issue covers its supported parts and public API, not just a static default-state demo.

## GPUI Kit/Base foundation

**Match: Partial composition.**

Use/evaluate: input::Input/InputState with masked/set_masked + Button and Tooltip; existing InputGroup/Field focus/action machinery and GPUI clipboard.

Limits: Base masking is an editor facility, not Kumo's SensitiveInput mode machine. Kumo uses fixed eight-dot presentation, reveal instructions, an independent eye control and a copy tab; masking alone is insufficient.

- [gpui-base 0.7.0: input/mod.rs](https://docs.rs/crate/gpui-base/0.7.0/source/src/input/mod.rs)
- [gpui-base 0.7.0: input/base/state.rs](https://docs.rs/crate/gpui-base/0.7.0/source/src/input/base/state.rs)
- [gpui-base 0.7.0: button.rs](https://docs.rs/crate/gpui-base/0.7.0/source/src/button.rs)
- [gpui-base 0.7.0: tooltip.rs](https://docs.rs/crate/gpui-base/0.7.0/source/src/tooltip.rs)

## Dependencies and scope

Build on: Input, InputGroup, Field, Tooltip, Button. These are dependencies, not a claim that every dependency is already complete. Preserve the existing project and API patterns.
- InputGroup composition follow-up: [#9](https://github.com/msmps/gpui-kumo/issues/9).

[Full coverage/backlog](https://github.com/msmps/gpui-kumo/blob/work/docs/component-coverage.md); [progress and selected milestone](https://github.com/msmps/gpui-kumo/blob/work/docs/port-progress.md).

## Family acceptance criteria

- [ ] Implement masked/revealed/empty modes, pointer/Enter/Space reveal, eye hide/reveal, Escape focus transfer and external-blur hide; focus within copy/eye controls must follow source rules.
- [ ] Preserve retained value/editor/selection, controlled updates and disabled/read-only contracts; verify masked accessibility tree does not expose the secret inadvertently.
- [ ] Match all four sizes, hover/focus instruction and copy-tab geometry; copy callback/feedback only on success, failure leaves accurate state, weak timer cleanup and Unicode clipboard round trip.

## Shared fidelity and validation gate

- [ ] Before coding, inspect the complete pinned Kumo source, styles, matching documentation/demos/tests and installed Base APIs; define a parts/variants/sizes/states/keyboard/focus/semantics parity matrix, explicitly marking N/A.
- [ ] Keep Kumo semantic tokens, presentation and light/dark behavior authoritative. Use suitable Base behavior; avoid gpui-component default styling, duplicate state/listeners and entities recreated during render.
- [ ] Add a realistic gallery example, observable regression tests and skeptical diff/render review. Run required formatting, tests, warning-denied workspace linting, builds and affected examples.
- [ ] Inspect both themes at wide/narrow sizes: actual icon/control centres, text baselines, spacing/padding, clipping, rounded edges and agreement of fills/borders/rings/shadows across layers.
- [ ] Record passed/failed/not-run evidence, native adaptations and unsupported semantics. Browser comparisons (#10), OS IME (#1) and screen-reader checks (#2–#4) are separate validation gates; no compilation-only full-fidelity claim.

## Pinned reference

- [Kumo SensitiveInput source](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/sensitive-input/sensitive-input.tsx) — revision `3fd5b648df578cb1ba214dedd30f475009f6a668`.
- GPUI Kit/Base `0.7.0`; GPUI family `0.3.7` (local documented tab-registration patch retained). Do not silently chase upstream APIs.
- Repository checkpoint `f9bb821cf2523951765e443867ca42c2cdd5a444`; component counts: 43 scoped /18 implemented /25 unported. Deprecated components, charts, Flow, Sidebar/app shells, branding and blocks remain excluded.
