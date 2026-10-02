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
pub enum Mode {
    Masked,
    Revealed,
    Empty,
}

#[derive(Clone, Debug)]
pub enum SensitiveInputEvent {
    Change,
    Submit {
        secondary: bool,
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
    pub fn value(&self, cx: &App) -> SharedString {
        self.input.read(cx).value(cx)
    }
    pub fn selected_value(&self, cx: &App) -> SharedString {
        self.input.read(cx).selected_value(cx)
    }
    pub fn mode(&self) -> Mode {
        self.mode
    }
    pub fn is_disabled(&self, cx: &App) -> bool {
        self.input.read(cx).is_disabled()
    }
    pub fn is_read_only(&self, cx: &App) -> bool {
        self.input.read(cx).is_read_only()
    }
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
    pub fn set_placeholder(
        &mut self,
        value: impl Into<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.input
            .update(cx, |input, cx| input.set_placeholder(value, window, cx));
    }
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
    optional: bool,
    description: Option<SharedString>,
    error: Option<(SharedString, bool)>,
    tooltip: Option<(Entity<TooltipState>, SharedString)>,
}
#[derive(IntoElement)]
#[must_use]
pub struct SensitiveInput {
    id: ElementId,
    state: Entity<SensitiveInputState>,
    presentation: Presentation,
}
impl SensitiveInput {
    pub fn new(id: impl Into<ElementId>, state: &Entity<SensitiveInputState>) -> Self {
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
    pub fn label(mut self, show: bool) -> Self {
        self.presentation.label = show;
        self
    }
    pub fn required(mut self, required: bool) -> Self {
        self.presentation.optional = !required;
        self
    }
    pub fn description(mut self, text: impl Into<SharedString>) -> Self {
        self.presentation.description = Some(text.into());
        self
    }
    pub fn error(mut self, text: impl Into<SharedString>, show: bool) -> Self {
        self.presentation.error = Some((text.into(), show));
        self
    }
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
                .accessibility_label(format!("{}, masked.", self.name))
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
                                    include_bytes!("../assets/eye-slash.svg").as_slice()
                                } else {
                                    include_bytes!("../assets/eye.svg").as_slice()
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
        Field::new("sensitive-field", self.name.clone(), body)
            .focus_target(&self.focus_handle(cx))
            .disabled(disabled)
            .hide_label(!self.presentation.label)
            .required(!self.presentation.optional)
            .when_some(self.presentation.description.clone(), |field, text| {
                field.description(text)
            })
            .when_some(self.presentation.error.clone(), |field, (text, show)| {
                field.error(text, show)
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

#[cfg(test)]
#[path = "sensitive_input_tests.rs"]
mod tests;
