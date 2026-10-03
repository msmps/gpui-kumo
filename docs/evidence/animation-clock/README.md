# Duration-animation CI repair evidence

Code:9ef600b1df7e3c74d133708b7dda57af1d7c5b96; test-import correction:85871b7689f987d3cdb035572e9f4b17687523f9. [Patch boundary/provenance/review/downstream/removal contract](../../gpui-animation-clock-patch.md). GitHub:#45/KUMO-060, blocking repeatable#43/KUMO-058.

## Actual measured outcomes

- Previous unchanged implementation passed full run37114390570 atc7fd554.
- Documentation-onlye06c866 failed [run37115097659 / job111180268250](https://github.com/msmps/gpui-kumo/actions/runs/37115097659/job/111180268250):168passes/one Switch failure atswitch_tests.rs:193, intermediate color equalled endpoint. [Verbatim excerpt](repeat-failure-excerpt.log).
- First clock candidate9ef600b failed [run37115532266 / job111181514619](https://github.com/msmps/gpui-kumo/actions/runs/37115532266/job/111181514619) during test compilation: missing InteractiveElement import for id. [Verbatim excerpt](first-candidate-compile-failure.log). Corrected in85871b7; no assertion changed.
- Repaired [run37115645935 / job111181831368](https://github.com/msmps/gpui-kumo/actions/runs/37115645935/job/111181831368) passes full shared script at85871b7:170library tests,1gallery,9doctests (7+2), workspace/exact adapter formatting, warnings-denied Clippy, locked all-target/all-feature/default-gallery builds, adapter12default/14all-feature tests. Actual Switch, Loader and new chained-animation regression all pass. [Verbatim excerpt](first-repaired-gate-excerpt.log). Host:aarch64-apple-darwin, Rust1.99.0(b940084d7), Cargo1.99.0(5f94df478), macos-15 runner. Dependency deprecation/future-compatibility notices remain visible.

The second complete unchanged-source gate [run37116002411 / job111182839628](https://github.com/msmps/gpui-kumo/actions/runs/37116002411/job/111182839628) at documentation-only5ea81bb7f8ff6a26adb1e02b0228d3fdac2865a8 also passes every command with the same170library/1gallery/9doctest/12default/14all-feature adapter counts. [Verbatim repeated excerpt](repeated-gate-excerpt.log). Both full runs were inspected before #45/#43 closure; resolution comments record the final documentation SHA. Final status-only documentation does not alter this verified source.

## Local and native limits

Workspace/exact adapter formatting and rustfmt of the changed vendored animation source pass locally. Linux tests cannot run because cached hdrhistogram metadata and adapter endi1.1.0 package are missing, and the configured managed proxy cannot be reached for normal Git/Cargo downloads. [Exact source-recovery/runtime/network evidence](../cloud-continuation-2026-10-03/README.md). No pins/locks changed and no network route bypassed. Original checkout is untouched; source snapshot remains separate.

Production dispatcher defaults to the same monotonic Instant now routed consistently through its existing executor clock. Three source substitutions only; easing/durations/repeating/chaining/reduced-motion/frame scheduling contracts unchanged. This CI gate verifies actual consumer rendered paint/geometry/motion assertions, including a host stall exceeding the Switch transition, but does not establish new native pixels/input/OS IME/speech acceptance. Native #42 foundation matrix, #26 dropdown recreation and #38 font work remain open.
