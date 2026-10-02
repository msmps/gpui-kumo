# KUMO-020: reconcile native screenshots with updated component state

Status: In progress — occlusion diagnosis verified; original gallery checks pending

## Evidence

macOS native inspection on 2026-10-02 exposed inconsistent state and captured pixels. Link activation appeared as three navigation requests in the accessible tree, while some subsequent captures retained earlier counts/themes. That session initially had two gallery processes; both were closed and an empty pgrep verified cleanup.

Banner testing then launched only the registered gallery. The rebuilt screenshot correctly showed its repaired text column, all four light treatments and compact compositions. Pointer/Return/Space exposed three banner activations, but subsequent screenshots retained zero and light appearance after theme inputs. Availability-control input did not yield reliable captured-state confirmation. This also follows LayerCard's earlier updated Input accessible value with lagging input pixels.

These observations establish a verification discrepancy, not its cause. Do not label it a GPUI rendering defect, an application notification defect or a computer-use capture defect without evidence. Automated actual-render/input tests currently pass; they do not resolve native presentation/capture consistency.

Follow-up isolated native probes: a direct Button/div counter, then a nested counter entity, then that entity inside a scrolling flex root all showed matching state/pixels. The full gallery twice exposed counter 2 then 5 while pixels remained 0. `apps/gallery/examples/native_frame_probe.rs` now retains the actual Banner panel alone inside a scrolling root; it also exposed counter 2 while pixels remained 0. It is a reduced reproduction, not yet minimal. Gallery SVG assets are omitted in that fixture.

The user's scrolling-lag question prompted [debug/release frame measurements](../performance-validation.md). Debug's matching scroll protocol recorded only one draw and no presentation samples. Release produced 483 presentation intervals at mean 8.33 ms. This narrows the evidence but does not identify the discrepancy's cause. All testing apps were quit and gallery/probe process absence verified.

## Next investigation and acceptance

### Occlusion diagnosis, 2026-10-02

Further reduction removed Kumo Text/Button, Base Button, Base Root and Base initialization. A pure GPUI retained counter still exposed Counter 2 with Counter 0 pixels. Removing cosmetic counter styling preserved the discrepancy. Capturing pixels before reading updated accessibility also retained Counter 0.

Four ranked predictions covered accessibility-observation ordering, native frame scheduling, capture freshness and debug-only behavior. Native zoom revealed Counter 3, and a subsequent input displayed Counter 4. Restoring the original small window exposed Counter 5 while retaining Counter 4 pixels. Targeted `Window::visibility()` instrumentation reported `Hidden` in the failing small-window condition and `Visible` in the enlarged passing condition; restoring the window restored `Hidden`. A release-only comparison was no longer needed to explain this debug reproduction.

The matching `gpui-pre-macos 0.3.7` source checks `NSWindowOcclusionStateVisible` in `MacWindowState::start_display_link` and stops its frame source when occluded. GPUI's `Window::is_visible` means frames will be shown, rather than merely having a shown window. Background computer-use input can advance state/accessibility while presentation is suspended.

The Banner-only fixture also showed visibility Hidden, accessible activation count 1 and captured count 0. Enlarging it showed Visible and matching count 4 in accessibility and pixels. This verifies the explanation for that reproduction; the combined input sequence does not establish one callback per event. The complete gallery's Link, Input, theme and availability checks remain pending under verified visibility. No library refresh workaround or dependency patch is justified.

Both diagnostic examples now display platform visibility; temporary debug tags were removed. They are native diagnostic fixtures, not headless regression tests: the headless platform does not reproduce AppKit occlusion. Both testing applications were quit and process absence verified. This finding does not establish the cause of the user's foreground scrolling impression or explain all earlier debug/release differences, whose visibility was not recorded.

- Launch only the registered application and verify exactly one process. Compare the same action's visible screen, accessibility state and subsequent frame without mixing process instances.
- Use existing counters/theme controls and Input to identify whether the issue affects real presented pixels, capture freshness, event delivery, or selected-dependency invalidation/presentation.
- Inspect the matching GPUI event, notify, draw and presentation paths when the evidence warrants it. Add an observable regression before any implementation workaround; do not blanket-refresh the window to hide an unknown cause.
- Record native light/dark, availability, input and counter checks separately. Resolve only when state and pixels agree through repeatable native input, with testing apps closed and process absence verified.

Independent component work can continue with this validation gap explicit. No user intervention is currently needed.
