# InputArea acceptance

Tracker [#23](https://github.com/msmps/gpui-kumo/issues/23), KUMO-038. Kumo3fd5b648df578cb1ba214dedd30f475009f6a668; Kit/Base0.7.0, GPUI0.3.7 with existing Tab registration patch. Complete source, matching Input test subsection, docs and all twelve demos inspected before implementation. Starting baseline109 tests/nine doctests; checkpoint113 tests/nine doctests.

## Selected milestone and ownership

Highest-value remaining form primitive; depends on existing shared Input metrics, Field/Tooltip and retained Base Textarea. Retain one InputAreaState/Base Textarea entity and explicit Change/Focus/Blur events. Source Enter inserts newline, Tab exits: do not inherit Base indentation. Programmatic set_value emits no Change. No recreated editor/duplicated value. Base supplies selection/history/Unicode/clipboard/IME internals; OS IME workflows remain separately unverified.

Source rows unspecified follows browser two-row default, despite demo describing four as default; four-row example must be explicit. All sizes replace Input height with content height and8px vertical padding; retain source X padding, typography/radius. [Base growth repair](gpui-base-textarea-growth-patch.md) reconciles existing content and width/font changes. Base TextElement's intrinsic rows are supplied only in AutoGrow mode; fixed rows use min=max clamp so switching policies preserves editor/selection. Positive rows enforced; contradictory max<min normalizes to min. Source manual textarea resize handle has no automatic native counterpart: fixed rows are owner-adjustable initially; native pointer resize and source scrollbar treatment pending, not claimed parity.

## Parity matrix / acceptance before coding

| Area | Acceptance |
| --- | --- |
| API/parts | Named retained state; value/selection/placeholder/availability; consumed four-size, rows, auto_resize/min/max, label/optional/helper/error/help; source Textarea alias |
| Themes/geometry | Semantic control/text/placeholder/focus/error;8px vertical padding, source X padding/radius/text; default/focused/invalid/disabled wide/narrow both themes |
| Row policy | Default2, explicit2/4/8; auto minimum1 or configured, maximum optional; grows/shrinks after edits and owner updates; width rewrap; capped content remains scrollable/caret reachable |
| Interaction | Pointer focus, multiline Enter/ShiftEnter, arrow selection, clipboard Unicode/newlines, undo; Tab/ShiftTab exit; disabled/read-only reject editing while read-only permits selection/copy |
| Ownership/events | Single retained editor across sizes/themes/policies; one Change per Base edit, no Change for owner updates; no stale controlled state |
| Focus/composition | Label forwards focus unless disabled; help separate; hidden help cleanup; no trap/open/selected/loading (N/A) |
| Semantics | Actual MultilineTextInput name/value/description/availability/invalid; action support verified against actual APIs; do not reuse one-line synthetic geometry for wrapped/multiline content |
| Edge cases | Empty/placeholder, long and wrapped Unicode, trailing newline, narrow width, repeated entities, invalid message exactly once, cap overflow |
| Pending checks | Rich slots, browser resize/scrollbar/motion comparison, OS IME and screen readers, platform export and multiline synthetic text geometry |

## Validation / skeptical review

Implementation checkpoint verified below. Run formatting, workspace tests/doctests, all-target/all-feature warning-denied workspace Clippy and build. Inspect gallery both themes at wide/narrow widths, grow/shrink/cap, focus/caret/corners/layers and actual Tab traversal. Review actual diff and observations independently before accepting checkpoint; record failures and fixes here.

Review findings so far: Medium Base policy/rewrap intrinsic rows stale; repaired narrowly without text resets (see patch boundary). Medium disabled pointer stole outside focus despite bubble prevent_default; disabled capture now prevents default and propagation before child focus paths. Four regressions exercise multiline/clipboard/undo/Tab, availability, row sizing, and selection/history preservation.

## Checkpoint evidence — 2026-10-03

Passed:113 workspace tests/nine doctests, formatting, all-target/all-feature build and workspace Clippy with -D warnings. Four added observable InputArea regressions pass. Existing GPUI profiler deprecation remains visible, unchanged. No CI workflows configured. Attempted `cargo test -p gpui-base --lib test_auto_grow --locked --offline` did not run: Cargo rejects testing a non-workspace package requiring dev-dependencies. Consumer integration regressions validate the patched dependency; no upstream suite pass claimed.

Linux Xvfb/software Vulkan native gallery reviewed: light (capture removed), dark (capture removed), narrow light (capture removed), narrow dark (capture removed), four-row cap light (capture removed), cap dark (capture removed), narrow cap dark (capture removed), shrink to minimum (capture removed), native multiline clipboard paste (capture removed), native Tab exit (capture removed), owner update (capture removed), fixed policy (capture removed), fixed empty policy (capture removed). Native owner updates and policy switches leave47 edit callbacks unchanged; clearing fixed four rows keeps height, while auto mode shrinks to two. Capped keyboard editing shows final four lines and keeps caret reachable. Native captures can lag an input batch; final cap evidence was captured after the next frame settled and inspected, not inferred from initial stale capture.

Reviewer confirmed only two intended upstream files changed, guarded growth reconciliation after font/wrap/token geometry, retained state and availability capture. No blocking findings remain. Reviewed shared text padding, label/help centring, wrapped rows, caret position, error message count, rounded border/ring layers and both themes. Not run: browser pixel/motion/resize comparison, OS IME/screen readers, macOS/Windows. Remaining implementation gaps: source native-browser manual resize and thin scrollbar presentation, rich label/helper/error slots, multiline synthetic TextRun selection/action bridge and platform export. Current semantics expose actual MultilineTextInput name/value/description/availability/invalid; no fabricated one-line text geometry. #23 remains open for these items.
