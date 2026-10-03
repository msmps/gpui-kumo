# SkeletonLine acceptance

Tracker [#29](https://github.com/msmps/gpui-kumo/issues/29), KUMO-044. Pinned Kumo3fd5b648df578cb1ba214dedd30f475009f6a668, Kit/Base0.7.0, GPUI0.3.7. Full source/CSS/keyframes/docs/five demos inspected; no matching source tests found. Baseline120 tests/nine doctests.

Completes the existing Loader catalog with a small dependency-free loading composition. Base Timing/Easing supplies source CSS easing/delay/repeat behavior and Base initializes native reduced-motion policy. Presentation is native canvas paint, including rounded shimmer rather than arbitrary descendant clipping.

| Area | Acceptance |
| --- | --- |
| API | Inclusive integer percent width range30–100 by default, duration1.3–1.7s/delay0–0.5s rounded to hundredths, typed pixel height/block height; reject reversed/nonfinite/negative ranges |
| Ownership | Mounted stable ID stores sampled values/start time; only range changes resample, theme/height/layout changes retain; no new random dependency, subscription or retained callback cycle |
| Paint |8px line/2px radius, light#f3f4f6+black8% peak; darkwhite6%+white5% peak; three-stop source shimmer translated minus100% to plus100%, Base EaseInOut |
| Layout | Percent width follows container, block vertically centers; repeated lines, narrow/card composition, custom heights and zero width/height |
| Motion | Initial delay, repeat, paint actually changes; native reduced-motion adaptation uses centered static shimmer and requests no frames; invisible/unmounted lines stop scheduling |
| Semantics/input | Decorative placeholder, no interactive role/name/focus/activation; owning content provides loading status. Pointer/keyboard/disabled/focus/invalid/selected N/A |
| Verification | Observable geometry/paint/frame regressions, both-theme wide/narrow/animated/reduced native evidence, skeptical independent review, required Rust gate |

## Validated working checkpoint — 2026-10-03

Five regressions cover actual geometry/paint and lifecycle: retained sampled percentage across rerenders/themes/dimension changes and range updates; delayed eased gradient translation, rounded masks, reduced motion and unmount; invalid ranges/dimensions; wholly clipped no-paint/no-frame; fractional paint geometry with adjacent masks at three phases and scale1/1.25/1.5/2. Full gate passes125 workspace tests/nine doctests, formatting, warning-denied all-target/all-feature Clippy and locked builds (gallery included). Existing upstream profiler deprecation remains visible; no CI configuration/runs exist.

Independent review found clipped lines scheduling frames (Low), fixed with current-content-mask guard and observable regression. Native review found a gradient peak seam (Medium): GPUI rounds content masks outward to device pixels, double-painting a shared column. Fixed one absolute snapped boundary, including a reviewer-found normalized-float reconstruction risk; fractional-scale actual-paint regression guards adjacency. Final independent review has no blocker. Test probes initially compared physical paint with logical layout; corrected using actual window scale and GPUI layout pixel rounding. No weakened visual/behavior assertions or warning suppression.

Native Linux/Xvfb1040px/520px in both themes inspected for8px line/2px corners, block vertical centres, label baselines, spacing, nearby card border/radius and continuous gradients. Animated light captures differ by22,670 pixels, all within skeleton region; the default random widths remain identical across theme/size changes. Two reduced-motion dark captures one second apart are pixel-identical (zero changed pixels). No source screenshot/browser pixel acceptance claimed.

Evidence: light (capture removed), later animated light (capture removed), dark (capture removed), reduced dark (capture removed), repeat static capture (capture removed), narrow dark (capture removed), narrow light (capture removed).

Native adaptations: typed inclusive integer percentage ranges and logical-pixel dimensions replace CSS values/classes; reversed/nonfinite/negative inputs rejected. Time samples round to hundredths then clamp to the supplied typed range, avoiding invalid tiny/huge Duration conversion. Zero duration paints statically. Standard-library randomized hashing supplies presentation samples without a dependency or application-state random generator. Source has no reduced-motion CSS for SkeletonLine itself; native preference intentionally freezes centered shimmer with zero frame requests. The owner provides loading semantics, as the source placeholder is decorative. OS preference-change delivery and browser comparison remain separate acceptance gates (#10); #29 is resolved as implementation-complete; #10 owns that remaining comparison and preference delivery. Shared layer review now checks device-pixel boundary overlap, not just nominal gradient endpoints.

## Tracker consolidation — 2026-10-03

Implementation #29 closed after browser comparison and OS preference-delivery acceptance were transferred to #10. Current shared gate and macOS37139317488 pass; earlier no-CI statements describe the historical implementation checkpoint. No new visual/platform measurements are claimed by this consolidation.
