# KUMO-047: Port Tabs with Kumo fidelity over GPUI Base

Status: Open

GitHub issue: https://github.com/msmps/gpui-kumo/issues/32

## Goal and status

Port the supported **Tabs** Kumo family to Rust/GPUI. A retained native core is implemented; overflow/motion and full fidelity remain open; the issue covers its supported parts and public API, not just a static default-state demo.

## GPUI Kit/Base foundation

**Match: Direct counterpart.**

Use/evaluate: Tabs/Tab + suitable focus/navigation primitives after checking installed APIs.

Limits: Base Tab source explicitly lacks complete compound keyboard navigation (roving focus, arrows, Home/End and Enter/Space). Direct names are not a complete desktop tabs contract; fill the verified gap or make a narrowly documented dependency repair.

- [gpui-base 0.7.0: tabs.rs](https://docs.rs/crate/gpui-base/0.7.0/source/src/tabs.rs)
- [gpui-base 0.7.0: toolbar.rs](https://docs.rs/crate/gpui-base/0.7.0/source/src/toolbar.rs)

## Dependencies and scope

Build on: Button, Focus foundation. These are dependencies, not a claim that every dependency is already complete. Preserve the existing project and API patterns.

[Full coverage/backlog](https://github.com/msmps/gpui-kumo/blob/work/docs/component-coverage.md); [progress and selected milestone](https://github.com/msmps/gpui-kumo/blob/work/docs/port-progress.md).

## Family acceptance criteria

- [ ] Port supported standard and segmented Tabs variants/parts and controlled selection; omit deprecated MenuBar.
- [ ] Implement source keyboard navigation/activation, disabled-item skipping, orientation rules and panel mounting/focus ownership; test reordered/removed tabs, rejected proposals and repeated instances.
- [ ] Verify actual selected/tab/panel semantics supported by GPUI, long/narrow labels and focus/ring layering in both themes.

## Shared fidelity and validation gate

- [ ] Before coding, inspect the complete pinned Kumo source, styles, matching documentation/demos/tests and installed Base APIs; define a parts/variants/sizes/states/keyboard/focus/semantics parity matrix, explicitly marking N/A.
- [ ] Keep Kumo semantic tokens, presentation and light/dark behavior authoritative. Use suitable Base behavior; avoid gpui-component default styling, duplicate state/listeners and entities recreated during render.
- [ ] Add a realistic gallery example, observable regression tests and skeptical diff/render review. Run required formatting, tests, warning-denied workspace linting, builds and affected examples.
- [ ] Inspect both themes at wide/narrow sizes: actual icon/control centres, text baselines, spacing/padding, clipping, rounded edges and agreement of fills/borders/rings/shadows across layers.
- [ ] Record passed/failed/not-run evidence, native adaptations and unsupported semantics. Browser comparisons (#10), OS IME (#1) and screen-reader checks (#2–#4) are separate validation gates; no compilation-only full-fidelity claim.

## Pinned reference

- [Kumo Tabs source](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/tabs/tabs.tsx) — revision `3fd5b648df578cb1ba214dedd30f475009f6a668`.
- GPUI Kit/Base `0.7.0`; GPUI family `0.3.7` (local documented tab-registration patch retained). Do not silently chase upstream APIs.
- Repository checkpoint `f9bb821cf2523951765e443867ca42c2cdd5a444`; component counts: 43 scoped /18 implemented /25 unported. Deprecated components, charts, Flow, Sidebar/app shells, branding and blocks remain excluded.
