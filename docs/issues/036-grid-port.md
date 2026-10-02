# KUMO-036: Port Grid with Kumo fidelity over GPUI Base

Status: Open

GitHub issue: https://github.com/msmps/gpui-kumo/issues/21

## Goal and status

Port the supported **Grid** Kumo family to Rust/GPUI. This family is currently unported; the issue covers its supported parts and public API, not just a static default-state demo.

## GPUI Kit/Base foundation

**Match: No dedicated counterpart.**

Use/evaluate: Native GPUI layout/container primitives; existing Text/LayerCard for consumer examples. No Base Grid component.

Limits: This is the Kumo layout family, not Base plot grid or a data-grid product. Translate CSS-dependent layouts into supported native equivalents and document limits without inventing a GPUI grid API.

Native GPUI layout must be verified against the installed 0.3.7 snapshot; there is no matching Base module.

## Dependencies and scope

Build on: Theme/layout tokens. These are dependencies, not a claim that every dependency is already complete. Preserve the existing project and API patterns.
- Rounded descendant clipping: [#5](https://github.com/msmps/gpui-kumo/issues/5).

[Full coverage/backlog](https://github.com/msmps/gpui-kumo/blob/work/docs/component-coverage.md); [progress and selected milestone](https://github.com/msmps/gpui-kumo/blob/work/docs/port-progress.md).

## Family acceptance criteria

- [ ] Inspect source column/span/gap/responsive variants and define their native equivalents before coding.
- [ ] Verify deterministic placement, supported spans, source gaps, resizing and narrow/empty/long-content behavior without clipping or unrelated application-shell expansion.

## Shared fidelity and validation gate

- [ ] Before coding, inspect the complete pinned Kumo source, styles, matching documentation/demos/tests and installed Base APIs; define a parts/variants/sizes/states/keyboard/focus/semantics parity matrix, explicitly marking N/A.
- [ ] Keep Kumo semantic tokens, presentation and light/dark behavior authoritative. Use suitable Base behavior; avoid gpui-component default styling, duplicate state/listeners and entities recreated during render.
- [ ] Add a realistic gallery example, observable regression tests and skeptical diff/render review. Run required formatting, tests, warning-denied workspace linting, builds and affected examples.
- [ ] Inspect both themes at wide/narrow sizes: actual icon/control centres, text baselines, spacing/padding, clipping, rounded edges and agreement of fills/borders/rings/shadows across layers.
- [ ] Record passed/failed/not-run evidence, native adaptations and unsupported semantics. Browser comparisons (#10), OS IME (#1) and screen-reader checks (#2–#4) are separate validation gates; no compilation-only full-fidelity claim.

## Pinned reference

- [Kumo Grid source](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/grid/grid.tsx) — revision `3fd5b648df578cb1ba214dedd30f475009f6a668`.
- GPUI Kit/Base `0.7.0`; GPUI family `0.3.7` (local documented tab-registration patch retained). Do not silently chase upstream APIs.
- Repository checkpoint `f9bb821cf2523951765e443867ca42c2cdd5a444`; component counts: 43 scoped /18 implemented /25 unported. Deprecated components, charts, Flow, Sidebar/app shells, branding and blocks remain excluded.
