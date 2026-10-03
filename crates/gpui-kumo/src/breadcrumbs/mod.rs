//! Kumo responsive navigation trails over Base-backed Link and Button.
#![deny(missing_docs)]
use crate::{Button, Link, SkeletonLine, button, theme};
use gpui_kit::{
    AnyElement, App, ClipboardItem, Context, ElementId, FocusHandle, FontWeight,
    InteractiveElement, IntoElement, ParentElement, Refineable, RenderOnce, Role, SharedString,
    StatefulInteractiveElement, StyleRefinement, Styled, Subscription, Task, Window, base,
    base::TestSupportExt, div, prelude::FluentBuilder, px, svg,
};
use std::time::Duration;

struct HoverState(bool);
/// Source breadcrumb density.
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub enum Size {
    ///13px text,40px row and2px part gaps.
    Sm,
    ///14px text,48px row and4px part gaps.
    #[default]
    Base,
}
/// Noninteractive current page, with optional source loading presentation.
#[derive(IntoElement)]
pub struct BreadcrumbCurrent {
    id: ElementId,
    label: SharedString,
    icon: Option<AnyElement>,
    loading: bool,
}
impl BreadcrumbCurrent {
    /// Create a named current page; the full name survives visual truncation.
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            loading: false,
        }
    }
    /// Add a decorative icon; independent controls belong in root extras.
    pub fn icon(mut self, icon: impl IntoElement) -> Self {
        self.icon = Some(icon.into_any_element());
        self
    }
    /// Display a125px SkeletonLine instead of the current label.
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }
}
impl RenderOnce for BreadcrumbCurrent {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = theme(cx);
        div()
            .id(self.id)
            .test_support()
            .flex()
            .min_w_0()
            .max_w_full()
            .items_center()
            .gap(theme.spacing.four)
            .when(!self.loading, |this| {
                this.role(Role::Label)
                    .aria_label(self.label.clone())
                    .aria_value(self.label.clone())
                    .font_weight(FontWeight::MEDIUM)
                    .a11y_synthetic_children(|builder| {
                        builder
                            .parent_node()
                            .set_aria_current(gpui_kit::accesskit::AriaCurrent::Page);
                    })
            })
            .when_some(self.icon, |this, icon| {
                this.child(div().flex_none().flex().items_center().child(icon))
            })
            .when(self.loading, |this| this.w(px(125.)))
            .child(if self.loading {
                SkeletonLine::new("loading").into_any_element()
            } else {
                div()
                    .min_w_0()
                    .truncate()
                    .child(self.label)
                    .into_any_element()
            })
    }
}
struct Feedback {
    value: SharedString,
    copied: bool,
    focus: FocusHandle,
    reset: Option<Task<()>>,
    _focus: Vec<Subscription>,
}
/// Ghost deeplink copy action. Stable IDs retain feedback while mounted.
#[derive(IntoElement)]
pub struct BreadcrumbClipboard {
    id: ElementId,
    text: SharedString,
    disabled: bool,
    group_hovered: bool,
}
impl BreadcrumbClipboard {
    /// Create a native clipboard action; empty payloads never copy.
    pub fn new(id: impl Into<ElementId>, text: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            disabled: false,
            group_hovered: false,
        }
    }
    /// Gate pointer/keyboard activation and remove traversal.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}
impl RenderOnce for BreadcrumbClipboard {
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
                    focus,
                    reset: None,
                    _focus: subscriptions,
                }
            })
        });
        feedback.update(cx, |state, _| {
            if state.value != self.text {
                state.value = self.text;
                state.copied = false;
                state.reset = None;
            }
        });
        let state = feedback.read(cx);
        let copied = state.copied;
        let focus = state.focus.clone();
        let visible =
            self.group_hovered || focus.is_focused(window) && window.last_input_was_keyboard();
        let opacity = window.with_id(self.id.clone(), |window| {
            base::transition(
                "copy-opacity",
                if visible { 1.0_f32 } else { 0. },
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
        });
        let theme = theme(cx);
        let activate = feedback.downgrade();
        div().flex_none().opacity(opacity).child(
            Button::icon(
                self.id,
                if copied { "Copied" } else { "Copy" },
                svg()
                    .data(if copied {
                        include_bytes!("../../assets/checkbox-check.svg").as_slice()
                    } else {
                        include_bytes!("../../assets/empty-copy.svg").as_slice()
                    })
                    .size(px(16.))
                    .text_color(if copied {
                        theme.colors.success
                    } else {
                        theme.text.default
                    }),
            )
            .variant(button::Variant::Ghost)
            .size(button::Size::Sm)
            .track_focus(&focus)
            .disabled(self.disabled)
            .on_click(move |_, _, cx| {
                let _ = activate.update(cx, |state, cx| {
                    if state.value.is_empty() {
                        return;
                    }
                    cx.write_to_clipboard(ClipboardItem::new_string(state.value.to_string()));
                    state.copied = true;
                    state.reset = Some(cx.spawn(async move |state, cx| {
                        cx.background_executor()
                            .timer(Duration::from_millis(2000))
                            .await;
                        let _ = state.update(cx, |state, cx| {
                            state.copied = false;
                            state.reset = None;
                            cx.notify();
                        });
                    }));
                    cx.notify();
                });
            }),
        )
    }
}
enum Part {
    Link(Box<Link>),
    Current(BreadcrumbCurrent),
    Separator,
    Clipboard(BreadcrumbClipboard),
    Extra(AnyElement),
}
impl Part {
    fn is_crumb(&self) -> bool {
        matches!(self, Self::Link(_) | Self::Current(_))
    }
}
/// Responsive breadcrumb trail. IDs of supplied parts must be unique within root.
#[derive(IntoElement)]
pub struct Breadcrumbs {
    id: ElementId,
    size: Size,
    parts: Vec<Part>,
    style: StyleRefinement,
}
impl Breadcrumbs {
    /// Create an empty navigation trail with accessible name "breadcrumb".
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            size: Size::Base,
            parts: Vec::new(),
            style: StyleRefinement::default(),
        }
    }
    /// Set the source size.
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
    /// Add an ancestor link. Keep its complete accessible label and injected route.
    pub fn link(mut self, link: Link) -> Self {
        self.parts.push(Part::Link(Box::new(link)));
        self
    }
    /// Add an explicit decorative separator.
    pub fn separator(mut self) -> Self {
        self.parts.push(Part::Separator);
        self
    }
    /// Add a current/loading part.
    pub fn current(mut self, current: BreadcrumbCurrent) -> Self {
        self.parts.push(Part::Current(current));
        self
    }
    /// Add a copy action, preserved by responsive collapse.
    pub fn clipboard(mut self, copy: BreadcrumbClipboard) -> Self {
        self.parts.push(Part::Clipboard(copy));
        self
    }
    /// Add arbitrary extra content, preserved after the mobile trail.
    pub fn extra(mut self, content: impl IntoElement) -> Self {
        self.parts.push(Part::Extra(content.into_any_element()));
        self
    }
}
impl Styled for Breadcrumbs {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
fn separator(cx: &App) -> impl IntoElement {
    svg().data(br#"<svg width="24" height="24" fill="none" viewBox="0 0 24 24"><path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M10.75 8.75L14.25 12L10.75 15.25"/></svg>"#.as_slice())
        .size(px(24.)).flex_none().text_color(theme(cx).text.inactive)
}
impl RenderOnce for Breadcrumbs {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let hover = window.with_id(self.id.clone(), |window| {
            window.use_keyed_state("hover", cx, |_, _| HoverState(false))
        });
        let hovered = hover.read(cx).0;
        let hover_handle = hover.downgrade();
        let theme = theme(cx);
        let count = self.parts.iter().filter(|part| part.is_crumb()).count();
        let compact = window.viewport_size().width < px(640.) && count > 2;
        let mut root = div()
            .id(self.id)
            .test_support()
            .role(Role::Navigation)
            .aria_label("breadcrumb")
            .hover_listener_mode(gpui_kit::HoverListenerMode::InputModalityIndependent)
            .on_hover(move |hovered, _, cx| {
                let _ = hover_handle.update(cx, |state, cx| {
                    state.0 = *hovered;
                    cx.notify();
                });
            })
            .flex()
            .flex_grow(1.)
            .min_w_0()
            .items_center()
            .overflow_hidden()
            .whitespace_nowrap()
            .mr(theme.spacing.sixteen)
            .text_color(theme.text.default)
            .text_size(if self.size == Size::Sm {
                theme.typography.sm.size
            } else {
                theme.typography.base.size
            })
            .line_height(if self.size == Size::Sm {
                theme.typography.sm.line_height
            } else {
                theme.typography.base.line_height
            })
            .h(px(if self.size == Size::Sm { 40. } else { 48. }))
            .gap(if self.size == Size::Sm {
                theme.spacing.four * 0.5
            } else {
                theme.spacing.four
            });
        root.style().refine(&self.style);
        if compact {
            root = root
                .child(div().flex_none().text_color(theme.text.subtle).child("..."))
                .child(separator(cx));
        }
        let mut crumb = 0;
        let mut retained = 0;
        let mut extras = Vec::new();
        for part in self.parts {
            if part.is_crumb() {
                crumb += 1;
                if compact && crumb <= count - 2 {
                    continue;
                }
                if compact && retained > 0 {
                    root = root.child(separator(cx));
                }
                retained += 1;
            }
            let element = match part {
                Part::Link(link) => (*link)
                    .breadcrumb()
                    .flex_none()
                    .self_center()
                    .gap(theme.spacing.four)
                    .text_color(theme.text.subtle)
                    .whitespace_nowrap()
                    .into_any_element(),
                Part::Current(current) => current.into_any_element(),
                Part::Separator if compact => continue,
                Part::Separator => separator(cx).into_any_element(),
                Part::Clipboard(mut copy) => {
                    copy.group_hovered = hovered;
                    let element = copy.into_any_element();
                    if compact {
                        extras.push(element);
                    } else {
                        root = root.child(element);
                    }
                    continue;
                }
                Part::Extra(extra) => {
                    if compact {
                        extras.push(extra);
                    } else {
                        root = root.child(extra);
                    }
                    continue;
                }
            };
            root = root.child(element);
        }
        // Source grow fills a row; contain it in a row so native column
        // consumers retain the authored40/48px height rather than growing vertically.
        div().flex().min_w_0().child(root.children(extras))
    }
}
#[cfg(test)]
mod tests;
