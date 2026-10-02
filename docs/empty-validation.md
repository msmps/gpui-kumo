# Empty contract and validation

Baseline: Kumo `3fd5b648df578cb1ba214dedd30f475009f6a668`, `empty/empty.tsx`, its documentation and `EmptyDemo.tsx`, plus `use-copy-feedback.ts`; GPUI Kit/Base 0.7.0 and GPUI 0.3.7. Regular Copy/Check SVG paths are from Phosphor React v2.1.10 (the Kumo dependency minimum), with the MIT notice retained in `PHOSPHOR-LICENSE`. No icon runtime dependency is added.

## Acceptance matrix

| Dimension | Native contract / validation |
| --- | --- |
| Sizes | sm: 24/32px padding, 16px gap; base: 40/64, 24; lg: 48/80, 32. Full width, 12px radius, 1px fill border, control surface |
| Parts | Required title; optional caller-sized decorative icon, description, command and caller-owned contents. Empty strings omit optional parts |
| Typography | With description: large heading, 20/28px. Without: secondary base copy presented as h2. Supporting copy centered, 14/21px, max 560px; ordinary wrapping replaces CSS text-balance |
| Command | 40px height, 80% maximum width, overlay fill, white border, line ring, xs shadow, 12/8px left/right padding, 8px gap, inherited monospace. `$` prompt is decorative and excluded from clipboard. Long commands horizontally scroll |
| State and events | Feedback entity retained by caller ID through GPUI keyed state. The Base-backed Kumo Button handles pointer/Enter/Space/accessibility activation. Native clipboard writes are synchronous. Feedback resets one second after the most recent copy; replacing command cancels old feedback. Unmount drops keyed state and timer |
| Focus/availability | Copy button uses Base focus/activation/traversal. No overall disabled/loading/value-selected/open/invalid states exist upstream; these are N/A. Consumer actions own their own availability; no focus trap or dismissal |
| Accessibility | Title has heading role/level 2, full title/description/command exposed through Text. Copy name becomes “Command copied” during success, a native feedback adaptation. No verified spoken/live announcement claim. Icons and prompt remain decorative |
| Themes | All surfaces/text/rings use existing semantic theme roles in both appearances. Success icon uses text-success; source values remain centralized |
| Edge cases | Minimal composition, repeated IDs under unique scopes, empty optionals, long Unicode description/command and narrow containers. Text balancing, group opacity/shadow rasterization and success bounce animation are native paint/motion differences awaiting comparison |

## Evidence

Focused rendered-input checks cover clipboard contents, pointer and keyboard operation, independent instances, one-second reset/restart, changed command, theme retention and minimal size geometry. Gallery entries exercise icon/action composition, all sizes and a narrow long-command case. All three focused tests pass. The full all-feature gate passes 66 workspace tests and six doctests, formatting, all-target/all-feature locked Clippy with warnings denied and gallery/example builds.

Browser pixel parity and spoken platform accessibility remain awaiting validation. GPUI's clipboard-write API returns no result; OS-level rejection cannot be observed, so this wrapper does not promise error detection. Native success feedback is static, retaining meaning under reduced motion; the authored bounce animation is not yet implemented.

Native Debian/Xvfb/software-Vulkan captures were inspected after rebuilding: [light](evidence/empty-light.png), [copy feedback](evidence/empty-copy-feedback.png), [dark](evidence/empty-dark.png). The copied Check icon appeared after pointer activation and reset before the later theme capture. The consumer action is centered; minimal sizes retain their padding with no empty contents gap. Native inspection found the initially left-aligned consumer action; its centering wrapper is now conditional, and regression checks protect centering and minimal height. One live gallery process was used; testing processes were terminated afterward (defunct OS process records have no live application/window).

Independent source review found no blocking correctness issue and requested narrow-copy and pending-timer cleanup evidence; both now have rendered-input tests. An intermediate unconditional contents wrapper introduced a trailing gap in minimal Empty; the size regression failed and the wrapper was made conditional, without weakening the expected geometry. Follow the shared checklist: optional parts must not leave spacing when absent, and caller content must be tested in its actual parent layout.

## Copy alignment repair

User visual review caught the copy control above the command centre. A new rendered regression fails on the previous implementation: 26px control at y109 inside 40px command at y108, a 6px centre offset. The shared Button wrapper forced `self_start`, overriding parent `items_center`. Removing that override respects the caller’s cross-axis layout while retaining intrinsic control/hitbox sizing. Both copy and success states now centre against command/text at 600px and 220px in both themes. All Button sizes and a 400px stretching-column composition have independent geometry checks.

Native inspected: [corrected light](evidence/empty-alignment-light.png), [copied success](evidence/empty-alignment-copied.png), [corrected dark](evidence/empty-alignment-dark.png). 77 workspace tests/six doctests, formatting, warning-denied all-target/all-feature Clippy and builds pass. Separate review found no focus/ring/hitbox regression; all live gallery applications terminated. The shared verification checklist and AGENTS now explicitly require alignment checks.
