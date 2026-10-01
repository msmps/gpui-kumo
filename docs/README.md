# Building on GPUI

This is the agent entry point for building a design system on GPUI primitives. Read the [glossary](../CONTEXT.md) when naming concepts. Framework facts link to official source; sections labeled **Design-system guidance** recommend future implementation practices.

## Choose the reference for the task

| Task | Read |
| --- | --- |
| Understand state ownership, rendering, or startup | [GPUI grounding](gpui-grounding.md) |
| Choose layout, text, assets, lists, overlays, or custom drawing | [Primitives](gpui-primitives.md) |
| Implement tokens, themes, typography, or state styling | [Styling](gpui-styling.md) |
| Implement activation, focus, availability, identity, or accessibility | [Interaction](gpui-interaction.md) |
| Design a component API or decide where its state lives | [Component construction](design-system-components.md) |
| Verify a component or diagnose a visual discrepancy | [Verification](gpui-verification.md) |
| Select dependencies, check an API, or refresh these notes | [Source baseline](gpui-sources.md) |

## Work sequence

1. Establish the dependency baseline using [Source baseline](gpui-sources.md). Finish when the selected GPUI revision and platform features are recorded in project configuration, and relevant APIs have been checked against that revision.
2. Define the component contract using [Component construction](design-system-components.md). Finish when variants, sizes, slots, value ownership, activation, availability, focus, and accessible meaning are explicit for that component.
3. Read the primitive, styling, and interaction branches needed by the contract. Finish when every required behavior maps to a verified GPUI API or an explicitly identified custom implementation.
4. Implement the smallest complete component and a representative example. Finish when its state and token choices are exercised through the actual rendering and input paths.
5. Apply [Verification](gpui-verification.md). Finish when the relevant checks pass and any unverified platform behavior is reported with its scope.

No palette, typography scale, component API, theme ownership policy, or supported-platform contract has been selected yet. Resolve those choices during implementation; the examples here illustrate mechanisms rather than a finished visual language.
