# KUMO-027: Port Breadcrumbs with Kumo fidelity over GPUI Base

Status: In progress

GitHub issue: https://github.com/msmps/gpui-kumo/issues/12

## Goal and status

Port the supported **Breadcrumbs** Kumo family to Rust/GPUI. Core implemented; source opacity motion and browser/platform acceptance keep this issue open. See [Breadcrumbs validation](../breadcrumbs-validation.md).

## GPUI Kit/Base foundation

**Match: No dedicated counterpart.**

Use/evaluate: Link and Button for actionable parts; native GPUI layout for separators and overflow.

Limits: There is no Base Breadcrumbs root. Own navigation/current-page semantics and source composition; do not add router or application-shell infrastructure.

- [gpui-base 0.7.0: link.rs](https://docs.rs/crate/gpui-base/0.7.0/source/src/link.rs)
- [gpui-base 0.7.0: button.rs](https://docs.rs/crate/gpui-base/0.7.0/source/src/button.rs)

## Dependencies and scope

Build on: Link, Button, Text. These are dependencies, not a claim that every dependency is already complete. Preserve the existing project and API patterns.

[Full coverage/backlog](https://github.com/msmps/gpui-kumo/blob/work/docs/component-coverage.md); [progress and selected milestone](https://github.com/msmps/gpui-kumo/blob/work/docs/port-progress.md).

## Family acceptance criteria

- [x] Match source parts, current item, separators and any supported truncation/overflow contract.
- [x] Document native destination/callback handling; test long labels, narrow composition, focus order and disabled/actionable distinctions.

## Shared fidelity and validation gate

- [ ] Before coding, inspect the complete pinned Kumo source, styles, matching documentation/demos/tests and installed Base APIs; define a parts/variants/sizes/states/keyboard/focus/semantics parity matrix, explicitly marking N/A.
- [ ] Keep Kumo semantic tokens, presentation and light/dark behavior authoritative. Use suitable Base behavior; avoid gpui-component default styling, duplicate state/listeners and entities recreated during render.
- [ ] Add a realistic gallery example, observable regression tests and skeptical diff/render review. Run required formatting, tests, warning-denied workspace linting, builds and affected examples.
- [ ] Inspect both themes at wide/narrow sizes: actual icon/control centres, text baselines, spacing/padding, clipping, rounded edges and agreement of fills/borders/rings/shadows across layers.
- [ ] Record passed/failed/not-run evidence, native adaptations and unsupported semantics. Browser comparisons (#10), OS IME (#1) and screen-reader checks (#2–#4) are separate validation gates; no compilation-only full-fidelity claim.

## Pinned reference

- [Kumo Breadcrumbs source](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/breadcrumbs/breadcrumbs.tsx) — revision `3fd5b648df578cb1ba214dedd30f475009f6a668`.
- GPUI Kit/Base `0.7.0`; GPUI family `0.3.7` (local documented tab-registration patch retained). Do not silently chase upstream APIs.
- Repository checkpoint `f9bb821cf2523951765e443867ca42c2cdd5a444`; component counts: 43 scoped /18 implemented /25 unported. Deprecated components, charts, Flow, Sidebar/app shells, branding and blocks remain excluded.

## Validated core checkpoint — 2026-10-03

Five rendered regressions, native light/dark1040/520 glyph/alignment/ring/copy/loading captures and actual Linux Unicode clipboard reads;134 tests/nine doctests and required Rust gate pass. Source opacity motion, root style overrides, browser/OS spoken-feedback acceptance remain pending.
