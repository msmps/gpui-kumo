//! Application-owned Field layout and message presentation.
use crate::{Label, Theme, theme};
use gpui_kit::{
    AnyElement, App, Div, ElementId, FocusHandle, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, SharedString, Styled, Window, div, prelude::FluentBuilder,
};

/// Native layout choice, replacing web descendant type selectors.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Layout {
    /// Label, control, then message, with 8px gaps.
    #[default]
    Stacked,
    /// Label then control on one row; message below.
    Inline,
    /// Control then label on one row; message below.
    ControlFirst,
}

/// A stateless field. Controls retain their own Base behavior and accessible name.
/// Validation lives in the application; no browser ValidityState is inferred.
#[derive(IntoElement)]
#[must_use]
pub struct Field {
    id: ElementId,
    label: SharedString,
    control: AnyElement,
    focus: Option<FocusHandle>,
    disabled: bool,
    optional: bool,
    hide_label: bool,
    layout: Layout,
    description: Option<SharedString>,
    error: Option<(SharedString, bool)>,
}
impl Field {
    /// Compose a named label and a caller-owned control.
    pub fn new(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        control: impl IntoElement,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            control: control.into_any_element(),
            focus: None,
            disabled: false,
            optional: false,
            hide_label: false,
            layout: Layout::default(),
            description: None,
            error: None,
        }
    }
    /// Associate label activation with the control's retained focus handle.
    pub fn focus_target(mut self, focus: &FocusHandle) -> Self {
        self.focus = Some(focus.clone());
        self
    }
    /// Explicit false adds the optional label indicator; other values omit it.
    /// This is presentation, not an application validation rule.
    pub fn required(mut self, required: bool) -> Self {
        self.optional = !required;
        self
    }
    /// Gate label focus forwarding. The consumer must also disable the control.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    /// Skip the label when the child owns its own label composition.
    pub fn hide_label(mut self, hide: bool) -> Self {
        self.hide_label = hide;
        self
    }
    /// Select stacked or explicit horizontal native layout.
    pub fn layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }
    /// Helper text, hidden whenever an error is supplied.
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }
    /// Error takes precedence over helper text, even when `show` is false.
    /// Synchronize the control's invalid state separately through its own API.
    pub fn error(mut self, error: impl Into<SharedString>, show: bool) -> Self {
        self.error = Some((error.into(), show));
        self
    }
}
impl RenderOnce for Field {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = theme(cx);
        let label = (!self.hide_label).then(|| {
            Label::new("label", self.label)
                .optional(self.optional)
                .disabled(self.disabled)
                .when_some(self.focus, |label, focus| label.focus_target(&focus))
        });
        let contents = div()
            .flex()
            .min_w_0()
            .gap(theme.spacing.eight)
            .when(self.layout == Layout::Stacked, |this| this.flex_col())
            .when(self.layout != Layout::Stacked, |this| this.items_center())
            .when(self.layout == Layout::ControlFirst, |this| {
                this.flex_row_reverse().flex_wrap()
            })
            .children(label.map(|label| {
                if self.layout == Layout::ControlFirst {
                    div().flex_1().min_w_0().child(label).into_any_element()
                } else {
                    label.into_any_element()
                }
            }))
            .child(self.control);
        let message = match self.error {
            Some((error, true)) => Some((error, true)),
            Some((_, false)) => None,
            None => self.description.map(|description| (description, false)),
        };
        div()
            .id(self.id)
            .w_full()
            .min_w_0()
            .flex()
            .flex_col()
            .gap(theme.spacing.eight)
            .child(contents)
            .when_some(message, |this, (text, invalid)| {
                this.child(message_element(theme, text, invalid))
            })
    }
}

pub(crate) fn message_element(theme: &Theme, message: SharedString, invalid: bool) -> Div {
    div()
        .text_size(theme.typography.field_description.size)
        .line_height(theme.typography.field_description.line_height)
        .text_color(if invalid {
            theme.text.danger
        } else {
            theme.text.subtle
        })
        .child(
            crate::Text::new("message", message).style(crate::text::Style::Copy {
                tone: if invalid {
                    crate::text::Tone::Error
                } else {
                    crate::text::Tone::Secondary
                },
                size: crate::text::Size::Sm,
                bold: false,
            }),
        )
}

#[cfg(test)]
#[path = "field_tests.rs"]
mod tests;
