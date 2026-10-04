//! Kumo Tooltip over Base popup semantics and positioning.
pub use crate::popover::Placement as Side;
use crate::{Button, Theme, theme};
use gpui_kit::base::TestSupportExt;
use gpui_kit::{
    App, Bounds, Context, ElementId, Entity, EventEmitter, FocusHandle, InteractiveElement,
    IntoElement, ParentElement, Pixels, Point, Render, RenderOnce, SharedString, Styled,
    Subscription, Task, Window, base, canvas, deferred, div, px, quad,
};
use std::{cell::Cell, rc::Rc, time::Duration};
gpui_kit::actions!(
    kumo_tooltip,
    [
        /// Request dismissal of the focused tooltip through its lifecycle policy.
        Dismiss
    ]
);
pub(crate) fn init(cx: &mut App) {
    cx.bind_keys([gpui_kit::KeyBinding::new(
        "escape",
        Dismiss,
        Some("KumoTooltip"),
    )]);
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
/// Cross-axis alignment relative to the anchor.
pub enum Align {
    /// Align with the leading edge.
    Start,
    #[default]
    /// Align with the centre.
    Center,
    /// Align with the trailing edge.
    End,
}
#[derive(Clone, Debug)]
/// Tooltip disclosure lifecycle notification.
pub struct TooltipEvent {
    /// Open.
    pub open: bool,
}
type Trigger = dyn Fn(&mut Window, &mut App) -> Button;
type HoverHook = dyn Fn(&bool, &mut Window, &mut App);
type WindowHook = dyn Fn(&mut Window, &mut App);
pub(crate) struct TriggerHooks {
    /// Bounds.
    pub bounds: Rc<Cell<Bounds<Pixels>>>,
    /// Hover.
    pub hover: Rc<HoverHook>,
    /// Press.
    pub press: Rc<WindowHook>,
    /// Moved.
    pub moved: Rc<WindowHook>,
}
struct Presentation {
    id: ElementId,
    content: SharedString,
    trigger: Rc<Trigger>,
    side: Side,
    align: Align,
    delay: Duration,
    close_delay: Duration,
}
/// Retain once per mounted Tooltip. Owns open state, timers and focus scope;
/// it never owns the trigger's application value or activation callback.
pub struct TooltipState {
    open: bool,
    disabled: bool,
    hovered: bool,
    keyboard_focus: bool,
    suppressed: bool,
    scope: FocusHandle,
    task: Option<Task<()>>,
    presentation: Option<Presentation>,
    bounds: Rc<Cell<Bounds<Pixels>>>,
    resolved: Rc<Cell<Option<base::ResolvedPosition>>>,
    _subscriptions: Vec<Subscription>,
}
impl EventEmitter<TooltipEvent> for TooltipState {}
impl TooltipState {
    /// Create a closed tooltip with retained disclosure timers and focus scope. Retain with `cx.new`.
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let scope = cx.focus_handle().tab_stop(false);
        let subscriptions = vec![
            cx.observe_global::<Theme>(|_, cx| cx.notify()),
            cx.on_focus_in(&scope, window, |state: &mut Self, window, cx| {
                state.keyboard_focus = window.last_input_was_keyboard();
                if state.keyboard_focus {
                    state.request(true, Duration::ZERO, window, cx);
                }
            }),
            cx.on_focus_out(&scope, window, |state: &mut Self, _, window, cx| {
                state.keyboard_focus = false;
                state.suppressed = false;
                state.pointer_moved(window.mouse_position(), window, cx);
            }),
        ];
        Self {
            open: false,
            disabled: false,
            hovered: false,
            keyboard_focus: false,
            suppressed: false,
            scope,
            task: None,
            presentation: None,
            bounds: Rc::new(Cell::new(Bounds::default())),
            resolved: Rc::new(Cell::new(None)),
            _subscriptions: subscriptions,
        }
    }
    /// Report whether the overlay is currently disclosed.
    pub fn is_open(&self) -> bool {
        self.open
    }
    /// Request programmatic disclosure according to the component lifecycle and availability policy.
    pub fn set_open(&mut self, open: bool, cx: &mut Context<Self>) {
        self.task = None;
        let open = open && !self.disabled;
        if self.open != open {
            self.open = open;
            cx.emit(TooltipEvent { open });
            cx.notify();
        }
    }
    /// Suppress tooltip disclosure; trigger availability remains caller-owned.
    pub fn set_disabled(&mut self, disabled: bool, cx: &mut Context<Self>) {
        self.disabled = disabled;
        if disabled {
            self.set_open(false, cx);
        }
    }
    fn close_delay(&self) -> Duration {
        self.presentation
            .as_ref()
            .map_or(Duration::ZERO, |p| p.close_delay)
    }
    fn pointer_moved(&mut self, point: Point<Pixels>, window: &mut Window, cx: &mut Context<Self>) {
        if !self.open || self.keyboard_focus {
            return;
        }
        let inside = self.bounds.get().contains(&point)
            || self.resolved.get().is_some_and(|popup| {
                popup.bounds.contains(&point)
                    || popup
                        .placement
                        .is_some_and(|side| in_bridge(point, self.bounds.get(), popup.bounds, side))
            });
        if inside {
            // Returning during closeDelay cancels dismissal without changing open
            // state or emitting a duplicate event.
            self.task = None;
        } else if self.task.is_none() {
            self.request(false, self.close_delay(), window, cx);
        }
    }
    fn request(
        &mut self,
        open: bool,
        delay: Duration,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.task = None;
        if open && (self.disabled || self.suppressed) {
            return;
        }
        if delay.is_zero() {
            self.set_open(open, cx);
            return;
        }
        self.task = Some(cx.spawn_in(window, async move |this, cx| {
            cx.background_executor().timer(delay).await;
            let _ = this.update_in(cx, |this, _, cx| this.set_open(open, cx));
        }));
    }
}
/// Text-content tooltip with a caller-configured Base-backed Button trigger.
/// The factory is retained. Capture weak handles for this state or its owner;
/// strong captures would form a reference cycle. Create TooltipState once.
#[derive(IntoElement)]
pub struct Tooltip {
    state: Entity<TooltipState>,
    presentation: Presentation,
}
impl Tooltip {
    /// Compose readable tooltip content and a Button factory over retained disclosure state.
    pub fn new(
        id: impl Into<ElementId>,
        state: &Entity<TooltipState>,
        content: impl Into<SharedString>,
        trigger: impl Fn(&mut Window, &mut App) -> Button + 'static,
    ) -> Self {
        Self {
            state: state.clone(),
            presentation: Presentation {
                id: id.into(),
                content: content.into(),
                trigger: Rc::new(trigger),
                side: Side::Top,
                align: Align::Center,
                delay: Duration::from_millis(600),
                close_delay: Duration::ZERO,
            },
        }
    }
    /// Select the preferred anchor side.
    pub fn side(mut self, side: Side) -> Self {
        self.presentation.side = side;
        self
    }
    /// Choose alignment along the surface’s cross axis.
    pub fn align(mut self, align: Align) -> Self {
        self.presentation.align = align;
        self
    }
    /// Set the hover disclosure delay.
    pub fn delay(mut self, delay: Duration) -> Self {
        self.presentation.delay = delay;
        self
    }
    /// Set the delayed dismissal interval.
    pub fn close_delay(mut self, delay: Duration) -> Self {
        self.presentation.close_delay = delay;
        self
    }
}
impl RenderOnce for Tooltip {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        self.state.update(cx, |state, cx| {
            state.presentation = Some(self.presentation);
            cx.notify();
        });
        self.state
    }
}
impl Render for TooltipState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(p) = &self.presentation else {
            return div().into_any_element();
        };
        let delay = p.delay;
        let hover_state = cx.entity().downgrade();
        let press_state = hover_state.clone();
        let moved_state = hover_state.clone();
        let trigger = (p.trigger)(window, cx).tooltip_trigger(TriggerHooks {
            bounds: self.bounds.clone(),
            moved: Rc::new(move |window, cx| {
                let _ = moved_state.update(cx, |_, cx| cx.notify());
                window.request_animation_frame();
            }),
            hover: Rc::new(move |hovered, window, cx| {
                let _ = hover_state.update(cx, |state, cx| {
                    state.hovered = *hovered;
                    if !hovered {
                        state.suppressed = false;
                    }
                    if *hovered {
                        if state.open {
                            state.task = None;
                        } else {
                            state.request(true, delay, window, cx);
                        }
                    } else if state.open {
                        state.pointer_moved(window.mouse_position(), window, cx);
                    } else {
                        // Cancel a pending opening even though there is no popup.
                        state.task = None;
                    }
                });
            }),
            press: Rc::new(move |_, cx| {
                let _ = press_state.update(cx, |state, cx| {
                    state.suppressed = true;
                    state.set_open(false, cx);
                });
            }),
        });
        let root = div()
            .id(p.id.clone())
            .test_support()
            .track_focus(&self.scope)
            .key_context("KumoTooltip")
            .relative()
            .flex()
            .flex_none()
            .on_mouse_down(gpui_kit::MouseButton::Left, |_, window, _| {
                window.prevent_default()
            })
            .on_action(cx.listener(|state, _: &Dismiss, _, cx| {
                if state.open {
                    state.suppressed = true;
                    state.set_open(false, cx);
                } else {
                    cx.propagate();
                }
            }))
            .child(
                div()
                    .flex()
                    .flex_none()
                    .on_mouse_down(
                        gpui_kit::MouseButton::Left,
                        cx.listener(|state, _, _, cx| {
                            state.suppressed = true;
                            state.set_open(false, cx);
                            cx.stop_propagation();
                        }),
                    )
                    .child(trigger),
            );
        if !self.open {
            return root.into_any_element();
        }
        let pointer_state = cx.entity().downgrade();
        let root = root.child(
            canvas(
                |_, _, _| (),
                move |_, _, window, _| {
                    let pointer_state = pointer_state.clone();
                    window.on_mouse_event(
                        move |event: &gpui_kit::MouseMoveEvent, phase, window, cx| {
                            if phase.capture() {
                                let _ = pointer_state.update(cx, |state, cx| {
                                    state.pointer_moved(event.position, window, cx)
                                });
                            }
                        },
                    );
                },
            )
            .absolute()
            .inset_0()
            .size_full(),
        );
        let t = theme(cx).clone();
        let placement = match p.side {
            Side::Top => base::Placement::Top,
            Side::Bottom => base::Placement::Bottom,
            Side::Left => base::Placement::Left,
            Side::Right => base::Placement::Right,
        };
        let align = match p.align {
            Align::Start => base::Align::Start,
            Align::Center => base::Align::Center,
            Align::End => base::Align::End,
        };
        let resolved = self.resolved.clone();
        let content = p.content.clone();
        let arrow = crate::popover::arrow::element_with_inset(
            self.bounds.clone(),
            self.resolved.clone(),
            &t,
            px(10.),
        );
        let popup = base::Tooltip::new("tooltip-popup")
            .flex()
            .flex_col()
            .relative()
            .rounded(t.radii.md)
            .bg(t.colors.base)
            .px(px(10.))
            .py(px(6.))
            .max_w((window.viewport_size().width - px(8.)).max(px(1.)))
            .font_family(t.typography.font_family.clone())
            .text_color(t.text.default)
            .text_size(t.typography.sm.size)
            .line_height(t.typography.sm.line_height)
            .shadow(t.effects.shadow_md.clone())
            .child(
                crate::Text::new("tooltip-text", content).style(crate::text::Style::Copy {
                    size: crate::text::Size::Sm,
                    bold: false,
                    tone: crate::text::Tone::Default,
                }),
            )
            .child(
                canvas(
                    |_, _, _| (),
                    move |bounds, _, window, _| {
                        window.paint_quad(quad(
                            bounds.dilate(px(1.)),
                            t.radii.md + px(1.),
                            t.colors.line.alpha(0.),
                            px(1.),
                            t.colors.line,
                            Default::default(),
                        ));
                    },
                )
                .absolute()
                .inset_0()
                .size_full(),
            )
            // Like the source SVG child, arrow fill paints over the popup outline
            // at the notch. Painting the outline last creates a false separator.
            .child(arrow);
        root.child(
            deferred(
                base::Positioner::side(exclusion_bounds(self.bounds.get(), p.side))
                    .placement(placement)
                    .align(align)
                    .margin(px(4.))
                    .on_position(move |position| resolved.set(Some(position)))
                    .child(popup),
            )
            .with_priority(200),
        )
        .into_any_element()
    }
}

// A native safe bridge joins the facing trigger/popup edges. It preserves
// hoverability across Kumo's gap without delaying dismissal elsewhere. Browser
// Base UI1.8 uses safePolygon; this bounded trapezoid adapts that pointer contract.
fn in_bridge(
    point: Point<Pixels>,
    trigger: Bounds<Pixels>,
    popup: Bounds<Pixels>,
    side: base::Placement,
) -> bool {
    let (main, start, end, cross, a0, a1, b0, b1) = match side {
        base::Placement::Top => (
            point.y,
            popup.bottom(),
            trigger.top(),
            point.x,
            popup.left(),
            popup.right(),
            trigger.left(),
            trigger.right(),
        ),
        base::Placement::Bottom => (
            point.y,
            trigger.bottom(),
            popup.top(),
            point.x,
            trigger.left(),
            trigger.right(),
            popup.left(),
            popup.right(),
        ),
        base::Placement::Left => (
            point.x,
            popup.right(),
            trigger.left(),
            point.y,
            popup.top(),
            popup.bottom(),
            trigger.top(),
            trigger.bottom(),
        ),
        base::Placement::Right => (
            point.x,
            trigger.right(),
            popup.left(),
            point.y,
            trigger.top(),
            trigger.bottom(),
            popup.top(),
            popup.bottom(),
        ),
    };
    if main < start || main > end || end <= start {
        return false;
    }
    let fraction = f32::from(main - start) / f32::from(end - start);
    cross >= a0 + (b0 - a0) * fraction && cross <= a1 + (b1 - a1) * fraction
}

// Base resolves flipping before applying offset. Expand only the main axis to
// include Kumo's 10px gap in fitting without shifting Start/End alignment.
fn exclusion_bounds(mut bounds: Bounds<Pixels>, side: Side) -> Bounds<Pixels> {
    match side {
        Side::Top | Side::Bottom => {
            bounds.origin.y -= px(10.);
            bounds.size.height += px(20.);
        }
        Side::Left | Side::Right => {
            bounds.origin.x -= px(10.);
            bounds.size.width += px(20.);
        }
    }
    bounds
}

/// Mount above the section's focusable controls so Escape also dismisses a
/// hover tooltip while focus is on another control. Delay grouping is pending.
#[derive(IntoElement)]
pub struct TooltipProvider {
    id: ElementId,
    states: Vec<gpui_kit::WeakEntity<TooltipState>>,
    child: gpui_kit::AnyElement,
}
impl TooltipProvider {
    /// Coordinate weakly held tooltip states for the supplied subtree.
    pub fn new(
        id: impl Into<ElementId>,
        states: impl IntoIterator<Item = gpui_kit::WeakEntity<TooltipState>>,
        child: impl IntoElement,
    ) -> Self {
        Self {
            id: id.into(),
            states: states.into_iter().collect(),
            child: child.into_any_element(),
        }
    }
}
impl RenderOnce for TooltipProvider {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .capture_key_down(move |event, _, cx| {
                if event.keystroke.key == "escape" {
                    let mut dismissed = false;
                    for state in &self.states {
                        let _ = state.update(cx, |state, cx| {
                            if state.open || state.task.is_some() {
                                state.suppressed = true;
                                state.set_open(false, cx);
                                dismissed = true;
                            }
                        });
                    }
                    if dismissed {
                        cx.stop_propagation();
                    }
                }
            })
            .child(self.child)
    }
}

impl std::fmt::Debug for TooltipState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_struct("TooltipState");
        debug.field("open", &self.open);
        debug.field("disabled", &self.disabled);
        debug.field("hovered", &self.hovered);
        debug.field("keyboard_focus", &self.keyboard_focus);
        debug.field("suppressed", &self.suppressed);
        debug.finish_non_exhaustive()
    }
}

impl std::fmt::Debug for Tooltip {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_struct("Tooltip");
        debug.field("id", &self.presentation.id);
        debug.field("state", &self.state.entity_id());
        debug.finish_non_exhaustive()
    }
}

impl std::fmt::Debug for TooltipProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_struct("TooltipProvider");
        debug.field("id", &self.id);
        debug.finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests;
