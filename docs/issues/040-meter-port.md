# KUMO-040: Meter: track and indicator styling

Status: Resolved

GitHub issue: https://github.com/msmps/gpui-kumo/issues/25

## Resolution — 2026-10-03

Meter now accepts `track_style(StyleRefinement)` and `indicator_style(StyleRefinement)`, applied after the semantic default recipe. The normalized animated width remains on its private owner-driven wrapper. A rendered regression checks custom height, fill/radii paint and unchanged numeric meaning in both themes. Existing numeric normalization, formatting and motion regressions pass.

Native macOS Metal rendering was inspected in Light/Dark at1040/520 logical widths, including text/control centres, baselines, sibling gaps, edge padding, rounded corners, fill/border/ring/shadow agreement and narrow layouts. [Capture fixture and measured evidence](https://github.com/msmps/gpui-kumo/blob/work/docs/evidence/quick-wins/README.md).

Full `scripts/check-rust.sh` gate:232 library tests,1gallery,9doctests; workspace and adapter formatting; warnings-denied Clippy; locked all-target/all-feature/default-gallery builds; adapter12default/14all-feature tests. Dependency warnings remain visible; no pins or vendor patches changed.

Browser/catalog comparison and OS reduced-motion preference delivery remain under #10; actual spoken accessibility remains under #2–#4. These separate gates are not claimed complete. GPUI's clipboard API cannot report delivery failure and arbitrary descendant rounded clipping remains under #5.

Pinned Kumo `3fd5b648df578cb1ba214dedd30f475009f6a668`; Kit/Base0.7.0; patched GPUI0.3.7. Component coverage remains32/43 working,11unported.
