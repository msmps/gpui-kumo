# Interaction, identity, and accessibility

Read before implementing controls, editable content, lists with selection, or overlays. APIs refer to the [source baseline](gpui-sources.md).

## Element identity

`div().id(...)` returns a `Stateful<Div>`, enabling methods from `StatefulInteractiveElement`, including `on_click`, `active`, scrolling, and accessibility roles. Identity is required for retained interaction state. The global ID combines identified ancestors and the local ID; keep it stable and unique in that scope. [Div traits](https://github.com/zed-industries/zed/blob/95cd535a5fad96d649513f96c5784ceefd379e47/crates/gpui/src/elements/div.rs), [identity guide](https://github.com/zed-industries/zed/blob/95cd535a5fad96d649513f96c5784ceefd379e47/crates/gpui/src/_accessibility.rs)

**Design-system guidance:** require caller-supplied identity for interactive components. Use stable data keys for reorderable items. An index identifies a position; after insertion it can transfer retained interaction state to another logical item. Give internal slots IDs beneath the component's own scope.

## Activation and event routing

`on_click` receives `&ClickEvent`, `&mut Window`, and `&mut App`. At the baseline it handles primary pointer clicks and synthesized unmodified Enter/Space activation when focused. The keyboard path pairs press/release and checks focus generation. Accessibility click dispatch is connected to click listeners too. Verify this behavior in the selected revision before adding a separate key handler. [Click implementation and regression tests](https://github.com/zed-industries/zed/blob/95cd535a5fad96d649513f96c5784ceefd379e47/crates/gpui/src/elements/div.rs)

Define semantic keyboard operations through `actions!`, `KeyBinding`, `cx.bind_keys(...)`, `.key_context(...)`, and `.on_action(...)`. Use `cx.listener(...)` to access owning view state. Register bindings as part of app/view setup, rather than accumulating them each render. [Input example](https://github.com/zed-industries/zed/blob/95cd535a5fad96d649513f96c5784ceefd379e47/crates/gpui/examples/input.rs)

Dispatch supports capture/bubble phases. `cx.stop_propagation()` stops propagation; `window.prevent_default()` suppresses applicable default handling. Choose these deliberately for nested controls. **Design-system guidance:** send pointer, keyboard, and accessible activation through one operation, and test that a nested accessory invokes exactly one intended operation. [Div event dispatch](https://github.com/zed-industries/zed/blob/95cd535a5fad96d649513f96c5784ceefd379e47/crates/gpui/src/elements/div.rs)

## Focus

Retain a `FocusHandle` when code must target or inspect focus. `.track_focus(&handle)` links it to a rendered element. `.focusable()` creates focusability; tab registration is a separate policy. `tab_index`, `tab_group`, and `tab_stop` define ordering, while `window.focus_next(cx)` and `focus_prev(cx)` perform traversal. Check how the application binds Tab/Shift-Tab. [Focus API](https://github.com/zed-industries/zed/blob/95cd535a5fad96d649513f96c5784ceefd379e47/crates/gpui/src/window.rs), [element registration](https://github.com/zed-industries/zed/blob/95cd535a5fad96d649513f96c5784ceefd379e47/crates/gpui/src/elements/div.rs)

**Design-system guidance:** define whether a control is a tab stop and whether a composite keeps focus on a container or moves it between items. Render a keyboard-visible focus indicator using [Styling](gpui-styling.md#interaction-refinements). Give focus and selection separate owners.

## Availability and value state

**Design-system guidance:** disabled appearance alone does not enforce availability. Gate primary activation, shortcuts, secondary operations, and accessibility actions together. Decide whether a control disabled while focused retains focus and how it leaves tab traversal. Loading also needs an explicit activation policy.

Keep selected/checked/expanded values under a named owner and synchronize their accessible state. A controlled component receives a value and requests a change; its owner applies that change and notifies. A component-owned value lives in a retained view. Choose one model for each value.

Zed's button layer demonstrates conditional handler installation, variant/state recipes, and preserving focus registration while removing disabled controls from tab stops. These are useful patterns to inspect, not a required dependency or a complete policy to copy. [Button composition](https://github.com/zed-industries/zed/blob/95cd535a5fad96d649513f96c5784ceefd379e47/crates/ui/src/components/button/button_like.rs)

## Accessible meaning

Accessible nodes require element identity and a role. Set an accessible name, relevant value/state, and actions; inspect `role`, `aria_label`, `aria_description`, `aria_selected`, `aria_expanded`, `aria_toggled`, and `on_a11y_action` in the selected version. `gpui::AccessibleAction` represents AccessKit actions and is distinct from GPUI's keyboard `Action` trait. [Accessibility guide](https://github.com/zed-industries/zed/blob/95cd535a5fad96d649513f96c5784ceefd379e47/crates/gpui/src/_accessibility.rs)

Plain string children paint text without an accessible text node. `text!(...)` supplies an ID derived from its source location. Repeated calls from one loop/helper need distinct IDs or distinct identified ancestors. For a labeled button, inaccessible child text can avoid duplicating an explicit parent name; for independent readable text, use accessible text. [Text identity](https://github.com/zed-industries/zed/blob/95cd535a5fad96d649513f96c5784ceefd379e47/crates/gpui/src/elements/text.rs)

**Design-system guidance:** include an accessible name for icon-only controls. A tooltip can provide supplementary help but should not be the only naming strategy. Validate exposure of disabled/loading state in the chosen API; do not infer semantics from opacity or cursor styling.

## Editable text and overlays

Text input is a retained behavior component, not a styled text label. The official input example handles selection, marked/composition text, Unicode boundaries, UTF-16 platform ranges, clipboard behavior, and `EntityInputHandler` registration. Preserve those requirements when extracting a reusable field. [Input example](https://github.com/zed-industries/zed/blob/95cd535a5fad96d649513f96c5784ceefd379e47/crates/gpui/examples/input.rs)

**Design-system guidance:** for each overlay, define initial focus, Escape/outside-click dismissal, focus restoration when the opener disappears, and behavior when nested. A modal additionally needs a deliberate focus/input boundary. Use [overlay primitives](gpui-primitives.md#overlays) for placement and drawing, then verify dismissal and input routing separately.
