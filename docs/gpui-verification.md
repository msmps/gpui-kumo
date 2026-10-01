# Verifying a GPUI component

Read when completing a component or investigating a rendering/input discrepancy. This is **Design-system guidance**. The workspace has a native gallery and headless Button tests; see the [implementation plan](implementation-plan.md) for current validation scope.

## Compile and exercise

Use the project's configured format/check/test commands after dependency selection. Compile examples against the same revision as the library. Launch the representative native example on each claimed platform; framework tests do not establish platform font, GPU, or assistive-technology behavior.

GPUI provides `#[gpui::test]`, `TestAppContext`, and `VisualTestContext` for controlled execution and simulated actions, keystrokes, text input, pointer input, resizing, and scale changes. Visual tests can draw elements and inspect selector bounds. Enable the selected revision's test-support features as needed. [Test context APIs](https://github.com/zed-industries/zed/blob/95cd535a5fad96d649513f96c5784ceefd379e47/crates/gpui/src/app/test_context.rs)

## Select checks from the contract

| Branch | Evidence to collect |
| --- | --- |
| Appearance | Variants/sizes, supported themes, foreground/surface contrast, long labels, narrow widths, text and density scaling |
| Activation | Pointer, Enter, Space, and accessible action invoke one operation; nested accessory follows its routing policy |
| Availability | Disabled/loading policy holds through every exposed activation path, including after a state change |
| Focus | Tab traversal, visible indicator, focus changes during a held key, and disabling a focused control follow the contract |
| Value | Checked/selected/expanded state updates through its owner and matches accessible state |
| Identity | Reordering/insertion preserve the logical item's retained state and produce unique global IDs |
| Theme update | Existing views, cached content, icons, and measured lists refresh after a token change |
| Overlay | Edge fitting, clipping, nested priorities, dismissal, input boundary, and focus restoration |
| Text input | Selection, graphemes, IME composition, paste/cut/copy, and platform text ranges |
| Motion | Reduced-motion rendering preserves meaning and stops unnecessary frame requests |

Test externally observable outcomes: operation counts, focus targets, values, geometry, and accessible meaning. Use state combinations that expose precedence problems, such as focused + hovered and selected + disabled. Screenshots establish appearance; input tests establish behavior.

## Troubleshooting map

| Symptom | Inspect |
| --- | --- |
| Text occupies space but glyphs are absent | [Platform setup](gpui-grounding.md#startup), loaded fonts, selected features |
| Fluent method is missing | [Prelude and generated utilities](gpui-styling.md#style-values-and-fluent-methods), selected dependency version |
| Click/scroll/role method is missing | [Identified element wrapper](gpui-interaction.md#element-identity) |
| State changes but UI remains stale | Owner notification, theme observers, subscriptions, cached output |
| Focus indicator disappears on hover | [Refinement precedence](gpui-styling.md#interaction-refinements) |
| A label is not announced or repeated labels disappear | [Accessible text identity](gpui-interaction.md#accessible-meaning) |
| An icon has no visible tint | [Explicit SVG color](gpui-primitives.md#icons-images-and-assets) |
| A list jumps after density/text changes | Measurement invalidation and retained scroll state |
| An overlay looks correct but leaks input | Focus/input boundary, propagation, occlusion, and dismissal |

A component is verified when all applicable contract branches have passing evidence. Report remaining platform checks precisely; do not equate source review or successful compilation with a verified native experience.
