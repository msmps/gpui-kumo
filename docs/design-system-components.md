# Constructing design-system components

Read when proposing a component API, token recipe, or state ownership boundary. The public API contract below is authoritative for this library; the remaining sections explain its construction approach. Dependency and palette choices live in their own maintained contracts.

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

## Public API contract

This section is authoritative when adding, changing or reviewing a public component API. Use the terminology in [CONTEXT.md](../CONTEXT.md). The component contracts linked from [docs/README.md](README.md) specify supported capabilities and platform limits; they do not introduce alternative spellings for equivalent concepts.

### Identity, naming and visibility

Require `impl Into<ElementId>` for element identity. Identity is stable within identified ancestry and independent of displayed, localised or editable text. Retained states do not own a second element ID. Collection items carry stable IDs independently of their selected values.

Text-bearing constructors use `new(id, label)` where sensible; preserve `Button::new(id, label)`. Retained form presentation uses `new(id, &state)`; its state constructor takes its initial default name before value/options and context arguments. Destination-bearing Link uses `new(id, label, href)`. SelectOption/RadioItem/TabItem use `new(id, value, label)`, keeping the typed application value between identity and presentation. SensitiveInputState uses `(name, initial_value, window, cx)`: value is secret data, never a semantic name.

On textual controls, `label(text)` changes visible text and its default accessible name. Standalone Input, InputGroup, InputArea, SensitiveInput and Select show their labels by default. `show_label(bool)` controls visibility reversibly; hiding a label does not discard its text, focus target or accessible name. `accessibility_label(text)` overrides the accessible name and wins over the default regardless of builder order. Repeated calls to the same property use the last value. Decorative icon/rich-content slots do not infer names. Icon-only controls require a nonblank authored name.

`AccessibleName::try_from` validates external text without a panic; convert the validated name into component arguments normally. Infallible named-control constructors/builders reject empty or whitespace-only semantic names in release builds. Preserve meaningful whitespace and Unicode instead of silently substituting a placeholder, ID or icon asset path. Empty content on read-only presentation is a distinct case and need not invent a control name.

Input, InputGroup, InputArea, SensitiveInput and Select accept optional rendered label/name props. Their fallback is the retained state's default name. Public `set_name(name, cx)` updates that fallback and notifies without replacing the entity, resetting text/selection/history, emitting value proposals, or closing overlays. Dialog, Popover and Dropdown expose the same retained update operation for their named surfaces. Names supplied by rendered Tabs/Toolbar props refresh with each owner render. Keep value text separate from control names; SelectValueContent and Meter value_text describe values, not labels.

### Safe label association

Use `Field::control(id, control, cx)` with the sealed `FieldControl` implementations for Input, InputGroup, InputArea, SensitiveInput and Select. It transfers the current label text, visibility, optional indicator, tooltip, description and error, hides the built-in presentation, and composes one associated label/message. The supported control remains concrete until rendering: Field errors also drive its invalid appearance and accessibility state; visible feedback retains its accessible description, and outer Field props override inherited presentation. Disable the retained state once; the associated label follows its availability automatically (including Select loading). `label_disabled(bool)` additionally gates label activation without modifying the child. Generic Field has no whole-control disabled setter. Supply the label once through the state constructor or control's `label(text)` prop. A control's explicit accessible-name override remains authoritative; localisation of the default is read on each render.

`Field::new(id, label, custom_content)` is the custom-content path. The caller explicitly names each interactive descendant and configures its availability/description; `focus_target` only forwards label activation. AnyElement does not permit reliable semantic inference. Standalone Label follows the same focus-only rule. Field does not guess semantic invalid/required/described-by relationships for arbitrary children.

Dialog and Popover content factories may contain arbitrary rich titles. Their state name is explicit and independent of the factory; update it through `set_name`. Do not read/update an entity from its own render factory. Retain form entities outside factories and capture owners weakly for later input handling. Explicit title/node relationships and spoken announcements retain the separate platform/component acceptance scope.

### Availability, errors and proposals

`disabled` rejects activation/editing across pointer, keyboard and accessibility paths. Loading rejects repeat activation where supported; links and editors do not acquire a loading capability simply because Button has one. Read-only editors retain selection and copying. Component-specific focus policies remain explicit: ordinary disabled controls leave traversal; Toolbar Buttons/editors may remain in roving focus according to `focusable_when_disabled`, while disabled Toolbar Links leave traversal.

`optional_indicator(true)` displays “(optional)”; false omits it. This does not validate values. Optional indicators do not author native required state; owners perform validation. Error text is application-owned: `error(text)` shows it; `error_visible(text, show)` also permits hidden-message presentation. Error presence suppresses supporting descriptions even while its message is hidden. Where a retained editor/Select exposes invalid semantics/rings, error presence drives them. Typed Field forwards errors to supported controls; generic Field cannot modify arbitrary content's invalid state. Radio/Checkbox/Switch group errors are group feedback: a group constraint does not identify every child as invalid. Radio/Checkbox item error variants remain an explicit owner choice; Switch has no invalid variant.

Callbacks retain their semantic payload: Button/Link activation contains the native ClickEvent, editor events report user editing, and collection events propose values. Owner setters are silent value reconciliations unless their component contract explicitly emits lifecycle events. Copy submission means a request to GPUI's clipboard API, which has no success/failure return; it does not guarantee OS delivery.

### Typed composition and failure contracts

Toolbar constructors return ToolbarButton, ToolbarLink, ToolbarInput or ToolbarInputGroup. Configure each kind, then `build()` into common ToolbarItem storage. Only Button supports loading, only Button/Link support icons, only InputGroup supports start/end/suffix, and only Button/editors support focusable_when_disabled. Collection erasure does not expose kind-specific operations. Rustdoc compile-fail checks cover these restrictions.

Retain release checks for programmer misuse: duplicate scoped IDs, mounting one retained editor twice in a Toolbar, blank authored control names, nonfinite or invalid layout ranges/widths, and impossible component configuration. Document reachable conditions under `# Panics`, including validation deferred until rendering. Use fallible validation for recoverable external data, rather than downgrading invariants to debug_assert. Runtime values continue through their component's existing normalisation or owner reconciliation policies. Private assertions protecting typed Toolbar dispatch are internal invariants.

Public Debug should reveal useful identity, configuration, availability and collection counts. Redact editable values, selected text, copied payloads and secrets; represent callbacks, factories, arbitrary elements and Base editor internals opaquely. Do not require application value types to implement Debug just to inspect a presentation wrapper. Closed style/size/placement/value enums are exhaustive; event and dismissal-reason enums are non_exhaustive so consumers include a fallback arm. Typed value/event payloads may derive Debug with application-value bounds; presentation wrappers and retained states omit those values. Add Default only when no identity, name, value or context is required; CollapsiblePanel already has it.

### Component-specific exceptions

| Family | Reasoned boundary |
| --- | --- |
| Text, Badge, Banner titles/descriptions, BreadcrumbCurrent, Label, Empty | Readable content is not a control label. Constructor content/title remains explicit; heading roles and independent readable values retain their existing rules. Rich decorative content keeps complete readable text |
| CheckboxGroup CheckboxItem, DropdownItem, Toast/ToastAction | SharedString selection values, action routing keys and deduplication/replacement keys are domain data. They remain stable across localisation. These are explicit domain-key exceptions, even when a domain key also scopes rendered identity |
| Checkbox, Switch and named form groups | Consumed controlled props refresh names without retained entity setters. Hidden group legends use show_label; rich legend content is decorative and does not replace the explicit textual name. Checkbox/Switch preserve the source’s full optional suffix in default names even when hidden; explicit overrides are complete names. Editor optional suffixes are visible label indicators configured with optional_indicator. Compact Toolbar/Pagination and SensitiveInput inner editors explicitly suppress their internal labels |
| Link, SelectOption, TabItem, RadioItem, DropdownItem | Rows and inline links retain their source text/content layout; they have no independent hidden-label mode. Name overrides are still independent of text. Dropdown/Popover trigger text names the trigger; retained state names the surface |
| Collapsible with custom Button trigger | The caller's Button owns its visible label/name and availability composition; the default trigger uses the Collapsible label/override. CollapsiblePanel is a layout surface and has no control label |
| ButtonGroup, Tabs, Toolbar, TooltipProvider | Named groups/collections expose their existing semantics. Tabs/Toolbar names are rendered props; TooltipProvider is a lifecycle coordinator, not an additional announced control |
| Loader, SkeletonLine, LayerCard/Section, Icon, assets and theme/token helpers | Presentation/configuration helpers do not invent interactive names or value ownership. Icon remains decorative. Loader permits an empty accessibility_label to remain an unnamed passive status. Styles/tokens are typed closed configuration |
| InlineCopyText and Toast actions | Visible/copy/value payloads and action names have separate meanings. InlineCopyText labels configures its copy/feedback names; Toast title/description remain content and its IDs are routing keys |
| Meter and Pagination parts | Formatted value text and navigation/action labels are distinct properties. PageSize show_label hides its label slot reversibly; the retained Select name remains independent of that slot |

### Author/reviewer checklist

For every new or changed export, account for constructors, identity/domain keys, visible/default/overridden/hidden names, retained state updates, availability, error/required semantics, callbacks, fallibility, Debug and enum evolution. Use an explicit justified exception for unsupported capabilities; do not add generic operations that compile and fail at runtime.

Exercise actual rendering/input: both builder orders for overrides, hidden/icon-only names, typed Field focus association, dynamic names with retained selection/value/history/open overlays, activation and traversal. Verify wrapper metadata transfer, error/reset semantics, retained-state availability, visible-by-default standalone labels and Select label focus. Use a shared contract fixture for common rules and compile-fail tests only for intentional type restrictions. Review both-theme wide/narrow/resized geometry, alignment and rounded layers under [verification](gpui-verification.md). Run the locked Rust gate and native accessibility probes; keep operational outcomes in the issue/PR and generated evidence outside git.
