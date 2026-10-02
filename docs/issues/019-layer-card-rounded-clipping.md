# KUMO-019: LayerCard: preserve rounded clipping for arbitrary descendants

Status: Open

GitHub issue: https://github.com/msmps/gpui-kumo/issues/5

## Problem and evidence

Kumo LayerCard roots and Primary sections combine rounded-lg with overflow-hidden. The selected GPUI 0.3.7 Style::overflow_mask computes rectangular Bounds, and ContentMask contains only bounds. It does not carry corner radii. LayerCard's actual-render oversized-child test confirms a rectangular mask matching its card bounds, while the card's background/ring remains rounded. Native light screenshots also reproduced square secondary backgrounds covering the root’s rounded top corners. A rendered regression failed twice on that exact paint path before the correction below.

This is a dependency rendering limit for arbitrary descendant content. Painting a surrounding-color cover would depend on unknown/translucent ancestors and cannot correctly reproduce general clipping. Preserve the source contract; do not silently narrow LayerCard to text-only children or change its radius to hide the mismatch.

## Default secondary fill correction

Secondary solid backgrounds now paint a quad rounded to the actual root bounds, intersected with the section’s rectangular bounds and the current ancestor mask. This preserves the −8px margins without rounding the header above the visible card. Ephemeral frame geometry stays private; descendants retain their original layout, hit testing and accessibility. Canvas bounds explicitly use zero insets, so section padding does not shift the paint mask.

The original top-corner regression passes, as does a header/footer regression in both themes with a 16px and oversized root radii and contrasting caller background. Custom painting now clamps radii to the actual root size, matching normal GPUI style paint. Native light and dark gallery inspection shows smooth top corners. General descendant clipping, section-specific custom radii and gradient backgrounds still use GPUI’s rectangular overflow path; this issue remains open for those branches.

## Remaining acceptance

- Compare an oversized contrasting child with the pinned browser card at all rounded corners, in light/dark and with caller radius overrides.
- Verify the installed renderer's available mask paths and determine a real subtree clipping implementation or scoped dependency patch.
- Preserve normal layout, outside ring/shadow painting, descendant input hit testing and accessibility.
- Add a rendered regression that observes the fixed paint boundary; confirm native pixels before marking parity complete.

LayerCard's composition, sizing, text recipes, rectangular overflow and nested controls have separate passing evidence in [LayerCard validation](../layer-card-validation.md). Keep those results distinct from this unresolved rounded clipping branch.

Baseline: Kumo `3fd5b648df578cb1ba214dedd30f475009f6a668`, GPUI Kit/Base0.7.0 and GPUI snapshot family0.3.7. Current checkpoint `d07646f8d24acf72248f954a03c8da056d84848d`.
