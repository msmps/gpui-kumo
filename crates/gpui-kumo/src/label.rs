//! Form labels with explicit native focus association.
use crate::{Tooltip, TooltipState, theme};
use gpui_kit::base::TestSupportExt;
use gpui_kit::{
    AnyElement, App, ElementId, Entity, FocusHandle, FontWeight, InteractiveElement, IntoElement,
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
    tooltip: Option<(Entity<TooltipState>, SharedString)>,
    as_content: bool,
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
            tooltip: None,
            as_content: false,
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
    /// Inherit surrounding typography when composed inside a styled label/control.
    pub fn as_content(mut self) -> Self {
        self.as_content = true;
        self
    }
    /// Contextual help uses a separately retained Tooltip and an independent
    /// info button. Label availability only gates the associated control focus.
    pub fn tooltip(
        mut self,
        state: &Entity<TooltipState>,
        content: impl Into<SharedString>,
    ) -> Self {
        let content = content.into();
        self.tooltip = Some((state.clone(), content));
        self
    }
    /// Rich decorative content; `new` remains the complete accessible name.
    /// Use `tooltip` for help; keep other interactive accessories outside the label.
    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }
}
impl RenderOnce for Label {
    fn render(mut self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        if self
            .tooltip
            .as_ref()
            .is_some_and(|(_, content)| content.is_empty())
            && let Some((state, _)) = self.tooltip.take()
        {
            state.update(cx, |state, cx| state.set_open(false, cx));
        }
        let theme = theme(cx);
        let name = if self.optional {
            format!("{} (optional)", self.text).into()
        } else {
            self.text.clone()
        };
        div()
            .id(self.id)
            .test_support()
            .when(!self.as_content, |this| {
                this.role(Role::Label)
                    .aria_label(name.clone())
                    .aria_value(name)
                    .on_mouse_down(gpui_kit::MouseButton::Left, |_, window, _| {
                        window.prevent_default()
                    })
            })
            .flex()
            .items_center()
            .gap(theme.spacing.four)
            .min_w_0()
            .when(!self.as_content, |this| {
                this.text_size(theme.typography.base.size)
                    .line_height(theme.typography.base.line_height)
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(theme.text.default)
            })
            .when(!self.disabled, |this| {
                this.when_some(self.focus, |this, focus| {
                    this.on_click(move |_, window, cx| focus.focus(window, cx))
                })
            })
            .child(
                div()
                    .min_w_0()
                    .child(self.content.unwrap_or_else(|| self.text.into_any_element())),
            )
            .when(self.optional, |this| {
                this.child(
                    div()
                        .font_weight(FontWeight::NORMAL)
                        .text_color(theme.text.subtle)
                        .child("(optional)"),
                )
            })
            .when_some(self.tooltip, |this, (state, content)| {
                this.child(Tooltip::new("label-tooltip", &state, content, |_, cx| {
                    crate::Button::icon(
                        "label-help",
                        "More information",
                        gpui_kit::svg()
                            .data(include_bytes!("../assets/info.svg"))
                            .size(gpui_kit::px(16.))
                            .text_color(crate::theme(cx).text.default),
                    )
                    .variant(crate::button::Variant::Ghost)
                    .size(crate::button::Size::Xs)
                    .on_click(|_, _, cx| cx.stop_propagation())
                }))
            })
    }
}

debug_struct!(Label {
    id,
    optional,
    disabled,
    as_content
});
