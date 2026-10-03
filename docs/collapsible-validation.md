# Collapsible acceptance

Tracker [#15](https://github.com/msmps/gpui-kumo/issues/15), KUMO-030. Pinned Kumo3fd5b648df578cb1ba214dedd30f475009f6a668, Kit/Base0.7.0 and GPUI0.3.7 with documented foundation patches. Full source/docs/seven demos inspected; no matching component tests found. Baseline117 tests/nine doctests.

Selected because disclosure unlocks form/settings compositions using existing Button/Text/Input. Base Collapsible supplies conditional content; Base Button supplies activation/focus. Stable-ID keyed state owns only uncontrolled open and native focus lifecycle. Explicit open prop is authoritative in controlled mode; callbacks propose changes without updating uncontrolled state. Owner changes do not fire callbacks. Consumer prevent_default cancels trigger activation.

| Area | Acceptance |
| --- | --- |
| Parts/API | Default trigger/panel, custom Kumo Button trigger and unstyled panel, controlled open/default_open, disabled, on_open_change, keep_mounted |
| Default presentation |14px medium text,4px gap,16px caret slot/12px bold glyph; panel8px vertical margin,2px fill border,4px vertical/right and16px left padding,16px child gap |
| States/themes | Closed/open, hover/pressed native activation, keyboard ring, disabled; light/dark, long/wrapped labels and narrow forms |
| Input/ownership | Pointer/Space/Enter once; controlled ignored proposals stay closed/open; independent IDs; cancellation, root/custom-button disabling; retained native editor |
| Focus | Closed descendants absent from Tab/actions/semantics; owner/action closing focused content restores trigger; forward/reverse traversal; no focus trap or Escape dismissal |
| Keep mounted | GPUI display:none panel requests child layout but skips descendant prepaint/paint; verify hidden actions/Tab/a11y exclusion and retained local/native state, rather than relying on zero-height clip |
| Semantics | Actual Button expanded/disabled metadata; unsupported control association/spoken OS semantics recorded honestly |
| Motion | Source100ms height/opacity/caret motion requires safe lifecycle; static core initially has immediate transitions, including reduced motion; do not claim animated parity |
| N/A | Sizes/variants, loading/invalid/selection independent of composed children; outside dismissal/trapping |

## Validated working checkpoint — 2026-10-03

Three observable rendered-input regressions pass: uncontrolled pointer/Space/Enter once, root unavailable gating and independent state; ignored controlled proposals, custom consumer cancellation and caller-owned trigger focus/restoration; keep-mounted hidden descendant snapshots/Tab/action exclusion and retained Unicode editor/keyed copy feedback. Stale retained child-button focus cannot dispatch its former Space/Enter actions after closure. Tests cover both themes. Full gate:120 workspace tests/nine doctests, formatting, locked all-target/all-feature Clippy with warnings denied and builds (gallery included). Existing dependency profiler deprecation remains visible, no warnings suppressed. No Actions configuration/runs exist.

Independent review found custom Button track_focus ownership was overwritten (Medium); repaired by selecting its supplied handle as the effective trigger/restoration handle, with observable regression. Final independent diff review has no blocker. Keep-mounted uses actual GPUI display:none, not zero-height masking: descendant prepaint/paint is skipped, so no hidden descendant control snapshots/actions/Tab registrations. OS spoken output and accessibility control association are not established by these tests.

Native Linux/Xvfb: pointer/Space/Enter gives exactly three controlled proposals; fourth activation reopens, owner Save closes without another proposal, Space on restored trigger reopens with fifth. Native appended text survives this cycle. Captures reviewed for12px caret in16px slot, label/control baselines, panel border/padding, focus rings in4px breathing room and rounded custom surfaces at1040px/520px in both themes. Long trigger wraps with centered caret, custom Button remains native keyboard control; nested disclosure entry inspected.

Evidence: [light](evidence/collapsible-light.png), [three proposals/closed keyboard](evidence/collapsible-closed-keyboard-light.png), [input focus](evidence/collapsible-input-focused-light.png), [Save owner close](evidence/collapsible-save-closed-light.png), [reopened retained text](evidence/collapsible-reopened-light.png), [custom panel](evidence/collapsible-custom-light.png), [dark](evidence/collapsible-dark.png), [narrow dark](evidence/collapsible-narrow-dark.png), [narrow Save ring](evidence/collapsible-narrow-save-focused-dark.png), [narrow input focus](evidence/collapsible-narrow-input-focused-dark.png), [narrow light](evidence/collapsible-narrow-light.png), [nested entry](evidence/collapsible-narrow-nested-light.png).

Remaining source100ms height/opacity/caret motion, arbitrary custom trigger element types, control association and OS/browser acceptance keep #15 open. Immediate transitions also apply under reduced motion; no animated parity claimed. Default trigger uses a native brand keyboard ring because browser focus-visible outline is platform-dependent. Root disabled affects the trigger, not content editors. Unmounted content's externally retained native editor survives because native ownership is explicit; keep_mounted additionally retains keyed component feedback through layout. Reordering/removing custom supplied trigger focus handles is a separate consumer lifecycle responsibility. Process improvement: custom trigger review must check caller-owned handles and cancellation rather than validating only the default trigger.
