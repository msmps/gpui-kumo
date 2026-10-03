//! Compact copy controls with Kumo Text recipes over Base Button activation.
use crate::{Text, text, theme};
use base::TestSupportExt;
use gpui_kit::{
    AnyElement, App, ClickEvent, ClipboardItem, Context, ElementId, FocusHandle,
    InteractiveElement, IntoElement, ParentElement, RenderOnce, SharedString,
    StatefulInteractiveElement, Styled, Subscription, Task, Window, base, canvas, div,
    prelude::FluentBuilder, px, quad, svg,
};
use std::time::Duration;

/// Supported non-heading Kumo Text recipes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Style {
    Copy {
        tone: text::Tone,
        size: text::Size,
        bold: bool,
    },
    Mono {
        tone: text::MonoTone,
        size: text::MonoSize,
    },
}
impl Default for Style {
    fn default() -> Self {
        Self::Mono {
            tone: text::MonoTone::Secondary,
            size: text::MonoSize::Standard,
        }
    }
}
impl From<Style> for text::Style {
    fn from(value: Style) -> Self {
        match value {
            Style::Copy { tone, size, bold } => Self::Copy { tone, size, bold },
            Style::Mono { tone, size } => Self::Mono { tone, size },
        }
    }
}
struct Feedback {
    value: SharedString,
    copied: bool,
    hovered: bool,
    disabled: bool,
    focus: FocusHandle,
    reset: Option<Task<()>>,
    _focus: Vec<Subscription>,
}
type Activation = Box<dyn Fn(&ClickEvent, &mut Window, &mut App)>;
type CopyHandler = Box<dyn Fn(&mut Window, &mut App)>;

/// Stable IDs retain local feedback while mounted. Observe Theme in the owning view.
/// Copy callbacks indicate submission to GPUI's native clipboard API, which has
/// no delivery/error result. Rich slots must be decorative, never interactive.
#[derive(IntoElement)]
#[must_use]
pub struct InlineCopyText {
    id: ElementId,
    text: SharedString,
    value: SharedString,
    content: Option<AnyElement>,
    style: Style,
    truncate: bool,
    disabled: bool,
    group_active: bool,
    copy_label: SharedString,
    copied_label: SharedString,
    on_click: Option<Activation>,
    on_copy: Option<CopyHandler>,
}
impl InlineCopyText {
    pub fn new(id: impl Into<ElementId>, text: impl Into<SharedString>) -> Self {
        let text = text.into();
        Self {
            id: id.into(),
            value: text.clone(),
            text,
            content: None,
            style: Style::default(),
            truncate: true,
            disabled: false,
            group_active: false,
            copy_label: "Copy to clipboard".into(),
            copied_label: "Copied".into(),
            on_click: None,
            on_copy: None,
        }
    }
    /// Display decorative rich content. Value is explicitly required; visible_text
    /// remains complete accessible content, independent from the copied payload.
    pub fn rich(
        id: impl Into<ElementId>,
        visible_text: impl Into<SharedString>,
        value: impl Into<SharedString>,
        content: impl IntoElement,
    ) -> Self {
        Self {
            value: value.into(),
            content: Some(content.into_any_element()),
            ..Self::new(id, visible_text)
        }
    }
    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = value.into();
        self
    }
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
    pub fn truncate(mut self, truncate: bool) -> Self {
        self.truncate = truncate;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    /// Native equivalent of enclosing row hover/focus-within. Does not change text tone.
    pub fn group_active(mut self, active: bool) -> Self {
        self.group_active = active;
        self
    }
    pub fn labels(
        mut self,
        copy: impl Into<SharedString>,
        copied: impl Into<SharedString>,
    ) -> Self {
        self.copy_label = copy.into();
        self.copied_label = copied.into();
        assert!(
            !self.copy_label.trim().is_empty() && !self.copied_label.trim().is_empty(),
            "InlineCopyText labels must be nonempty"
        );
        self
    }
    /// Runs before copy. window.prevent_default() cancels the clipboard/copy callback.
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
    pub fn on_copy(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_copy = Some(Box::new(handler));
        self
    }
}
impl RenderOnce for InlineCopyText {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let feedback = window.with_id(self.id.clone(), |window| {
            window.use_keyed_state("copy-feedback", cx, |window, cx: &mut Context<Feedback>| {
                let focus = cx.focus_handle();
                let subscriptions = vec![
                    cx.on_focus_in(&focus, window, |_, _, cx| cx.notify()),
                    cx.on_focus_out(&focus, window, |_, _, _, cx| cx.notify()),
                ];
                Feedback {
                    value: SharedString::default(),
                    copied: false,
                    hovered: false,
                    disabled: false,
                    focus,
                    reset: None,
                    _focus: subscriptions,
                }
            })
        });
        feedback.update(cx, |state, _| {
            if state.value != self.value {
                state.value = self.value;
                state.copied = false;
                state.reset = None;
            }
            state.disabled = self.disabled;
            state.focus.clone().tab_stop(!self.disabled);
        });
        let state = feedback.read(cx);
        let copied = state.copied;
        let disabled = state.disabled;
        let focus = state.focus.clone();
        let active = state.hovered || focus.is_focused(window) && window.last_input_was_keyboard();
        let show_icon = copied || active || self.group_active;
        let icon_opacity = if copied {
            1.
        } else {
            window.with_id(self.id.clone(), |window| {
                // The source mounts Check immediately; only CopySimple transitions.
                base::transition(
                    "copy-opacity",
                    if show_icon { 1.0_f32 } else { 0. },
                    base::Transition::new(Duration::from_millis(100)).easing(
                        base::Easing::CubicBezier {
                            x1: 0.4,
                            y1: 0.,
                            x2: 0.2,
                            y2: 1.,
                        },
                    ),
                    window,
                    cx,
                )
            })
        };
        let announcement = if copied {
            self.copied_label.clone()
        } else {
            SharedString::default()
        };
        let mut style = self.style;
        if active
            && let Style::Mono {
                tone: text::MonoTone::Secondary,
                size,
            } = style
        {
            style = Style::Mono {
                tone: text::MonoTone::Default,
                size,
            };
        }
        let theme = theme(cx).clone();
        // Icons are siblings of Text upstream: inherit enclosing foreground,
        // independently from the Text recipe's secondary/error/success tone.
        let foreground = window.text_style().color;
        let line_height = window.line_height();
        let hovered = feedback.downgrade();
        let activate = hovered.clone();
        let handler = self.on_click;
        let on_copy = self.on_copy;
        let control = base::Button::new(self.id)
            .accessibility_label(if copied {
                self.copied_label
            } else {
                self.copy_label
            })
            .disabled(disabled)
            .track_focus(&focus)
            .a11y_synthetic_children(move |builder| {
                if disabled {
                    builder.parent_node().set_disabled();
                }
            })
            .min_w_0()
            .max_w_full()
            .when(!disabled, |this| this.cursor_pointer())
            .justify_start()
            .gap(theme.spacing.four)
            .rounded(theme.radii.xs)
            .relative()
            .bg(theme.colors.control.alpha(0.))
            .text_color(foreground)
            .line_height(line_height)
            .on_hover(move |value, _, cx| {
                let _ = hovered.update(cx, |state, cx| {
                    if state.hovered != *value {
                        state.hovered = *value;
                        cx.notify();
                    }
                });
            })
            .on_click(move |event, window, cx| {
                if activate
                    .upgrade()
                    .is_none_or(|state| state.read(cx).disabled)
                {
                    return;
                }
                if let Some(handler) = &handler {
                    handler(event, window, cx);
                }
                if window.default_prevented() {
                    return;
                }
                let accepted = activate.update(cx, |state, cx| {
                    if state.disabled {
                        return false;
                    }
                    cx.write_to_clipboard(ClipboardItem::new_string(state.value.to_string()));
                    state.copied = true;
                    state.reset = Some(cx.spawn(async move |state, cx| {
                        cx.background_executor()
                            .timer(Duration::from_millis(1500))
                            .await;
                        let _ = state.update(cx, |state, cx| {
                            state.copied = false;
                            state.reset = None;
                            cx.notify();
                        });
                    }));
                    cx.notify();
                    true
                });
                if matches!(accepted, Ok(true))
                    && let Some(on_copy) = &on_copy
                {
                    on_copy(window, cx);
                }
            })
            .child(
                Text::new("text", self.text)
                    .style(style.into())
                    .truncate(self.truncate)
                    .when_some(self.content, |text, content| text.rich_content(content)),
            )
            .child(
                div()
                    .id("copy-icon")
                    .test_support()
                    .size(px(14.))
                    .flex_shrink_0()
                    .opacity(icon_opacity)
                    .child(
                        svg()
                            .data(if copied {
                                include_bytes!("../../assets/empty-check.svg").as_slice()
                            } else {
                                include_bytes!("../../assets/copy-simple.svg").as_slice()
                            })
                            .size(px(14.))
                            .text_color(foreground),
                    ),
            )
            .child(
                div()
                    .id("copied-announcement")
                    .test_support()
                    .role(gpui_kit::Role::Label)
                    .aria_value(announcement)
                    .a11y_synthetic_children(|builder| {
                        builder
                            .parent_node()
                            .set_live(gpui_kit::accesskit::Live::Polite);
                    })
                    .absolute()
                    .size(px(0.)),
            )
            .child(
                canvas(
                    |_, _, _| (),
                    move |bounds, _, window, _| {
                        if !disabled && focus.is_focused(window) && window.last_input_was_keyboard()
                        {
                            let width = theme.effects.keyboard_focus_ring_width;
                            window.paint_quad(quad(
                                bounds.dilate(width),
                                theme.radii.xs + width,
                                theme.colors.brand.alpha(0.),
                                width,
                                theme.colors.brand,
                                Default::default(),
                            ));
                        }
                    },
                )
                .absolute()
                .top_0()
                .left_0()
                .size_full(),
            );
        // As with Button, a neutral row wrapper keeps the interactive surface
        // intrinsic in columns while honoring the owner's row alignment.
        div().flex().min_w_0().max_w_full().child(control)
    }
}
#[cfg(test)]
mod tests;
