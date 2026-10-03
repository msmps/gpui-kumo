# InlineCopyText acceptance

Tracker [#22](https://github.com/msmps/gpui-kumo/issues/22), KUMO-037. Pinned Kumo3fd5b648df578cb1ba214dedd30f475009f6a668; Kit/Base0.7.0, GPUI0.3.7 with documented local foundation repairs. Complete source, tests, docs and all three demos inspected before coding. Baseline113 tests/nine doctests.

## Selected milestone / native ownership

Common dense-row composition using existing Text and Base Button; ClipboardText tooltip mode additionally depends on unported Toast, so this family comes first. Reuse Empty's stable-ID keyed feedback pattern. The component owns copied/hover/focus/timer state per mounted ID; owner supplies current content/payload, disabled flag and callbacks. Rich decorative content requires an explicit copy payload and full accessible visible text. No new public editor entity or value model.

Source has no variants of its own: expose valid copy and mono Text styles, excluding headings structurally. Default mono-secondary standard, truncate true. Source14px CopySimple/Check glyph,4px gap,2px radius and brand keyboard ring. Enclosing web `group` is an explicit native group_active flag supplied by an owner's hover/focus state. Copy event means native API submission; pinned GPUI cannot return clipboard write success/failure. Live-region speech/motion are separate unsupported/unverified gates. Owner copy-payload replacement clears stale feedback (documented native adaptation).

## Matrix / acceptance before coding

| Area | Criteria |
| --- | --- |
| API/parts | String content defaults payload; explicit copy override; rich content with explicit payload/full visible text; non-heading Text recipes, truncate, disabled, labels, on_click/on_copy |
| Default/hover/focus/copied | Hidden but reserved14px copy glyph; own hover/keyboard focus or enclosing group reveals; copied check visible1500ms after latest accepted activation; default mono-secondary upgrades on own hover/keyboard focus |
| Themes/geometry | Both-theme semantic Text/brand tokens,4px gap,2px radius, centres/baselines, narrow truncation and long Unicode; no own background/border |
| Input | Base pointer/Enter/Space once, consumer cancellation before clipboard, disabled current-focus rejection, Tab entry/exit |
| Lifecycle | Keyed state stable across rerenders/theme/text styles; repeated instances isolated, replace/reset weak timer, unmount does not retain a control |
| Semantics | Actual Button role and localized pre/post names; copied live region unsupported/unverified; no OS success claim |
| N/A | Loading/invalid/open/trapping/selected persistent state, heading and web as-element mechanisms |
| Remaining validation | Rust gate, meaningful regression tests, gallery both themes/wide/narrow, actual native clipboard, independent diff/render review |

## Validated checkpoint — 2026-10-03

Four observable regressions cover both themes: pointer/Space/Enter, cancellation and current disabled state; latest-copy timer replacement, payload replacement and repeated-instance isolation; supported Text recipes and14px glyph geometry; centering beside a40px Button. Full workspace gate passes117 tests/nine doctests, formatting, all-target/all-feature warning-denied Clippy and locked builds (including gallery). Existing upstream profiler deprecation remains visible; no CI configuration/runs exist.

Native Linux/Xvfb gallery: light/dark at1040px and520px inspected. Pointer followed by Space/Enter produces three activations/three copy callbacks; cancellation produces one cancellation without an extra copy. Actual Ctrl+V into retained InputArea reproduces UUID payload. Captures below show hidden/reserved glyph, hover/group reveal, copied check, compact keyboard ring, long Unicode truncation, rich content, disabled/cancelled controls, and centering beside taller Save.

Independent review found glyph tone must inherit the enclosing foreground rather than Text's tone (fixed). Native review found stretched focus geometry and unresolved generic Linux monospace (fixed with existing Button wrapper and centralized DejaVu Sans Mono mapping). A second review found self_start would break taller-row alignment; replaced it with the neutral wrapper and added the taller-sibling regression. Final review has no remaining diff blocker. Explicitly checked glyph/control centres, baselines, both parent axes, compact ring padding and nearby rounded layers in both themes. No own layered background or arbitrary descendant clipping claim.

Evidence: light (capture removed), hover (capture removed), copied (capture removed), keyboard focus/three callbacks (capture removed), clipboard round trip (capture removed), cancelled (capture removed), row group (capture removed), dark (capture removed), narrow light (capture removed), narrow dark (capture removed).

Remaining: source opacity/check transition motion, copied live-region speech and OS clipboard failure reporting are not implemented/verified; GPUI's write API has no success result. Browser pixel comparison and supported-platform screen-reader acceptance remain separate gates. #22 stays open. Process improvement: shared alignment checklist now includes intrinsic geometry in a column and centering beside a taller row sibling; verify concrete font fallback through actual paint, not family-name assertions.

## Quick-win completion — 2026-10-03

CopySimple now fades using the pinned 100ms opacity curve, with rapid reversal and immediate reduced-motion rendering. Check mounts immediately, as in the pinned source; the discarded Copy transition cannot leak into reset. A stable zero-size Label carries the localized copied message and polite live metadata, clears at reset/payload replacement and stays isolated across controls. Pointer/Space/Enter, cancellation, focus and clipboard regressions remain passing.

This supersedes the historical implementation gaps above. Issue #22 is resolved for its bounded implementation scope. Catalog/browser and OS reduced-motion delivery remain #10; speech remains #2–#4. [New native matrix, source fixture and review](evidence/quick-wins/README.md). Full locked workspace/adapter gate passes232library/1gallery/9doctests and adapter12/14, formatting, warnings-denied Clippy and builds.
