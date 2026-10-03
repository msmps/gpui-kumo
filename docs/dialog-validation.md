# Dialog contract and validation

Status: retained native modal core implemented; full-family acceptance remains open in #19. Pinned Kumo `3fd5b648df578cb1ba214dedd30f475009f6a668`, Base0.7.0 and GPUI0.3.7 remain authoritative.

Inspected Kumo `dialog.tsx`, its positioning regression and `DialogDemo.tsx`; inspected installed Base `dialog.rs`, `focus_trap.rs` and Root's Tab handling. The gallery connects the document action menu to retained edit and delete dialogs. Toast feedback is the next announcement slice.

| Source contract | Implementation requirement |
| --- | --- |
| Root | Standard Dialog or AlertDialog, application-controlled opening/dismissal; alert role prevents outside-pointer dismissal |
| Parts | Trigger, panel, title, description and close controls; caller content owns form/action state |
| Width | At viewport≥640px: Small288, Base384, Large512, ExtraLarge768px. Below640px: full width capped at viewport−32px |
| Position | Horizontal center; top32px below640px and64px at/above640px. The source regression explicitly rejects vertical centering |
| Surface | LayerCard composition, Kumo base/default/line tokens,12px corners, outside ring; literal source shadow override uses3% black at20/25/−5 and8/10/−6px |
| Backdrop | Fixed viewport, recessed fill at80% opacity,150ms transition |
| Typography/padding | Title/description wrappers inherit styles. Demo's24px semibold title, supporting description and32px padding are caller presentation, not mandatory component defaults |
| Motion | Panel opacity/scale transitions at150ms, scale90% entering/exiting; inspect effective reduced-motion behavior in the actual source fixture |
| Focus/input | Initial focus, forward/reverse trapping, Escape/backdrop policy, prevented close, nested overlays, surviving opener restoration and removal/unmount cleanup must pass rendered and actual native input |

Do not use the exported `KUMO_DIALOG_STYLING` metadata as geometry authority: its Small width350 conflicts with the actual supported `sm:w-72`288px class. Use actual classes/rendered browser results. Similarly, source title/description wrappers do not themselves assign the metadata's20/16px typography.

Base provides unstyled Dialog/DialogHandle/Trigger/Popup/Backdrop/Title/Description/Close and a FocusTrapElement. Its default Dialog context binds Enter to Confirm; that must not accidentally confirm while an editor or cancel action owns focus. Base Trigger opens on mouse-down, so keyboard/accessible activation needs deliberate Kumo Button composition. Base's focus trap registers a container, while Base Root supplies the forward/reverse Tab traversal boundary. The current focused gallery previews bind their own Tab actions without a Base Root; verify a real trapping host or explicit interception before claiming modal behavior. A role and container registration alone are insufficient.

Keep native form entities and callbacks application-retained with weak owner capabilities. Reuse the existing input engine. Separate operation success from dismiss requests so a rejected submit can retain the dialog and focus. Record native role/name/description/modal state independently from spoken screen-reader and other-platform acceptance. Preserve open-menu→dialog event ordering without retaining the menu's removed row as the restoration target.

## Native core

`DialogState` owns disclosure, close policies, modal ordering and focus lifecycle. The application retains form entities and operations. `Dialog` supplies caller content, one of four sizes, a description and optional explicit initial focus; `DialogTrigger` composes a Kumo Button. A weak `DialogClose` capability supports guarded action requests or a Close Button. Trigger and Close composition preserve consumer callbacks and respect `prevent_default`; Button availability continues to gate pointer, keyboard and accessible activation.

Standard outside-left-press and Escape dismissal are enabled by default. AlertDialog rejects outside-pointer dismissal. Applications can disable Escape or pointer dismissal and supply a close guard. Rejected requests emit `ClosePrevented` and preserve content/focus; accepted requests emit `OpenChanged(false)` then one `Closed(reason)`. Imperative `set_open` is authoritative and bypasses the guard. Keep owner captures weak and do not read DialogState from its own content builder.

The panel exposes its actual bounds, accessible name, description and modal flag. Base DialogPopup/Backdrop remain private implementation details. The native core uses explicit pre-binding Tab interception and focus containment without requiring Base Root. Installed Base FocusTrapContainer does not forward its child's accessibility role/info; using that wrapper hid the native named Dialog, so it was removed. Base's default Enter-to-confirm binding is also avoided: Enter in an editor does not submit and Enter on Cancel closes through that button once.

Opening records the previous weak focus target and surviving trigger mount. Default initial focus moves after the panel is painted; explicit initial focus uses the caller handle. Forward/reverse Tab wraps and skips unavailable controls; an empty panel retains container focus without looping. A per-window modal stack gives the topmost Dialog ownership of Escape/Tab. Nested Dialog closure restores the parent trigger. Removing an opener prevents stale restoration even when the application retains its focus handle. Unmount closes retained state, releases its content factory, restores a surviving opener and remounts closed. A painted weak focus snapshot permits unmount restoration without stealing newer outside focus.

## Evidence and scope

Actual pinned source and native Linux Light/Dark ×1040/520 matrices pass. Both edit panels measure512×237 at(264,64) wide and488×237 at(16,32) narrow. The caller composition uses32px padding,16px vertical gaps,24/32px semibold title and14/21px description; editor/actions are36px high with8px button gap. Native accessibility measures the inner editor at panel+44px because it excludes the12px input padding; the outer input surface matches source panel+32px. The source/native screenshot review compares the full input surface and action row rather than treating the inner accessible editor bounds as the input width.

Inspected both-theme wide/narrow edit and alert surfaces: horizontal centering, wrapped supporting text, input/title edges, action baselines and gaps agree;12px panel corners, outline and source literal shadows remain coherent. Simple caller content stays inside the surface. Arbitrary-descendant rounded clipping remains subject to LayerCard's existing contract. Native AlertDialog maps to AT-SPI Alert, while standard Dialog maps to Dialog; both export Modal. No spoken screen-reader acceptance follows from those observations.

[Reproduction scripts, raw results and screenshots](validation-fixtures.md). Rendered regressions cover initial focus, Tab/reverse Tab, editor Enter, disabled skip, rejection/draft retention, Escape/pointer policies, surviving/removed opener, authoritative close, unmount/remount, all four source widths, long scrolling, empty content and two-Dialog ordering. Full pinned Rust gate results and remote CI are recorded at the published checkpoint.

## Remaining full-family work

- Panel motion currently provides a150ms entrance fade and immediate reduced-motion rendering. Source90% scale and exit transition are not yet ported.
- Dropdown/Select/Popover inside a Dialog, nested-parent removal, rich/custom trigger and explicit title/description node relationships need further composition acceptance. Two-Dialog ordering is verified separately.
- Long content scrolling is rendered-test acceptance; native/browser long-content and arbitrary-descendant clipping matrices remain open. The native viewport cap and scroll region are an explicit adaptation to the source's overflow-hidden panel.
- RTL, other-platform font/native-input validation, OS IME and spoken screen-reader checks remain open. Native description is authored; speech and announcement ordering are not claimed.

Track full acceptance in [GitHub #19/#19](https://github.com/msmps/gpui-kumo/issues/19). This core checkpoint does not close the issue.
