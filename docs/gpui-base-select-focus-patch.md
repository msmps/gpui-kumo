# Retired Base Select corrections — upstream reference

Kumo uses [library-owned disclosure, confirmation and availability metadata](library-controls.md). The vendored `src/select.rs` now exactly matches published gpui-base 0.7.0 (archive SHA-256 `64bb52b29a8dcdc8d3f595b8b0839c076f237572ab601ba4577d3da8ae085728`). Consumers no longer need these Select corrections for Kumo. Other dependency overrides remain required.

Upstream candidates preserved at commit `e843e81071eac4a965e105ebb6cb66d8a9547f27`:

- [Patched Select source](https://github.com/msmps/gpui-kumo/blob/e843e81071eac4a965e105ebb6cb66d8a9547f27/vendor/gpui-base-0.7.0/src/select.rs): open-state confirmation leaves focus ownership to the consumer callback instead of overwriting restored trigger/outside focus with popup content focus. Closed-state opening still transfers focus to content.
- The same source annotates the existing disabled ComboBox node through AccessKit `set_disabled`, preserving its role and avoiding a duplicate control.
- [Direct Base confirmation regression](https://github.com/msmps/gpui-kumo/blob/e843e81071eac4a965e105ebb6cb66d8a9547f27/crates/gpui-kumo/src/select/base_tests.rs): rendered Enter opens, confirms once, closes and preserves both consumer-selected focus destinations through rerender. It fails on stock Base, so it is archived rather than part of Kumo's current suite.
- [Original patch rationale](https://github.com/msmps/gpui-kumo/blob/e843e81071eac4a965e105ebb6cb66d8a9547f27/docs/gpui-base-select-focus-patch.md).

For upstream submission, compare the archived source against the checksum-verified published file and adapt the regression to Base's own test harness. These defects remain in stock Base for direct users; replacing Kumo's control does not repair Base itself.
