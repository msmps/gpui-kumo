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
    /// Transfer label and feedback presentation, retaining the control until Field renders.
    fn into_field(self, id: impl Into<ElementId>, cx: &App) -> Field;
}

type ErrorPresentation = Option<(SharedString, bool)>;
type FinishControl = Box<dyn FnOnce(ErrorPresentation, Option<SharedString>) -> AnyElement>;
enum Control {
    Custom(AnyElement),
    Associated(FinishControl),
}

pub(crate) struct ControlPresentation {
    pub label: SharedString,
    pub focus: FocusHandle,
    pub disabled: bool,
    pub show_label: bool,
    pub optional: bool,
    pub tooltip: Option<(Entity<TooltipState>, SharedString)>,
    pub description: Option<SharedString>,
    pub error: ErrorPresentation,
}

/// A stateless field. Controls retain their own Base behavior and accessible name.
/// Validation lives in the application; no browser ValidityState is inferred.
#[derive(IntoElement)]
#[must_use]
pub struct Field {
    id: ElementId,
    label: SharedString,
    control: Control,
    focus: Option<FocusHandle>,
    label_disabled: bool,
    control_disabled: bool,
    optional: bool,
    hide_label: bool,
    layout: Layout,
    description: Option<SharedString>,
    tooltip: Option<(Entity<TooltipState>, SharedString)>,
    error: Option<(SharedString, bool)>,
}
impl Field {
    pub(crate) fn associated(
        id: ElementId,
        presentation: ControlPresentation,
        finish: impl FnOnce(ErrorPresentation, Option<SharedString>) -> AnyElement + 'static,
    ) -> Self {
        Self {
            id,
            label: presentation.label,
            focus: Some(presentation.focus),
            control: Control::Associated(Box::new(finish)),
            control_disabled: presentation.disabled,
            label_disabled: false,
            optional: presentation.optional,
            hide_label: !presentation.show_label,
            layout: Layout::default(),
            description: presentation.description,
            tooltip: presentation.tooltip,
            error: presentation.error,
        }
    }
    /// Compose a supported control using its label once, with automatic focus association.
    /// Label visibility, optional indicator, tooltip, description and error are inherited.
    /// Field props override inherited presentation; errors also mark the control invalid.
    /// Availability follows retained state; accessible-name overrides remain authoritative.
    ///
    /// ```no_run
    /// use gpui_kumo::{Field, Input, InputState};
    /// use gpui_kit::{App, Entity};
    /// fn field(input: &Entity<InputState>, cx: &App) -> Field {
    ///     Field::control("project-field", Input::new("project", input).optional_indicator(true), cx)
    /// }
    /// fn disable(input: &Entity<InputState>, cx: &mut App) {
    ///     input.update(cx, |state, cx| state.set_disabled(true, cx));
    /// }
    /// ```
    /// Whole-control availability belongs to retained state, not the wrapper:
    /// ```compile_fail,E0599
    /// use gpui_kumo::{Field, Input, InputState};
    /// use gpui_kit::{App, Entity};
    /// fn invalid(input: &Entity<InputState>, cx: &App) {
    ///     Field::control("field", Input::new("input", input), cx).disabled(true);
    /// }
    /// ```
    pub fn control(id: impl Into<ElementId>, control: impl FieldControl, cx: &App) -> Self {
        control.into_field(id, cx)
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
            control: Control::Custom(control.into_any_element()),
            focus: None,
            label_disabled: false,
            control_disabled: false,
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
    /// Show or hide the optional label indicator.
    /// This is presentation, not an application validation rule.
    pub fn optional_indicator(mut self, optional: bool) -> Self {
        self.optional = optional;
        self
    }
    /// Gate label activation for custom content without disabling the child.
    /// Supported controls automatically inherit availability from their retained state.
    pub fn label_disabled(mut self, disabled: bool) -> Self {
        self.label_disabled = disabled;
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
    /// Supported controls also receive invalid appearance/semantics; values are retained.
    /// Custom content owns its own invalid state.
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
        let control = match self.control {
            Control::Custom(control) => control,
            Control::Associated(finish) => finish(
                self.error.clone(),
                resolve_message(self.description.clone(), self.error.clone()).map(|(text, _)| text),
            ),
        };
        if self.hide_label
            && let Some((state, _)) = &self.tooltip
        {
            state.update(cx, |state, cx| state.set_open(false, cx));
        }
        let theme = theme(cx);
        let label = (!self.hide_label).then(|| {
            Label::new("label", self.label)
                .optional(self.optional)
                .disabled(self.label_disabled || self.control_disabled)
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
            .child(control);
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

impl std::fmt::Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Field")
            .field("id", &self.id)
            .field("label", &self.label)
            .field("label_disabled", &self.label_disabled)
            .field("control_disabled", &self.control_disabled)
            .field("optional", &self.optional)
            .field("hide_label", &self.hide_label)
            .field("layout", &self.layout)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests;
