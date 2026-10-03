//! Empty-state presentation with retained, native command-copy feedback.

use crate::{Button, Text, button, text, theme};
use gpui_kit::base::TestSupportExt;
use gpui_kit::{
    AnyElement, App, ClipboardItem, ElementId, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, Styled, Task, Window, canvas, div,
    prelude::FluentBuilder, px, quad, relative, svg,
};
use std::time::Duration;

/// Kumo's empty-state container sizes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Size {
    /// 24px horizontal / 32px vertical padding and 16px gap.
    Sm,
    /// 40px horizontal / 64px vertical padding and 24px gap.
    #[default]
    Base,
    /// 48px horizontal / 80px vertical padding and 32px gap.
    Lg,
}

#[derive(Default)]
struct CopyFeedback {
    command: SharedString,
    copied: bool,
    reset: Option<Task<()>>,
}

/// A content-empty surface. The stable ID retains command feedback while mounted.
/// Icon content is decorative; put named interactive controls in `contents`.
#[derive(IntoElement)]
#[must_use]
pub struct Empty {
    id: ElementId,
    title: SharedString,
    size: Size,
    description: Option<SharedString>,
    command: Option<SharedString>,
    icon: Option<AnyElement>,
    contents: Option<AnyElement>,
}

impl Empty {
    /// Create the minimal, supporting-tone level-two heading.
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            size: Size::default(),
            description: None,
            command: None,
            icon: None,
            contents: None,
        }
    }
    /// Select the authored container geometry.
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
    /// Supporting text; nonempty text also selects the large title treatment.
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        let description = description.into();
        self.description = (!description.is_empty()).then_some(description);
        self
    }
    /// A command copied verbatim, without the decorative `$` prompt.
    pub fn command_line(mut self, command: impl Into<SharedString>) -> Self {
        let command = command.into();
        self.command = (!command.is_empty()).then_some(command);
        self
    }
    /// Caller-sized decorative icon. No implicit icon dimensions are imposed.
    pub fn icon(mut self, icon: impl IntoElement) -> Self {
        self.icon = Some(icon.into_any_element());
        self
    }
    /// Caller-owned actions, links or supplementary content.
    pub fn contents(mut self, contents: impl IntoElement) -> Self {
        self.contents = Some(contents.into_any_element());
        self
    }
}

impl RenderOnce for Empty {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = theme(cx).clone();
        // Geometry follows the pinned Empty recipe; this does not create a density.
        let (padding_x, padding_y, gap) = match self.size {
            Size::Sm => (24., 32., 16.),
            Size::Base => (40., 64., 24.),
            Size::Lg => (48., 80., 32.),
        };
        let feedback = window.with_id(self.id.clone(), |window| {
            window.use_keyed_state("command-feedback", cx, |_, _| CopyFeedback::default())
        });
        let command = self.command.clone().unwrap_or_default();
        feedback.update(cx, |state, _| {
            if state.command != command {
                state.command = command;
                state.copied = false;
                state.reset = None;
            }
        });
        let copied = feedback.read(cx).copied;
        let title_style = if self.description.is_some() {
            text::Style::Heading(text::HeadingSize::Large)
        } else {
            text::Style::Copy {
                tone: text::Tone::Secondary,
                size: text::Size::Base,
                bold: false,
            }
        };
        div()
            .id(self.id)
            .test_support()
            .w_full()
            .min_w_0()
            .flex()
            .flex_col()
            .items_center()
            .px(px(padding_x))
            .py(px(padding_y))
            .gap(px(gap))
            .rounded(px(12.))
            .border_1()
            .border_color(theme.colors.fill)
            .bg(theme.colors.control)
            .text_color(theme.text.default)
            .font_family(theme.typography.font_family.clone())
            .text_size(theme.typography.base.size)
            .line_height(theme.typography.base.line_height)
            .children(self.icon)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .text_center()
                    .gap(px(10.))
                    .max_w_full()
                    .child(
                        Text::new("title", self.title)
                            .style(title_style)
                            .heading_level(text::HeadingLevel::Two),
                    )
                    .when_some(self.description, |this, description| {
                        this.child(
                            div()
                                .w_full()
                                .max_w(px(560.))
                                .text_center()
                                .line_height(px(21.))
                                .child(Text::new("description", description).style(
                                    text::Style::Copy {
                                        tone: text::Tone::Secondary,
                                        size: text::Size::Base,
                                        bold: false,
                                    },
                                )),
                        )
                    }),
            )
            .when_some(self.command, |this, command| {
                let copy = command.clone();
                this.child(
                    div()
                        .id("command")
                        .test_support()
                        .flex()
                        .items_center()
                        .min_w_0()
                        .max_w(relative(0.8))
                        .relative()
                        .h(px(40.))
                        .pl(theme.spacing.twelve)
                        .pr(theme.spacing.eight)
                        .gap(theme.spacing.eight)
                        .rounded(theme.radii.lg)
                        .border_1()
                        .border_color(gpui_kit::white())
                        .bg(theme.colors.overlay)
                        .shadow(theme.effects.shadow_xs.clone())
                        .font_family(theme.typography.mono_font_family.clone())
                        .child(
                            canvas(
                                |_, _, _| (),
                                move |bounds, _, window, _| {
                                    window.paint_quad(quad(
                                        bounds.dilate(px(1.)),
                                        theme.radii.lg + px(1.),
                                        theme.colors.line.alpha(0.),
                                        px(1.),
                                        theme.colors.line,
                                        Default::default(),
                                    ));
                                },
                            )
                            .absolute()
                            .inset_0()
                            .size_full(),
                        )
                        .child(div().text_color(theme.text.subtle).child("$"))
                        .child(
                            div()
                                .id("command-scroll")
                                .min_w_0()
                                .overflow_x_scroll()
                                .whitespace_nowrap()
                                .child(Text::new("command-text", command)),
                        )
                        .child(
                            Button::icon(
                                "copy-command",
                                if copied {
                                    "Command copied"
                                } else {
                                    "Copy command"
                                },
                                svg()
                                    .data(if copied {
                                        include_bytes!("../assets/empty-check.svg")
                                    } else {
                                        include_bytes!("../assets/empty-copy.svg")
                                    })
                                    .size(px(16.))
                                    .text_color(if copied {
                                        theme.text.success
                                    } else {
                                        theme.text.subtle
                                    }),
                            )
                            .size(button::Size::Sm)
                            .variant(button::Variant::Ghost)
                            .on_click(move |_, _, cx| {
                                cx.write_to_clipboard(ClipboardItem::new_string(copy.to_string()));
                                feedback.update(cx, |state, cx| {
                                    state.copied = true;
                                    state.reset = Some(cx.spawn(async move |state, cx| {
                                        cx.background_executor()
                                            .timer(Duration::from_secs(1))
                                            .await;
                                        let _ = state.update(cx, |state, cx| {
                                            state.copied = false;
                                            cx.notify();
                                        });
                                    }));
                                    cx.notify();
                                });
                            }),
                        ),
                )
            })
            .when_some(self.contents, |this, contents| {
                this.child(
                    div()
                        .w_full()
                        .min_w_0()
                        .flex()
                        .justify_center()
                        .child(contents),
                )
            })
    }
}

#[cfg(test)]
mod tests;
