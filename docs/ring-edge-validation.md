# Rounded ring edges

Native Button and Switch recipes paint layered inset rings and softened thumb shadows. Preserve the supported corner radius and agreement between fill, inset/outside borders and focus rings when changing these recipes.

Inspect both themes at native scale and wide/narrow layouts, including rest, hover, focus, disabled and checked states. Compare high-contrast control corners against the pinned browser recipe. Look for dark fringes, square child fills over rounded parents, clipped focus rings and inconsistent shadow/ring curves. Centre glyphs and thumbs within their controls, and check label baselines, gaps and edge padding.

The original fixed-coordinate pixel sentinel depended on historical gallery positions and light-only captures. It is retired; do not infer current edge correctness from its old threshold. Use actual rendered paint/input regressions and fresh native review following [verification](gpui-verification.md). Track new discrepancies in [GitHub issues](https://github.com/msmps/gpui-kumo/issues).
