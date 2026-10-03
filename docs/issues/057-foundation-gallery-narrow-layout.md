# KUMO-057: Narrow foundation gallery composition

Status: Complete; publishedf03da05 and GitHub#42 closed

GitHub issue: https://github.com/msmps/gpui-kumo/issues/42

Native #39/#40 final review confirms pre-existing color/typography/gradient foundation columns below Button samples overflow520px. Evidence: ../evidence/linux-busy-state/button-*-520-sizes.png. Gallery composition repair only; preserve token/gradient source presentation and wide layout, wrap/stack columns/swatches with actual bounded geometry. Review both-theme1040/520 centres/baselines/gaps/padding/corners/fills/borders/shadows; required Rust/example gates and independent review. No library API or app-shell expansion.

Backlog after active Pagination parts/shared expansion repair.

## Handoff candidate

Extracted shared foundation renderer; stack columns below738px, preserve wide layout/token recipes, bound flex children. Both-theme1040/520/737/738 rendered containment/resize regression passes. Independent source review found no blocker; panel rectangles do not establish text or shadow paint containment. Focused native example provides width/theme startup flags. Only saved native final evidence is Light1040; Light520 inspected with an earlier too-short preview, complete narrow shadows and both Dark captures remain outstanding. [Evidence](../evidence/foundation-layout/README.md). Do not close#42 until final native gate is complete.

## Native continuation — 2026-10-03

Both-theme focused/full-gallery 1040/520 and real 737/738 resize transitions now pass painted text, shadow, alignment and corner review over3eee132. Full pinned Linux script passes175 library/1gallery/9doctests, fmt/lint/locked builds and standalone adapter12/14tests. No new recipe change needed. [Captures, exact scripts, review and limits](../evidence/foundation-layout/linux/README.md). Publishedf03da05 succeeds in remote CI37119510684; GitHub#42 checklist is reconciled/closed through the connector. CLI API denial remains separate.

GitHub#42 checklist reconciled and closed as completed on2026-10-03 after verifying publishedf03da05 and remote CI37119510684.
