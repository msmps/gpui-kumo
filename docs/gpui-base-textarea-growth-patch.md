# GPUI Base Textarea intrinsic growth patch

Pinned gpui-base0.7.0 published crate (registry checksum64bb52b29a8dcdc8d3f595b8b0839c076f237572ab601ba4577d3da8ae085728) is retained under vendor/gpui-base-0.7.0. Only src/input/base/state.rs and src/input/base/element.rs differ from the installed registry source; Cargo cache markers omitted. Version and all dependency versions remain unchanged. Cargo root patch is necessary for downstream consumers, just like the existing GPUI Tab registration patch. No Kit component styling, editor kind or public API changes.

Observed failure before patch: fixed4-row InputArea switched to auto1–3 with existing five-line content stayed at one row (32px Xs), the same height as empty content. Base set_auto_grow initializes rows to minimum without reconciling the current display map. Prepaint rewraps after font/width changes but does not reconcile ordinary-text intrinsic rows; token paths happen to do so. Resetting text to force layout would destroy selection/history and is rejected.

Correction: set_auto_grow measures existing display-map rows immediately. TextElement prepaint reconciles AutoGrow rows after font, wrapping width/indent and inline metrics are prepared; notification only when row count changes. PlainText and CodeEditor row modes are untouched. This uses Base's own wrapped display map and caret/scroll machinery, with no duplicate shaping/sizing model.

Regression evidence lives in InputArea observable tests: initial/existing content policy changes, grow/shrink/cap, width rewrap and all four font/size recipes, editor identity, selected Unicode and undo across presentation changes. Independent review passed;113 workspace tests/nine doctests and Rust gate pass. Upstream unit command was not run: Cargo rejects non-workspace packages needing dev-dependencies; no upstream suite success claimed. Remove the root patch when a pinned upstream version passes these regressions without it. Upstream broad vendoring/migration remains unnecessary; this is a reproducible narrow repair of an observed foundation defect.

The crate metadata records upstream Kit revision0c830f4d257e69fdd17200650533ab4ca9a40cc0, path crates/base. Consumer root example (adapt paths to the checked-out library):

```toml
[patch.crates-io]
gpui-pre = { path = "gpui-kumo/vendor/gpui-pre-0.3.7" }
gpui-base = { path = "gpui-kumo/vendor/gpui-base-0.7.0" }
```
