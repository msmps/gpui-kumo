# KUMO-058: Reproducible Rust validation

GitHub: [#43](https://github.com/msmps/gpui-kumo/issues/43). Milestone: Reproducible migration validation. Status: Reopened — a subsequent unchanged-code run exposed a scheduling-sensitive Switch regression; [KUMO-060](060-animation-clock.md) repairs its clock boundary.

No workflows existed at published33ab766. Implement locked formatting, workspace all-feature tests/doctests, warnings-denied Clippy/all-target builds and default gallery build; exact excluded AccessKit adapter default/all-feature tests, formatting and warnings-denied Clippy. Record host/toolchain and retain root patches. The successful remote run below is retained evidence; repeatability requires the KUMO-060 repair and new passing gates.

Workflow uses macos-15 and immutable checkout v4 revision, read-only contents and no credential persistence; pinned Rust1.99.0, exact lockfiles and dependency patches. Shared commands: `bash scripts/check-rust.sh`. GPUI/Base consumer rendering tests exercise the patched behavior. Whole upstream suites still have documented missing assets/non-workspace harness limits; do not imply those run. Native pixels/input/speech/IME/platform acceptance remain separate.

## Observed remote acceptance — 2026-10-03

Run [37114390570](https://github.com/msmps/gpui-kumo/actions/runs/37114390570) passed the complete shared script at `c7fd5541cf8627303936eaae07c4e94165c5c870`: macOS 15.7.9, macos-15-arm64 runner, Rust 1.99.0 (`b940084d7`), Cargo 1.99.0 (`5f94df478`), `aarch64-apple-darwin`. Logs confirm 169 library tests, one gallery test, nine doctests (seven plus two), adapter 12 default/14 all-feature tests, formatting, warnings-denied Clippy, locked all-target/all-feature builds and default gallery build. Existing GPUI profiler deprecation and `block` future-compatibility notices remain visible.

The first run 37114282490 was cancelled after the documentation push triggered its successor; it did not establish a test failure. Permissions and immutable checkout were reviewed against the pushed workflow. Root patches and consumer regressions remain in place. [Evidence and the separate Linux environment limits](../evidence/cloud-continuation-2026-10-03/README.md).
