# Choosing GPUI primitives

Read when selecting building blocks for a component. These are GPUI mechanisms; button, checkbox, menu, and dialog contracts belong to the design system. APIs refer to the [source baseline](gpui-sources.md).

## Composition, layout, and text

| Need | Primitive | Constraint |
| --- | --- | --- |
| Container | `div()` | Styling and interaction traits expose different capabilities; see [Styling](gpui-styling.md) and [Interaction](gpui-interaction.md) |
| One child / a sequence | `.child(...)` / `.children(...)` | Child values implement `IntoElement` |
| Conditional construction | `.when(...)`, `.when_some(...)`, `.map(...)` | Fluent Rust transformations |
| Mixed stored content | `AnyElement` | Erase types where a slot or collection needs heterogeneity |
| Visible text | String / `SharedString` children | Plain string elements have no accessible node identity |
| Accessible text | `text!(...)` or `Text::new(...)` | Give repeated instances unique identity |
| Highlighted text | `StyledText` / `TextRun` | Preserve run lengths and text boundary correctness |

Composition and conversion are defined in [element traits](https://github.com/zed-industries/zed/blob/95cd535a5fad96d649513f96c5784ceefd379e47/crates/gpui/src/element.rs); text behavior in [text elements](https://github.com/zed-industries/zed/blob/95cd535a5fad96d649513f96c5784ceefd379e47/crates/gpui/src/elements/text.rs). Fluent helpers come through the prelude; check imports against the selected dependency.

Use the `Styled` API for flex, grid, size, min/max constraints, padding, margin, gap, and overflow. Taffy computes web-inspired layout. **Design-system guidance:** supply bounded space when a component relies on it, and test narrow widths and long content. [Styled API](https://github.com/zed-industries/zed/blob/95cd535a5fad96d649513f96c5784ceefd379e47/crates/gpui/src/styled.rs), [element layout](https://github.com/zed-industries/zed/blob/95cd535a5fad96d649513f96c5784ceefd379e47/crates/gpui/src/element.rs)

`container_query(...)` chooses contents from its assigned size. Its children are built after its layout, so they cannot determine its size. Use it for responsive composition within a bounded container. [Container query](https://github.com/zed-industries/zed/blob/95cd535a5fad96d649513f96c5784ceefd379e47/crates/gpui/src/elements/container_query.rs)

## Icons, images, and assets

`svg().path(...)` uses application assets; `external_path(...)` and `data(...)` support other sources. At the baseline, SVG painting is tinted by the element's explicit `text_color`; set it on the icon rather than assuming inherited text color will suffice. SVG transformation affects painting without changing layout or hitboxes. This suits monochrome icons; verify multicolor artwork separately. [SVG](https://github.com/zed-industries/zed/blob/95cd535a5fad96d649513f96c5784ceefd379e47/crates/gpui/src/elements/svg.rs)

`img(...)` accepts resource paths/URIs and decoded sources. `StyledImage` supplies `object_fit`, loading presentation, and error fallback. **Design-system guidance:** reserve stable image bounds and provide explicit loading/error states. [Images](https://github.com/zed-industries/zed/blob/95cd535a5fad96d649513f96c5784ceefd379e47/crates/gpui/src/elements/img.rs)

An `AssetSource` supplies bytes through `load` and paths through `list`; configure application assets for packaged resources. **Design-system guidance:** expose semantic icon names and keep resource paths inside the asset adapter. [Assets](https://github.com/zed-industries/zed/blob/95cd535a5fad96d649513f96c5784ceefd379e47/crates/gpui/src/assets.rs)

## Scrolling and lists

An identified `div` can use scrolling overflow and a retained `ScrollHandle`. Preserve the handle across renders when programmatic position matters. The scroll mechanism and scrollbar appearance are separate responsibilities. [Div and scroll handles](https://github.com/zed-industries/zed/blob/95cd535a5fad96d649513f96c5784ceefd379e47/crates/gpui/src/elements/div.rs)

`uniform_list(id, count, render_range)` lazily renders equal-height rows in a bounded viewport. Keep row heights consistent, including selected and loading states. `list(ListState, render_item)` handles varying heights and retains measurements in `ListState`; invalidate affected measurements with its update APIs when offscreen heights change. Theme and density changes may invalidate measurements too. [Uniform list](https://github.com/zed-industries/zed/blob/95cd535a5fad96d649513f96c5784ceefd379e47/crates/gpui/src/elements/uniform_list.rs), [variable-height list](https://github.com/zed-industries/zed/blob/95cd535a5fad96d649513f96c5784ceefd379e47/crates/gpui/src/elements/list.rs)

## Overlays

`anchored()` places children at an anchor, with window/local coordinate modes and fitting strategies such as switching corners or snapping inside window margins. `deferred(child)` delays drawing; higher deferred priority draws above lower priority. [Anchoring](https://github.com/zed-industries/zed/blob/95cd535a5fad96d649513f96c5784ceefd379e47/crates/gpui/src/elements/anchored.rs), [deferred drawing](https://github.com/zed-industries/zed/blob/95cd535a5fad96d649513f96c5784ceefd379e47/crates/gpui/src/elements/deferred.rs)

**Design-system guidance:** an overlay also needs open-state ownership, dismissal, focus movement/restoration, input occlusion, and accessibility. Define those in [Interaction](gpui-interaction.md). Anchoring and priority alone do not supply a complete popover or modal contract.

## Drawing and motion

Use `canvas(prepaint, paint)` for bounded custom drawing without a full custom element. It accepts styled layout and passes computed bounds to callbacks. [Canvas](https://github.com/zed-industries/zed/blob/95cd535a5fad96d649513f96c5784ceefd379e47/crates/gpui/src/elements/canvas.rs)

A custom `Element` requests layout, resolves geometry/hitboxes in prepaint, then paints using that state. It owns integration with clipping, input, and accessibility. **Design-system guidance:** compose existing elements until measurement or rendering needs this control. [Element lifecycle](https://github.com/zed-industries/zed/blob/95cd535a5fad96d649513f96c5784ceefd379e47/crates/gpui/src/element.rs)

`AnimationExt::with_animation(...)` uses an ID, timing, easing, and element transformation. At the baseline these animations honor `App::reduce_motion`: they render a static endpoint and schedule no animation frames when enabled. **Design-system guidance:** expose duration/easing tokens and preserve this behavior; manual animation must handle the preference separately. [Animation](https://github.com/zed-industries/zed/blob/95cd535a5fad96d649513f96c5784ceefd379e47/crates/gpui/src/elements/animation.rs)
