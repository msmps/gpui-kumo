# Breadcrumbs acceptance

Tracker [#12](https://github.com/msmps/gpui-kumo/issues/12). Kumo3fd5b648, Kit/Base0.7.0, GPUI0.3.7. Complete291-line source, documentation and all five demos inspected. No matching component tests in pinned Kumo; Figma generator tests are not interaction tests.

Highest-value common navigation family depends on Link/Button/SkeletonLine, semantic tokens and native clipboard. No Base Breadcrumbs root: own presentation/native navigation semantics and reuse Base-backed Link/Button behaviors. No router infrastructure.

| Area | Acceptance |
| --- | --- |
| API/parts | sm/base, explicitly composable links/separators/current/loading/copy/extras and decorative icons, injected destination callbacks |
| Geometry |13px/40px/2px gap sm;14px/48px/4px gap base;16px end margin; ancestor links nonshrinking, current-only truncation;24px authored chevron/4px inner gaps |
| Responsive | Window viewport below640px: more than two crumbs collapse to ellipsis+separator+last two+separator, preserve extras; retained item IDs/focus remain logical |
| Paint/states | Both themes, subtle ancestors/default medium current/inactive separators;125px loading SkeletonLine; copy ghost/square/sm, regular Copy vs bold success Check, group hover/keyboard visibility and2000ms feedback |
| Behavior | Pointer/Enter/Space routing once through Base; owner href updates authoritative; hidden ancestors not focusable/active; disabled adaptation gates paths; native clipboard copy once, replacement clears feedback, empty no copy |
| Semantics | Actual Navigation/name, Link/destination/name, noninteractive current with AccessKit AriaCurrent::Page; decorative separator/ellipsis no nodes; clipboard named Button. OS spoken acceptance separate |
| Edge cases | Empty/root-only/two/many parts, long Unicode current, narrow rows, multiple/reordered instances, loading→current, focused responsive changes |
| N/A | Selection/value/invalid/open, focus trapping/dismissal, IME/input editing; disabled only actionable native parts |
| Verification | Observable rendered input/geometry/focus/clipboard regressions, source review, both-theme wide/narrow native alignment and clipped rings/layers, full Rust gate/independent review |

Native adaptations: explicit injected navigation (no automatic browser launch); native viewport uses source640px breakpoint; keyboard copy visibility improves on pointer-only source hover. Focus indication remains native and must be inspected inside overflow root. Historical pre-implementation acceptance.

## Implementation notes

Five rendered regressions pass:640/639px breakpoint and both-theme part geometry,48/40px source density, full current name with truncated presentation,125px loading width, authoritative route updates and once-only pointer/Enter/Space routing, disabled routing, hidden focused ancestor action exclusion and visible parent focus retention, empty trail, responsive extras reorder with preserved action focus, Unicode clipboard writes, pointer-focus ownership before keyboard activation, repeated2000ms feedback with1999/2000 boundary, payload replacement/empty no-op and disabled copy paths. Actual painted keyboard-ring bounds stay inside the source overflow root.

Native Linux software Vulkan1040×1000 and520×1000: ancestors retain width, below640px the first ancestor disappears and ellipsis/last two appear, current label alone truncates, decorative24px chevrons and16px actual Phosphor House/Copy/Check align with text, ghost copy reveal/feedback and root-only icon row reviewed in both themes. Source125px loading SkeletonLine inspected in both themes. Rounded copied ghost action and panel layers have no fringe/clipping discrepancy. Captures in [evidence/breadcrumbs](https://github.com/msmps/gpui-kumo/tree/838062e/docs/evidence/breadcrumbs). Actual Linux clipboard read through Tk/X11 yields `https://example.test/projects/café` after pointer and Space copies; native clipboard failure remains unreportable by GPUI API.

Review fixes: Medium copied interval1500→source default2000ms (complete utility inspected, not inferred from InlineCopyText); Low CopySimple→actual regular Copy, regular→bold feedback Check. Native missing glyph fixed by explicit gallery SVG tint, then gallery House replaced with exact pinned Phosphor2.1.10 path. Native root clipping cut an outer ring's left edge; breadcrumb-only rings now draw inset with painted containment regression. Normal Link behavior preserved. Clippy caught large Part variant; box its consumed Link rather than suppressing lint.

Native adaptations: neutral outer row prevents source flex-grow from expanding authored row height in a column; breakpoint uses native logical viewport. Breadcrumb ancestor style privately reuses existing Base-backed Link without generic Link's underline/hover fade/wrapping. Routing remains caller-injected. Keyboard copy visibility, inset ring and copied accessible action name improve native feedback; source keeps fixed "Copy" name. Native Navigation and supported AccessKit `AriaCurrent::Page` are authored; role/name geometry tests and API source inspection do not establish OS spoken current-page behavior.

Remaining acceptance: source opacity transition on copy reveal, web arbitrary root styles, pinned browser/OS screen-reader checks. Source root overflow can still hide an ancestor beyond available width, intentionally preserving ancestor nonshrink; narrow current truncation is tested. Source pointer title translates to action accessible name; no generic native title tooltip claimed. #12 stays open for these gaps.

Rust gate:134 workspace all-feature tests/nine doctests, formatting, warning-denied all-target/all-feature workspace Clippy and all-target/all-feature build pass. Logs `/tmp/breadcrumbs-final-{test,clippy,build,fmt}.log`. Pre-existing upstream profiler deprecation warning remains visible, no suppression. Native gallery closed after review. No CI config/Actions runs exist; no CI pass claimed.

Asset provenance: gallery House, reused regular Copy and bold Check from Phosphor React2.1.10, MIT; existing `crates/gpui-kumo/assets/PHOSPHOR-LICENSE` applies. Chevron copies the exact pinned Kumo path.

## Quick-win completion

Breadcrumbs implements `Styled` for its actual navigation root. Its copy action now uses the pinned 100ms opacity transition with cubic-bezier(0.4,0,0.2,1), retained root hover, keyboard visibility, interruption and reduced-motion behavior. Existing composable links/current/separators/extras remain supported. Rendered tests verify caller geometry, preserved names/roles, reversal and cessation of frame requests.

This supersedes the historical implementation gaps above. Issue #12 is resolved for its bounded implementation scope. Catalog/browser and OS reduced-motion delivery remain #10; speech remains #2–#4. [New native matrix, source fixture and review](validation-fixtures.md). Full locked workspace/adapter gate passes232library/1gallery/9doctests and adapter12/14, formatting, warnings-denied Clippy and builds.
