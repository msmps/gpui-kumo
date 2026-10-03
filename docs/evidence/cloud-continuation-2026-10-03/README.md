# Cloud continuation and remote CI acceptance — 2026-10-03

Remote branch inspected: `work`, `c7fd5541cf8627303936eaae07c4e94165c5c870`. Code/workflow/script checkpoint: `744354a1f4c222920f6bc74dfd4cc70202623c73`.

## Remote gate actually observed

[Run 37114390570](https://github.com/msmps/gpui-kumo/actions/runs/37114390570), [job 111178292212](https://github.com/msmps/gpui-kumo/actions/runs/37114390570/job/111178292212), completed successfully. Job metadata and decoded logs were read through the GitHub connector. Host: macOS 15.7.9, macos-15-arm64 image, aarch64-apple-darwin; Rust 1.99.0 (b940084d7), Cargo 1.99.0 (5f94df478).

`bash scripts/check-rust.sh` passed: workspace formatting; 169 library tests, one foundation gallery regression and nine doctests (seven ordinary and two separately run); warnings-denied workspace all-target/all-feature Clippy; locked all-target/all-feature and default-gallery builds; standalone exact adapter formatting, 12 default/14 all-feature tests and warnings-denied Clippy. [Verbatim result excerpt](ci-results-excerpt.log). Existing GPUI profiler deprecation and block future-compatibility notices remain visible; no warnings were suppressed.

The first run37114282490 was cancelled after the successor push under the workflow concurrency policy. No repair was justified by a cancelled run. Reviewed the pushed workflow: work/main push, pull_request and manual triggers; immutable checkout revision, read-only contents, no persisted credentials, pinned toolchain. Reviewed shared commands and exact Cargo-root patches; consumer regressions execute the actual GPUI/Base corrections, and adapter tests execute state/event repairs. Whole upstream suites and native pixels/input/accessibility/speech/OS IME remain separate.

## Measured Linux continuation limits

Original checkout `/workspace/gpui-kumo`: clean work at31d2c5480988ce6fe30e5fd95ba0f9020edded54, only origin/main tracking ref, no stash or extra worktree. Preserved it. Connector branch inventory contains main and work, with no candidate checkpoint branch; chat attachments contain no candidate artifact. The accessible earlier Input task predates the reported Pagination candidate. This does not prove inaccessible sessions have no candidate.

Pinned Rust is installed under `/workspace/.rustup`; activate `/workspace/.kumo-setup/activate.sh`. Rust1.99.0 reports x86_64-unknown-linux-gnu. Xvfb and xdotool are available in the setup sysroot; a current native gallery was not built or run.

Reconstructed a separate source snapshot in `/workspace/gpui-kumo-current` from immutable c7fd554 through the GitHub connector and matching cached files. All636 source/configuration/document blobs, including exact vendor packages and lockfiles, were individually SHA-1 checked using Git blob framing against the recursive remote tree. Existing binary/text evidence files were excluded from this source snapshot. It is not a Git checkout or a replacement for fetching the branch.

Workspace `cargo fmt --all -- --check` and exact adapter `cargo fmt --manifest-path vendor/accesskit_atspi_common-0.19.1/Cargo.toml -- --check` pass. `CARGO_NET_OFFLINE=true bash scripts/check-rust.sh` exits101 during dependency resolution before running tests: missing hdrhistogram index metadata. Standalone adapter default test exits101 attempting to download endi1.1.0 offline. [Workspace raw outcome](linux-rust-gate.log), [adapter raw outcome](linux-adapter-test.log).

Ordinary `cargo fetch --locked` exits101 after three retries because it cannot connect to index.crates.io through the configured proxy. Git fetch and HTTPS probes to GitHub/static.rust-lang.org independently fail to connect to proxy:8080. [Cargo fetch outcome](cargo-fetch.log). `/etc/codex/network-policy.json` allows these hosts through the package-manager preset; no destination denial was observed. Runtime status reports running/connected but policy readiness unknown. Proxy routing was preserved. No credentials were inspected, dependency pins changed, assertions weakened or current native acceptance claimed.

Next action: restore managed proxy connectivity, fetch actual latest work, activate the setup and run the locked script; finish #42’s final Light/Dark1040/520 native text/shadow/full-gallery gate, then recover/recreate #26 over the pushed branch. #42/#26/#38 remain open. #43’s bounded macOS CI acceptance is satisfied independently of this Linux environment blocker.
