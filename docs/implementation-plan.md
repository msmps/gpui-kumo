# First native Kumo slice

Status: workspace, native theme foundation and initial Button implementation complete, updated 2026-10-01. Steps 1–2 have a native macOS baseline. Step 3 has Button with typed props, Base behavior and tested activation/availability; Input and Popover remain pending. Step 4 has Button examples and foundation previews. Step 5 remains open, including browser visual parity, documented paint differences, accessibility gaps and other platforms. Read the [Base assessment](gpui-base-assessment.md) for the evidence and constraints.

## Outcome

A working native window containing Kumo Button, Input and Popover components, with a small gallery for inspecting their appearance and behavior. The example should establish whether Base supports our component contracts and visual requirements cleanly.

## Sequence

1. **Establish the Rust workspace and dependency baseline.** Select and pin a compatible GPUI Kit/Base release and GPUI snapshot. Disable the styled Component layer. Start with macOS native validation in the current development environment and explicitly record the scope of platform verification. Use [Source baseline](gpui-sources.md) and check APIs against the selected dependencies.

   Complete when the native application builds and opens a window, Base initialization and window hosting work, and dependency versions and required platform features are recorded in project configuration.

2. **Implement the minimum Kumo theme.** Extract the light/dark colors, typography, spacing, radii, gradients, shadows and focus treatment needed by the three components. Keep Kumo's semantic token model separate from Base's internal theme, with an explicit adapter where Base consumes theme values. Use [Kumo grounding](kumo-grounding.md) and [Styling](gpui-styling.md).

   Implemented in [theme.rs](../crates/gpui-kumo/src/theme.rs), using [Kumo tokens](kumo-tokens.md) and [Component recipes](kumo-component-recipes.md), pinned to Kumo commit `3fd5b648df578cb1ba214dedd30f475009f6a668`. The token reference records native font, disabled foreground, selection and color-mapping policies. The gallery switches appearance and previews representative colors, typography, gradients, highlights and shadows. Browser cascade, color parity and component paint/interaction remain validation work.

   Complete when the example can switch appearance modes and the selected recipes have traceable upstream references and resolved native values.

3. **Build the three components behind our own APIs.** Define each contract using [Component construction](design-system-components.md); choose Base reuse per component.

   Button's implementation and native policies are recorded in [Component recipes](kumo-component-recipes.md#native-button-contract). Tests exercise activation, availability, traversal and rendered geometry. Next implement Input's durable editing contract, then Popover composition and focus/positioning behavior.

   | Component | What the slice must establish |
   | --- | --- |
   | Button | Kumo variants, sizes, shapes, slots, loading, availability, focus and activation |
   | Input | Durable editing state, accessible labeling, selection, clipboard, IME, validation and availability |
   | Popover | Trigger/content composition, placement, open-state ownership, dismissal and focus restoration |

   Complete when each component renders with its Kumo recipe, its ownership and behavior contract is explicit, and Base-specific implementation details are contained within the component boundary.

4. **Create the component gallery.** Present representative sizes, variants and interaction states with a light/dark switcher. Include enabled, unavailable, loading and invalid examples where applicable, plus ordinary keyboard navigation and overlay usage.

   Complete when the gallery exercises all three components through real native rendering and input paths and supports comparison against the selected Kumo reference.

5. **Evaluate the foundation.** Apply the relevant checks from [Verification](gpui-verification.md): visual fidelity, activation exactly once, availability, keyboard traversal, overlay dismissal and focus restoration, editing behavior, and accessibility metadata. Check the known Base disabled-button accessibility gap. Record build/dependency costs and any structural or styling constraints encountered.

   Complete when the results identify which Base behavior to retain, adapt or replace, and list unresolved platform or accessibility checks. Record the resulting dependency decision and rationale before expanding the catalog.

## Scope

This slice establishes the foundation and a reusable gallery. Broader component coverage, exhaustive token extraction and additional platform validation follow the evaluation. Native accessibility and visual fidelity are acceptance work, not guarantees inherited from either upstream library.
