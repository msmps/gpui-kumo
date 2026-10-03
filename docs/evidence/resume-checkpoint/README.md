# Independently reproduced macOS validation

Host: aarch64-apple-darwin. Pinned Rust1.99.0, Base/Kit0.7.0, GPUI family0.3.7. `macos-before-shortcut-repair.log`:166 library passes/three InputArea test failures. Exact Base bindings establish test-only platform modifier error; no production editor change. Independent reviewer found no weakened assertion or input path.

`macos-rust-gate.log`: complete `bash scripts/check-rust.sh` passed before final preview shortcut simplification (which removes unused, unverified preview keyboard actions).169 library tests +1 foundation gallery test +9 doctests; workspace and adapter formatting, warning-denied workspace all-target/all-feature Clippy/build/default-gallery build; adapter12default/14all-feature tests and warning-denied Clippy. Final affected gates completed successfully in `macos-final-rust-gate.log` after preview simplification; the full script exited0. Existing dependency profiler deprecation/block future-compatibility notices are visible and distinct from workspace Clippy errors.

No Linux native, speech, IME or CI remote pass independently reproduced here. See ../foundation-layout/README.md for incomplete native visual review. Historical candidate174-test/source-browser evidence belongs to missing cloud work, not this tree.
