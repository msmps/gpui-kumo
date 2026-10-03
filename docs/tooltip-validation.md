# Tooltip core acceptance and evidence

Baseline: Kumo `3fd5b648df578cb1ba214dedd30f475009f6a668`, complete `components/tooltip/tooltip.tsx`, source tests, documentation and demos inspected. GPUI Kit/Base0.7.0; GPUI family0.3.7. This checkpoint is an interactive core, not complete Tooltip parity.

## Contract and dependencies

Retain one `Entity<TooltipState>` per mounted tooltip. `Tooltip::new(id, &state, text, trigger_factory)` composes a Kumo Button; the factory preserves consumer activation and value ownership. Four `Side` values, three `Align` values, configurable `Duration` opening/closing delays defaulting to600ms/0ms. `TooltipEvent` emits only actual open changes. State disclosure availability is distinct from caller-owned Button availability. Use weak consumer captures. No focusable popup contents or focus transfer.

Base0.7.0 supplies Tooltip popup role and Positioner collision/placement. Its TooltipOverlay hardcodes500ms opening and300ms grace closing with no public timing customization or trigger part. A retained controller composes the actual Base-backed Button hover/focus/activation and cancellable weak entity tasks to preserve Kumo's timing contract. This is a documented adaptation, not a claim that Base's lifecycle was reused unchanged.

The explicit weak-list `TooltipProvider` mounts above participating focusable controls, dismissing open/pending disclosures on Escape even with another trigger focused. GPUI application shells bind Tab/Shift-Tab to native focus_next/focus_prev; the gallery now demonstrates those bindings. Tests separately mark keyboard input and exercise the same native focus navigation. Source provider grouping semantics remain pending.

## Parity matrix

| Area | Core acceptance | Evidence / remaining gap |
| --- | --- | --- |
| API/parts | Retained state, text content, Button factory, side/align/delays, change events | Implemented; rich noninteractive content and arbitrary native triggers pending. Deprecated asChild excluded |
| Themes | Semantic base/default/line, source shadowMd, roundedMd,10px×6px padding,13px copy | Both themes rendered; pinned browser comparison pending |
| Geometry | Four sides, three aligns,10px gap, viewport fitting, moving anchor | Rendered tests assert all24 theme/placement/align combinations and open-trigger movement. Native wide/narrow review |
| Hover |600ms open;0ms leave; cancellation on exit/disable | Actual hover tests599ms closed/600ms open, leave immediately closes, pending exit/disable cancels |
| Focus | Keyboard focus discloses immediately; pointer focus does not disclose; no tooltip focus stealing | Native focus navigation tests, Escape retains trigger focus; gallery application bindings |
| Activation | Pointer/Space/Enter each retain consumer callback once and dismiss | Keyboard counters and provider dismissal tests; native sequence reviewed |
| Disabled | Tooltip-disabled state cancels pending/open; unavailable Base Button inert | Disclosure cancellation tested; Button's existing availability tests remain applicable |
| Escape | Focused trigger and provider capture suppress reopening until exit/reentry | Tested focus preserved and outside-trigger focus dismissal |
| Persistent content | Hovering tooltip content / safe passage through10px gap | Native bounded trapezoid bridge and popup hover implemented; all four sides/both themes tested. Browser pointer-intent heuristics differ (documented below) |
| Motion/provider |150ms source opacity/scale and grouped instantaneous switching | Pending; current popup appears/disappears without motion |
| Accessibility | Base role Tooltip; Text label metadata | Trigger-to-tooltip spoken relation, native platform screen reader verification pending. Role alone is insufficient |
| Edge cases | Unicode, long text, narrow widths, repeated retained instances | Gallery exercises; empty text/oversized height and nested overlay stress pending |
| Selection/loading/invalid | Not applicable to tooltip state | Caller trigger states remain caller-owned |
| Label/Field integration | Source optional contextual help | Depends on this core; pending |

## Skeptical review

Independent reviewer found two medium geometry defects before acceptance: button measurement changed a Cell without notifying and the10px exclusion dilation also shifted Start/End. Fixed guarded changed-bounds notification plus an animation frame, and expand only the placement's main axis. Installed Base chooses flipping before applying offset: main-axis exclusion includes the gap in fit selection while preserving the original cross-axis edges. Actual rendered regression asserts correct alignments and10px gap; moving an open trigger asserts80px movement. Stable geometry does not schedule continuous redraw.

Actual Button hover/measurement hooks avoid a stretched column wrapper opening or anchoring a tooltip over empty space. Timer tasks are cancelled by dropping the retained Task; weak captures avoid ownership cycles. Popup border uses source outline rather than a second inside border; shared Popover arrow paths use Tooltip's10px edge inset without changing Popover's18px inset.

## Validation

100 workspace tests and nine doctests passed; formatting, warning-denied all-target/all-feature Clippy and affected workspace/example builds passed at the core gate. Re-run after any subsequent changes. Native Linux software Vulkan/Xvfb captures are in `docs/evidence/tooltip-*.png`; final capture review results are recorded in the port checkpoint. Required macOS/screen reader/browser checks remain unrun here. No repository CI workflow is configured.

Final native review:1040×800 Top disclosure has a clean outline/arrow join in light/dark;520×800 long Unicode content wraps inside the4px viewport margin without corner clipping. `tooltip-activation-dark.png` shows counter3 after one pointer, Space and Enter activation and no popup. `tooltip-keyboard-dark.png` shows Tab moving focus to Bottom and disclosure; `tooltip-escape-dark.png` records dismissal with the keyboard focus outline retained. Initial1s capture ran before a presented popup; repeated3s settled capture confirmed disclosure, without changing600ms tested timing. Screenshots establish appearance only; event/timer/focus assertions are separate tests. Apps were terminated after review.


## Implementation notes

Kumo's lockfile selects `@base-ui/react`1.8.0. Matching upstream TooltipRoot/TooltipTrigger inspected at Git tag `v1.8.0`: `disableHoverablePopup=false` and `safePolygon()` are the default. Public npm access was unavailable; the same-tag GitHub source was readable. The native implementation uses a bounded trapezoid connecting the facing trigger/popup edges, based on Base's actual resolved/flipped geometry. It is direction-independent and does not reproduce browser velocity/pointer-intent heuristics. No additional delay is imposed outside that region.

While open, a frame-scoped weak MouseMove capture handler preserves disclosure over the trigger, bridge or popup. Outside movement schedules the configured closeDelay once; returning cancels it without another open event. Keyboard-focus persistence remains until blur/Escape/activation. Pending opening still cancels on trigger exit. Returning to an already-open trigger must cancel closing rather than schedule another600ms opening; a regression explicitly covers reentry followed by immediate exit.

Acceptance tests traverse gap→popup→trigger→outside in four sides and both themes; a200ms closeDelay remains open at199ms, cancels on return, and closes at200ms on a subsequent exit. Independent skeptical review found no blocker and required documenting the browser heuristic difference. Native gap and popup captures in both themes verify persistence while the pointer rests; exit capture verifies dismissal.100 workspace tests/nine doctests, warning-denied Clippy, formatting and workspace/example builds pass. Earlier wide/narrow popup appearance remains unchanged.


## User-found arrow separator repair

The initial native review incorrectly accepted a horizontal separator across the arrow mouth. Kumo's three SVG paths blend their base fill into the popup: popup outline must paint underneath the SVG child. Tooltip had reversed that order (Popover already uses the correct order). Paint Tooltip outline first, then arrow paths. Updated native captures `tooltip-arrow-join-light.png`/`tooltip-arrow-join-dark.png` show a continuous body; at the unchanged join centre(80,202), light changed from(230,230,230) to base(255,255,255), dark from line(51,51,51) to base(15,15,15). Adjacent outline pixels remain unchanged. Pixel checks and rendered captures establish this repair; compilation alone did not catch it. Earlier core/gap screenshots retain the pre-repair state and must not be used as final visual evidence. Hover-popup captures were refreshed after repair.

Shared verification now explicitly checks arrow-to-body joins, including outline paint order and absence of a separator through the mouth, in addition to rounded corners, alignments and layers in both themes.

Follow-up native review captured all four placements in both themes (`tooltip-arrow-*`) and refreshed both520px narrow captures. Arrow mouth joins now remain continuous on top, bottom, left and right; surrounding outlines, rotated edge strokes, corner radii and narrow Unicode wrapping remain intact. Independent review confirmed the local order fix matches Popover and changes no geometry or events. Final100 tests/nine doctests and all Rust gates pass after the repair.
