# KUMO-025: Components: complete pinned browser comparisons for the implemented catalog

GitHub issue: https://github.com/msmps/gpui-kumo/issues/10

Status: Open

## Problem

Native tests and both-theme gallery inspection establish specific geometry, input and paint outcomes, but do not establish browser-equivalent appearance across the implemented catalog. Earlier local KUMO-004/008/012 cover first-slice Button/Input/Popover comparisons; this issue tracks the remaining implemented families and cross-component integration rather than declaring known visual defects.

Category: **Outstanding validation**.

## Acceptance criteria

- [ ] Record pinned Kumo/browser dependencies and generated CSS, font profile, viewport, scale factor and capture method. Use the actual 18-family coverage inventory as the baseline and update it as coverage grows.
- [ ] For each implemented family, compare relevant variants/sizes/parts and default/hover/pressed/focused/disabled/selected/loading/invalid/open states; explicitly mark irrelevant states.
- [ ] Compare light/dark plus long text and narrow layouts. Inspect glyph/icon/control centres, baselines, sibling gaps, edge padding, rounded inner/outer edges, fill/border/ring/shadow agreement and shared seams.
- [ ] Cover Checkbox/Switch/Radio labels, Field/helper/help, InputGroup joins, ButtonGroup compositions and Tooltip arrow/body integration. Existing repaired defects must remain fixed.
- [ ] Distinguish measured discrepancies, intended native adaptations, missing implementation and checks not run. Link actual captures/results; raise concrete defects separately.
- [ ] Track Button/Input/Popover first-slice comparison results alongside KUMO-004/008/012 without duplicating their work; do not infer keyboard, IME or spoken accessibility acceptance from screenshots.

## Evidence and baseline

- [component-coverage.md](https://github.com/msmps/gpui-kumo/blob/work/docs/component-coverage.md)
- [gpui-verification.md](https://github.com/msmps/gpui-kumo/blob/work/docs/gpui-verification.md)
- [ring-edge-validation.md](https://github.com/msmps/gpui-kumo/blob/work/docs/ring-edge-validation.md)
- [tooltip-validation.md](https://github.com/msmps/gpui-kumo/blob/work/docs/tooltip-validation.md)

Baseline: Kumo `3fd5b648df578cb1ba214dedd30f475009f6a668`; GPUI Kit/Base 0.7.0; GPUI snapshot family 0.3.7. Current implementation checkpoint: `d07646f8d24acf72248f954a03c8da056d84848d`. Keep dependency versions pinned; do not claim full parity from compilation or screenshots alone.


## Consolidated SkeletonLine acceptance — 2026-10-03

SkeletonLine implementation #29 is resolved; browser geometry/gradient comparison and supported-platform OS reduced-motion preference delivery remain tracked here in #10. Component reduced-motion policy/frame regressions already pass. Current coverage32/43 working,11unported. Source/native Tabs/Dropdown/Dialog/Toast matrices are bounded passes; Pagination/Select matrices remain partial.

## Consolidated quick-win validation — 2026-10-03

Meter#25, Breadcrumbs#12 and InlineCopyText#22 bounded implementation work is complete. Keep shared browser comparisons for caller Meter track/fill styles, styled Breadcrumbs and100ms copy fades here; OS reduced-motion delivery remains here, and actual copied-message speech stays #2–#4. Native Metal1040/520 Light/Dark results and rendered regression outcomes are recorded in [quick-win evidence](../evidence/quick-wins/README.md). This does not claim catalog/platform acceptance. Select#27 sizing is fixed; its other capabilities and #26 matrix remain open.
