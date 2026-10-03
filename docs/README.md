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
| Integrate the exact-version reverse-Tab correction | [GPUI patch and consumer setup](gpui-tab-registration-patch.md) |
| Understand state ownership, rendering, or startup | [GPUI grounding](gpui-grounding.md) |
| Choose layout, text, assets, lists, overlays, or custom drawing | [Primitives](gpui-primitives.md) |
| Implement tokens, themes, typography, or state styling | [Styling](gpui-styling.md) |
| Implement activation, focus, availability, identity, or accessibility | [Interaction](gpui-interaction.md) |
| Design a component API or decide where its state lives | [Component construction](design-system-components.md) |
| Inspect rounded border fringes and Badge hover rings | [Ring edge repair](ring-edge-validation.md) |
| Verify a component or diagnose a visual discrepancy | [Verification](gpui-verification.md), [Popover results](popover-validation.md) |
| Investigate scrolling lag, animation cost or debug/release runtime differences | [Performance measurements](performance-validation.md) |
| Compose retained native Tooltip disclosure | [Tooltip acceptance](tooltip-validation.md) |
| Review remaining component work | [Local issue tracker](issues/README.md) |
| Resume the full port and select the next milestone | [Port progress and backlog](port-progress.md) |
| Use or verify supported Text styles and composition | [Text contract and results](text-validation.md) |
| Use or verify Loader sizing, motion and status semantics | [Loader contract and results](loader-validation.md) |
| Implement navigation links or compact Banner inline actions | [Link acceptance](link-validation.md) |
| Use controlled binary Switches and groups | [Switch acceptance](switch-validation.md) |
| Use typed Radio choices and keyboard navigation | [Radio acceptance](radio-validation.md) |
| Compose native form labels, helpers and errors | [Label/Field acceptance](field-validation.md) |
| Use empty-state composition and command copy | [Empty acceptance](empty-validation.md) |
| Implement contextual messages and accent-aware actions | [Banner acceptance](banner-validation.md) |
| Implement status/category badges or link-hover composition | [Badge acceptance](badge-validation.md) |
| Implement supported simple and layered card containers | [LayerCard acceptance](layer-card-validation.md) |
| Select dependencies, check an API, or refresh these notes | [Source baseline](gpui-sources.md) |

## Work sequence

1. Establish the dependency baseline using [Source baseline](gpui-sources.md). Finish when the selected GPUI revision and platform features are recorded in project configuration, and relevant APIs have been checked against that revision.
2. Define the component contract using [Component construction](design-system-components.md). Finish when variants, sizes, slots, value ownership, activation, availability, focus, and accessible meaning are explicit for that component.
3. Read the primitive, styling, and interaction branches needed by the contract. Finish when every required behavior maps to a verified GPUI API or an explicitly identified custom implementation.
4. Implement the smallest complete component and a representative example. Finish when its state and token choices are exercised through the actual rendering and input paths.
5. Apply [Verification](gpui-verification.md). Finish when the relevant checks pass and any unverified platform behavior is reported with its scope.

The first slice has pinned Kumo references, an application-owned typed theme, a Base adapter, and Button/Input/Popover components. The gallery uses the macOS system font and switches appearance. Component APIs and supported-platform contracts remain implementation decisions; see [Kumo tokens](kumo-tokens.md#initial-native-implementation) for native policies and the [implementation plan](implementation-plan.md) for progress and acceptance criteria.

- [Checkbox acceptance and native evidence](checkbox-validation.md)

For current catalog counts and explicit exclusions, see [component coverage](component-coverage.md).

SensitiveInput: [acceptance, native adaptations and evidence](sensitive-input-validation.md).

InputArea: [acceptance/evidence](input-area-validation.md), [Base growth patch](gpui-base-textarea-growth-patch.md).

InlineCopyText: [acceptance/evidence](inline-copy-text-validation.md).

Collapsible: [acceptance/evidence](collapsible-validation.md).

SkeletonLine: [acceptance/evidence](skeleton-line-validation.md).

Meter: [acceptance/evidence/native adaptations](meter-validation.md).

Breadcrumbs: [acceptance/evidence/native adaptations](breadcrumbs-validation.md).

Select: [core acceptance, remaining composition and native evidence](select-validation.md).

Pagination: [source contract, acceptance and continuation](pagination-validation.md).

Readable text: [Label value authoring and native evidence](readable-label-validation.md).

Linux availability: [pinned disabled-state patch and consumer setup](linux-disabled-state-patch.md).

Linux loading semantics: [authored Busy acceptance and evidence](linux-busy-state-validation.md).

Linux disclosures: [authored expansion acceptance and evidence](linux-expansion-state-validation.md).

Cloud continuation: [self-contained handoff prompt](cloud-handoff-prompt.md), [resume audit](issue-audit-2026-10-03.md).

Tabs: [core contract, native acceptance and remaining overflow/motion work](tabs-validation.md).
