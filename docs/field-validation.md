# Label and Field contract

Public naming, visibility, typed composition and failure policy follow the [canonical public API contract](design-system-components.md#public-api-contract). Use that contract when changing the APIs described here.

Pinned source: Kumo `3fd5b648df578cb1ba214dedd30f475009f6a668`, `label/label.tsx`, `field/field.tsx`, Label docs/demos and Input composition. GPUI Base 0.7.0 has no Label/Field primitive. These components use ordinary GPUI presentation and explicit focus association; Input and other wrapped controls retain Base interaction/state ownership.

| Acceptance area | Contract and evidence |
| --- | --- |
| Label | 14/21px, medium/default foreground, inline 4px gaps; optional indicator uses normal/subtle text. optional_indicator(true) adds optional; false/unspecified omit it |
| Content | Label has a complete accessible string and optional decorative rich content. No heading/control role is invented. Contextual Tooltip help uses the independent info-button API; other interactive accessories belong outside decorative content |
| Association | `focus_target` replaces DOM `htmlFor`: pointer label activation focuses an existing handle. No new entity, listener subscription or value state is owned. Typed Field derives label availability from the retained control; disable that state once. Custom content uses label_disabled to gate forwarding and owns its availability. Checkbox/switch label value activation must be composed with their own Base behavior later |
| Field layout | Stacked label/control/message with 8px gaps. Inline and ControlFirst are explicit native choices instead of web descendant selectors. ControlFirst reverses order, wraps, and grows the label to fill remaining space. Tests exercise 400px and 120px widths |
| Validation | Caller supplies error and explicit `show` result, replacing web ValidityState matches. An error hides description even while its message is not displayed. Typed Field forwards errors to supported controls for invalid appearance/semantics without changing values; generic Field leaves child semantics caller-owned |
| Message | 13/17.875px supporting/danger text with full accessible content; shared helper now used by Input. Native message is independent accessible text; Field does not add a described-by/error relation to arbitrary children. Consumers must configure their control's name and semantic description |
| Themes and states | Existing semantic/default/subtle/danger tokens in both appearances. Hover/pressed/loading/open/selection/trapping are N/A for this presentation wrapper; focused styling belongs to the associated control |
| Composition | Gallery demonstrates optional Field around retained Input. Input's built-in label/message presentation reuses the same implementation. Empty helper/error and hidden label are supported; long labels/messages use native wrapping |
| Deferred parity | Label/Field contextual help implemented; Checkbox/Switch labelTooltip convenience and rich help content remain pending. Browser pixel comparison, spoken association and RTL horizontal layout are awaiting validation. The native focus association is not an accessibility labelled-by relationship |

Rendered-input regression verifies pointer label focus, optional readable name, retained Unicode editing through errors/themes, 8px stacked spacing, error/help precedence, hidden-error suppression and disabled label rejection. Source review found missing growing/wrapping behavior in ControlFirst; it was repaired and wide/narrow order/alignment tests were added. No duplicated editing/validation state or callback path was introduced. Full validation is recorded in the port checkpoint after the final gate.

Validation gate: 68 all-feature workspace tests and six doctests, formatting, warning-denied all-target/all-feature Clippy and workspace builds pass. Linux/Xvfb captures: light layout (capture removed), dark layout with retained input (capture removed). Clicking the label then typing `phone draft` reached the retained input, visible after theme change. The immediate light capture did not show the typed value; it is layout evidence only, not proof of timely presentation. Browser comparison and platform spoken semantics remain pending. The testing application was terminated; only zombie process entries remained.


## Implementation notes

Complete pinned Label/Field source, Label documentation/demos and matching `@phosphor-icons/react`2.1.10 Info definition/license inspected. `Label::tooltip(&retained_state, text)` and `Field::label_tooltip` compose Kumo's source ghost/Xs square info trigger:14px button,16px regular Info SVG,4px sibling gap, centered with label/optional content. Embedded SVG uses explicit semantic default foreground: installed Svg does not paint without its own text-color refinement. Native review caught the missing icon before accepting this checkpoint; wide/narrow light/dark captures were refreshed after correction.

| Contextual help acceptance | Evidence |
| --- | --- |
| Pointer label association | Label text/optional activation focuses retained Input, preserving Unicode value |
| Independent help activation | Base-backed info button owns focus; click stops label propagation and dismisses without opening or focusing the input |
| Keyboard | Native reverse traversal from Input to preceding help, immediate disclosure, Escape retains help focus. Forward traversal remains Base-owned |
| Disabled | Label association and Input editing disabled; contextual explanation independently available. Disabled label pointer prevents default ancestor focus, preserving help focus |
| Hidden/empty help | Field show_label(false) cancels open/pending help and skips its trigger. Empty Label tooltip cancels disclosure and omits icon. Helper/message precedence and Input value unchanged |
| Themes/layout |14px trigger vertical centre equals short and wrapped long-label centre; actual painted keyboard ring asserted; stacked8px gap retained. Both-theme wide and520px native captures inspect icon/optional baseline, helper wrapping, border/corners and seamless tooltip arrow |
| Accessibility | Info trigger name More information, existing Input focus/name/value/selection metadata preserved. Control-help spoken relationship remains a platform/API gap |
| Other APIs/states | Source variant/size additions N/A; optional/disabled/open/focus exercised. Loading/invalid/selected belong to wrapped controls. Rich help content and other form convenience APIs remain backlog |

The new consumer regression exposed pre-existing Input reverse-Tab trapping from duplicate focus registration. A [documented exact-version dependency correction](gpui-tab-registration-patch.md) preserves accessible Input focus instead of removing its tracked semantic frame. Independent review verified latest-registration ordering and cached replay, Label help event isolation and hidden-label cancellation. Downstream consumers need the same workspace-root override.

101 workspace tests/nine doctests, formatting, warning-denied workspace Clippy and all-target/example builds pass. The patched dependency's existing profiler deprecation warning is visible and unmodified. Its own targeted unit attempt could not compile because external font test assets are absent; actual consumer regression runs and passes. Native `field-tooltip-*` captures show visible info treatment in both themes/wide/narrow, hover explanation, actual Shift-Tab from the edited input to help, Escape dismissal and retained `phone draft` after theme change. Direct Input/InputGroup convenience now uses the same help ([checkpoint](input-group-validation.md#inputinputgroup-label-help-checkpoint)). Browser comparison, macOS/screen reader and rich/other form integration remain pending.

## Supported-control composition

`Field::control(id, control, cx)` transfers label visibility/text, optional indicator, contextual help, description and error from Input, InputGroup, InputArea, SensitiveInput or Select. The concrete control is retained until Field renders so outer error props update its invalid state; the outer Field owns the one displayed message. Explicit outer props override inherited presentation. The retained control owns disabled state and the label follows it automatically. `label_disabled` only adds a label-activation guard for custom composition; it cannot enable a disabled supported control.

Standalone form controls show their supplied labels by default. Compact internal controls explicitly hide labels. `optional_indicator(true)` displays “(optional)” and has no required/validation semantics. Migration: `required(false)` becomes `optional_indicator(true)`, `required(true)` becomes `optional_indicator(false)`, and generic Field `disabled` becomes `label_disabled`; disable supported controls through their retained state.
