# Maintaining GPUI Kumo

Use [CONTEXT.md](../CONTEXT.md) for terminology. [GitHub issues](https://github.com/msmps/gpui-kumo/issues) are the source of truth for outstanding work, acceptance criteria and resolutions. Keep this directory focused on current contracts, design decisions and maintenance procedures.

## Foundations

| Task | Reference |
| --- | --- |
| Check dependency revisions and refresh source references | [Source baseline](gpui-sources.md) |
| Understand the selective Base dependency boundary | [Base assessment](gpui-base-assessment.md) |
| Map supported Kumo components and exclusions | [Coverage](component-coverage.md), [Kumo grounding](kumo-grounding.md) |
| Maintain semantic tokens and native presentation policies | [Tokens](kumo-tokens.md), [Button/Input/Popover recipes](kumo-component-recipes.md) |
| Design component APIs and durable state | [Component construction](design-system-components.md), [State and rendering](gpui-grounding.md) |
| Choose layout, drawing, assets and overlays | [Primitives](gpui-primitives.md) |
| Maintain themes and style precedence | [Styling](gpui-styling.md) |
| Maintain activation, focus and accessibility | [Interaction](gpui-interaction.md), [Readable labels](readable-label-validation.md) |
| Verify rendering/input, alignment and layered corners | [Verification](gpui-verification.md), [Ring edges](ring-edge-validation.md) |
| Reproduce browser/native comparisons | [Validation fixtures](validation-fixtures.md) |
| Diagnose development-build lag and frame presentation | [Performance](performance-validation.md) |

Before changing a component, check its contract and the selected dependency source. Retain durable state in entities and keep Base details inside component boundaries. Run formatting, relevant tests and warnings-denied Clippy; `bash scripts/check-rust.sh` runs the full locked workspace and patched-adapter gate.

## Maintained dependency corrections

These documents specify the correction boundary, downstream root configuration, regression coverage and removal conditions. Upgrade compatible GPUI/Kit/Base revisions together; dependency patches do not propagate from a library to its consumers.

- [Repeated focus registration](gpui-tab-registration-patch.md)
- [Duration-animation executor clock](gpui-animation-clock-patch.md)
- [Base Textarea growth](gpui-base-textarea-growth-patch.md)
- [Linux disabled state](linux-disabled-state-patch.md)
- [Linux authored busy state](linux-busy-state-validation.md)
- [Linux authored expansion state](linux-expansion-state-validation.md)

## Retired corrections retained for upstreaming

[Library-owned Select and Link controls](library-controls.md) no longer require Base modifications. The historical source and regression references remain in [Select confirmation focus](gpui-base-select-focus-patch.md) and [retained Link focus](gpui-base-link-focus-patch.md).

## Component contracts

The `*-validation.md` filenames retain their existing links. Their supported behavior and native limitations guide maintenance; current outstanding work belongs in GitHub issues. Implementation coverage does not mean complete browser or platform fidelity.

| Area | Contracts |
| --- | --- |
| Content | [Text](text-validation.md), [Badge](badge-validation.md), [Banner](banner-validation.md), [Empty](empty-validation.md), [LayerCard](layer-card-validation.md), [Link](link-validation.md) |
| Forms | [Checkbox](checkbox-validation.md), [Radio](radio-validation.md), [Switch](switch-validation.md), [Field/Label](field-validation.md), [InputGroup](input-group-validation.md), [InputArea](input-area-validation.md), [SensitiveInput](sensitive-input-validation.md) |
| Navigation and composition | [Breadcrumbs](breadcrumbs-validation.md), [ButtonGroup](button-group-validation.md), [Pagination](pagination-validation.md), [Select](select-validation.md), [Tabs](tabs-validation.md), [Toolbar](toolbar-validation.md) |
| Overlays | [Popover](popover-validation.md), [Tooltip](tooltip-validation.md), [Dropdown](dropdown-validation.md), [Dialog](dialog-validation.md) |
| Feedback and disclosure | [Loader](loader-validation.md), [Meter](meter-validation.md), [SkeletonLine](skeleton-line-validation.md), [Collapsible](collapsible-validation.md), [Toast](toast-validation.md) |

## Documentation policy

Keep API contracts, native adaptations, patch ownership/removal rules and reproducible verification instructions here. Put work lists and acceptance outcomes in GitHub issues. Keep authored validation tools in `tools/validation/`; generated artifacts are disposable. Issue-referenced historical images remain under `docs/evidence/`. Historical reports are available through immutable Git links in the relevant issues.
