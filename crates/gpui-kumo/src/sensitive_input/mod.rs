//! Retained secret editing with Kumo's reveal, hide and copy presentation.
use std::time::Duration;

use crate::{Field, Input, InputEvent, InputState, Theme, TooltipState, input::Size, theme};
use gpui_kit::base::TestSupportExt;
use gpui_kit::{
    App, AppContext, ClipboardItem, Context, ElementId, Entity, EventEmitter, FocusHandle,
    Focusable, InteractiveElement, IntoElement, ParentElement, Render, RenderOnce, SharedString,
    StatefulInteractiveElement, Styled, Subscription, Task, Window, base, canvas, div,
    prelude::FluentBuilder, px, quad, svg,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Secret presentation mode; this does not expose the secret value.
pub enum Mode {
    /// Present the secret with redacted visible and accessible values.
    Masked,
    /// Present the retained editable value.
    Revealed,
    /// Present the empty editor.
    Empty,
}

#[derive(Clone, Debug)]
#[non_exhaustive]
/// Sensitive editor changes and clipboard requests; values remain in state.
pub enum SensitiveInputEvent {
    /// The user changed the value.
    Change,
    /// The user requested submission.
    Submit {
        /// Secondary.
        secondary: bool,
        /// Shift.
        shift: bool,
    },
    /// Submitted to GPUI's clipboard API, which offers no delivery result.
    Copy,
}

/// Create once, retain, and mount in one SensitiveInput. The editor is the sole
/// value owner; programmatic set_value does not emit Change.
pub struct SensitiveInputState {
    input: Entity<InputState>,
    name: SharedString,
    mode: Mode,
    scope: FocusHandle,
    masked_focus: FocusHandle,
    eye_focus: FocusHandle,
    copy_focus: FocusHandle,
    hovered: bool,
    copy_hovered: bool,
    eye_hovered: bool,
    copied: bool,
    reset: Option<Task<()>>,
    presentation: Presentation,
    _subscriptions: Vec<Subscription>,
}
impl EventEmitter<SensitiveInputEvent> for SensitiveInputState {}
impl SensitiveInputState {
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

    /// Create a retained named sensitive editor from an initial secret value. Retain with `cx.new`.
    pub fn new(
        name: impl Into<SharedString>,
        value: impl Into<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let name = name.into();
        let value = value.into();
        let mode = if value.is_empty() {
            Mode::Empty
        } else {
            Mode::Masked
        };
        let input = cx.new(|cx| {
            let mut state = InputState::new(name.clone(), window, cx);
            state.set_value(value, window, cx);
            state.set_masked(true, window, cx);
            state
        });
        let scope = cx.focus_handle().tab_stop(false);
        let subscriptions = vec![
            cx.subscribe(&input, |state: &mut Self, _, event, cx| {
                match event {
                    InputEvent::Change => {
                        if state.mode == Mode::Empty && !state.value(cx).is_empty() {
                            state.mode = Mode::Revealed;
                        }
                        cx.emit(SensitiveInputEvent::Change);
                    }
                    InputEvent::Submit { secondary, shift } => {
                        cx.emit(SensitiveInputEvent::Submit {
                            secondary: *secondary,
                            shift: *shift,
                        })
                    }
                    InputEvent::Focus | InputEvent::Blur => {}
                }
                cx.notify();
            }),
            cx.observe_global::<Theme>(|_, cx| cx.notify()),
            cx.on_focus_in(&scope, window, |_, _, cx| cx.notify()),
            cx.on_focus_out(&scope, window, |state: &mut Self, _, window, cx| {
                if !state.scope.contains_focused(window, cx) && !state.value(cx).is_empty() {
                    state.hide(false, window, cx);
                }
                cx.notify();
            }),
        ];
        Self {
            input,
            name,
            mode,
            scope,
            masked_focus: cx.focus_handle(),
            eye_focus: cx.focus_handle(),
            copy_focus: cx.focus_handle(),
            hovered: false,
            copy_hovered: false,
            eye_hovered: false,
            copied: false,
            reset: None,
            presentation: Presentation::default(),
            _subscriptions: subscriptions,
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
        self.input
            .update(cx, |input, cx| input.set_name(name.clone(), cx));
        self.name = name;
        cx.notify();
    }

    /// Read the current value from its owner; this does not request a change.
    pub fn value(&self, cx: &App) -> SharedString {
        self.input.read(cx).value(cx)
    }
    /// Read the currently selected text without changing selection.
    pub fn selected_value(&self, cx: &App) -> SharedString {
        self.input.read(cx).selected_value(cx)
    }
    /// Read the current masked or revealed mode.
    pub fn mode(&self) -> Mode {
        self.mode
    }
    /// Report whether the owner has disabled this control.
    pub fn is_disabled(&self, cx: &App) -> bool {
        self.input.read(cx).is_disabled()
    }
    /// Report whether user editing is blocked while selection and copying remain available.
    pub fn is_read_only(&self, cx: &App) -> bool {
        self.input.read(cx).is_read_only()
    }
    /// Replace the owner value programmatically; this is not a user activation proposal.
    pub fn set_value(
        &mut self,
        value: impl Into<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let focused = self.focus_handle(cx).is_focused(window);
        let copy_focused = self.copy_focus.is_focused(window);
        let eye_focused = self.eye_focus.is_focused(window);
        self.input
            .update(cx, |input, cx| input.set_value(value, window, cx));
        if self.mode == Mode::Masked && self.value(cx).is_empty() {
            self.mode = Mode::Empty;
        }
        self.copied = false;
        self.reset = None;
        let empty = self.value(cx).is_empty();
        let eye_removed = empty && self.mode != Mode::Revealed;
        if empty {
            self.copy_hovered = false;
        }
        if eye_removed {
            self.eye_hovered = false;
        }
        if !self.is_disabled(cx) && (focused || copy_focused && empty || eye_focused && eye_removed)
        {
            self.focus_handle(cx).focus(window, cx);
        }
        cx.notify();
    }
    /// Update empty-editor guidance without replacing the retained editor.
    pub fn set_placeholder(
        &mut self,
        value: impl Into<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.input
            .update(cx, |input, cx| input.set_placeholder(value, window, cx));
    }
    /// Update availability and notify presentation while retaining the value.
    pub fn set_disabled(&mut self, disabled: bool, cx: &mut Context<Self>) {
        if disabled {
            self.copy_hovered = false;
            self.eye_hovered = false;
        }
        self.input
            .update(cx, |input, cx| input.set_disabled(disabled, cx));
        self.masked_focus.clone().tab_stop(!disabled);
        self.eye_focus.clone().tab_stop(!disabled);
        cx.notify();
    }
    /// Update editing availability while retaining focus, selection and value.
    pub fn set_read_only(&mut self, read_only: bool, cx: &mut Context<Self>) {
        self.input
            .update(cx, |input, cx| input.set_read_only(read_only, cx));
        cx.notify();
    }
    fn reveal(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.is_disabled(cx) {
            return;
        }
        self.mode = Mode::Revealed;
        self.input
            .update(cx, |input, cx| input.set_masked(false, window, cx));
        if self.is_read_only(cx) {
            self.eye_focus.focus(window, cx);
        } else {
            self.input.read(cx).focus_handle(cx).focus(window, cx);
        }
        cx.notify();
    }
    fn hide(&mut self, restore: bool, window: &mut Window, cx: &mut Context<Self>) {
        self.eye_hovered = false;
        self.mode = if self.value(cx).is_empty() {
            Mode::Empty
        } else {
            Mode::Masked
        };
        self.input
            .update(cx, |input, cx| input.set_masked(true, window, cx));
        if restore {
            self.focus_handle(cx).focus(window, cx);
        }
        cx.notify();
    }
    fn copy(&mut self, cx: &mut Context<Self>) {
        if self.is_disabled(cx) || self.value(cx).is_empty() {
            return;
        }
        cx.write_to_clipboard(ClipboardItem::new_string(self.value(cx).to_string()));
        self.copied = true;
        self.reset = Some(cx.spawn(async move |state, cx| {
            cx.background_executor().timer(Duration::from_secs(2)).await;
            let _ = state.update(cx, |state, cx| {
                state.copied = false;
                state.reset = None;
                cx.notify();
            });
        }));
        cx.emit(SensitiveInputEvent::Copy);
        cx.notify();
    }
}
impl Focusable for SensitiveInputState {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        if self.mode == Mode::Masked && !self.value(cx).is_empty() {
            self.masked_focus.clone()
        } else {
            self.input.read(cx).focus_handle(cx)
        }
    }
}
#[derive(Default)]
struct Presentation {
    size: Size,
    label: bool,
    label_text: Option<SharedString>,
    accessible_name: Option<SharedString>,
    optional: bool,
    description: Option<SharedString>,
    error: Option<(SharedString, bool)>,
    tooltip: Option<(Entity<TooltipState>, SharedString)>,
}
#[derive(IntoElement)]
#[must_use]
/// Masked or revealed presentation of a retained sensitive editor.
pub struct SensitiveInput {
    id: ElementId,
    state: Entity<SensitiveInputState>,
    presentation: Presentation,
}
impl SensitiveInput {
    /// Present a retained sensitive editor; names and secret values stay separate.
    pub fn new(id: impl Into<ElementId>, state: &Entity<SensitiveInputState>) -> Self {
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
        state: &Entity<TooltipState>,
        text: impl Into<SharedString>,
    ) -> Self {
        self.presentation.tooltip = Some((state.clone(), text.into()));
        self
    }
}
impl RenderOnce for SensitiveInput {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        self.state.update(cx, |state, cx| {
            if let Some((old, _)) = &state.presentation.tooltip
                && self
                    .presentation
                    .tooltip
                    .as_ref()
                    .is_none_or(|(new, _)| old != new)
            {
                old.update(cx, |tooltip, cx| tooltip.set_open(false, cx));
            }
            state.presentation = self.presentation;
        });
        div().id(self.id).w_full().min_w_0().child(self.state)
    }
}
impl Render for SensitiveInputState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = theme(cx).clone();
        let disabled = self.is_disabled(cx);
        let has_value = !self.value(cx).is_empty();
        let masked = self.mode == Mode::Masked && has_value;
        // Native editor masking follows the source's type=text/password modes.
        self.input.update(cx, |input, cx| {
            input.set_masked(self.mode != Mode::Revealed, window, cx)
        });
        let size = self.presentation.size;
        let (height, padding, radius, text) = crate::input::metrics(size, &theme);
        let icon = match size {
            Size::Xs | Size::Sm => 12.,
            Size::Base | Size::Lg => 16.,
        };
        let focused = !disabled && self.scope.contains_focused(window, cx);
        let invalid = self.presentation.error.is_some();
        let ring = if invalid {
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
        let instruction = !disabled && (self.hovered || focused);
        let content = if masked {
            base::Button::new("masked")
                .accessibility_label(format!("{}, masked.", self.accessible_name()))
                .aria_description("Click or press Enter to reveal.")
                .track_focus(&self.masked_focus)
                .disabled(disabled)
                .on_click(cx.listener(|state, _, window, cx| state.reveal(window, cx)))
                .w_full()
                .min_w_0()
                .h(px(height))
                .justify_start()
                .px(padding)
                .rounded(radius)
                .relative()
                .flex()
                .items_center()
                .bg(theme.colors.control)
                .text_color(if disabled {
                    theme.native.disabled_input_foreground
                } else if instruction {
                    theme.text.subtle
                } else {
                    theme.text.default
                })
                .font_family(theme.typography.font_family.clone())
                .text_size(text.size)
                .line_height(text.line_height)
                .child(
                    div()
                        .id("mask-copy")
                        .test_support()
                        .min_w_0()
                        .overflow_hidden()
                        .whitespace_nowrap()
                        .child(if instruction {
                            "Click to reveal"
                        } else {
                            "••••••••"
                        }),
                )
                .child(
                    canvas(
                        |_, _, _| (),
                        move |bounds, _, window, _| {
                            window.paint_quad(quad(
                                bounds.dilate(ring_width),
                                radius + ring_width,
                                ring.alpha(0.),
                                ring_width,
                                ring,
                                Default::default(),
                            ));
                        },
                    )
                    .absolute()
                    .top_0()
                    .left_0()
                    .size_full(),
                )
                .into_any_element()
        } else {
            Input::new("editor", &self.input)
                .accessibility_label(self.accessible_name())
                .size(size)
                .sensitive(
                    px(match size {
                        Size::Xs => 14.,
                        Size::Sm => 16.,
                        Size::Base => 20.,
                        Size::Lg => 24.,
                    }),
                    &self.scope,
                )
                .when_some(self.presentation.error.clone(), |input, (message, _)| {
                    input.error_visible(message, false)
                })
                .into_any_element()
        };
        let show_eye =
            !disabled && (self.mode == Mode::Revealed || self.mode == Mode::Empty && has_value);
        let body = div()
            .id("sensitive-controls")
            .test_support()
            .when(!disabled, |body| body.track_focus(&self.scope))
            .w_full()
            .min_w_0()
            .relative()
            .on_hover(cx.listener(|state, hovered: &bool, _, cx| {
                state.hovered = *hovered;
                cx.notify();
            }))
            .on_key_down(
                cx.listener(|state, event: &gpui_kit::KeyDownEvent, window, cx| {
                    if event.keystroke.key == "escape"
                        && state.mode == Mode::Revealed
                        && !state.is_disabled(cx)
                    {
                        state.hide(true, window, cx);
                        cx.stop_propagation();
                    }
                }),
            )
            .child(content)
            .when(show_eye, |body| {
                body.child(
                    base::Button::new("visibility")
                        .occlude()
                        .accessibility_label(if self.mode == Mode::Revealed {
                            "Hide value"
                        } else {
                            "Reveal value"
                        })
                        .track_focus(&self.eye_focus)
                        .on_hover(cx.listener(|state, hovered: &bool, _, cx| {
                            state.eye_hovered = *hovered;
                            cx.notify();
                        }))
                        .on_click(cx.listener(|state, _, window, cx| {
                            cx.stop_propagation();
                            if state.is_disabled(cx) {
                                return;
                            }
                            if state.mode == Mode::Revealed {
                                state.hide(true, window, cx);
                            } else {
                                state.reveal(window, cx);
                            }
                        }))
                        .absolute()
                        .right(padding)
                        .top(px((height - icon) / 2.))
                        .size(px(icon))
                        .text_color(theme.text.subtle)
                        .hover(|s| s.text_color(theme.text.default))
                        .focus(|s| s.text_color(theme.text.default))
                        .child(
                            svg()
                                .data(if self.mode == Mode::Revealed {
                                    include_bytes!("../../assets/eye-slash.svg").as_slice()
                                } else {
                                    include_bytes!("../../assets/eye.svg").as_slice()
                                })
                                .size(px(icon))
                                .text_color(
                                    if self.eye_hovered || self.eye_focus.is_focused(window) {
                                        theme.text.default
                                    } else {
                                        theme.text.subtle
                                    },
                                ),
                        )
                        .child(focus_ring(
                            self.eye_focus.is_focused(window) && window.last_input_was_keyboard(),
                            theme.colors.brand,
                            theme.radii.sm,
                        )),
                )
            })
            .when(has_value && !disabled, |body| {
                body.child(
                    base::Button::new("copy")
                        .occlude()
                        .track_focus(&self.copy_focus)
                        .accessibility_label(if self.copied {
                            "Copied"
                        } else {
                            "Copy to clipboard"
                        })
                        .on_click(cx.listener(|state, _, _, cx| {
                            cx.stop_propagation();
                            state.copy(cx);
                        }))
                        .on_hover(cx.listener(|state, hovered: &bool, _, cx| {
                            state.copy_hovered = *hovered;
                            cx.notify();
                        }))
                        .absolute()
                        .right(theme.spacing.eight)
                        .top(px(-21.))
                        .h(px(20.))
                        .px(theme.spacing.eight)
                        .rounded_t(theme.radii.md)
                        .bg(theme.colors.brand)
                        .text_color(theme.text.on_brand)
                        .text_size(theme.typography.xs.size)
                        .line_height(theme.typography.xs.line_height)
                        .opacity(if self.hovered || self.copy_hovered || focused {
                            1.
                        } else {
                            0.
                        })
                        .child(focus_ring(
                            self.copy_focus.is_focused(window) && window.last_input_was_keyboard(),
                            theme.colors.brand,
                            theme.radii.md,
                        ))
                        .child(if self.copied { "Copied" } else { "Copy" }),
                )
            });
        Field::new("sensitive-field", self.label_text(), body)
            .focus_target(&self.focus_handle(cx))
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

fn focus_ring(focused: bool, color: gpui_kit::Hsla, radius: gpui_kit::Pixels) -> impl IntoElement {
    canvas(
        |_, _, _| (),
        move |bounds, _, window, _| {
            if focused {
                window.paint_quad(quad(
                    bounds.dilate(px(2.)),
                    radius + px(2.),
                    color.alpha(0.),
                    px(2.),
                    color,
                    Default::default(),
                ));
            }
        },
    )
    .absolute()
    .top_0()
    .left_0()
    .size_full()
}

impl crate::field::sealed::Sealed for SensitiveInput {}
impl crate::field::FieldControl for SensitiveInput {
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

impl std::fmt::Debug for SensitiveInputState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_struct("SensitiveInputState");
        debug.field("name", &self.name);
        debug.field("mode", &self.mode);
        debug.field("hovered", &self.hovered);
        debug.field("copy_hovered", &self.copy_hovered);
        debug.field("eye_hovered", &self.eye_hovered);
        debug.field("copied", &self.copied);
        debug.finish_non_exhaustive()
    }
}

impl std::fmt::Debug for SensitiveInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_struct("SensitiveInput");
        debug.field("id", &self.id);
        debug.finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests;
