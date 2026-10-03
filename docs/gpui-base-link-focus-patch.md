# Retired Base retained Link focus correction — upstream reference

Kumo and Toolbar share a [library-owned GPUI Link root](library-controls.md). The vendored `src/link.rs` now exactly matches published gpui-base 0.7.0 (archive SHA-256 `64bb52b29a8dcdc8d3f595b8b0839c076f237572ab601ba4577d3da8ae085728`). Consumers no longer need this Link correction for Kumo. Other dependency overrides remain required.

The stock Link's `InteractiveElement::track_focus` initially stores the supplied handle, but rendering replaces it with a keyed handle. Native Toolbar navigation then targets a handle absent from the rendered dispatch/tab registry; pointer focus diverges, arrows stop at the Link and Tab traversal restarts incorrectly.

Upstream candidate preserved at commit `e843e81071eac4a965e105ebb6cb66d8a9547f27`:

- [Patched Link source](https://github.com/msmps/gpui-kumo/blob/e843e81071eac4a965e105ebb6cb66d8a9547f27/vendor/gpui-base-0.7.0/src/link.rs): optional caller-provided focus handle, an inherent `track_focus` hook and keyed fallback when no handle is supplied. No navigation, activation or styling rewrite.
- [Historical Toolbar regressions](https://github.com/msmps/gpui-kumo/blob/e843e81071eac4a965e105ebb6cb66d8a9547f27/crates/gpui-kumo/src/toolbar/tests.rs): retained pointer focus/current destination/modifier activation, arrows across Link, Tab exit/reentry and owner reorder/removal. The old implementation fails the retained pointer-focus test against stock Base; the replacement passes the same input path.
- [Original patch rationale](https://github.com/msmps/gpui-kumo/blob/e843e81071eac4a965e105ebb6cb66d8a9547f27/docs/gpui-base-link-focus-patch.md).

For upstream submission, compare the archived source against the checksum-verified published file and adapt the caller-handle regression to Base's own test harness. This defect remains in stock Base for direct users; replacing Kumo's control does not repair Base itself.
