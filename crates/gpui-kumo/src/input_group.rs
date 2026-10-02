//! Shared-container InputGroup presentation over an existing retained editor.
use crate::{Input, InputState, Theme, input::Size};
use gpui_kit::base::TestSupportExt;
use gpui_kit::{
    App, ElementId, Entity, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, Styled, Window, div, px,
};

#[derive(Clone)]
pub enum InputGroupAddon {
    Text(SharedString),
    Icon(SharedString),
}
impl InputGroupAddon {
    pub fn text(text: impl Into<SharedString>) -> Self {
        Self::Text(text.into())
    }
    pub fn icon(path: impl Into<SharedString>) -> Self {
        Self::Icon(path.into())
    }
    pub(crate) fn render(
        &self,
        id: &'static str,
        outer: f32,
        icon_size: f32,
        start: bool,
        theme: &Theme,
    ) -> gpui_kit::AnyElement {
        let addon = div()
            .id(id)
            .test_support()
            .flex()
            .items_center()
            .flex_shrink_0()
            .text_color(theme.text.subtle);
        let addon = if start {
            addon.pl(px(outer))
        } else {
            addon.pr(px(outer))
        };
        match self {
            Self::Text(text) => addon.child(text.clone()).into_any_element(),
            Self::Icon(path) => addon
                .child(crate::Icon::new(path.clone()).size(px(icon_size)))
                .into_any_element(),
        }
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
/// Compact action buttons and individual/hybrid zones remain pending.
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
