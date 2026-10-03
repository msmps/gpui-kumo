# KUMO-037: InlineCopyText: copy fade and localized announcements

Status: Resolved

GitHub issue: https://github.com/msmps/gpui-kumo/issues/22

## Resolution — 2026-10-03

CopySimple now fades using the pinned 100ms opacity curve, with rapid reversal and immediate reduced-motion rendering. Check mounts immediately, as in the pinned source; the discarded Copy transition cannot leak into reset. A stable zero-size Label carries the localized copied message and polite live metadata, clears at reset/payload replacement and stays isolated across controls. Pointer/Space/Enter, cancellation, focus and clipboard regressions remain passing.

Native macOS Metal rendering was inspected in Light/Dark at1040/520 logical widths, including text/control centres, baselines, sibling gaps, edge padding, rounded corners, fill/border/ring/shadow agreement and narrow layouts. [Capture fixture and measured evidence](https://github.com/msmps/gpui-kumo/blob/work/docs/evidence/quick-wins/README.md).

Full `scripts/check-rust.sh` gate:232 library tests,1gallery,9doctests; workspace and adapter formatting; warnings-denied Clippy; locked all-target/all-feature/default-gallery builds; adapter12default/14all-feature tests. Dependency warnings remain visible; no pins or vendor patches changed.

Browser/catalog comparison and OS reduced-motion preference delivery remain under #10; actual spoken accessibility remains under #2–#4. These separate gates are not claimed complete. GPUI's clipboard API cannot report delivery failure and arbitrary descendant rounded clipping remains under #5.

Pinned Kumo `3fd5b648df578cb1ba214dedd30f475009f6a668`; Kit/Base0.7.0; patched GPUI0.3.7. Component coverage remains32/43 working,11unported.
