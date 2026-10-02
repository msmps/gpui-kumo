# KUMO-020: reconcile native screenshots with updated component state

Status: Open

## Evidence

macOS native inspection on 2026-10-02 exposed inconsistent state and captured pixels. Link activation appeared as three navigation requests in the accessible tree, while some subsequent captures retained earlier counts/themes. That session initially had two gallery processes; both were closed and an empty pgrep verified cleanup.

Banner testing then launched only the registered gallery. The rebuilt screenshot correctly showed its repaired text column, all four light treatments and compact compositions. Pointer/Return/Space exposed three banner activations, but subsequent screenshots retained zero and light appearance after theme inputs. Availability-control input did not yield reliable captured-state confirmation. This also follows LayerCard's earlier updated Input accessible value with lagging input pixels.

These observations establish a verification discrepancy, not its cause. Do not label it a GPUI rendering defect, an application notification defect or a computer-use capture defect without evidence. Automated actual-render/input tests currently pass; they do not resolve native presentation/capture consistency.

## Next investigation and acceptance

- Launch only the registered application and verify exactly one process. Compare the same action's visible screen, accessibility state and subsequent frame without mixing process instances.
- Use existing counters/theme controls and Input to identify whether the issue affects real presented pixels, capture freshness, event delivery, or selected-dependency invalidation/presentation.
- Inspect the matching GPUI event, notify, draw and presentation paths when the evidence warrants it. Add an observable regression before any implementation workaround; do not blanket-refresh the window to hide an unknown cause.
- Record native light/dark, availability, input and counter checks separately. Resolve only when state and pixels agree through repeatable native input, with testing apps closed and process absence verified.

Independent component work can continue with this validation gap explicit. No user intervention is currently needed.
