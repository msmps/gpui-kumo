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
}
/// Initial shared-container slice. The retained InputState owns editing and availability.
/// Passive addons use source padding and icon sizes; suffixes follow displayed text.
/// Compact actions retain Base activation and shared focus-within.
/// Individual/hybrid zones remain pending.
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

fn compact_size(size: Size) -> crate::button::Size {
    match size {
        Size::Xs | Size::Sm => crate::button::Size::Xs,
        Size::Base => crate::button::Size::Sm,
        Size::Lg => crate::button::Size::Base,
    }
}
