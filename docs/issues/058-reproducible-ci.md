# KUMO-058: Reproducible Rust validation

GitHub: [#43](https://github.com/msmps/gpui-kumo/issues/43). Milestone: Reproducible migration validation. Status: In progress.

No workflows existed at published33ab766. Implement locked formatting, workspace all-feature tests/doctests, warnings-denied Clippy/all-target builds and default gallery build; exact excluded AccessKit adapter default/all-feature tests, formatting and warnings-denied Clippy. Record host/toolchain and retain root patches. Observe actual successful remote run before closure.

Workflow uses macos-15 and immutable checkout v4 revision, read-only contents and no credential persistence; pinned Rust1.99.0, exact lockfiles and dependency patches. Shared commands: `bash scripts/check-rust.sh`. GPUI/Base consumer rendering tests exercise the patched behavior. Whole upstream suites still have documented missing assets/non-workspace harness limits; do not imply those run. Native pixels/input/speech/IME/platform acceptance remain separate.
