# KUMO-044: Port SkeletonLine with Kumo fidelity over GPUI Base

Status: Working checkpoint; open browser/platform acceptance

GitHub issue: https://github.com/msmps/gpui-kumo/issues/29

## Goal and status

Port the supported **SkeletonLine** Kumo family to Rust/GPUI. Working implementation is available; the issue covers its supported parts and public API, not just a static default-state demo.

## GPUI Kit/Base foundation

**Match: No dedicated counterpart.**

Use/evaluate: Base motion/animation and reduced-motion infrastructure + native GPUI paint/layout.

Limits: No Base SkeletonLine. Source memoizes randomized width/duration/delay and accepts optional blockHeight; do not generate different random values every render or treat animation state as application data.

- [gpui-base 0.7.0: motion.rs](https://docs.rs/crate/gpui-base/0.7.0/source/src/motion.rs)
- [gpui-base 0.7.0: animation.rs](https://docs.rs/crate/gpui-base/0.7.0/source/src/animation.rs)
- [gpui-base 0.7.0: reduce_motion.rs](https://docs.rs/crate/gpui-base/0.7.0/source/src/reduce_motion.rs)

## Dependencies and scope

Build on: Semantic tokens, Motion policy. These are dependencies, not a claim that every dependency is already complete. Preserve the existing project and API patterns.

[Full coverage/backlog](https://github.com/msmps/gpui-kumo/blob/work/docs/component-coverage.md); [progress and selected milestone](https://github.com/msmps/gpui-kumo/blob/work/docs/port-progress.md).

## Family acceptance criteria

- [ ] Inspect pinned skeleton CSS/keyframes and map min/max width/duration/delay, stable sample lifetime and block height into idiomatic native units.
- [ ] Test source-range handling, stable values across rerender/theme changes, repeated instances, reduced motion, cleanup and actual shimmer/paint behavior; inspect both themes/narrow composition.

## Shared fidelity and validation gate

- [ ] Before coding, inspect the complete pinned Kumo source, styles, matching documentation/demos/tests and installed Base APIs; define a parts/variants/sizes/states/keyboard/focus/semantics parity matrix, explicitly marking N/A.
- [ ] Keep Kumo semantic tokens, presentation and light/dark behavior authoritative. Use suitable Base behavior; avoid gpui-component default styling, duplicate state/listeners and entities recreated during render.
- [ ] Add a realistic gallery example, observable regression tests and skeptical diff/render review. Run required formatting, tests, warning-denied workspace linting, builds and affected examples.
- [ ] Inspect both themes at wide/narrow sizes: actual icon/control centres, text baselines, spacing/padding, clipping, rounded edges and agreement of fills/borders/rings/shadows across layers.
- [ ] Record passed/failed/not-run evidence, native adaptations and unsupported semantics. Browser comparisons (#10), OS IME (#1) and screen-reader checks (#2–#4) are separate validation gates; no compilation-only full-fidelity claim.

## Pinned reference

- [Kumo SkeletonLine source](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/loader/skeleton-line.tsx) — revision `3fd5b648df578cb1ba214dedd30f475009f6a668`.
- GPUI Kit/Base `0.7.0`; GPUI family `0.3.7` (local documented tab-registration patch retained). Do not silently chase upstream APIs.
- Repository checkpoint `f9bb821cf2523951765e443867ca42c2cdd5a444`; component counts: 43 scoped /18 implemented /25 unported. Deprecated components, charts, Flow, Sidebar/app shells, branding and blocks remain excluded.

## Checkpoint — 2026-10-03

125 tests/nine doctests, required Rust gate and native wide/narrow animated/static both-theme review pass. [Matrix/evidence/native adaptations](../skeleton-line-validation.md). Browser comparison/OS policy delivery remain separate acceptance gates.
