//! Application-owned Field layout and message presentation.
use crate::{Label, Theme, TooltipState, theme};
use gpui_kit::{
    AnyElement, App, Div, ElementId, Entity, FocusHandle, InteractiveElement, IntoElement,
    ParentElement, Pixels, RenderOnce, SharedString, Styled, Window, div, prelude::FluentBuilder,
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

pub(crate) mod sealed {
    /// Sealed under the Layout composition contract.
    pub trait Sealed {}
}

/// A supported retained form control whose label and focus target can be associated safely.
/// Implementations are sealed; arbitrary content uses [`Field::new`] and owns its semantics.
pub trait FieldControl: sealed::Sealed + Sized {
    /// Consume the control with its built-in visible label hidden, retaining its semantic name.
    fn into_field_parts(self, cx: &App) -> (SharedString, FocusHandle, AnyElement);
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
    tooltip: Option<(Entity<TooltipState>, SharedString)>,
    error: Option<(SharedString, bool)>,
}
impl Field {
    /// Compose a supported control using its label once, with automatic focus association.
    /// Accessible-name overrides remain authoritative. Name changes are read on each owner render.
    ///
    /// ```no_run
    /// use gpui_kumo::{Field, Input, InputState};
    /// use gpui_kit::{App, Entity};
    /// fn field(input: &Entity<InputState>, cx: &App) -> Field {
    ///     Field::control("project-field", Input::new("project", input), cx)
    /// }
    /// ```
    pub fn control(id: impl Into<ElementId>, control: impl FieldControl, cx: &App) -> Self {
        let (label, focus, control) = control.into_field_parts(cx);
        Self::new(id, label, control).focus_target(&focus)
    }
    /// Compose a named label and arbitrary caller-owned content.
    /// This path cannot infer or update a child's accessible name. Set its semantics explicitly,
    /// and use `focus_target` for pointer association; prefer [`Self::control`] for supported forms.
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
            tooltip: None,
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
    /// Show or hide the label without changing control semantics or its focus target.
    pub fn show_label(mut self, show: bool) -> Self {
        self.hide_label = !show;
        self
    }
    /// Select stacked or explicit horizontal native layout.
    pub fn layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }
    /// Add an independently focusable contextual help button beside the label.
    pub fn label_tooltip(
        mut self,
        state: &Entity<TooltipState>,
        content: impl Into<SharedString>,
    ) -> Self {
        self.tooltip = Some((state.clone(), content.into()));
        self
    }
    /// Helper text, hidden whenever an error is supplied.
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }
    /// Show an application-owned error, suppressing the description.
    /// Error presentation does not validate or discard the control's value.
    pub fn error(self, text: impl Into<SharedString>) -> Self {
        self.error_visible(text, true)
    }
    /// Set an error with explicit message visibility; a hidden error still suppresses help.
    pub fn error_visible(mut self, error: impl Into<SharedString>, show: bool) -> Self {
        self.error = Some((error.into(), show));
        self
    }
}
impl RenderOnce for Field {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        if self.hide_label
            && let Some((state, _)) = &self.tooltip
        {
            state.update(cx, |state, cx| state.set_open(false, cx));
        }
        let theme = theme(cx);
        let label = (!self.hide_label).then(|| {
            Label::new("label", self.label)
                .optional(self.optional)
                .disabled(self.disabled)
                .when_some(self.focus, |label, focus| label.focus_target(&focus))
                .when_some(self.tooltip, |label, (state, content)| {
                    label.tooltip(&state, content)
                })
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
        let message = resolve_message(self.description, self.error);
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

pub(crate) fn resolve_message(
    description: Option<SharedString>,
    error: Option<(SharedString, bool)>,
) -> Option<(SharedString, bool)> {
    match error {
        Some((error, true)) => Some((error, true)),
        Some((_, false)) => None,
        None => description.map(|description| (description, false)),
    }
}

pub(crate) fn message_element(theme: &Theme, message: SharedString, invalid: bool) -> Div {
    text_message(
        "message",
        message,
        invalid,
        theme.typography.field_description.line_height,
    )
}

pub(crate) fn group_message_element(
    theme: &Theme,
    id: impl Into<ElementId>,
    message: SharedString,
    invalid: bool,
) -> Div {
    text_message(id, message, invalid, theme.typography.sm.line_height)
}

fn text_message(
    id: impl Into<ElementId>,
    message: SharedString,
    invalid: bool,
    line_height: Pixels,
) -> Div {
    div()
        .line_height(line_height)
        .child(
            crate::Text::new(id, message).style(crate::text::Style::Copy {
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

debug_struct!(Field {
    id,
    label,
    disabled,
    optional,
    hide_label,
    layout
});

#[cfg(test)]
mod tests;
