//! Form labels with explicit native focus association.
use crate::theme;
use gpui_kit::base::TestSupportExt;
use gpui_kit::{
    AnyElement, App, ElementId, FocusHandle, FontWeight, InteractiveElement, IntoElement,
    ParentElement, RenderOnce, Role, SharedString, StatefulInteractiveElement, Styled, Window, div,
    prelude::FluentBuilder,
};

/// Label presentation. Association is a focus target, replacing web `htmlFor`.
/// It does not assign the target's accessible name; consumers must name controls.
#[derive(IntoElement)]
#[must_use]
pub struct Label {
    id: ElementId,
    text: SharedString,
    optional: bool,
    disabled: bool,
    focus: Option<FocusHandle>,
    content: Option<AnyElement>,
}
impl Label {
    /// Create a label from its complete readable name.
    pub fn new(id: impl Into<ElementId>, text: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            optional: false,
            disabled: false,
            focus: None,
            content: None,
        }
    }
    /// Show Kumo's normal-weight supporting “(optional)” indicator.
    pub fn optional(mut self, optional: bool) -> Self {
        self.optional = optional;
        self
    }
    /// Associate pointer activation with an existing control's focus handle.
    pub fn focus_target(mut self, focus: &FocusHandle) -> Self {
        self.focus = Some(focus.clone());
        self
    }
    /// Prevent label activation without changing readable label presentation.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    /// Rich decorative content; `new` remains the complete accessible name.
    /// Keep independent interactive accessories outside the label.
    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }
}
impl RenderOnce for Label {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = theme(cx);
        let name = if self.optional {
            format!("{} (optional)", self.text).into()
        } else {
            self.text.clone()
        };
        div()
            .id(self.id)
            .test_support()
            .role(Role::Label)
            .aria_label(name)
            .flex()
            .items_center()
            .gap(theme.spacing.four)
            .min_w_0()
            .text_size(theme.typography.base.size)
            .line_height(theme.typography.base.line_height)
            .font_weight(FontWeight::MEDIUM)
            .text_color(theme.text.default)
            .when(!self.disabled, |this| {
                this.when_some(self.focus, |this, focus| {
                    this.on_click(move |_, window, cx| focus.focus(window, cx))
                })
            })
            .child(self.content.unwrap_or_else(|| self.text.into_any_element()))
            .when(self.optional, |this| {
                this.child(
                    div()
                        .font_weight(FontWeight::NORMAL)
                        .text_color(theme.text.subtle)
                        .child("(optional)"),
                )
            })
    }
}
