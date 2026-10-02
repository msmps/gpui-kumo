//! Shared-container InputGroup presentation over an existing retained editor.
use crate::{Input, InputState, Theme, input::Size};
use gpui_kit::base::TestSupportExt;
use gpui_kit::{
    App, ElementId, Entity, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, Styled, Window, div, px,
};

type ActionFactory = dyn Fn(Size, &mut Window, &mut App) -> crate::Button;
pub(crate) struct AddonContext<'a> {
    pub theme: &'a Theme,
    pub size: Size,
    pub disabled: bool,
    pub start: bool,
}

#[derive(Clone)]
pub enum InputGroupAddon {
    Text(SharedString),
    Icon(SharedString),
    Parts(Vec<InputGroupAddon>),
    Action(std::rc::Rc<ActionFactory>),
}
impl InputGroupAddon {
    pub fn text(text: impl Into<SharedString>) -> Self {
        Self::Text(text.into())
    }
    pub fn icon(path: impl Into<SharedString>) -> Self {
        Self::Icon(path.into())
    }
    /// Several addon items share source padding and a 6px gap.
    /// Nested collections are flattened; empty collections render no addon.
    pub fn parts(parts: impl IntoIterator<Item = Self>) -> Self {
        Self::Parts(parts.into_iter().collect())
    }
    pub(crate) fn is_empty(&self) -> bool {
        match self {
            Self::Parts(parts) => parts.iter().all(Self::is_empty),
            _ => false,
        }
    }
    fn items<'a>(&'a self, output: &mut Vec<&'a Self>) {
        match self {
            Self::Parts(parts) => parts.iter().for_each(|part| part.items(output)),
            _ => output.push(self),
        }
    }
    /// Configure a source-sized ghost button each render.
    ///
    /// This factory is retained by InputState. Capture WeakEntity handles for
    /// that state and its owner: strong captures form a reference cycle.
    /// ```
    /// # use gpui_kumo::{InputGroup, InputGroupAddon, InputState};
    /// # use gpui_kit::Entity;
    /// # fn clear_group(input: &Entity<InputState>) -> InputGroup {
    /// let target = input.downgrade();
    /// InputGroup::new("search", input).end(InputGroupAddon::button(
    ///     "clear", "Clear", move |button, _, _| {
    ///         let target = target.clone();
    ///         button.on_click(move |_, window, cx| {
    ///             let _ = target.update(cx, |state, cx| state.set_value("", window, cx));
    ///         })
    ///     },
    /// ))
    /// # }
    /// ```
    pub fn button(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        configure: impl Fn(crate::Button, &mut Window, &mut App) -> crate::Button + 'static,
    ) -> Self {
        let id = id.into();
        let label = label.into();
        Self::Action(std::rc::Rc::new(move |size, window, cx| {
            configure(
                crate::Button::new(id.clone(), label.clone())
                    .variant(crate::button::Variant::Ghost)
                    .size(compact_size(size)),
                window,
                cx,
            )
        }))
    }
    /// Icon-only action; the factory follows the same weak-capture contract as `button`.
    pub fn icon_button(
        id: impl Into<ElementId>,
        name: impl Into<SharedString>,
        path: impl Into<SharedString>,
        configure: impl Fn(crate::Button, &mut Window, &mut App) -> crate::Button + 'static,
    ) -> Self {
        let id = id.into();
        let name = name.into();
        let path = path.into();
        Self::Action(std::rc::Rc::new(move |size, window, cx| {
            let icon_size = match size {
                Size::Xs => 10.,
                Size::Sm => 13.,
                Size::Base => 18.,
                Size::Lg => 20.,
            };
            configure(
                crate::Button::icon(
                    id.clone(),
                    name.clone(),
                    crate::Icon::new(path.clone()).size(px(icon_size)),
                )
                .variant(crate::button::Variant::Ghost)
                .size(compact_size(size)),
                window,
                cx,
            )
        }))
    }
    pub(crate) fn render(
        &self,
        id: &'static str,
        context: AddonContext<'_>,
        window: &mut Window,
        cx: &mut App,
    ) -> gpui_kit::AnyElement {
        let AddonContext {
            theme,
            size,
            disabled,
            start,
        } = context;
        let (outer, icon_size) = match size {
            Size::Xs => (6., 10.),
            Size::Sm => (6., 13.),
            Size::Base => (8., 18.),
            Size::Lg => (10., 20.),
        };
        let mut items = Vec::new();
        self.items(&mut items);
        if items.is_empty() {
            return div().into_any_element();
        }
        let outer = if items.iter().any(|item| matches!(item, Self::Action(_))) {
            match (size, start) {
                (Size::Lg, true) => 6.,
                (Size::Lg, false) => 2.,
                _ => 4.,
            }
        } else {
            outer
        };
        let addon = div()
            .id(id)
            .test_support()
            .flex()
            .items_center()
            .flex_shrink_0()
            .gap(theme.spacing.six)
            .text_color(theme.text.subtle);
        let addon = if start {
            addon.pl(px(outer))
        } else {
            addon.pr(px(outer))
        };
        addon
            .children(items.into_iter().enumerate().map(|(index, item)| {
                let child = div().flex().items_center().child(match item {
                    Self::Action(render) => div()
                        .on_mouse_down(gpui_kit::MouseButton::Left, |_, _, cx| {
                            cx.stop_propagation()
                        })
                        .child(render(size, window, cx).input_group_action(disabled))
                        .into_any_element(),
                    Self::Text(text) => div().child(text.clone()).into_any_element(),
                    Self::Icon(path) => crate::Icon::new(path.clone())
                        .size(px(icon_size))
                        .into_any_element(),
                    Self::Parts(_) => {
                        unreachable!("collections are flattened before rendering")
                    }
                });
                // Action IDs are caller-owned; positional ancestor IDs would
                // recreate Base's keyed focus when passive parts move.
                if matches!(item, Self::Action(_)) {
                    child.into_any_element()
                } else {
                    child
                        .id(("addon-item", index))
                        .test_support()
                        .into_any_element()
                }
            }))
            .into_any_element()
    }
}
#[derive(Default)]
pub(crate) struct Container {
    pub start: Option<InputGroupAddon>,
    pub end: Option<InputGroupAddon>,
    pub suffix: Option<SharedString>,
    pub buttons: Vec<std::rc::Rc<ActionFactory>>,
}
/// Initial shared-container slice. The retained InputState owns editing and availability.
/// Passive addons use source padding and icon sizes; suffixes follow displayed text.
/// Compact actions retain Base activation and shared focus-within.
/// Direct non-ghost actions use individual or hybrid joined borders.
/// This builder currently places the retained editor before direct actions.
#[derive(IntoElement)]
#[must_use]
pub struct InputGroup {
    input: Input,
    container: Container,
}
impl InputGroup {
    pub fn new(id: impl Into<ElementId>, state: &Entity<InputState>) -> Self {
        Self {
            input: Input::new(id, state),
            container: Container::default(),
        }
    }
    pub fn size(mut self, size: Size) -> Self {
        self.input = self.input.size(size);
        self
    }
    pub fn label(mut self, show: bool) -> Self {
        self.input = self.input.label(show);
        self
    }
    /// Independent contextual help beside the group's visible label.
    pub fn label_tooltip(
        mut self,
        state: &Entity<crate::TooltipState>,
        content: impl Into<SharedString>,
    ) -> Self {
        self.input = self.input.label_tooltip(state, content);
        self
    }
    /// Explicit false adds the optional indicator to a visible label.
    /// This is presentation; validation and editor required semantics stay owner-controlled.
    pub fn required(mut self, required: bool) -> Self {
        self.input = self.input.required(required);
        self
    }
    /// An error suppresses helper text even when its message is hidden.
    /// The group remains invalid; `show` replaces the browser match result.
    pub fn error_visible(mut self, text: impl Into<SharedString>, show: bool) -> Self {
        self.input = self.input.error_visible(text.into(), show);
        self
    }
    pub fn description(mut self, text: impl Into<SharedString>) -> Self {
        self.input = self.input.description(text);
        self
    }
    pub fn error(mut self, text: impl Into<SharedString>) -> Self {
        self.input = self.input.error(text);
        self
    }
    pub fn start(mut self, addon: InputGroupAddon) -> Self {
        self.container.start = Some(addon);
        self
    }
    /// Text immediately follows the displayed value; long suffixes truncate.
    pub fn suffix(mut self, text: impl Into<SharedString>) -> Self {
        self.container.suffix = Some(text.into());
        self
    }
    /// Append a direct action. Non-ghost variants select joined border zones.
    /// The group controls sizing; use addon buttons for compact ghost actions.
    /// Factories are retained: capture weak handles for the editor or its owner.
    pub fn button(
        mut self,
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        variant: crate::button::Variant,
        configure: impl Fn(crate::Button, &mut Window, &mut App) -> crate::Button + 'static,
    ) -> Self {
        let id = id.into();
        let label = label.into();
        self.container
            .buttons
            .push(std::rc::Rc::new(move |size, window, cx| {
                configure(
                    crate::Button::new(id.clone(), label.clone())
                        .variant(variant)
                        .size(button_size(size)),
                    window,
                    cx,
                )
            }));
        self
    }
    pub fn end(mut self, addon: InputGroupAddon) -> Self {
        self.container.end = Some(addon);
        self
    }
}
impl RenderOnce for InputGroup {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        self.input.group(self.container)
    }
}

#[cfg(test)]
#[path = "input_group_tests.rs"]
mod tests;

// Installed Base 0.7.0 reserves this right-side caret-scroll margin.
// Glyph/hitbox clipping excludes the overlap; reserve the platform caret width
// as a small native gap so an End caret remains visible.
pub(crate) const EDITOR_CARET_MARGIN: gpui_kit::Pixels = px(10.);

#[cfg(not(target_os = "macos"))]
pub(crate) const SUFFIX_OVERLAP: gpui_kit::Pixels = px(8.);
#[cfg(target_os = "macos")]
pub(crate) const SUFFIX_OVERLAP: gpui_kit::Pixels = px(8.5);

pub(crate) fn compact_size(size: Size) -> crate::button::Size {
    match size {
        Size::Xs | Size::Sm => crate::button::Size::Xs,
        Size::Base => crate::button::Size::Sm,
        Size::Lg => crate::button::Size::Base,
    }
}

pub(crate) fn height(size: Size) -> gpui_kit::Pixels {
    px(match size {
        Size::Xs => 24.,
        Size::Sm => 28.,
        Size::Base => 36.,
        Size::Lg => 44.,
    })
}
pub(crate) fn button_size(size: Size) -> crate::button::Size {
    match size {
        Size::Xs => crate::button::Size::Xs,
        Size::Sm => crate::button::Size::Sm,
        Size::Base => crate::button::Size::Base,
        Size::Lg => crate::button::Size::Lg,
    }
}
#[derive(Clone)]
pub(crate) struct Zone {
    pub height: gpui_kit::Pixels,
    pub radius: gpui_kit::Pixels,
    pub last: bool,
    pub borders: crate::button::JoinedRingQueue,
}
// Preserve native layout/prepaint/Tab order; paint each inside border and one
// seam after sibling surfaces. No global deferred paint escapes a popup.
pub(crate) struct Zoned {
    pub body: gpui_kit::AnyElement,
    pub borders: crate::button::JoinedRingQueue,
}
impl IntoElement for Zoned {
    type Element = Self;
    fn into_element(self) -> Self {
        self
    }
}
impl gpui_kit::Element for Zoned {
    type RequestLayoutState = ();
    type PrepaintState = ();
    fn id(&self) -> Option<ElementId> {
        None
    }
    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }
    fn request_layout(
        &mut self,
        _: Option<&gpui_kit::GlobalElementId>,
        _: Option<&gpui_kit::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (gpui_kit::LayoutId, ()) {
        (self.body.request_layout(window, cx), ())
    }
    fn prepaint(
        &mut self,
        _: Option<&gpui_kit::GlobalElementId>,
        _: Option<&gpui_kit::InspectorElementId>,
        _: gpui_kit::Bounds<gpui_kit::Pixels>,
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        self.body.prepaint(window, cx);
    }
    fn paint(
        &mut self,
        _: Option<&gpui_kit::GlobalElementId>,
        _: Option<&gpui_kit::InspectorElementId>,
        _: gpui_kit::Bounds<gpui_kit::Pixels>,
        _: &mut (),
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        self.body.paint(window, cx);
        let mut borders = std::mem::take(&mut *self.borders.borrow_mut());
        borders.sort_by(|a, b| a.1.bounds.left().partial_cmp(&b.1.bounds.left()).unwrap());
        for (index, (_, quad, mask)) in borders.iter().enumerate() {
            let mut quad = quad.clone();
            if index > 0 {
                quad.border_widths.left = px(0.);
            }
            if index + 1 < borders.len() {
                quad.border_widths.right = px(0.);
            }
            window.with_content_mask(Some(*mask), |window| window.paint_quad(quad));
        }
        for pair in borders.windows(2) {
            let chosen = if pair[0].0 { &pair[0] } else { &pair[1] };
            let seam = gpui_kit::quad(
                gpui_kit::Bounds::new(
                    pair[1].1.bounds.origin,
                    gpui_kit::size(px(1.), pair[1].1.bounds.size.height),
                ),
                px(0.),
                chosen.1.border_color,
                px(0.),
                chosen.1.border_color.alpha(0.),
                Default::default(),
            );
            window.with_content_mask(Some(chosen.2), |window| window.paint_quad(seam));
        }
    }
}
