# Dropdown action-menu core

Pinned Kumo `3fd5b648df578cb1ba214dedd30f475009f6a668`, Base0.7.0, GPUI0.3.7 and Rust1.99.0 remain unchanged. Source Dropdown, its documentation/demos, and installed Base popup/menu-related APIs were inspected. Deprecated Item.href is intentionally omitted; recommended LinkItem remains a follow-up. Base has no menu primitive; its Popup supplies measurement, positioning, collision fitting and occlusion without importing Select's combobox semantics.

## Contract

`DropdownState::new(name, parts, cx)` is application-retained. Mount it through one `Dropdown::new(id, &state, trigger_label)`. The state owns open/focus lifecycle; the application owns operations and subscribes to `DropdownEvent::Activated(id)`. `OpenChanged(bool)` reports transitions. There is no callback or application-value mirror. `set_parts` reconciles handles by stable unique item ID, prunes removed rows and recovers a removed focused row. `set_disabled` dismisses an open menu and gates all activation; `set_open` supports application control. A mounted-state token releases open/deferred resources after unmount even when the application retains the entity; remount starts closed. Factories and per-render subscriptions are unnecessary in this slice.

| Dimension | Implemented contract |
| --- | --- |
| Parts | Flat action items, readable labels, separator, decorative icon, inset, selected check and informational shortcut |
| Variants | Default and danger action rows; disabled opacity50% with guarded operation |
| Keyboard entry | Enter/Space opens; Down opens first row, Up last; pointer opening focuses the menu container |
| Navigation | Up/Down loop by default; Home/End; configurable non-looping navigation; stable IDs across reorder |
| Disabled focus | Arrow/typeahead navigation includes disabled items, matching pinned Base UI's empty disabledIndices; activation remains blocked |
| Typeahead | Case-insensitive label prefix, repeated-letter cycling,500ms reset; Space continues an active word search, then activates after timeout |
| Activation | GPUI pointer/Enter/Space/accessible click path emits one action identity and closes; current owner parts/availability are checked at activation |
| Dismissal | Escape restores trigger. Forward Tab dismisses and traverses after trigger; Shift+Tab dismisses to trigger, matching actual pinned browser. Interception precedes application Tab bindings. Outside pointer dismissal is occluded from the underlying control |
| Semantics | Menu/MenuItem/Label/Splitter; disabled authored on the row; trigger Expanded/Collapsed menu description and HasPopup Menu. Native MenuItem names omit the decorative shortcut suffix |
| Layout |220px native default, explicit width≥144px,8px gap; Base center anchor and8px window margin.6px surface padding,8px row horizontal/6px vertical padding,14px/21px text,33px rows,6px row corners/8px surface corners,1px outside outline, Kumo control/overlay/danger/hairline/shadow tokens. Long lists scroll and reveal focused direct rows |

The outside outline paints in an unclipped sibling of the scrolling menu. Painting it as a scroll child clips the straight edges and leaves corner fragments; a rendered regression checks the combined GPUI border-strip masks at all four edges/corners in both themes and wide/narrow viewports. The transparent fill retains the border RGB to avoid edge fringes. The menu keeps its existing scroll handle, role and focus target. Native macOS light/dark review at approximately 520 and 1040 pixels confirms continuous straight/corner edges and consistent row baselines, padding, sibling spacing, focus rounding and shadow surfaces. Resizes forced fresh frames because of the separate frame-presentation issue (#51); arbitrary-descendant clipping remains outside this contract.

## Results and limits

[Source/native evidence](validation-fixtures.md) exercises the actual pinned imports and native gallery in Light/Dark1040/520. Seven rendered regressions cover keyboard entry/focus/availability, once-only activation, typeahead words/timeout, Tab dismissal, owner reorder/removal, mount cleanup/remount and long-list row height/focused reveal. Full-family acceptance remains open.

Native adaptations: the Base host keeps an8px window margin versus source5px collision padding, giving a3px horizontal offset in this near-left-edge fixture. Native width is explicit; source has intrinsic sizing with144px minimum. The Linux host currently paints the600-weight group label with a visibly different face from browser DejaVu Sans; requesting700 did not resolve the discrepancy. The source600 recipe is retained. This measured font-resolution discrepancy remains open for the target-platform polish pass; header typography parity is not claimed. Pointer-versus-keyboard focus-visible modality remains a known difference; source can retain a focus ring after keyboard use and pointer hover where native last-input modality hides it. Rounded clipping is bounded to supported flat rows; arbitrary descendant surfaces are not claimed.

Remaining Dropdown work: CheckboxItem, RadioGroup/Item/Indicator, LinkItem/current-target navigation, Sub/SubTrigger/SubContent/nested boundaries, semantic Group composition, joined Save ButtonGroup trigger, custom/rich trigger replacement, configurable placement/alignment, source motion/reduced-motion lifecycle, parent Popover composition, arbitrary rich descendants and complete modality/RTL fidelity. Native accessibility queries establish exposed roles/state, not spoken screen-reader acceptance; macOS/Windows interactive recording acceptance remains separate from macOS Rust CI.

Announcement goal: build Dialog and Toast over this action-menu flow, compose the menu→edit/confirm→feedback workflow and run a focused target-platform polish/recording pass. Toolbar's remaining joined/popup composition stays tracked in #35.
