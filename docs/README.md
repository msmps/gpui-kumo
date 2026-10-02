# Building on GPUI

This is the agent entry point for building a design system on GPUI primitives. Read the [glossary](../CONTEXT.md) when naming concepts. Framework facts link to official source; sections labeled **Design-system guidance** recommend future implementation practices.

## Choose the reference for the task

| Task | Read |
| --- | --- |
| Start the native example or review implementation milestones | [Implementation plan](implementation-plan.md) |
| Map Kumo tokens, component contracts, or native-port differences | [Kumo grounding](kumo-grounding.md) |
| Implement the first slice's palette, typography, spacing or effects | [Kumo token extraction](kumo-tokens.md) |
| Translate Button, Input or Popover geometry and visual states | [Kumo component recipes](kumo-component-recipes.md) |
| Compare raw GPUI with GPUI Base as the behavior foundation | [GPUI Base assessment](gpui-base-assessment.md) |
| Understand state ownership, rendering, or startup | [GPUI grounding](gpui-grounding.md) |
| Choose layout, text, assets, lists, overlays, or custom drawing | [Primitives](gpui-primitives.md) |
| Implement tokens, themes, typography, or state styling | [Styling](gpui-styling.md) |
| Implement activation, focus, availability, identity, or accessibility | [Interaction](gpui-interaction.md) |
| Design a component API or decide where its state lives | [Component construction](design-system-components.md) |
| Verify a component or diagnose a visual discrepancy | [Verification](gpui-verification.md), [Popover results](popover-validation.md) |
| Review remaining component work | [Local issue tracker](issues/README.md) |
| Resume the full port and select the next milestone | [Port progress and backlog](port-progress.md) |
| Select dependencies, check an API, or refresh these notes | [Source baseline](gpui-sources.md) |

## Work sequence

1. Establish the dependency baseline using [Source baseline](gpui-sources.md). Finish when the selected GPUI revision and platform features are recorded in project configuration, and relevant APIs have been checked against that revision.
2. Define the component contract using [Component construction](design-system-components.md). Finish when variants, sizes, slots, value ownership, activation, availability, focus, and accessible meaning are explicit for that component.
3. Read the primitive, styling, and interaction branches needed by the contract. Finish when every required behavior maps to a verified GPUI API or an explicitly identified custom implementation.
4. Implement the smallest complete component and a representative example. Finish when its state and token choices are exercised through the actual rendering and input paths.
5. Apply [Verification](gpui-verification.md). Finish when the relevant checks pass and any unverified platform behavior is reported with its scope.

The first slice has pinned Kumo references, an application-owned typed theme, a Base adapter, and Button/Input/Popover components. The gallery uses the macOS system font and switches appearance. Component APIs and supported-platform contracts remain implementation decisions; see [Kumo tokens](kumo-tokens.md#initial-native-implementation) for native policies and the [implementation plan](implementation-plan.md) for progress and acceptance criteria.
