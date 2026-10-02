# KUMO-006: Input: provide accessible text ranges, selection and editing actions

Status: Blocked — integration implemented; remaining adapter/platform acceptance is listed below.

GitHub issue: Pending publication

## Problem

At the baseline, live Linux AT-SPI inspection found the named Input entry and its focus, but only Accessible/Component interfaces. Base supplies no accessible text-run subtree, so aria_value alone did not expose the Text interface. The integration below now exposes Text and selection; native accessibility editing and cross-platform acceptance retain concrete limits.

## Evidence

- [Native Input contract](../kumo-component-recipes.md#native-input-contract)
- [Input implementation](../../crates/gpui-kumo/src/input.rs)
- [Input tests](../../crates/gpui-kumo/src/input_tests.rs)
- [Observed AT-SPI limitation](../popover-validation.md#concrete-limits)

## Acceptance criteria

- [x] Expose an accessible text subtree with stable identity, current text, Unicode character boundaries and valid bounds.
- [x] Expose caret/selection state and route supported accessibility editing/selection actions into the retained Base editor.
- [x] Test updates, selection and composition with non-ASCII text without duplicate Change events or a second editing engine.
- [ ] Validate text reads and selection/editing through live AT-SPI and the target platform adapters.
- [x] Preserve disabled/read-only restrictions and avoid introducing entity retention cycles.

Baseline: commit `38c2ea8`, GPUI Kit/Base 0.7.0 and GPUI 0.3.7. Where a browser comparison is needed, use Kumo commit `3fd5b648df578cb1ba214dedd30f475009f6a668`.

## Base audit and implementation — 2026-10-02

Checked the resolved Base 0.7.0 and GPUI 0.3.7 sources, without changing dependencies:

| Existing support | Reused integration |
| --- | --- |
| `InputBase` supplies TextInput role, accessible label, fluent metadata and focus tracking. Its observed-element wrapper forwards synthetic accessibility children. | Keep the existing frame and add GPUI's `a11y_synthetic_children` to it. No duplicate entry or replacement frame. |
| `InputState` owns Unicode-safe byte selection, caret, UTF-16 native text ranges, composition, clipboard, undo, availability gates and Change events. | Route SetTextSelection to `set_selected_range`; route SetValue and ReplaceSelectedText to `EntityInputHandler::replace_text_in_range`. Programmatic `set_value` retains its silent reset contract. |
| `range_to_bounds`, `selected_range` and `cursor` expose the editor's actual geometry and selection. Base publishes the geometry during paint, after accessibility prepaint. | Retain a complete painted text snapshot, then notify for the next accessibility frame only when it changes. Keep an empty run available on first layout. Character lengths use UTF-8 lengths of Unicode scalar values; positions and widths come from Base's layout. |
| Base emits Change on committed edits, while preedit notifications and selection changes are separate. | Observe editor notifications to refresh accessible state without emitting new Change events. Action and paint callbacks capture weak entities; snapshot storage contains no entities. |

Implementation: [Input composition](../../crates/gpui-kumo/src/input.rs), [private bridge](../../crates/gpui-kumo/src/input/accessibility.rs), [regression tests](../../crates/gpui-kumo/src/input_tests.rs). The text run uses a stable synthetic key under the existing control ID. Read-only controls retain selection actions; disabled controls omit them. Editing actions are installed only for editable controls, and handlers recheck current availability to reject requests arriving before a rerender.

## Measured validation

- `cargo test --locked -p gpui-kumo`: **30 passed**, including 11 Input tests. New coverage checks accented/astral Unicode text, range bounds, outgoing reversed keyboard selection, composition text/selection, exactly one forwarded Change per committed edit with silent preedit, accessibility replacement and Base undo, invalid/foreign positions, current-state availability, and entity release after unmounting.
- `cargo build --locked -p kumo-gallery`, `cargo fmt --all -- --check`, and `cargo clippy --locked --workspace --all-targets -- -D warnings`: passed.
- [Live AT-SPI script](../../scripts/validate-input-accessibility.py) passed on Debian 13/Xvfb/software Vulkan using a private D-Bus session. The same retained entry exposes **Accessible, Component, Text**. Reads return `café 🦀` as six characters; Text selection `(3, 6)` selects `é 🦀`; caret movement, native clipboard replacement with `日本`, keyboard undo, read-only selection, disabled rejection, theme/reset updates, and 101-character horizontal scrolling pass. The Unicode range has positive bounds `(69, 447, 48, 21)`; the scrolled final crab has `(854, 447, 17, 21)`. [Saved results](../evidence/input-accessibility.json).
- PyAT-SPI logs the previously observed AccessKit cache-signature warnings. The script clears node caches before tree reads and asserts live Text queries. Unicode is entered through GTK clipboard/paste; this validates the native paste path, not an OS IME or spoken screen reader.

Reproduce on Linux with the built gallery, X11 display, `python3-gi`, GTK 3 introspection, `python3-pyatspi`, AT-SPI services and `xdotool` available:

```sh
cargo build --locked -p kumo-gallery
dbus-run-session -- /usr/bin/python3 scripts/validate-input-accessibility.py
```

## Remaining acceptance and concrete blockers

1. **Linux accessibility editing:** the selected `accesskit_atspi_common` 0.19.1 `NodeWrapper::interfaces` exposes Text but has no EditableText interface. Text selection/caret dispatch works on the live bus. SetValue/ReplaceSelectedText routes are tested against the retained Base engine, but cannot be invoked as AT-SPI EditableText methods with this adapter. Native keyboard/clipboard edits pass; they do not establish assistive-technology editing. Completing that acceptance requires upstream adapter support or a maintained dependency patch outside this integration.
2. **Directed selection requests:** Base's public `set_selected_range` accepts an ordered byte range, without a reversed-anchor parameter. Incoming reversed requests are normalized to that range, with the caret at its end. Outgoing keyboard-reversed selections are reported correctly. Exact reversed-anchor round trips require a Base API change; this integration does not introduce another selection engine.
3. **Target platform adapters:** source review confirms macOS 0.26.3 maps selected text ranges to SetTextSelection and text values to SetValue, and Windows 0.34.0 maps UIA text-range selection and Value.SetValue to those actions. macOS VoiceOver and Windows UIA runtime checks cannot run on this Linux host and remain pending. Source review is not native validation.
4. Disabled/read-only/invalid property export remains KUMO-005. OS IME testing remains KUMO-007; spoken-reader workflows remain KUMO-014/015/016.

KUMO-006 remains Blocked rather than Resolved because the platform acceptance checkbox is incomplete. GitHub issue publication remains pending; the implementation and saved validation evidence are recorded with this continuation in Git history.
