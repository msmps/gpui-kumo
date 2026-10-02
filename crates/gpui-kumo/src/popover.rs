//! Nonmodal Popover: Kumo lifecycle and semantics over Base popup positioning.

use std::{cell::Cell, rc::Rc, time::Duration};

use gpui_kit::base::TestSupportExt;

use gpui_kit::{
    AnyElement, App, Bounds, Context, Entity, EventEmitter, FocusHandle, FontWeight, Hsla,
    InteractiveElement, IntoElement, KeyBinding, ParentElement, Pixels, Render, RenderOnce, Role,
    SharedString, StatefulInteractiveElement, Styled, Subscription, WeakFocusHandle, Window, base,
    canvas, deferred, div, px, quad,
};

use crate::{Button, Theme, theme};

gpui_kit::actions!(kumo_popover, [Dismiss]);

pub(crate) fn init(cx: &mut App) {
    cx.bind_keys([KeyBinding::new("escape", Dismiss, Some("KumoPopover"))]);
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Placement {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
}

/// Notifications after the retained state changes, rather than change requests.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PopoverEvent {
    pub open: bool,
}

/// Weak close capability safe to retain in content callbacks.
#[derive(Clone)]
pub struct PopoverClose {
    state: gpui_kit::WeakEntity<PopoverState>,
}

impl PopoverClose {
    pub fn dismiss(&self, window: &mut Window, cx: &mut App) {
        let _ = self.state.update(cx, |state, cx| state.dismiss(window, cx));
    }
}

type ContentBuilder = Rc<dyn Fn(PopoverClose, &mut Window, &mut App) -> AnyElement>;

struct Presentation {
    trigger_label: SharedString,
    placement: Placement,
    offset: Pixels,
    width: Pixels,
    arrow: bool,
    initial_focus: Option<FocusHandle>,
    content: Option<ContentBuilder>,
    parent: Option<gpui_kit::WeakEntity<PopoverState>>,
}

impl Default for Presentation {
    fn default() -> Self {
        Self {
            trigger_label: "Open".into(),
            placement: Placement::Bottom,
            offset: px(8.),
            width: px(280.),
            arrow: false,
            initial_focus: None,
            content: None,
            parent: None,
        }
    }
}

/// Application-retained open state and focus lifecycle. Mount in one Popover.
/// A closed popup releases its Base deferred-context registration immediately.
pub struct PopoverState {
    name: SharedString,
    open: bool,
    disabled: bool,
    opened_at: Option<std::time::Instant>,
    trigger_focus: FocusHandle,
    content_focus: FocusHandle,
    previous_focus: Option<WeakFocusHandle>,
    deferred_context: Option<base::DeferredPopover>,
    presentation: Presentation,
    trigger_bounds: Rc<Cell<Bounds<Pixels>>>,
    resolved_position: Rc<Cell<Option<base::ResolvedPosition>>>,
    parent: Option<gpui_kit::WeakEntity<PopoverState>>,
    children: Vec<gpui_kit::WeakEntity<PopoverState>>,
    _theme_subscription: Subscription,
}

impl EventEmitter<PopoverEvent> for PopoverState {}

impl PopoverState {
    /// `name` names the nonmodal dialog and must be nonempty.
    pub fn new(name: impl Into<SharedString>, cx: &mut Context<Self>) -> Self {
        let name = name.into();
        assert!(
            !name.trim().is_empty(),
            "Popover requires an accessible name"
        );
        Self {
            name,
            open: false,
            disabled: false,
            opened_at: None,
            trigger_focus: cx.focus_handle(),
            content_focus: cx.focus_handle(),
            previous_focus: None,
            deferred_context: None,
            presentation: Presentation::default(),
            trigger_bounds: Rc::new(Cell::new(Bounds::default())),
            resolved_position: Rc::new(Cell::new(None)),
            parent: None,
            children: Vec::new(),
            _theme_subscription: cx.observe_global::<Theme>(|_, cx| cx.notify()),
        }
    }

    pub fn is_open(&self) -> bool {
        self.open
    }
    pub fn is_disabled(&self) -> bool {
        self.disabled
    }
    pub fn trigger_focus(&self) -> FocusHandle {
        self.trigger_focus.clone()
    }

    /// Open or dismiss programmatically. Repeating the current value is a no-op.
    /// Restore the live previous focus target only if focus remains in the popup.
    pub fn set_open(&mut self, open: bool, window: &mut Window, cx: &mut Context<Self>) {
        if self.open == open || (open && self.disabled) {
            return;
        }
        if open {
            self.opened_at = Some(cx.background_executor().now());
            self.previous_focus = window.focused(cx).map(|focus| focus.downgrade());
            self.deferred_context = Some(base::GlobalState::register_deferred_popover(cx));
            self.presentation
                .initial_focus
                .as_ref()
                .unwrap_or(&self.content_focus)
                .focus(window, cx);
        } else {
            self.opened_at = None;
            for child in &self.children {
                let _ = child.update(cx, |child, cx| child.dismiss(window, cx));
            }
            self.children.retain(|child| child.upgrade().is_some());
            self.deferred_context = None;
            if self.content_focus.contains_focused(window, cx) {
                if let Some(previous) = self
                    .previous_focus
                    .as_ref()
                    .and_then(WeakFocusHandle::upgrade)
                {
                    previous.focus(window, cx);
                } else {
                    window.blur(cx);
                }
            }
            self.previous_focus = None;
        }
        self.open = open;
        cx.emit(PopoverEvent { open });
        cx.notify();
    }

    pub fn dismiss(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.set_open(false, window, cx);
    }

    /// Disabling an open Popover also dismisses it.
    pub fn set_disabled(&mut self, disabled: bool, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled == disabled {
            return;
        }
        self.disabled = disabled;
        if disabled {
            self.dismiss(window, cx);
        }
        cx.notify();
    }
}

/// A consumed trigger/content composition over an application-retained state.
/// Content builders may rerun; retain editing and other durable state outside them.
#[derive(IntoElement)]
#[must_use]
pub struct Popover {
    id: gpui_kit::ElementId,
    state: Entity<PopoverState>,
    presentation: Presentation,
}

impl Popover {
    pub fn new(
        id: impl Into<gpui_kit::ElementId>,
        state: &Entity<PopoverState>,
        trigger_label: impl Into<SharedString>,
    ) -> Self {
        Self {
            id: id.into(),
            state: state.clone(),
            presentation: Presentation {
                trigger_label: trigger_label.into(),
                ..Default::default()
            },
        }
    }
    pub fn placement(mut self, placement: Placement) -> Self {
        self.presentation.placement = placement;
        self
    }
    /// Gap between trigger and popup; must be finite and nonnegative.
    pub fn offset(mut self, offset: Pixels) -> Self {
        assert!(
            f32::from(offset).is_finite() && offset >= px(0.),
            "Popover offset must be finite and nonnegative"
        );
        self.presentation.offset = offset;
        self
    }
    /// Explicit native width, constrained by the window. Must be finite and positive.
    pub fn width(mut self, width: Pixels) -> Self {
        assert!(
            f32::from(width).is_finite() && width > px(0.),
            "Popover width must be finite and positive"
        );
        self.presentation.width = width;
        self
    }
    /// Paint Kumo's optional decorative arrow toward the measured trigger.
    pub fn arrow(mut self, arrow: bool) -> Self {
        self.presentation.arrow = arrow;
        self
    }
    pub fn initial_focus(mut self, focus: &FocusHandle) -> Self {
        self.presentation.initial_focus = Some(focus.clone());
        self
    }
    /// Associate a nested popup with its parent's weak close capability.
    /// Closing the parent also closes registered descendants.
    /// Reassigning or omitting this association removes the old registration.
    /// Panics if the association would create an ancestry cycle.
    pub fn parent(mut self, parent: &PopoverClose) -> Self {
        self.presentation.parent = Some(parent.state.clone());
        self
    }

    pub fn content<E: IntoElement>(
        mut self,
        builder: impl Fn(PopoverClose, &mut Window, &mut App) -> E + 'static,
    ) -> Self {
        self.presentation.content = Some(Rc::new(move |close, window, cx| {
            builder(close, window, cx).into_any_element()
        }));
        self
    }
}

impl RenderOnce for Popover {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let child = self.state.downgrade();
        // Validate before changing either side: a rejected association must not
        // leave a partial relationship that can recurse during dismissal.
        let mut ancestor = self.presentation.parent.clone();
        while let Some(parent) = ancestor.and_then(|parent| parent.upgrade()) {
            assert_ne!(
                parent.entity_id(),
                child.entity_id(),
                "A Popover parent association cannot create an ancestry cycle"
            );
            ancestor = parent.read(cx).parent.clone();
        }
        let previous = self.state.read(cx).parent.clone();
        if previous.as_ref().map(|parent| parent.entity_id())
            != self
                .presentation
                .parent
                .as_ref()
                .map(|parent| parent.entity_id())
            && let Some(previous) = previous
        {
            let _ = previous.update(cx, |parent, _| {
                parent
                    .children
                    .retain(|existing| existing.entity_id() != child.entity_id());
            });
        }
        if let Some(parent) = &self.presentation.parent {
            let _ = parent.update(cx, |parent, _| {
                if !parent
                    .children
                    .iter()
                    .any(|existing| existing.entity_id() == child.entity_id())
                {
                    parent.children.push(child);
                }
            });
        }
        self.state.update(cx, |state, _| {
            state.parent = self.presentation.parent.clone();
            state.presentation = self.presentation;
        });
        div()
            .id(self.id)
            .flex()
            .flex_shrink_0()
            .self_start()
            .child(self.state)
    }
}

impl Render for PopoverState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = theme(cx).clone();
        let open = self.open;
        let opacity = if cx.reduce_motion() {
            1.
        } else {
            self.opened_at.map_or(1., |started| {
                (cx.background_executor()
                    .now()
                    .saturating_duration_since(started)
                    .as_secs_f32()
                    / Duration::from_millis(150).as_secs_f32())
                .min(1.)
            })
        };
        if open && opacity < 1. {
            // Retain timing in the entity, independent of deferred element IDs.
            let owner = cx.entity().downgrade();
            window.on_next_frame(move |_, cx| {
                let _ = owner.update(cx, |_, cx| cx.notify());
            });
        }
        let trigger = Button::new("trigger", self.presentation.trigger_label.clone())
            .track_focus(&self.trigger_focus)
            .disabled(self.disabled)
            .open(open)
            .popover_expanded(open)
            .on_click(cx.listener(|state, _, window, cx| state.set_open(!state.open, window, cx)));
        let trigger_bounds = self.trigger_bounds.clone();
        let owner = cx.entity().downgrade();
        let root = div().relative().child(trigger).child(
            canvas(
                move |bounds, window, cx| {
                    if trigger_bounds.replace(bounds) != bounds {
                        // Positioner's public side API takes measured bounds. A
                        // changed trigger schedules the next frame to synchronize.
                        let _ = owner.update(cx, |_, cx| cx.notify());
                        window.request_animation_frame();
                    }
                },
                |_, _, _, _| {},
            )
            .absolute()
            .top_0()
            .left_0()
            .size_full(),
        );
        if !open || self.trigger_bounds.get().size.width == px(0.) {
            return root.into_any_element();
        }
        let content = self.presentation.content.as_ref().map(|builder| {
            builder(
                PopoverClose {
                    state: cx.entity().downgrade(),
                },
                window,
                cx,
            )
        });
        let outline_width = theme.effects.popover_outline_width;
        let outline_offset = theme.effects.popover_outline_offset;
        let radius = theme.radii.lg;
        let outline_color = theme.colors.line;
        let popup_width = self
            .presentation
            .width
            .min((window.viewport_size().width - px(48.)).max(px(1.)));
        let surface = div()
            .id("surface")
            .test_support()
            .role(Role::Dialog)
            .aria_label(self.name.clone())
            .track_focus(&self.content_focus)
            .key_context("KumoPopover")
            .tab_group()
            .occlude()
            .on_action(cx.listener(|state, _: &Dismiss, window, cx| {
                state.dismiss(window, cx);
                cx.stop_propagation();
            }))
            .on_mouse_down_out(cx.listener(
                |state, event: &gpui_kit::MouseDownEvent, window, cx| {
                    if !state.trigger_bounds.get().contains(&event.position) {
                        state.dismiss(window, cx);
                    }
                },
            ))
            .relative()
            .flex()
            .flex_col()
            .w(popup_width)
            .max_h((window.viewport_size().height - px(48.)).max(px(1.)))
            .px(theme.spacing.sixteen)
            .py(theme.spacing.twelve)
            .rounded(radius)
            .bg(theme.colors.base)
            .text_color(theme.text.default)
            .font_family(theme.typography.font_family.clone())
            .font_weight(FontWeight::NORMAL)
            .text_size(theme.typography.sm.size)
            .line_height(theme.typography.sm.line_height)
            .shadow(theme.effects.shadow_md.clone())
            .child(
                div()
                    .id("scroll")
                    .flex()
                    .flex_col()
                    .max_h((window.viewport_size().height - px(72.)).max(px(1.)))
                    // Leave paint room for child focus rings and small shadows.
                    .m(-px(8.))
                    .p(px(8.))
                    .overflow_y_scroll()
                    .children(content),
            )
            .child(
                canvas(
                    |_, _, _| (),
                    move |bounds, _, window, _| {
                        let dilation = outline_width + outline_offset;
                        window.paint_quad(quad(
                            bounds.dilate(dilation),
                            radius + dilation,
                            Hsla::transparent_black(),
                            outline_width,
                            outline_color,
                            Default::default(),
                        ));
                    },
                )
                .absolute()
                .top_0()
                .left_0()
                .size_full(),
            )
            .children(self.presentation.arrow.then(|| {
                arrow::element(
                    self.trigger_bounds.clone(),
                    self.resolved_position.clone(),
                    &theme,
                )
            }));
        let surface = surface.opacity(opacity);
        let resolved = self.resolved_position.clone();
        let placement = match self.presentation.placement {
            Placement::Top => base::Placement::Top,
            Placement::Bottom => base::Placement::Bottom,
            Placement::Left => base::Placement::Left,
            Placement::Right => base::Placement::Right,
        };
        root.child(
            deferred(
                base::Positioner::side(self.trigger_bounds.get())
                    .placement(placement)
                    .offset(self.presentation.offset)
                    .margin(px(8.))
                    .occlude()
                    .on_position(move |position| resolved.set(Some(position)))
                    .child(surface),
            )
            .with_priority(base::POPUP_PRIORITY),
        )
        .into_any_element()
    }
}

#[path = "popover_arrow.rs"]
mod arrow;

#[cfg(test)]
#[path = "popover_tests.rs"]
mod tests;
