# GPUI 0.3.7 duration-animation clock correction

Tracker: [#45/KUMO-060](https://github.com/msmps/gpui-kumo/issues/45), blocking repeatable [#43](https://github.com/msmps/gpui-kumo/issues/43). Exact gpui-pre0.3.7 vendor provenance/checksum/license and existing Tab correction remain as documented in [the Tab patch](gpui-tab-registration-patch.md). No version or dependency resolution changes.

## Reproduced problem and boundary

CI run37114390570 passed169 library tests atc7fd554. Documentation-onlye06c866 changed no source, manifests or lockfiles, yet run37115097659 failed the Switch intermediate track-color assertion with168 passes/one failure. Actual endpoint colors were equal at switch_tests.rs:193. The test slept40ms before sampling a150ms animation; host scheduling can resume it after the endpoint.

Selected AnimationElement initialized/restarted with scheduler::Instant::now and sampled start.elapsed for ordinary duration animations. Those calls use host time, while synchronized animations and the existing scheduler expose background_executor().now. This prevents virtual-clock consumer tests from controlling ordinary animations. PlatformDispatcher::now defaults to the same monotonic Instant; TestDispatcher supplies its controlled clock.

The additional vendor diff is exactly three substitutions in src/elements/animation.rs: initialize start, calculate elapsed and restart a chained segment with the executor clock. No animation API, duration, easing, modulo/repeat, segment transition, reduced-motion or frame-scheduling code changes. SpringAnimation is untouched. Native production uses the platform's monotonic clock; no test-only implementation branch or animation engine is added. The existing vendor source changes now span tab_stop.rs and elements/animation.rs, each with its own maintenance boundary.

## Observable regressions and review

Switch keeps all intermediate track-color/thumb-position, reversal continuity, endpoint and reduced-motion assertions. An intentional180ms host stall (longer than its150ms duration) now precedes a rendered start-position/start-color assertion, proving wall time cannot advance the simulated animation; only an explicit40ms executor advance moves it. The endpoint uses an explicit180ms executor advance. Loader keeps painted arc endpoint and reduced-motion frame-request assertions while advancing its repeating animation clock by250ms explicitly.

The chained consumer regression renders actual identified Div bounds at both segment midpoints and endpoints, verifying initialization, elapsed sampling and the second segment restart on the same clock, then reduced-motion settling/no continuing frame requests. These consumer tests use the actual patched AnimationElement; no upstream suite success is implied.

Separate skeptical source review checked all three clock sites, native clock provenance, unchanged segment/easing/repeat/reduced-motion branches, actual painted Switch/Loader assertions, and chained rendered geometry. No assertion was removed or tolerance relaxed. Local workspace/exact adapter formatting passes. Linux tests cannot resolve/download missing cached dependencies through the unreachable managed proxy. Remote compilation/tests/lint/build pass at85871b7 in run37115645935:170 library tests, one gallery test, nine doctests and adapter12default/14all-feature tests, formatting, warnings-denied Clippy and locked builds. The first candidate compile failed due to a missing test trait import, repaired in85871b7; no assertion changed. Repeated full run37116002411 at documentation-only5ea81bb also passes unchanged source with the same counts. [Raw evidence and repeated acceptance](evidence/animation-clock/README.md). Native runtime pixels/input/speech acceptance remains separate.

## Maintenance and downstream setup

Maintainers own this bounded clock correction. Remove it only after adopting a pinned compatible GPUI family with consistent executor-clock duration animations and passing Switch/Loader/chained regressions and native motion/preference checks. Remove independently of the Tab correction if upstream fixes only this boundary. Preserve GPUI/Base/AccessKit version pins and patches.

Downstream Cargo-root overrides do not propagate from library dependencies. Copy the exact vendored gpui-pre package and retain gpui-pre = { path = "vendor/gpui-pre-0.3.7" } under the consuming workspace's [patch.crates-io], alongside the documented Base/AccessKit overrides. An unpatched0.3.7 consumer still uses wall-clock duration animation sampling. No package release or upstream publication is performed.
