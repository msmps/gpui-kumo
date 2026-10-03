# KUMO-040: Port Meter with Kumo fidelity over GPUI Base

Status: In progress

GitHub issue: https://github.com/msmps/gpui-kumo/issues/25

## Goal and status

Port the supported **Meter** Kumo family to Rust/GPUI. Core port implemented; remaining generic style-slot and browser/platform acceptance keeps this issue open. See [Meter validation](../meter-validation.md).

## GPUI Kit/Base foundation

**Match: Partial composition.**

Use/evaluate: Progress/ProgressTrack/ProgressIndicator provide related range/track infrastructure; Text for labels/value.

Limits: No Base Meter primitive. Progress authors ProgressIndicator semantics; Kumo Meter represents measurement within a range. Verify a real Meter/range semantic adaptation or implement a minimal native root instead of exposing incorrect progress semantics.

- [gpui-base 0.7.0: progress.rs](https://docs.rs/crate/gpui-base/0.7.0/source/src/progress.rs)

## Dependencies and scope

Build on: Text, Semantic tokens. These are dependencies, not a claim that every dependency is already complete. Preserve the existing project and API patterns.

[Full coverage/backlog](https://github.com/msmps/gpui-kumo/blob/work/docs/component-coverage.md); [progress and selected milestone](https://github.com/msmps/gpui-kumo/blob/work/docs/port-progress.md).

## Family acceptance criteria

- [x] Match source range normalization, label, customValue/showValue and track/indicator composition; distinguish invalid/out-of-range handling from progress indeterminate state.
- [x] Verify displayed percentage/range, empty/extreme values, readable value metadata, long labels, narrow track and light/dark fill/edge alignment.

## Shared fidelity and validation gate

- [ ] Before coding, inspect the complete pinned Kumo source, styles, matching documentation/demos/tests and installed Base APIs; define a parts/variants/sizes/states/keyboard/focus/semantics parity matrix, explicitly marking N/A.
- [ ] Keep Kumo semantic tokens, presentation and light/dark behavior authoritative. Use suitable Base behavior; avoid gpui-component default styling, duplicate state/listeners and entities recreated during render.
- [ ] Add a realistic gallery example, observable regression tests and skeptical diff/render review. Run required formatting, tests, warning-denied workspace linting, builds and affected examples.
- [ ] Inspect both themes at wide/narrow sizes: actual icon/control centres, text baselines, spacing/padding, clipping, rounded edges and agreement of fills/borders/rings/shadows across layers.
- [ ] Record passed/failed/not-run evidence, native adaptations and unsupported semantics. Browser comparisons (#10), OS IME (#1) and screen-reader checks (#2–#4) are separate validation gates; no compilation-only full-fidelity claim.

## Pinned reference

- [Kumo Meter source](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/meter/meter.tsx) — revision `3fd5b648df578cb1ba214dedd30f475009f6a668`.
- GPUI Kit/Base `0.7.0`; GPUI family `0.3.7` (local documented tab-registration patch retained). Do not silently chase upstream APIs.
- Repository checkpoint `f9bb821cf2523951765e443867ca42c2cdd5a444`; component counts: 43 scoped /18 implemented /25 unported. Deprecated components, charts, Flow, Sidebar/app shells, branding and blocks remain excluded.

## Validated core checkpoint — 2026-10-03

Actual Meter semantics, Base parts and retained width motion; four observable regressions and six Linux both-theme wide/narrow/update captures. Source arithmetic huge-number limitation recorded explicitly. Required Rust gate recorded in Meter validation. Generic track/indicator style overrides, browser comparison and OS speech acceptance remain pending; issue stays In progress.
