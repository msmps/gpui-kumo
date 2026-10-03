# Meter acceptance

Tracker [#25](https://github.com/msmps/gpui-kumo/issues/25), KUMO-040. Kumo3fd5b648df578cb1ba214dedd30f475009f6a668, Kit/Base0.7.0, GPUI0.3.7. Full source/docs/six demos and Base UI1.8.0 Root/Value/Indicator/clamp/valueToPercent inspected; no matching Kumo tests found. Baseline125 tests/nine doctests.

Small common quota composition before complex collection/overlays. Reuse Base ProgressTrack/Indicator neutral parts and Base transition; its Progress root has the wrong hardcoded role. Native GPUI/AccessKit exposes actual Meter and min/max/numeric/text metadata. Controlled value/range is owner-authoritative; only Base animation presentation is retained per stable ID.

| Area | Acceptance |
| --- | --- |
| API | Named Meter, value/range0–100 default, custom_value/show_value precedence, semantic indicator color and native format/value-text callbacks |
| Range | Finite ordered inclusive range, NaN raw→min/0%, infinity clamps, zero/full and degenerate range behavior inspected against source; reject invalid range |
| Text |12px subtle label,13px medium tabular value,16px header gap,8px root gap; nonempty custom text visible even show_value=false, empty falls back; default percentage half-up rounded |
| Paint |8px fill track/brand indicator, full rounding and source300ms EaseOut width; no arbitrary-child corner-mask dependence, both themes/zero/tiny/full |
| Motion | First render target width; retained Base interpolation on owner changes, rapid reversal, semantic/text target immediately authoritative, reduced motion stops frames |
| Semantics | Actual Meter role/name/current numeric/min/max and localized string value; custom visible text does not silently change source semantic text; no focus or interaction role |
| Composition | Long Unicode labels/value text and narrow layout; multiple independently animated meters; realistic quota/update gallery |
| N/A | Hover/pressed/focused/disabled/selected/loading/invalid/open interactive states, callback activation, navigation/dismissal/trapping |
| Validation | Observable metadata/paint/dynamic updates/frame regressions, both-theme wide/narrow actual native round-cap/alignment review, full Rust gate and independent skepticism |

Native formatting: deterministic default whole percentage; consumers provide formatting/value-text callbacks for locale, units and speech instead of importing web Intl. Unavailable spoken/platform/browser checks remain separate gates. Historical pre-implementation acceptance.

## Passed checkpoint — 2026-10-03

Four actual rendered regressions cover both-theme normalized geometry, numeric/accessibility metadata, custom/show precedence, default value's absence from accessible children, long Unicode label/value wrapping at160/240px, native formatting/speech callbacks, rapid animation reversal and reduced-motion frame cessation. The default value paints without an accessible child, preserving Base UI Value's `aria-hidden`; custom visible text retains its separate text semantics. The label names the actual Meter root. Unsupported OS speech behavior is not inferred from these metadata checks.

Native Linux software Vulkan gallery reviewed at1040×1000 and520×1000 in both themes:12px labels and13px tabular medium values align, source16px gap/8px track/root gap retained, long label/value wrap without overlap, zero/tiny/full rounded fills have clean inner/outer edges and no layered border discrepancy. Pointer Increase changes owner65→90; Decrease returns65. Six captures in [evidence/meter](evidence/meter/); narrow updated images show the owner target. Read-only meters add no activation/focus path; existing Button behavior supplies gallery actions.

Review findings: Medium forced value nonshrink fixed by normal shrinkable header children and a160/240px containment regression. Medium suspected huge-number normalization discrepancy rejected against exact pinned `valueToPercent.ts`: source multiplies before division too. Regression records source behavior (`1e307` in0..=1e308 becomes100% due arithmetic overflow); this is a known upstream numerical limitation, not mathematical correctness claimed by the port. Success/warning indicators use semantic **surface** fills rather than similarly named text tones. Default value avoids duplicating accessible numeric/text state.

Remaining: arbitrary web track/indicator class overrides translate only to typed semantic indicator colors; no generic styled track slots yet. Browser comparison and OS screen-reader checks remain unrun. Native locale formatting is an explicit caller callback; no Intl runtime is bundled. Platform reduced-motion delivery remains separate from tested GPUI policy. Keep #25 open for these acceptance gaps.

Rust gate: `cargo test --locked --workspace --all-features` passes129 tests/nine doctests; `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, all-target/all-feature build and formatting check pass. Logs `/tmp/meter-final-{test,clippy,build,fmt}.log`. Pre-existing upstream profiler AtomicUsize deprecation remains visible; no warnings suppressed. Native testing gallery closed after review. No CI configuration exists; Actions acceptance is not claimed.

## Quick-win completion — 2026-10-03

Meter now accepts `track_style(StyleRefinement)` and `indicator_style(StyleRefinement)`, applied after the semantic default recipe. The normalized animated width remains on its private owner-driven wrapper. A rendered regression checks custom height, fill/radii paint and unchanged numeric meaning in both themes. Existing numeric normalization, formatting and motion regressions pass.

This supersedes the historical implementation gaps above. Issue #25 is resolved for its bounded implementation scope. Catalog/browser and OS reduced-motion delivery remain #10; speech remains #2–#4. [New native matrix, source fixture and review](evidence/quick-wins/README.md). Full locked workspace/adapter gate passes232library/1gallery/9doctests and adapter12/14, formatting, warnings-denied Clippy and builds.
