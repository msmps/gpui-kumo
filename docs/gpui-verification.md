# Verifying a GPUI component

Read when completing a component or investigating a rendering/input discrepancy. This is **Design-system guidance**. The workspace has a native gallery and headless Button, Input and Popover tests; see the [implementation plan](implementation-plan.md) for current validation scope.

## Compile and exercise

Use the project's configured format/check/test commands after dependency selection. Compile examples against the same revision as the library. Launch the representative native example on each claimed platform; framework tests do not establish platform font, GPU, or assistive-technology behavior.

GPUI provides `#[gpui::test]`, `TestAppContext`, and `VisualTestContext` for controlled execution and simulated actions, keystrokes, text input, pointer input, resizing, and scale changes. Visual tests can draw elements and inspect selector bounds. Enable the selected revision's test-support features as needed. [Test context APIs](https://github.com/zed-industries/zed/blob/95cd535a5fad96d649513f96c5784ceefd379e47/crates/gpui/src/app/test_context.rs)

## Select checks from the contract

For native screenshot checks, verify platform presentation visibility as well as accessible state. On the selected macOS dependency, `Window::visibility()` / `Window::is_visible()` describe whether frames will be shown. A fully occluded background window can process input and expose updated accessibility while its frame source is suspended. Raising it through accessibility alone did not restore visibility in our probe. Establish a visible window before interpreting stale pixels as a component defect; see [KUMO-020](issues/020-native-frame-consistency.md). Keep testing-app cleanup and process-absence verification in the native loop.

| Branch | Evidence to collect |
| --- | --- |
| Appearance | Variants/sizes, supported themes, foreground/surface contrast, long labels, narrow widths, text and density scaling |
| Corners and layers | Inspect rounded corners at useful scale: parent/child fills, border continuity (including arrow-to-body joins: no separator across the arrow mouth, outline paints beneath its fill), corner-mask offsets, shadow/ring extents and nested layer contrast. Check both themes, resized/narrow bounds and state changes; preserve documented clipping limits. Inspect enlarged inner edges for dark fringes; border-only transparent fills must retain ring RGB ([renderer evidence](ring-edge-validation.md)) |
| Alignment | Inspect and measure visible icon/control centre lines (verify the glyph actually paints, not only its layout box), text/prompt baselines, labels, sibling gaps and edge padding. Compare actual font/glyph treatment across retained views; confirm configured font families resolve on the host rather than relying on CSS aliases. Check normal/feedback/loading content in both themes and wide/narrow compositions; ensure child wrappers respect parent cross-axis alignment |
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

## First-slice evidence

[Popover validation](popover-validation.md) records the pinned browser comparison, actual Linux AT-SPI activation/focus checks, collision and motion tests, and concrete upstream/platform limits. Headless metadata assertions and operating-system accessibility checks are distinct evidence.

When reviewing composable controls, exercise Base's default keyed focus while inserting or reordering passive parts. Explicit `track_focus` handles can conceal unstable ancestor IDs; keep action identity independent of positional decoration wrappers.

Collection controls: distinguish the action that opens from the action that confirms; propagated Base actions must not activate twice. Exercise closed multi-character/repeated typeahead and open word gaps, then Space after timeout. Activate test windows and settle focus listeners before asserting focus-out dismissal; choose outside targets beyond the popup hitbox. Retained factories/comparators must use weak captures of their owner; include open-overlay unmount release when adding retained rich slots.

Factories invoked during an entity's Render must not read or update that same entity, even through a weak handle: it is already mutably borrowed. Pass needed values to the factory, retain child entities outside rendering, and use weak owner handles in later event callbacks.
