# Native runtime performance

Measured 2026-10-02 on MacBookPro18,1, macOS 26.6.2, 10 CPUs, 16 GiB RAM. Pinned dependencies and gallery component coverage are in [port progress](component-coverage.md). This answers the user's report of lag when scrolling a `cargo run` dev build.

## Development profile and findings

`Cargo.toml` sets `[profile.dev.package.gpui-pre] opt-level = 3` so ordinary `cargo run` optimizes GPUI rendering while retaining workspace debug checks. Keep that setting unless representative measurements justify changing it.

The unoptimized build has substantially higher full-gallery redraw cost. Release scrolling produced healthy frame intervals in the measured run. Animated gallery content also produces continuous work while idle; reduced motion stopped those draws. These are whole-gallery measurements, including Base, GPUI and platform integration. Incremental overhead of Kumo over an equivalent Base-only screen has not been isolated.

| Workload | Build | Draws | Mean draw | p95 draw | Other evidence |
| --- | --- | ---: | ---: | ---: | --- |
| Ten alternating theme changes, initial viewport | Debug | 10 | 62.91 ms | 84.28 ms | No presentation samples |
| Same, reduced motion | Debug | 10 | 64.82 ms | 79.43 ms | No presentation samples |
| Ten alternating theme changes, initial viewport | Release | 10 | 13.66 ms | 16.91 ms | No presentation samples |
| Same, reduced motion | Release | 10 | 14.41 ms | 18.69 ms | No presentation samples |
| Three down / three up scroll gestures | Release | 484 | 4.05 ms | 4.47 ms | 483 presentation intervals: mean 8.33 ms, p95 9.11 ms; eight input-to-frame samples: mean 6.64 ms, max 8.92 ms |
| Same scroll protocol | Debug | 1 | 55.72 ms | Single sample | No presentation/input latency samples; unsuitable for an FPS claim |
| Idle, normal motion, 13.42 s | Release | 1,610 | 4.16 ms | 4.55 ms | Presentation intervals averaged 8.33 ms |
| Idle, reduced motion, 11.73 s | Release | 0 | N/A | N/A | No draw/presentation/input samples |

The matching normal-motion theme redraw sample is about 4.6 times faster in release. Theme changes invalidate more content than ordinary scrolling; use these as distinct workloads. At 60 Hz the frame budget is 16.67 ms; at 120 Hz it is 8.33 ms. A mean below budget does not establish every frame is smooth. GPUI's draw timer measures CPU-side window drawing, not GPU completion or display scanout.

The idle sample represents approximately 6.7 seconds spent inside draw across 13.4 seconds elapsed. That is about half the elapsed interval in draw work, not a measured process CPU percentage. It is a concrete efficiency concern for the demo and warrants isolated animation/culling profiling.

[Raw selected histogram results](https://github.com/msmps/gpui-kumo/blob/838062e/docs/fixtures/performance-samples.csv) include the remaining percentiles and metric sample counts. Zero samples mean no evidence for that metric, not zero latency.

## Implementation costs and attribution

- Text, Badge, Banner and LayerCard construct layout and paint elements when rendered. Their normal static appearance adds no recurring timer. Button and Link add retained Base interaction/focus behavior and presentation elements.
- LayerCard's rounded secondary fill adds a shared bounds cell, a bounds canvas and a masked quad. It does not allocate a separate render texture. Focus rings similarly add quads; individual timing/allocation costs remain unmeasured.
- Loader uses GPUI's repeating animation and builds stroked paths plus cap quads for its arc/track. Loading Buttons share it. The gallery includes four standalone Loaders and six loading variant examples. Reduced motion preserves static indicators.
- The gallery composes every panel in one scrolling tree. It is not a virtualized catalog. Layout, element construction and painting can therefore involve work beyond the currently visible controls.

A ten-second macOS `sample` capture of the optimized gallery reported footprint `110.7M` and peak `115.3M`. Taffy flex layout was the largest named runnable leaf in the collapsed stack summary (441 samples), ahead of memory movement (373). Loader arc painting appeared in the call graph but did not dominate that summary. The intended redraw drive was interrupted by a computer-use state-change guard; treat this as a qualitative mixed-workload profile, not a percentage breakdown or a per-component benchmark. Raw capture remains in `/tmp/kumo-release-profile.txt` and is disposable.

## Repeat the measurement

The gallery's optional `frame-profiler` feature enables the selected GPUI snapshot's built-in histograms. Ordinary builds do not compile this diagnostic module or enable its profiler dependency.

```sh
cargo run --locked -p kumo-gallery --features frame-profiler
cargo run --release --locked -p kumo-gallery --features frame-profiler
```

Use Cmd-Shift-P to start an interval and Cmd-Shift-O to save it. Cmd-Shift-M toggles only this application's reduced-motion mode; hold that mode constant within an interval. The collector rejects an interval whose ending mode differs from its starting mode, or whose window was hidden at the start, end or any observed visibility transition in between. Re-raising the window does not make an interrupted interval valid; start a fresh one. Cmd-L switches theme. Output appends to ignored `target/frame-measurements.csv` with columns `profile,reduced_motion,seconds,metric,samples,mean_ms,p50_ms,p95_ms,p99_ms,max_ms`. Starting again replaces the unfinished interval; Stop with no interval is inert.

Native protocol: 1040×800 logical window, 2× captures, one registered application process, initial viewport. For scroll samples, send three down and three up gestures at screenshot coordinate `[500,400]`, one page each, with an accessibility observation after each gesture. For full redraw samples, alternate Cmd-L ten times with an observation after each. Both builds used the same profiler and baseline. Idle intervals had no deliberate user input between Start and Stop. The two theme/motion samples do not include cold startup.

## Foreground presentation protocol

The opt-in profiler gallery displays `Presentation: Visible` or `Hidden` and observes platform visibility changes. Ordinary gallery builds do not add this diagnostic row. On macOS, a shown but fully occluded window can accept input and update accessibility while its display link is suspended. This was the cause of the stale-pixel reproduction tracked in [#51](https://github.com/msmps/gpui-kumo/issues/51); raising the actual window restored matching state and pixels without resizing or changing library invalidation.

1. Launch one gallery process. Close previous gallery/probe instances and verify process absence before launching another.
2. Bring the actual window to the foreground. For accessibility automation, use the window's exposed `Raise` action, then observe its state. Verify `Presentation: Visible` in both accessibility and pixels before interpreting captures. If it remains Hidden, uncover or enlarge the window and recheck.
3. Compare each input's counter/value, accessible state and subsequent captured frame from the same process. Accessibility and screenshot capture are separate observations; allow the resulting frame to present before judging a mismatch. Repeat theme, availability and Input changes as well as activation counters.
4. Check both themes, wide and narrow layouts, alignment and layered corners using the [verification branches](gpui-verification.md#select-checks-from-the-contract). This workflow does not broaden the documented arbitrary-descendant clipping contract.
5. Start a fresh measurement only while Visible, hold motion mode constant, and stop before hiding or switching away. A rejected interval produces no CSV rows. The regression test exercises hidden-at-start and mid-interval occlusion followed by visibility restoration through the test platform's real visibility observer path; it does not simulate AppKit presentation.
6. Quit the gallery/probes and verify process absence. Keep screenshots, traces and logs in `/tmp` or ignored `target/`, and record acceptance outcomes in the issue rather than committing generated evidence.

## Limits and next measurement

The older measurements above did not record visibility. Their zero-presentation intervals cannot establish foreground scrolling performance; the successful presentation streams remain measured evidence for those intervals. Resolving #51 establishes a reliable native inspection workflow, not a new debug scrolling FPS claim or a complete explanation of debug/release performance differences.

Next: measure animation work with Loader-only and equivalent static fixtures, then measure representative Kumo controls against Base-only equivalents. Optimize the measured dominant path after confirming a repeatable regression. Preserve the ordinary gallery's component behavior while collecting this evidence.
