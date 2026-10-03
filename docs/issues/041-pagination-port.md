# KUMO-041: Port Pagination with Kumo fidelity over GPUI Base

Status: In progress

GitHub issue: https://github.com/msmps/gpui-kumo/issues/26

## Goal and status

Port the supported **Pagination** Kumo family to Rust/GPUI. This family is currently unported; source inspection and dependency implementation are in progress; the issue covers its supported parts and public API, not just a static default-state demo.

## GPUI Kit/Base foundation

**Match: Direct counterpart.**

Use/evaluate: Pagination/PaginationState/PaginationItem + Button/Link for source destinations.

Limits: Base clamps page counts and owns visible-page generation/proposals. Check Kumo's page/window/ellipsis policy and public value conventions before reuse; do not silently change source behavior to match Base defaults.

- [gpui-base 0.7.0: pagination.rs](https://docs.rs/crate/gpui-base/0.7.0/source/src/pagination.rs)
- [gpui-base 0.7.0: button.rs](https://docs.rs/crate/gpui-base/0.7.0/source/src/button.rs)
- [gpui-base 0.7.0: link.rs](https://docs.rs/crate/gpui-base/0.7.0/source/src/link.rs)

## Dependencies and scope

Build on: Button, Link. These are dependencies, not a claim that every dependency is already complete. Preserve the existing project and API patterns.

[Full coverage/backlog](https://github.com/msmps/gpui-kumo/blob/work/docs/component-coverage.md); [progress and selected milestone](https://github.com/msmps/gpui-kumo/blob/work/docs/port-progress.md).

## Family acceptance criteria

- [ ] Port supported source first/previous/page input or dropdown/next/last, Info/PageSize/Separator and full/simple composition with explicit owner-controlled page.
- [ ] Test first/last boundaries, empty/small/large totals, total changes, rejected proposals, keyboard activation once, disabled directions and long/narrow layouts.

## Shared fidelity and validation gate

- [ ] Before coding, inspect the complete pinned Kumo source, styles, matching documentation/demos/tests and installed Base APIs; define a parts/variants/sizes/states/keyboard/focus/semantics parity matrix, explicitly marking N/A.
- [ ] Keep Kumo semantic tokens, presentation and light/dark behavior authoritative. Use suitable Base behavior; avoid gpui-component default styling, duplicate state/listeners and entities recreated during render.
- [ ] Add a realistic gallery example, observable regression tests and skeptical diff/render review. Run required formatting, tests, warning-denied workspace linting, builds and affected examples.
- [ ] Inspect both themes at wide/narrow sizes: actual icon/control centres, text baselines, spacing/padding, clipping, rounded edges and agreement of fills/borders/rings/shadows across layers.
- [ ] Record passed/failed/not-run evidence, native adaptations and unsupported semantics. Browser comparisons (#10), OS IME (#1) and screen-reader checks (#2–#4) are separate validation gates; no compilation-only full-fidelity claim.

## Pinned reference

- [Kumo Pagination source](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/pagination/pagination.tsx) — revision `3fd5b648df578cb1ba214dedd30f475009f6a668`.
- GPUI Kit/Base `0.7.0`; GPUI family `0.3.7` (local documented tab-registration patch retained). Do not silently chase upstream APIs.
- Repository checkpoint `f9bb821cf2523951765e443867ca42c2cdd5a444`; component counts: 43 scoped /18 implemented /25 unported. Deprecated components, charts, Flow, Sidebar/app shells, branding and blocks remain excluded.

## Source inspection and selected dependency

Pinned source has no numbered-page/ellipsis strip. Compound API is recommended; legacy root/text API is deprecated and excluded. Controls use InputGroup and Select; PageSize defaults25/50/100/250. Known totals govern next/last; unknown totals use sequential controls. Base guarded controlled requests/Navigation apply, visible-items strip does not. Leading InputGroup/fixed centred editor foundation is the next coherent checkpoint under#9, then retained controlled page/draft handling and parts under#26. [Plan](../port-progress.md).
