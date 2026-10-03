# ButtonGroup acceptance

Pinned source, docs, all three demos and tests inspected. Group is horizontal-only, intrinsic-width and role Group. Children retain variants/sizes/shapes. No matching Base ButtonGroup exists; compose existing Base-backed Button, preserving ordinary Tab traversal. Toolbar roving navigation belongs to a separate component.

| Area | Acceptance |
| --- | --- |
| API | Named group, zero/one/multiple independently configured Button children; unique scoped IDs; no invented orientation or size/variant inheritance |
| Paint | First keeps start corners, last keeps end corners, interior corners zero; one pixel overlap; drop shadows removed while emphasis child inset highlight remains; focused control paints above adjacent seam |
| Input | Individual callback ownership, pointer/Space/Return once; disabled/loading gating; ordinary Tab across enabled children, no group keyboard selection state |
| Alignment/layers | Intrinsic group width, unchanged button heights, wide/narrow parent centering; mixed variants/sizes and circle/square geometry; focus ring not clipped at joins, overlay priority remains above joined focus |
| Themes/states | All existing Button themes/states retained; group selected/invalid/loading/open state N/A |
| Semantics | Group name/role and individual Base button nodes; no extra focus target; platform spoken grouping remains pending |
| Dependencies/gaps | Wrapped overlay triggers and styled LinkButton joining depend on their supported native composition APIs; existing Popover has a fixed internal trigger. Do not claim those through opaque AnyElement children |

Native group children are typed Button values so corner/shadow overrides reach the actual interactive surface and its ring. Generic web child-selector restyling is unavailable. Kumo keeps the emphasis gradient's inset highlight in a decorative child even when dropping root shadow-xs; the native merged paint retains that distinction.


Validation checkpoint: 88 workspace tests and eight doctests pass; formatting, warning-denied all-target/all-feature Clippy and all-target/all-feature locked builds pass. Observable tests verify intrinsic bounds, source corners, one divider per join with child opacity, independent pointer/Space/Return callbacks, disabled gating and Tab traversal. A containing-Popover test verifies focused rings stay above popup fill without changing traversal order.

User review found doubled middle-button dividers through translucent surfaces. Joined paint now suppresses duplicate internal vertical rings and paints one divider after surfaces, before keyboard focus rings. Child disabled opacity remains exactly the shared pinned Button contract. Independent review caught and prevented a proposed global disabled-opacity regression. Focus rings alone are lifted locally; whole-child paint reordering changes GPUI traversal, while global deferral escapes popup paint scope. Max-content grid preserves intrinsic width with one-pixel negative margins. Private optional accent/join metadata is boxed without changing the public Button API.

Corrected native Linux/Xvfb/software-Vulkan light (capture removed), dark (capture removed) and narrow dark (capture removed) captures inspect joins, end corners, icon/text alignment and layers. Keyboard captures are presentation evidence only: native screenshot counters did not reliably settle after the combined key sequence; exactly-once input and traversal evidence here comes from rendered-input tests. Browser seam comparison, mixed-shape/size stress and native platform spoken semantics remain pending. The Save arrow demonstrates a named independent callback; a real Dropdown menu awaits that component and custom-trigger composition. This checkpoint does not claim the menu or complete browser/platform parity.
