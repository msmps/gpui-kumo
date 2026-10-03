//! Kumo disclosure presentation over Base Collapsible and Button behavior.
use crate::{Button, theme};
use base::TestSupportExt;
use gpui_kit::{
    AnyElement, App, ClickEvent, Context, Div, ElementId, FocusHandle, FontWeight,
    InteractiveElement, IntoElement, ParentElement, RenderOnce, SharedString,
    StatefulInteractiveElement, StyleRefinement, Styled, Transformation, Window, base, canvas, div,
    prelude::FluentBuilder, px, quad, radians, svg,
};

struct State {
    local_open: bool,
    controlled: Option<bool>,
    disabled: bool,
    was_open: bool,
    own_trigger: FocusHandle,
    trigger: FocusHandle,
    panel: FocusHandle,
}
impl State {
    fn open(&self) -> bool {
        self.controlled.unwrap_or(self.local_open)
    }
}
type OpenHandler = Box<dyn Fn(bool, &mut Window, &mut App)>;

/// A consumed panel. Default styling matches Kumo's DefaultPanel; `unstyled`
/// corresponds to Panel. Keep-mounted children retain native/keyed state while
/// hidden without registering descendant focus, actions or accessibility nodes.
#[derive(IntoElement)]
#[must_use]
pub struct CollapsiblePanel {
    base: Div,
    default_style: bool,
    keep_mounted: bool,
}
impl CollapsiblePanel {
    pub fn new() -> Self {
        Self {
            base: div(),
            default_style: true,
            keep_mounted: false,
        }
    }
    pub fn unstyled() -> Self {
        Self {
            default_style: false,
            ..Self::new()
        }
    }
    pub fn keep_mounted(mut self, keep: bool) -> Self {
        self.keep_mounted = keep;
        self
    }
    fn surface(self, cx: &App) -> Div {
        let theme = theme(cx);
        if self.default_style {
            div().overflow_hidden().child(
                div()
                    .flex()
                    .flex_col()
                    .gap(theme.spacing.sixteen)
                    .my(theme.spacing.eight)
                    .border_l(px(2.))
                    .border_color(theme.colors.fill)
                    .py(theme.spacing.four)
                    .pr(theme.spacing.four)
                    .pl(theme.spacing.sixteen)
                    .child(self.base.flex().flex_col().gap(theme.spacing.sixteen)),
            )
        } else {
            self.base
        }
    }
}
impl Default for CollapsiblePanel {
    fn default() -> Self {
        Self::new()
    }
}
impl ParentElement for CollapsiblePanel {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.base.extend(elements);
    }
}
impl Styled for CollapsiblePanel {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}
impl RenderOnce for CollapsiblePanel {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        self.surface(cx)
    }
}

/// Stable unique IDs retain uncontrolled state while mounted. `open` enables
/// controlled mode: proposals never change the supplied value. Owner updates
/// do not invoke callbacks. Observe Theme in the owning view.
#[derive(IntoElement)]
#[must_use]
pub struct Collapsible {
    id: ElementId,
    label: SharedString,
    label_content: Option<AnyElement>,
    trigger: Option<Button>,
    panel: Option<CollapsiblePanel>,
    open: Option<bool>,
    default_open: bool,
    disabled: bool,
    on_open_change: Option<OpenHandler>,
}
impl Collapsible {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        let label = label.into();
        assert!(
            !label.trim().is_empty(),
            "Collapsible requires a nonempty trigger name"
        );
        Self {
            id: id.into(),
            label,
            label_content: None,
            trigger: None,
            panel: None,
            open: None,
            default_open: false,
            disabled: false,
            on_open_change: None,
        }
    }
    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }
    /// Initial uncontrolled value, read only when this stable ID first mounts.
    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    /// Decorative rich content; constructor label remains the complete accessible name.
    pub fn label_content(mut self, content: impl IntoElement) -> Self {
        self.label_content = Some(content.into_any_element());
        self
    }
    /// Compose an existing Kumo Button. Its consumer callback runs first and
    /// may cancel with window.prevent_default(); disabled/loading remain effective.
    pub fn trigger(mut self, trigger: Button) -> Self {
        self.trigger = Some(trigger);
        self
    }
    pub fn panel(mut self, panel: CollapsiblePanel) -> Self {
        self.panel = Some(panel);
        self
    }
    /// Once per accepted activation, before an uncontrolled update. A controlled
    /// owner may ignore the proposal; prevent_default cancels a local update.
    pub fn on_open_change(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(Box::new(handler));
        self
    }
}
impl RenderOnce for Collapsible {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = window.with_id(self.id.clone(), |window| {
            window.use_keyed_state("disclosure", cx, |_, cx: &mut Context<State>| {
                let trigger = cx.focus_handle();
                State {
                    local_open: self.default_open,
                    controlled: None,
                    disabled: false,
                    was_open: false,
                    own_trigger: trigger.clone(),
                    trigger,
                    panel: cx.focus_handle().tab_stop(false),
                }
            })
        });
        state.update(cx, |state, cx| {
            state.trigger = self
                .trigger
                .as_ref()
                .and_then(Button::provided_focus)
                .unwrap_or_else(|| state.own_trigger.clone());
            state.controlled = self.open;
            state.disabled = self.disabled;
            let open = state.open();
            if state.was_open && !open && state.panel.contains_focused(window, cx) {
                state.trigger.focus(window, cx);
            }
            state.was_open = open;
        });
        let current = state.read(cx);
        let open = current.open();
        let disabled = current.disabled;
        let focus = current.trigger.clone();
        let panel_focus = current.panel.clone();
        let weak = state.downgrade();
        let handler = self.on_open_change;
        let activate = move |_: &ClickEvent, window: &mut Window, cx: &mut App| {
            let Some(state) = weak.upgrade() else {
                return;
            };
            if state.read(cx).disabled {
                return;
            }
            let proposed = !state.read(cx).open();
            if let Some(handler) = &handler {
                handler(proposed, window, cx);
            }
            if window.default_prevented() {
                return;
            }
            state.update(cx, |state, cx| {
                if !state.disabled && state.controlled.is_none() {
                    state.local_open = proposed;
                    cx.notify();
                }
            });
        };
        let theme = theme(cx).clone();
        let trigger = if let Some(button) = self.trigger {
            button
                .disclosure_trigger(&focus, disabled, open, activate)
                .into_any_element()
        } else {
            let ring_focus = focus.clone();
            let button = base::Button::new("trigger")
                .accessibility_label(self.label.clone())
                .aria_expanded(open)
                .disabled(disabled)
                .track_focus(&focus)
                .a11y_synthetic_children(move |builder| {
                    if disabled {
                        builder.parent_node().set_disabled();
                    }
                })
                .min_w_0()
                .max_w_full()
                .justify_start()
                .gap(theme.spacing.four)
                .font_family(theme.typography.font_family.clone())
                .text_size(theme.typography.base.size)
                .line_height(theme.typography.base.line_height)
                .font_weight(FontWeight::MEDIUM)
                .text_color(theme.text.default)
                .relative()
                .when(!disabled, |this| this.cursor_pointer())
                .on_click(activate)
                .child(
                    div().min_w_0().child(
                        self.label_content
                            .unwrap_or_else(|| self.label.into_any_element()),
                    ),
                )
                .child(
                    div()
                        .id("caret")
                        .test_support()
                        .size(px(16.))
                        .flex_shrink_0()
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            svg()
                                .data(include_bytes!("../assets/caret-down-bold.svg").as_slice())
                                .size(px(12.))
                                .text_color(theme.text.default)
                                .with_transformation(Transformation::rotate(radians(if open {
                                    std::f32::consts::PI
                                } else {
                                    0.
                                }))),
                        ),
                )
                .child(
                    canvas(
                        |_, _, _| (),
                        move |bounds, _, window, _| {
                            if !disabled
                                && ring_focus.is_focused(window)
                                && window.last_input_was_keyboard()
                            {
                                let width = theme.effects.keyboard_focus_ring_width;
                                window.paint_quad(quad(
                                    bounds.dilate(width),
                                    width,
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
            div()
                .flex()
                .min_w_0()
                .max_w_full()
                .child(button)
                .into_any_element()
        };
        let keep = self.panel.as_ref().is_some_and(|p| p.keep_mounted);
        let mut root = base::Collapsible::new()
            .open(open || keep)
            .flex()
            .flex_col()
            .min_w_0()
            .w_full()
            .child(trigger);
        if let Some(panel) = self.panel {
            root = root.content(
                div()
                    .id("panel")
                    .track_focus(&panel_focus)
                    .when(!open, |this| this.hidden())
                    .child(panel.surface(cx)),
            );
        }
        div().id(self.id).min_w_0().w_full().child(root)
    }
}

#[cfg(test)]
#[path = "collapsible_tests.rs"]
mod tests;
