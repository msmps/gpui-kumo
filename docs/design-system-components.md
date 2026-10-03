# Constructing design-system components

Read when proposing a component API, token recipe, or state ownership boundary. This document is **Design-system guidance**; it proposes a working approach and does not record a selected palette, API, or dependency.

## Module layout

Keep components with tests or private helpers together in `src/<component>/`: the implementation and public API live in `mod.rs`, rendered regressions in `tests.rs`, and helpers in named sibling modules. For example, `button/mod.rs` and `button/tests.rs` both belong to the `button` module. Standalone modules can remain single files. This filesystem choice preserves public Rust paths such as `gpui_kumo::button::Button`.

## Define the contract first

For a new component, write a compact contract covering:

| Dimension | Question to resolve |
| --- | --- |
| Purpose | What operation or information does it expose? |
| Variant and size | Which named choices are supported, and what changes together? |
| Slots | Which content can callers supply, and who styles/names it? |
| Value state | What value does it display, and who owns that value? |
| Availability | What makes activation unavailable, including loading? |
| Activation | Which operation runs, and which input paths invoke it? |
| Identity and focus | Who supplies identity and how is focus retained/traversed? |
| Accessible meaning | What role, name, value, and actions are exposed? |
| Layout | What constraints, truncation, wrapping, and hit-target rules apply? |

Finish the contract when every relevant row has an explicit answer; omit dimensions that do not apply to a presentation-only component.

Stress-test the language with concrete cases: a hovered selected row is both an interaction state and a value state; a disabled primary button keeps its variant while losing availability; an icon-only button needs a name; a popover can be nonmodal; a loading button needs a policy for repeat activation. Use the [glossary](../CONTEXT.md) to keep these concepts distinct.

## Choose a construction level

Use `RenderOnce` for consumed props and callbacks composing existing primitives. Use a retained `Entity<T>` implementing `Render` when the component owns state across renders. A stateless component may still use GPUI's identified interaction state; `RenderOnce` does not imply an absence of focus or hover behavior. Use `canvas` or a custom `Element` only when layout/painting requirements justify it. See [Grounding](gpui-grounding.md) and [Primitives](gpui-primitives.md).

This presentation-only example illustrates the composition shape. It is source-checked but has not been compiled in this workspace:

```rust
use gpui::{div, prelude::*, App, Hsla, SharedString, Window};

#[derive(IntoElement)]
struct Label {
    text: SharedString,
    color: Hsla,
}

impl RenderOnce for Label {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .text_color(self.color)
            .child(gpui::text!(self.text))
    }
}
```

The example's generated text identity follows [Interaction](gpui-interaction.md#accessible-meaning): repeated labels need distinct identified ancestry or an explicit ID strategy before becoming a general-purpose component.

## Keep the layers small

| Layer | Owns | Produces |
| --- | --- | --- |
| Foundations | Typed token values, theme/density inputs | Semantic token sets |
| Recipes | Variant/size/value-to-appearance mapping | Base styles and interaction refinements |
| Components | Slots, activation, focus, accessible meaning | Composed GPUI elements |
| Views | Durable values, tasks, subscriptions, application operations | Props and callbacks for components |

Prefer enums for closed variant/size sets. Store semantic choices and resolve current theme values while rendering. Retain callbacks and state under a deliberate owner. Avoid making each component own a second copy of an application value.

A slot can accept `impl IntoElement` at a builder boundary and store `AnyElement` when type erasure is needed. Determine whether custom content inherits typography, whether a leading icon is decorative, and how interactive accessories participate in events and tab order.

## Override policy

Choose one explicit policy before exposing styling extensions: semantic props only, selected layout overrides, or a general refinement escape hatch. Document how overrides combine with the recipe and which properties remain responsible for focus, accessibility, and hit-target behavior.

Implement `Styled` on a wrapper only if its `style()` reference is applied to the rendered element intentionally. Decide whether caller refinements precede or follow recipe values. Combine interaction properties in one callback for each state; see [Styling](gpui-styling.md). An escape hatch should have a predictable precedence rather than depend on incidental builder order.

## Build one complete slice

Start with the component requested by the user and the foundations it needs. For an initial button slice, a bounded scope could be a text label, leading icon, small variant/size sets, disabled/loading behavior, keyboard focus, and accessible activation. Add an example that exercises those paths before extracting shared button-like behavior.

Complete the slice when its contract maps to implementation, the representative example covers state combinations, and [Verification](gpui-verification.md) passes. Shared recipes should emerge from repeated contracts; a catalog of unimplemented components does not establish a working design system.

## Record decisions at the right level

Use `CONTEXT.md` only for canonical terms. Keep implementation contracts alongside their relevant docs or code. Create an ADR only for a consequential, costly-to-reverse choice with a real trade-off that future readers need explained. At this stage there is no such accepted implementation decision to record.
