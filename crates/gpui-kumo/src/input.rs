//! Single-line Input with retained editing state and Kumo-owned presentation.

use gpui_kit::{
    App, AppContext, Context, ElementId, Entity, EventEmitter, FocusHandle, Focusable, FontWeight,
    Hsla, InteractiveElement, IntoElement, ParentElement, Render, RenderOnce, SharedString,
    StatefulInteractiveElement, Styled, Subscription, Window, base, canvas, div,
    prelude::FluentBuilder, px, quad,
};

use crate::{Theme, theme};

mod accessibility;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Size {
    Xs,
    Sm,
    #[default]
    Base,
    Lg,
}

/// Notifications from the editing engine. Read the value from the emitting state.
#[derive(Clone, Debug)]
pub enum InputEvent {
    Change,
    Submit { secondary: bool, shift: bool },
    Focus,
    Blur,
}

/// Durable value, selection, clipboard/IME handling, history and focus.
/// Create once with `cx.new`, retain the entity, and mount it in one Input.
/// Programmatic `set_value` resets selection/history without emitting Change.
pub struct InputState {
    editor: Entity<base::input::InputState>,
    name: SharedString,
    disabled: bool,
    read_only: bool,
    presentation: Presentation,
    accessibility: accessibility::Bridge,
    _subscriptions: Vec<Subscription>,
}

impl EventEmitter<InputEvent> for InputState {}

impl InputState {
    /// The accessible name must be nonempty; a placeholder is not a label.
    pub fn new(name: impl Into<SharedString>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let name = name.into();
        assert!(!name.trim().is_empty(), "Input requires an accessible name");
        let editor = cx.new(|cx| base::input::InputState::new(window, cx));
        let events = cx.subscribe(&editor, |_, _, event, cx| {
            let event = match event {
                base::input::InputEvent::Change => InputEvent::Change,
                base::input::InputEvent::PressEnter { secondary, shift } => InputEvent::Submit {
                    secondary: *secondary,
                    shift: *shift,
                },
                base::input::InputEvent::Focus => InputEvent::Focus,
                base::input::InputEvent::Blur => InputEvent::Blur,
            };
            cx.emit(event);
            cx.notify();
        });
        let theme = cx.observe_global::<Theme>(|_, cx| cx.notify());
        // Selection-only notifications do not emit InputEvent::Change. Refresh
        // the containing accessibility node when Base moves its caret as well.
        let editor_observer = cx.observe(&editor, |_, _, cx| cx.notify());
        Self {
            editor,
            name,
            disabled: false,
            read_only: false,
            presentation: Presentation::default(),
            accessibility: accessibility::Bridge::default(),
            _subscriptions: vec![events, theme, editor_observer],
        }
    }

    pub fn value(&self, cx: &App) -> SharedString {
        self.editor.read(cx).value()
    }

    pub fn selected_value(&self, cx: &App) -> SharedString {
        self.editor.read(cx).selected_value()
    }

    pub fn set_value(
        &mut self,
        value: impl Into<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let value = value.into();
        self.editor
            .update(cx, |editor, cx| editor.set_value(value, window, cx));
        cx.notify();
    }

    pub fn set_placeholder(
        &mut self,
        placeholder: impl Into<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let placeholder = placeholder.into();
        self.editor.update(cx, |editor, cx| {
            editor.set_placeholder(placeholder, window, cx)
        });
        cx.notify();
    }

    pub fn is_disabled(&self) -> bool {
        self.disabled
    }
    pub fn is_read_only(&self) -> bool {
        self.read_only
    }

    /// Reject user edits and exclude the control from Tab traversal.
    /// Existing focus is retained; the owner may move it explicitly.
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

    /// Preserve focus, selection and copying while rejecting user edits.
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

impl Focusable for InputState {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.read(cx).focus_handle(cx)
    }
}

#[derive(Default)]
struct Presentation {
    size: Size,
    label: bool,
    description: Option<SharedString>,
    error: Option<SharedString>,
}

/// Consumed presentation over an application-retained `Entity<InputState>`.
/// Width fills the parent; use a bounded parent for narrower fields.
/// Errors are application-owned display state and do not reject editing.
#[derive(IntoElement)]
#[must_use]
pub struct Input {
    id: ElementId,
    state: Entity<InputState>,
    presentation: Presentation,
}

impl Input {
    pub fn new(id: impl Into<ElementId>, state: &Entity<InputState>) -> Self {
        Self {
            id: id.into(),
            state: state.clone(),
            presentation: Presentation::default(),
        }
    }
    pub fn size(mut self, size: Size) -> Self {
        self.presentation.size = size;
        self
    }
    /// Show the state's accessible name as a clickable field label.
    pub fn label(mut self, show: bool) -> Self {
        self.presentation.label = show;
        self
    }
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.presentation.description = Some(description.into());
        self
    }
    /// Error text replaces the description and selects the danger ring.
    pub fn error(mut self, error: impl Into<SharedString>) -> Self {
        self.presentation.error = Some(error.into());
        self
    }
}

impl RenderOnce for Input {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        // Presentation is refreshed by the owner; the editing entity is never recreated.
        self.state
            .update(cx, |state, _| state.presentation = self.presentation);
        div().id(self.id).w_full().min_w_0().child(self.state)
    }
}

impl Render for InputState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = theme(cx).clone();
        let (height, padding, radius, text) = match self.presentation.size {
            Size::Xs => (20., theme.spacing.six, theme.radii.sm, theme.typography.xs),
            Size::Sm => (
                26.,
                theme.spacing.eight,
                theme.radii.md,
                theme.typography.xs,
            ),
            Size::Base => (
                36.,
                theme.spacing.twelve,
                theme.radii.lg,
                theme.typography.base,
            ),
            Size::Lg => (
                40.,
                theme.spacing.sixteen,
                theme.radii.lg,
                theme.typography.base,
            ),
        };
        let focus = self.focus_handle(cx);
        let focused = !self.disabled && focus.is_focused(window);
        let invalid = self.presentation.error.is_some();
        let ring_color = if invalid {
            theme.colors.danger
        } else if focused {
            theme.colors.focus.opacity(0.5)
        } else {
            theme.colors.line
        };
        let ring_width = if focused {
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
        let editor = self.editor.read(cx);
        let placeholder = editor.presentation().placeholder().clone();
        let value = editor.value();
        let description = self
            .presentation
            .error
            .clone()
            .or_else(|| self.presentation.description.clone());
        let semantic_description = [
            description.as_deref(),
            self.disabled.then_some("Unavailable"),
            self.read_only.then_some("Read only"),
            invalid.then_some("Invalid"),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join(". ");
        let label_focus = focus.clone();
        let disabled = self.disabled;
        let read_only = self.read_only;
        let accessibility = self.accessibility.clone();
        let text_prepaint = accessibility.clone();
        let text_editor = self.editor.downgrade();
        let selection_state = cx.entity().downgrade();
        let value_state = selection_state.clone();
        let replace_state = selection_state.clone();
        let paint_state = selection_state.clone();
        div()
            .flex()
            .flex_col()
            .w_full()
            .min_w_0()
            .gap(theme.spacing.eight)
            .when(self.presentation.label, |this| {
                this.child(
                    div()
                        .id("label")
                        .text_size(theme.typography.base.size)
                        .line_height(theme.typography.base.line_height)
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(theme.text.default)
                        .on_click(move |_, window, cx| {
                            if !disabled {
                                label_focus.focus(window, cx);
                            }
                        })
                        .child(self.name.clone()),
                )
            })
            .child(
                base::input::InputBase::new("control")
                    .accessibility_label(self.name.clone())
                    .aria_value(value)
                    .aria_placeholder(placeholder)
                    .aria_description(semantic_description)
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
                        accessibility.build(builder);
                    })
                    .when(!disabled, |this| {
                        let run_id = text_prepaint.run_id.clone();
                        this.on_a11y_action(
                            gpui_kit::AccessibleAction::SetTextSelection,
                            move |data, window, cx| {
                                let _ = selection_state.update(cx, |state, cx| {
                                    state.accessibility_action(run_id.get(), data, window, cx);
                                });
                            },
                        )
                    })
                    .when(!disabled && !self.read_only, |this| {
                        this.on_a11y_action(
                            gpui_kit::AccessibleAction::SetValue,
                            move |data, window, cx| {
                                let _ = value_state.update(cx, |state, cx| {
                                    state.accessibility_action(None, data, window, cx);
                                });
                            },
                        )
                        .on_a11y_action(
                            gpui_kit::AccessibleAction::ReplaceSelectedText,
                            move |data, window, cx| {
                                let _ = replace_state.update(cx, |state, cx| {
                                    state.accessibility_replace_selection(data, window, cx);
                                });
                            },
                        )
                    })
                    .track_focus(&focus)
                    .w_full()
                    .min_w_0()
                    .h(px(height))
                    .px(padding)
                    .rounded(radius)
                    .relative()
                    .bg(theme.colors.control)
                    .text_color(foreground)
                    .font_family(theme.typography.font_family.clone())
                    .text_size(text.size)
                    .line_height(text.line_height)
                    .font_weight(FontWeight::NORMAL)
                    .child(base::input::Input::new(&self.editor))
                    .child(
                        canvas(
                            |_, _, _| (),
                            move |bounds, _, window, cx| {
                                // Base publishes range geometry during paint. Retain
                                // that complete snapshot and refresh the tree next frame.
                                if window.is_a11y_active()
                                    && let Some(editor) = text_editor.upgrade()
                                    && text_prepaint.capture(editor.read(cx))
                                {
                                    let _ = paint_state.update(cx, |_, cx| cx.notify());
                                }
                                window.paint_quad(quad(
                                    bounds.dilate(ring_width),
                                    radius + ring_width,
                                    Hsla::transparent_black(),
                                    ring_width,
                                    ring_color,
                                    Default::default(),
                                ));
                            },
                        )
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full(),
                    ),
            )
            .when_some(description, |this, description| {
                this.child(
                    div()
                        .text_size(theme.typography.field_description.size)
                        .line_height(theme.typography.field_description.line_height)
                        .text_color(if invalid {
                            theme.text.danger
                        } else {
                            theme.text.subtle
                        })
                        .child(gpui_kit::text!(description)),
                )
            })
    }
}

#[cfg(test)]
#[path = "input_tests.rs"]
mod tests;
