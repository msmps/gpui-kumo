# KUMO-027: Breadcrumbs: root styling and copy motion

Status: Resolved

GitHub issue: https://github.com/msmps/gpui-kumo/issues/12

## Resolution — 2026-10-03

Breadcrumbs implements `Styled` for its actual navigation root. Its copy action now uses the pinned 100ms opacity transition with cubic-bezier(0.4,0,0.2,1), retained root hover, keyboard visibility, interruption and reduced-motion behavior. Existing composable links/current/separators/extras remain supported. Rendered tests verify caller geometry, preserved names/roles, reversal and cessation of frame requests.

Native macOS Metal rendering was inspected in Light/Dark at1040/520 logical widths, including text/control centres, baselines, sibling gaps, edge padding, rounded corners, fill/border/ring/shadow agreement and narrow layouts. [Capture fixture and measured evidence](https://github.com/msmps/gpui-kumo/blob/work/docs/evidence/quick-wins/README.md).

Full `scripts/check-rust.sh` gate:232 library tests,1gallery,9doctests; workspace and adapter formatting; warnings-denied Clippy; locked all-target/all-feature/default-gallery builds; adapter12default/14all-feature tests. Dependency warnings remain visible; no pins or vendor patches changed.

Browser/catalog comparison and OS reduced-motion preference delivery remain under #10; actual spoken accessibility remains under #2–#4. These separate gates are not claimed complete. GPUI's clipboard API cannot report delivery failure and arbitrary descendant rounded clipping remains under #5.

Pinned Kumo `3fd5b648df578cb1ba214dedd30f475009f6a668`; Kit/Base0.7.0; patched GPUI0.3.7. Component coverage remains32/43 working,11unported.
