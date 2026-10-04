//! Multiline InputArea over a retained Base Textarea, with Kumo-owned presentation.
use crate::{Theme, input::Size, theme};
use base::TestSupportExt;
use gpui_kit::{
    App, AppContext, Context, ElementId, Entity, EventEmitter, FocusHandle, Focusable, FontWeight,
    InteractiveElement, IntoElement, ParentElement, Render, RenderOnce, SharedString,
    StatefulInteractiveElement, Styled, Subscription, Window, base, canvas, div,
    prelude::FluentBuilder, quad,
};

/// Notifications from the editing engine; read the current value from the state.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum InputAreaEvent {
    /// The user changed the value.
    Change,
    /// The retained control received focus.
    Focus,
    /// The retained control lost focus.
    Blur,
}

/// Retain once per mount. Base owns text, selection, history and native input.
/// Owner set_value does not emit Change and resets selection/history like Input.
pub struct InputAreaState {
    editor: Entity<base::input::TextareaState>,
    name: SharedString,
    disabled: bool,
    read_only: bool,
    presentation: Presentation,
    applied_rows: (usize, usize),
    _subscriptions: Vec<Subscription>,
}
impl EventEmitter<InputAreaEvent> for InputAreaState {}
impl InputAreaState {
    fn label_text(&self) -> SharedString {
        self.presentation
            .label_text
            .clone()
            .unwrap_or_else(|| self.name.clone())
    }
    fn accessible_name(&self) -> SharedString {
        self.presentation
            .accessible_name
            .clone()
            .unwrap_or_else(|| self.label_text())
    }

    /// Create a retained named multiline editor with an initially empty value. Retain with `cx.new`.
    ///
    /// # Panics
    /// Panics when the required name or label is blank.
    pub fn new(name: impl Into<SharedString>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let name = name.into();
        assert!(
            !name.trim().is_empty(),
            "InputArea requires an accessible name"
        );
        let editor = cx.new(|cx| base::input::TextareaState::new(window, cx).auto_grow(2, 2));
        let events = cx.subscribe(&editor, |_, _, event, cx| match event {
            base::input::InputEvent::Change => cx.emit(InputAreaEvent::Change),
            base::input::InputEvent::Focus => cx.emit(InputAreaEvent::Focus),
            base::input::InputEvent::Blur => cx.emit(InputAreaEvent::Blur),
            base::input::InputEvent::PressEnter { .. } => {}
        });
        let observer = cx.observe(&editor, |_, _, cx| cx.notify());
        let theme = cx.observe_global::<Theme>(|_, cx| cx.notify());
        Self {
            editor,
            name,
            disabled: false,
            read_only: false,
            presentation: Presentation::default(),
            applied_rows: (2, 2),
            _subscriptions: vec![events, observer, theme],
        }
    }
    /// Read the retained default name; rendered props may explicitly override it.
    pub fn name(&self) -> &SharedString {
        &self.name
    }

    /// Refresh the default label and accessible name while preserving value, focus and lifecycle.
    ///
    /// # Panics
    /// Panics when `name` is blank. Validate external text with [`crate::AccessibleName`].
    pub fn set_name(&mut self, name: impl Into<SharedString>, cx: &mut Context<Self>) {
        let name = name.into();
        assert!(
            !name.trim().is_empty(),
            "A control requires a nonblank accessible name"
        );
        self.name = name;
        cx.notify();
    }

    /// Read the current value from its owner; this does not request a change.
    pub fn value(&self, cx: &App) -> SharedString {
        self.editor.read(cx).value()
    }
    /// Read the currently selected text without changing selection.
    pub fn selected_value(&self, cx: &App) -> SharedString {
        self.editor.read(cx).selected_value()
    }
    /// Replace the owner value programmatically; this is not a user activation proposal.
    pub fn set_value(
        &mut self,
        value: impl Into<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let value = value.into();
        self.editor
            .update(cx, |editor, cx| editor.set_value(value, window, cx));
    }
    /// Update empty-editor guidance without replacing the retained editor.
    pub fn set_placeholder(
        &mut self,
        value: impl Into<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let value = value.into();
        self.editor
            .update(cx, |editor, cx| editor.set_placeholder(value, window, cx));
    }
    /// Report whether the owner has disabled this control.
    pub fn is_disabled(&self) -> bool {
        self.disabled
    }
    /// Report whether user editing is blocked while selection and copying remain available.
    pub fn is_read_only(&self) -> bool {
        self.read_only
    }
    /// Update availability and notify presentation while retaining the value.
    pub fn set_disabled(&mut self, disabled: bool, cx: &mut Context<Self>) {
        if self.disabled == disabled {
            return;
        }
        self.disabled = disabled;
        self.editor
            .update(cx, |editor, cx| editor.set_disabled(disabled, cx));
        self.focus_handle(cx).tab_stop(!disabled);
        cx.notify();
    }
    /// Update editing availability while retaining focus, selection and value.
    pub fn set_read_only(&mut self, read_only: bool, cx: &mut Context<Self>) {
        if self.read_only == read_only {
            return;
        }
        self.read_only = read_only;
        self.editor
            .update(cx, |editor, cx| editor.set_readonly(read_only, cx));
        cx.notify();
    }
}
impl Focusable for InputAreaState {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.read(cx).focus_handle(cx)
    }
}
struct Presentation {
    size: Size,
    rows: usize,
    auto_resize: Option<(usize, Option<usize>)>,
    label: bool,
    label_text: Option<SharedString>,
    accessible_name: Option<SharedString>,
    optional: bool,
    description: Option<SharedString>,
    error: Option<(SharedString, bool)>,
    tooltip: Option<(Entity<crate::TooltipState>, SharedString)>,
}
impl Default for Presentation {
    fn default() -> Self {
        Self {
            size: Size::Base,
            rows: 2,
            auto_resize: None,
            label: false,
            label_text: None,
            accessible_name: None,
            optional: false,
            description: None,
            error: None,
            tooltip: None,
        }
    }
}
/// Consumed presentation over an application-retained state.
#[derive(IntoElement)]
#[must_use]
pub struct InputArea {
    id: ElementId,
    state: Entity<InputAreaState>,
    presentation: Presentation,
}
impl InputArea {
    /// Present the retained multiline editor under a stable element identity.
    pub fn new(id: impl Into<ElementId>, state: &Entity<InputAreaState>) -> Self {
        Self {
            id: id.into(),
            state: state.clone(),
            presentation: Presentation::default(),
        }
    }
    /// Select the component dimensions and corresponding spacing and typography.
    pub fn size(mut self, size: Size) -> Self {
        self.presentation.size = size;
        self
    }
    /// Fixed visible rows; ignored while auto_resize is enabled. Zero normalizes to one.
    pub fn rows(mut self, rows: usize) -> Self {
        self.presentation.rows = rows.max(1);
        self
    }
    /// Grow and shrink with wrapped content. None means no maximum.
    /// Zero minimum normalizes to one; a lower maximum normalizes to the minimum.
    pub fn auto_resize(mut self, min_rows: usize, max_rows: Option<usize>) -> Self {
        let min_rows = min_rows.max(1);
        self.presentation.auto_resize = Some((min_rows, max_rows.map(|max| max.max(min_rows))));
        self
    }
    /// Set visible text and its default accessible name; visibility is controlled separately.
    ///
    /// # Panics
    /// Panics when `text` is blank.
    pub fn label(mut self, text: impl Into<SharedString>) -> Self {
        let text = crate::name::nonblank(text);
        self.presentation.label_text = Some(text);
        self
    }
    /// Override the accessible name independently of label visibility and builder order.
    ///
    /// # Panics
    /// Panics when `name` is blank.
    pub fn accessibility_label(mut self, name: impl Into<SharedString>) -> Self {
        self.presentation.accessible_name = Some(crate::name::nonblank(name));
        self
    }
    /// Show or hide visible label presentation while retaining its accessible name.
    pub fn show_label(mut self, show: bool) -> Self {
        self.presentation.label = show;
        self
    }
    /// Choose the optional indicator: false shows “(optional)”. This does not perform validation.
    pub fn required(mut self, required: bool) -> Self {
        self.presentation.optional = !required;
        self
    }
    /// Supply supporting text; errors take precedence even when their message is hidden.
    pub fn description(mut self, text: impl Into<SharedString>) -> Self {
        self.presentation.description = Some(text.into());
        self
    }
    /// Error styling applies even when its message is hidden. Owner selects validity.
    /// Show an application-owned error, suppressing the description.
    /// Error presentation does not validate or discard the control's value.
    pub fn error(self, text: impl Into<SharedString>) -> Self {
        self.error_visible(text, true)
    }
    /// Set an error with explicit message visibility; a hidden error still suppresses help.
    pub fn error_visible(mut self, text: impl Into<SharedString>, show: bool) -> Self {
        self.presentation.error = Some((text.into(), show));
        self
    }
    /// Compose independently focusable contextual help beside the visible label.
    pub fn label_tooltip(
        mut self,
        state: &Entity<crate::TooltipState>,
        text: impl Into<SharedString>,
    ) -> Self {
        self.presentation.tooltip = Some((state.clone(), text.into()));
        self
    }
}
impl RenderOnce for InputArea {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        self.state.update(cx, |state, cx| {
            let old = state.presentation.tooltip.as_ref().map(|(s, _)| s.clone());
            let new = self.presentation.tooltip.as_ref().map(|(s, _)| s);
            if let Some(old) = old
                && new != Some(&old)
            {
                old.update(cx, |s, cx| s.set_open(false, cx));
            }
            state.presentation = self.presentation;
            if !state.presentation.label
                && let Some((help, _)) = &state.presentation.tooltip
            {
                help.update(cx, |s, cx| s.set_open(false, cx));
            }
        });
        div().id(self.id).w_full().min_w_0().child(self.state)
    }
}
impl Render for InputAreaState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = theme(cx).clone();
        let (_, padding, radius, text) = crate::input::metrics(self.presentation.size, &theme);
        let rows = self.presentation.auto_resize.map_or(
            (self.presentation.rows, self.presentation.rows),
            |(min, max)| (min, max.unwrap_or(usize::MAX)),
        );
        if rows != self.applied_rows {
            self.editor
                .update(cx, |editor, cx| editor.set_auto_grow(rows.0, rows.1, cx));
            self.applied_rows = rows;
        }
        let focus = self.focus_handle(cx);
        let focused = !self.disabled && focus.is_focused(window);
        let invalid = self.presentation.error.is_some();
        let ring = if invalid {
            theme.colors.danger
        } else if focused {
            theme.colors.focus.opacity(0.5)
        } else {
            theme.colors.line
        };
        let width = if focused {
            theme.effects.input_focus_ring_width
        } else {
            theme.effects.control_ring_width
        };
        let foreground = if self.disabled {
            theme.native.disabled_input_foreground
        } else {
            theme.text.default
        };
        self.editor.update(cx, |editor, _| {
            editor.set_editor_style(base::input::InputEditorStyle {
                foreground,
                muted_foreground: theme.text.placeholder,
                selection: theme.native.selection,
                caret: theme.text.default,
                ..Default::default()
            })
        });
        let message = crate::field::resolve_message(
            self.presentation.description.clone(),
            self.presentation.error.clone(),
        );
        let disabled = self.disabled;
        let read_only = self.read_only;
        let target = focus.clone();
        let control = base::input::InputBase::new("control")
            .role(gpui_kit::Role::MultilineTextInput)
            .accessibility_label(self.accessible_name())
            .aria_value(self.value(cx))
            .aria_placeholder(self.editor.read(cx).presentation().placeholder().clone())
            .when_some(message, |this, (text, _)| this.aria_description(text))
            .a11y_synthetic_children(move |builder| {
                let node = builder.parent_node();
                if disabled {
                    node.set_disabled();
                }
                if read_only {
                    node.set_read_only();
                }
                if invalid {
                    node.set_invalid(gpui_kit::accesskit::Invalid::True);
                }
            })
            .track_focus(&focus)
            .w_full()
            .min_w_0()
            .child(base::input::Textarea::new(&self.editor));
        let surface = div()
            .id("surface")
            .test_support()
            .w_full()
            .min_w_0()
            .relative()
            .px(padding)
            .py(theme.spacing.eight)
            .rounded(radius)
            .bg(theme.colors.control)
            .text_color(foreground)
            .font_family(theme.typography.font_family.clone())
            .font_weight(FontWeight::NORMAL)
            .text_size(text.size)
            .line_height(text.line_height)
            .capture_key_down(move |event, window, cx| {
                if !disabled
                    && event.keystroke.key == "tab"
                    && !event.keystroke.modifiers.control
                    && !event.keystroke.modifiers.alt
                    && !event.keystroke.modifiers.platform
                {
                    if event.keystroke.modifiers.shift {
                        window.focus_prev(cx);
                    } else {
                        window.focus_next(cx);
                    }
                    window.prevent_default();
                    cx.stop_propagation();
                }
            })
            .capture_any_mouse_down(move |_, window, cx| {
                if disabled {
                    window.prevent_default();
                    cx.stop_propagation();
                }
            })
            .on_mouse_down(gpui_kit::MouseButton::Left, move |_, window, cx| {
                if !disabled {
                    target.focus(window, cx);
                }
                window.prevent_default();
            })
            .child(control)
            .child(
                canvas(
                    |_, _, _| (),
                    move |bounds, _, window, _| {
                        window.paint_quad(quad(
                            bounds.dilate(width),
                            radius + width,
                            ring.alpha(0.),
                            width,
                            ring,
                            Default::default(),
                        ));
                    },
                )
                .absolute()
                .top_0()
                .left_0()
                .size_full(),
            );
        crate::Field::new("field", self.label_text(), surface)
            .focus_target(&focus)
            .disabled(disabled)
            .show_label(self.presentation.label)
            .required(!self.presentation.optional)
            .when_some(self.presentation.description.clone(), |field, text| {
                field.description(text)
            })
            .when_some(self.presentation.error.clone(), |field, (text, show)| {
                field.error_visible(text, show)
            })
            .when_some(self.presentation.tooltip.clone(), |field, (state, text)| {
                field.label_tooltip(&state, text)
            })
    }
}
/// Discoverability alias matching Kumo's Textarea export.
pub type Textarea = InputArea;
impl crate::field::sealed::Sealed for InputArea {}
impl crate::field::FieldControl for InputArea {
    fn into_field_parts(self, cx: &App) -> (SharedString, FocusHandle, gpui_kit::AnyElement) {
        let label = self
            .presentation
            .label_text
            .clone()
            .unwrap_or_else(|| self.state.read(cx).name.clone());
        let focus = self.state.read(cx).focus_handle(cx);
        (label, focus, self.show_label(false).into_any_element())
    }
}

impl std::fmt::Debug for InputAreaState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InputAreaState")
            .field("name", &self.name)
            .field("disabled", &self.disabled)
            .field("read_only", &self.read_only)
            .finish_non_exhaustive()
    }
}

impl std::fmt::Debug for InputArea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InputArea")
            .field("id", &self.id)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests;
