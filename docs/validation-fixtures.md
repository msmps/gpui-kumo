# Historical validation fixtures

The manual browser/native tools were removed from the current tree during public-release cleanup. Past component validation documents link here to identify historical evidence; those links do not claim that the tools remain maintained or runnable in this checkout.

The [archived fixture inventory](https://github.com/msmps/gpui-kumo/blob/cd1d263a/docs/validation-fixtures.md) records the original entry points, pinned Kumo reference and platform setup. The [historical tool sources](https://github.com/msmps/gpui-kumo/tree/cd1d263a/tools/validation) remain available in Git history. Their absolute managed-runtime paths require adaptation on other machines.

Issue-referenced screenshots remain under `docs/evidence/`. Acceptance results and outstanding work live in [GitHub issues](https://github.com/msmps/gpui-kumo/issues), including native input accessibility [#49](https://github.com/msmps/gpui-kumo/issues/49) and Dropdown/PageSize behavior [#26](https://github.com/msmps/gpui-kumo/issues/26). Removing the manual tools does not close those acceptance gaps.

## Current verification

Run `bash scripts/check-rust.sh` from the repository root for the locked Rust workspace and exact adapter gate. CI runs that script and the packaged dependency checks. Component regression tests remain alongside implementations; focused native examples remain under `apps/gallery/examples/`.

For manual visual/input review, follow [verification](gpui-verification.md). Inspect both themes, wide/narrow layouts, focus and keyboard paths, control centres, text baselines, sibling spacing, and rounded surfaces. Rust checks alone do not establish browser pixel parity, OS IME behavior or spoken screen-reader acceptance.
