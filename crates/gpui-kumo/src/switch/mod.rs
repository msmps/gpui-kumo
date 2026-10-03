//! Kumo Switch presentation and grouping over Base's controlled binary behavior.
use crate::{Label, Theme, theme};
use gpui_kit::{
    Animation, AnimationExt, AnyElement, App, BoxShadow, ClickEvent, ElementId, FocusHandle,
    FontWeight, Hsla, InteractiveElement, IntoElement, ParentElement, RenderOnce, Role,
    SharedString, StatefulInteractiveElement, Styled, TestSupportExt, Window, base, canvas, div,
    prelude::FluentBuilder, px, quad,
};
use std::{cell::Cell, rc::Rc, time::Duration};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Size {
    Sm,
    #[default]
    Base,
    Lg,
}
impl Size {
    fn side(self) -> f32 {
        match self {
            Self::Sm => 16.,
            Self::Base => 18.,
            Self::Lg => 20.,
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Variant {
    #[default]
    Default,
    Neutral,
}
type ChangeHandler = Box<dyn Fn(bool, &ClickEvent, &mut Window, &mut App)>;
/// Controlled value with Base pointer/keyboard activation. Decorative rich labels
/// require a complete readable name; interactive content belongs outside the control.
#[derive(IntoElement)]
#[must_use]
pub struct Switch {
    id: ElementId,
    name: SharedString,
    checked: bool,
    disabled: bool,
    size: Size,
    variant: Variant,
    control_first: bool,
    optional: bool,
    show_label: bool,
    content: Option<AnyElement>,
    focus: Option<FocusHandle>,
    transitioning: bool,
    group_item: bool,
    on_change: Option<ChangeHandler>,
}
impl Switch {
    pub fn new(id: impl Into<ElementId>, name: impl Into<SharedString>) -> Self {
        let name = name.into();
        assert!(!name.trim().is_empty(), "Switch requires a readable name");
        Self {
            id: id.into(),
            name,
            checked: false,
            disabled: false,
            size: Size::Base,
            variant: Variant::Default,
            control_first: true,
            optional: false,
            show_label: true,
            content: None,
            focus: None,
            transitioning: false,
            group_item: false,
            on_change: None,
        }
    }
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }
    pub fn control_first(mut self, first: bool) -> Self {
        self.control_first = first;
        self
    }
    pub fn required(mut self, required: bool) -> Self {
        self.optional = !required;
        self
    }
    pub fn bare(mut self) -> Self {
        self.show_label = false;
        self
    }
    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }
    /// Exposes busy state; this does not disable activation or own asynchronous work.
    pub fn transitioning(mut self, transitioning: bool) -> Self {
        self.transitioning = transitioning;
        self
    }
    pub fn track_focus(mut self, focus: &FocusHandle) -> Self {
        self.focus = Some(focus.clone());
        self
    }
    pub fn on_change(
        mut self,
        handler: impl Fn(bool, &ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}
impl RenderOnce for Switch {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = theme(cx).clone();
        let focus = self.focus.unwrap_or_else(|| {
            window
                .use_keyed_state(self.id.clone(), cx, |_, cx| cx.focus_handle())
                .read(cx)
                .clone()
        });
        let target = if self.checked { 1. } else { 0. };
        let (from, current, generation) = window.with_id(self.id.clone(), |window| {
            let motion = window.use_keyed_state("switch-motion", cx, |_, _| Motion {
                target,
                from: target,
                current: Rc::new(Cell::new(target)),
                generation: 0,
            });
            let reduced = cx.reduce_motion();
            motion.update(cx, |motion, _| {
                if target != motion.target {
                    motion.from = motion.current.get();
                    motion.target = target;
                    motion.generation += 1;
                }
                if reduced {
                    motion.from = target;
                    motion.current.set(target);
                }
                (motion.from, motion.current.clone(), motion.generation)
            })
        });
        let disabled = self.disabled;
        let busy = self.transitioning;
        let group_item = self.group_item;
        let side = self.size.side();
        let graphic = Graphic {
            theme: theme.clone(),
            focus: focus.clone(),
            variant: self.variant,
            checked: self.checked,
            disabled,
            group_item,
            side,
            position: target,
        };
        let graphic = if from == target {
            graphic.into_any_element()
        } else {
            graphic
                .with_animation(
                    ("switch-motion", generation),
                    Animation::new(Duration::from_millis(150)),
                    move |mut graphic, progress| {
                        graphic.position = from + (target - from) * ease_out(progress);
                        current.set(graphic.position);
                        graphic
                    },
                )
                .into_any_element()
        };
        let visual = div()
            .id("visual")
            .test_support()
            .w(px(side * 2.))
            .h(px(side))
            .flex_shrink_0()
            .child(graphic);
        let label = Label::new("label", self.name.clone())
            .as_content()
            .optional(self.optional)
            .when_some(self.content, |label, content| label.content(content));
        let mut root = base::Switch::new(self.id)
            .checked(self.checked)
            .disabled(disabled)
            .track_focus(&focus)
            .accessibility_label(if self.optional && self.show_label {
                SharedString::from(format!("{} (optional)", self.name))
            } else {
                self.name
            })
            .a11y_synthetic_children(move |builder| {
                if disabled {
                    builder.parent_node().set_disabled();
                }
                if busy {
                    builder.parent_node().set_busy();
                }
            })
            .flex()
            .items_center()
            .gap(theme.spacing.eight)
            .min_w_0()
            .max_w_full()
            .text_size(theme.typography.base.size)
            .line_height(theme.typography.base.line_height)
            .font_weight(FontWeight::MEDIUM)
            .text_color(theme.text.default)
            .opacity(if disabled && group_item { 0.5 } else { 1. })
            .when(disabled, |this| this.cursor_not_allowed())
            .when(!disabled, |this| this.cursor_pointer())
            .when(!self.control_first, |this| this.flex_row_reverse())
            .child(visual)
            .when(self.show_label, |this| this.child(label));
        if let Some(handler) = self.on_change {
            root = root.on_change(handler);
        }
        div().flex().min_w_0().max_w_full().child(root)
    }
}

// Presentation progress alone is retained: semantic checked state remains caller-owned.
struct Motion {
    target: f32,
    from: f32,
    current: Rc<Cell<f32>>,
    generation: usize,
}
#[derive(IntoElement)]
struct Graphic {
    theme: Theme,
    focus: FocusHandle,
    variant: Variant,
    checked: bool,
    disabled: bool,
    group_item: bool,
    side: f32,
    position: f32,
}
impl RenderOnce for Graphic {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let theme = self.theme;
        let neutral = self.variant == Variant::Neutral;
        let off = if neutral {
            (
                theme.switch.neutral_off_track,
                theme.colors.hairline,
                theme.switch.off_thumb,
            )
        } else {
            (
                theme.switch.off_track,
                theme.switch.off_ring,
                theme.switch.off_thumb,
            )
        };
        let on = if neutral {
            (
                theme.switch.neutral_on_track,
                theme.switch.neutral_on_ring,
                theme.switch.neutral_on_thumb,
            )
        } else {
            (
                theme.switch.on_track,
                theme.switch.on_ring,
                theme.switch.on_thumb,
            )
        };
        let mix = |from: Hsla, to: Hsla| {
            if self.position <= 0. {
                from
            } else if self.position >= 1. {
                to
            } else {
                from.blend(to.alpha(self.position))
            }
        };
        let (fill, ring, thumb) = (mix(off.0, on.0), mix(off.1, on.1), mix(off.2, on.2));
        let side = self.side;
        let disabled = self.disabled;
        let group_item = self.group_item;
        let ring_focus = self.focus.clone();
        let ring_theme = theme.clone();
        let outline = canvas(
            |_, _, _| (),
            move |bounds, _, window, _| {
                let focused = !disabled && ring_focus.is_focused(window);
                let keyboard = focused && window.last_input_was_keyboard();
                let width = if keyboard { px(2.) } else { px(1.) };
                let color = if keyboard {
                    ring_theme.colors.brand
                } else if focused && group_item {
                    ring_theme.colors.focus.alpha(0.5)
                } else {
                    ring
                };
                window.paint_quad(quad(
                    bounds.dilate(width),
                    px(5.) + width,
                    color.alpha(0.),
                    width,
                    color,
                    Default::default(),
                ));
            },
        )
        .absolute()
        .inset_0()
        .size_full();
        base::SwitchTrack::new("track")
            .checked(self.checked)
            .disabled(disabled)
            .relative()
            .w(px(side * 2.))
            .h(px(side))
            .flex_shrink_0()
            .rounded(px(5.))
            .bg(fill)
            .opacity(if disabled && !group_item { 0.5 } else { 1. })
            .child(
                base::SwitchThumb::new(self.checked)
                    .disabled(disabled)
                    .absolute()
                    .top_0()
                    .bottom_0()
                    .left(px(self.position * side))
                    .w(px(side))
                    .rounded(px(5.))
                    .bg(thumb)
                    .shadow(vec![
                        BoxShadow::new(px(0.), px(0.), theme.colors.shadow_edge)
                            .blur_radius(px(1.))
                            .spread_radius(px(0.5)),
                        BoxShadow::new(px(0.), px(1.), theme.colors.shadow_drop)
                            .blur_radius(px(2.)),
                    ]),
            )
            .child(outline)
    }
}
// CSS ease-out cubic-bezier(0,0,.58,1), inverted on x to obtain y.
fn ease_out(progress: f32) -> f32 {
    let mut low = 0.;
    let mut high = 1.;
    for _ in 0..20 {
        let t = (low + high) / 2.;
        let x = 3. * (1. - t) * t * t * 0.58 + t * t * t;
        if x < progress {
            low = t;
        } else {
            high = t;
        }
    }
    let t = (low + high) / 2.;
    3. * (1. - t) * t * t + t * t * t
}

/// Source Group has no aggregate value. Every child keeps its own controlled
/// value and callback; the group owns only presentation and availability.
#[derive(IntoElement)]
#[must_use]
pub struct SwitchGroup {
    id: ElementId,
    name: SharedString,
    items: Vec<Switch>,
    disabled: bool,
    control_first: bool,
    hidden_legend: bool,
    legend: Option<AnyElement>,
    error: Option<SharedString>,
    description: Option<SharedString>,
}
impl SwitchGroup {
    pub fn new(id: impl Into<ElementId>, name: impl Into<SharedString>) -> Self {
        let name = name.into();
        assert!(
            !name.trim().is_empty(),
            "SwitchGroup requires a readable name"
        );
        Self {
            id: id.into(),
            name,
            items: Vec::new(),
            disabled: false,
            control_first: true,
            hidden_legend: false,
            legend: None,
            error: None,
            description: None,
        }
    }
    /// A Switch is the idiomatic native Item; per-item callbacks remain intact.
    pub fn item(mut self, item: Switch) -> Self {
        assert!(
            !self.items.iter().any(|i| i.id == item.id),
            "Switch item IDs must be unique"
        );
        self.items.push(item);
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn control_first(mut self, first: bool) -> Self {
        self.control_first = first;
        self
    }
    pub fn hide_legend(mut self) -> Self {
        self.hidden_legend = true;
        self
    }
    pub fn legend(mut self, legend: impl IntoElement) -> Self {
        self.legend = Some(legend.into_any_element());
        self
    }
    pub fn error(mut self, error: impl Into<SharedString>) -> Self {
        self.error = Some(error.into());
        self
    }
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }
}
impl RenderOnce for SwitchGroup {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = theme(cx);
        let items = self.items.into_iter().map(|mut item| {
            item.group_item = true;
            item.disabled |= self.disabled;
            item.control_first = self.control_first;
            item
        });
        div()
            .id(self.id)
            .test_support()
            .role(Role::Group)
            .aria_label(self.name.clone())
            .flex()
            .flex_col()
            .min_w_0()
            .gap(theme.spacing.sixteen)
            .when(!self.hidden_legend, |this| {
                this.child(
                    Label::new("legend", self.name)
                        .when_some(self.legend, |label, content| label.content(content)),
                )
            })
            .child(
                div()
                    .flex()
                    .flex_col()
                    .min_w_0()
                    .gap(theme.spacing.eight)
                    .children(items),
            )
            .when_some(self.error, |this, error| {
                this.child(crate::field::group_message_element(
                    theme, "error", error, true,
                ))
            })
            .when_some(self.description, |this, description| {
                this.child(crate::field::group_message_element(
                    theme,
                    "description",
                    description,
                    false,
                ))
            })
    }
}
#[cfg(test)]
mod tests;
