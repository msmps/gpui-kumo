# Dialog source audit — next announcement slice

Status: unported. This audit records the next implementation contract; it is not native acceptance or a coverage increment. Pinned Kumo `3fd5b648df578cb1ba214dedd30f475009f6a668`, Base0.7.0 and GPUI0.3.7 remain authoritative.

Inspected Kumo `dialog.tsx`, its positioning regression and `DialogDemo.tsx`; inspected installed Base `dialog.rs`, `focus_trap.rs` and Root's Tab handling. The next bounded slice should connect the new document action menu to a retained edit/confirmation dialog, followed by Toast feedback.

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

Track full acceptance in [KUMO-034/#19](issues/034-dialog-port.md). This audit does not resolve nested overlays, clipping, platform typography, OS IME or speech checks.
