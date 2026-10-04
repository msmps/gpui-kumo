# SensitiveInput acceptance

Public naming, visibility, typed composition and failure policy follow the [canonical public API contract](design-system-components.md#public-api-contract). Use that contract when changing the APIs described here.

Baseline: Kumo `3fd5b648df578cb1ba214dedd30f475009f6a668`; complete521-line source, matching tests, docs and all demos inspected. GPUI Kit/Base0.7.0 and GPUI snapshot family0.3.7 retained. Tracker: [#28](https://github.com/msmps/gpui-kumo/issues/28), local GitHub #28.

## Ownership and native adaptation

Retain one SensitiveInputState entity per mount. Its InputState/Base editor is the sole value/selection/history owner. SensitiveInputState owns presentation mode, focus handles and the cancellable weak copy-feedback timer. Programmatic set_value emits no Change. User edits emit once per Base edit; subscribing consumers can reconcile/reject an edit through set_value rather than keeping an internal second value. No render-time entity creation or new dependency.

Base Button supplies pointer/Space/Enter and focus; retained Base Input supplies native selection, clipboard/Unicode/IME handling. Shared Input metrics keep source20/26/36/40px heights, source padding/radii/typography authoritative. Source uses DOM-exclusive event targets; native eye/copy overlays occlude underlying input/label hitboxes and stop activation propagation. Masked presentation is a named Base Button with fixed eight dots/instruction; retained editor is unmounted from the accessible presentation while masked. Empty with a programmatically inserted value follows the source's password-input mode and separate reveal eye. Native password semantics redact aria_value and omit plaintext synthetic text runs; Snapshot declines masked text. This is a metadata/privacy boundary, not OS screen-reader or protected text-service acceptance.

Read-only reveal moves native focus to the eye rather than leaving focus on an unmounted masked container. Hide with an empty value normalizes to Empty, preserving the source's editable empty presentation. Programmatic value replacement clears stale copy feedback; clearing a focused disappearing masked/copy/eye control moves focus to the retained editor. These explicit native policies preserve usable focus and accurate feedback without recreating value state.

GPUI write_to_clipboard returns no success/error result. Copy event means submission to that native API, not confirmed system delivery. The source browser async failure callback/fallback cannot be claimed equivalent; native round-trip evidence and this limitation must be recorded. Copy feedback uses the pinned2000ms duration and cancels/replaces older reset tasks.

## Parity matrix

| Area | Acceptance and status |
| --- | --- |
| API/parts | Retained state, value/set_value/placeholder, selected_value, availability, Mode, Change/Submit/Copy; consumed size/label/optional/helper/error/help builders. Rich ReactNode label/helper/error slots remain pending |
| Variants/sizes | Four source sizes; default/error-derived presentation. Deprecated explicit error variant omitted; error_visible owner contract replaces browser matching |
| Themes/paint | Central semantic colours, shared Input recipes, regular12/16px Phosphor eye/eye-slash, source right reserve20/24/32/40px, upper-right Copy tab; light/dark/wide/narrow native captures reviewed |
| Default/hover/focus | Masked fixed eight dots, source hover/focus instruction, copy tab on hover/focus; own copy hover keeps floating tab reachable; visible eye inherits effective hover/focus tone |
| Pressed/selected/loading | Base pressed activation retained; persistent selected/loading N/A. Source copy feedback is the relevant temporary state |
| Disabled/read-only | Disabled blocks reveal/edit/copy and removes actions; retained-focus pointer/keyboard rejection tested. Read-only allows reveal/copy and rejects editing |
| Invalid | Inner Input owns invalid ring/metadata only; outer Field owns exactly one error message. Control height stable through all modes |
| Value/modes | Initial nonempty masked, empty editable, typing reveals, programmatic Empty insertion stays password-masked, external empty masked becomes Empty; value/editor identity retained |
| Pointer/keyboard | Masked click/Space/Enter reveals once; eye hides/reveals; copy pointer/Space/Enter once; Escape hides and restores masked control; Tab traversal regression and native copy keyboard activation verified |
| Focus | Input/eye/copy share scope; moving between them must not hide; external blur hides; help remains separate. No trap (N/A). Read-only and disappearing-control native focus policies documented above |
| Semantics | Actual Base Button/TextInput/PasswordInput names and redacted values; masked editor not mounted. Spoken help relation/live region unsupported/unverified; platform metadata export remains separate |
| Edge cases | Empty/external insertion, Unicode/long secrets, repeated sizes, timer reset, disabled current focus, wide/narrow and corners/layering |
| Platform gaps | Browser pixel/motion comparison, OS IME and screen-reader workflows, native clipboard failure reporting, rich slots and localisation remain pending |

## Skeptical review findings

- Medium: inner Input and outer Field both rendered visible error, changing body height on reveal. Fixed by inner invalid-only presentation; both-theme/four-size body-height regression covers modes.
- Low: explicit subtle SVG paint ignored parent hover/focus colour. Fixed explicit effective glyph colour; both-theme native captures reviewed.
- Medium, rendered-input regression: floating copy tab competed with overlapping label and eye with input surface, losing pointer focus and keyboard-copy activation. Fixed native occlusion/activation propagation; assert actual copy focus before Space/Enter.
- Medium: clearing an externally populated Empty-mode value removed the focused reveal eye. Fixed conditional editor focus restoration and removed-eye hover cleanup; both-theme Tab-to-eye/clear/Tab-exit regression passes.
- Review/process: GPUI test activation is asynchronous. Settle platform activation before claiming focus-out listener evidence; after proper activation, external-blur regression passes without a component workaround.

Regular Eye/EyeSlash paths extracted from Phosphor Reactv2.1.10 defs; existing assets/PHOSPHOR-LICENSE applies. No runtime icon dependency.

## Validation

Pre-change baseline104 tests/nine doctests passed. Final checkpoint:109 workspace tests/nine doctests pass (four SensitiveInput regressions plus Input password privacy added). Formatting, workspace/all-target/all-feature Clippy with -D warnings and all-target/all-feature build pass. The existing upstream profiler deprecation remains visible; no workspace warnings suppressed. No CI workflows configured.

Linux Xvfb/software Vulkan gallery captures at1040×1000 and520×1000: light (capture removed), dark (capture removed), revealed dark (capture removed), narrow light (capture removed), narrow dark (capture removed), narrow read-only dark (capture removed). Reviewed icon centres, shared input heights, long-text reserve/clipping, copy-tab corners/border layering and invalid ring in both themes. Copy light (capture removed) records three callbacks from pointer/Space/Enter; clipboard light (capture removed) shows the unique native-cafe-secret value pasted into another retained editor. Read-only light (capture removed) preserves the long fixture after keyboard Backspace. Native captures precede the final disappearing-eye fix, which changes focus restoration only and has a both-theme interaction regression. Independent reviewer confirmed that fix and reviewed copy/narrow layered geometry with no remaining blocking finding.

Not run: browser pixel/motion comparisons, OS IME and screen readers, macOS/Windows native checks. Remaining API/presentation gaps: rich slots, localisation, source hover brightness/opacity transitions; clipboard success/failure contract is the native limitation above. #28 remains open for these acceptance items; working implementation does not mean full fidelity accepted.
