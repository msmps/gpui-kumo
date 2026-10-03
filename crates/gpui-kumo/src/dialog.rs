//! Retained Kumo modal lifecycle over Base's unstyled dialog parts.
use crate::{Button, LayerCard, Theme, theme};
use gpui_kit::base::TestSupportExt;
use gpui_kit::{
    AnyElement, AnyWindowHandle, App, Context, ElementId, Entity, EntityId, EventEmitter,
    FocusHandle, Global, IntoElement, Render, RenderOnce, Role, SharedString, Subscription,
    WeakEntity, WeakFocusHandle, Window, base, div, prelude::*, px,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc, time::Instant};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DialogSize {
    Small,
    #[default]
    Base,
    Large,
    ExtraLarge,
}
impl DialogSize {
    fn width(self, viewport: f32) -> f32 {
        let width: f32 = match self {
            Self::Small => 288.,
            Self::Base => 384.,
            Self::Large => 512.,
            Self::ExtraLarge => 768.,
        };
        if viewport < 640. {
            (viewport - 32.).max(1.)
        } else {
            width.min((viewport - 32.).max(1.))
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DialogRole {
    #[default]
    Dialog,
    AlertDialog,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DialogCloseReason {
    Escape,
    Backdrop,
    CloseButton,
    Action,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DialogEvent {
    OpenChanged(bool),
    Closed(DialogCloseReason),
    ClosePrevented(DialogCloseReason),
}

#[derive(Default)]
struct ModalStack(HashMap<AnyWindowHandle, Vec<WeakEntity<DialogState>>>);
impl Global for ModalStack {}
pub(crate) fn init(cx: &mut App) {
    cx.set_global(ModalStack::default());
}
fn topmost(id: EntityId, window: &Window, cx: &App) -> bool {
    cx.global::<ModalStack>()
        .0
        .get(&window.window_handle())
        .and_then(|stack| stack.iter().rev().find(|state| state.upgrade().is_some()))
        .is_some_and(|state| state.entity_id() == id)
}
fn remove(id: EntityId, window: AnyWindowHandle, cx: &mut App) {
    if let Some(stack) = cx.global_mut::<ModalStack>().0.get_mut(&window) {
        stack.retain(|state| state.entity_id() != id && state.upgrade().is_some());
    }
}

type Content = Rc<dyn Fn(DialogClose, &mut Window, &mut App) -> AnyElement>;
type CloseGuard = Rc<dyn Fn(DialogCloseReason, &mut Window, &mut App) -> bool>;
struct ReturnTarget {
    focus: WeakFocusHandle,
    opener: Option<WeakEntity<Opener>>,
}
struct Opener {
    focus: FocusHandle,
}
struct Mount {
    cleanup: Option<Box<dyn FnOnce()>>,
}
impl Drop for Mount {
    fn drop(&mut self) {
        if let Some(cleanup) = self.cleanup.take() {
            cleanup();
        }
    }
}

/// Application-retained modal state. Mount one Dialog per state.
pub struct DialogState {
    name: SharedString,
    description: Option<SharedString>,
    open: bool,
    role: DialogRole,
    size: DialogSize,
    pointer_dismissal: bool,
    escape_dismissal: bool,
    content_focus: FocusHandle,
    initial_focus: Option<FocusHandle>,
    previous: Option<ReturnTarget>,
    openers: Vec<WeakEntity<Opener>>,
    content: Option<Content>,
    close_guard: Option<CloseGuard>,
    mount: Option<WeakEntity<Mount>>,
    opened_at: Option<Instant>,
    initial_pending: bool,
    last_inside: Rc<RefCell<Option<WeakFocusHandle>>>,
    _theme: Subscription,
    _tab: Subscription,
    focus_out: Option<Subscription>,
}
impl EventEmitter<DialogEvent> for DialogState {}
impl DialogState {
    pub fn new(name: impl Into<SharedString>, cx: &mut Context<Self>) -> Self {
        let name = name.into();
        assert!(
            !name.trim().is_empty(),
            "Dialog requires an accessible name"
        );
        let owner = cx.entity().downgrade();
        Self {
            name,
            description: None,
            open: false,
            role: DialogRole::Dialog,
            size: DialogSize::Base,
            pointer_dismissal: true,
            escape_dismissal: true,
            content_focus: cx.focus_handle().tab_stop(false),
            initial_focus: None,
            previous: None,
            openers: vec![],
            content: None,
            close_guard: None,
            mount: None,
            opened_at: None,
            initial_pending: false,
            last_inside: Rc::new(RefCell::new(None)),
            _theme: cx.observe_global::<Theme>(|_, cx| cx.notify()),
            _tab: cx.intercept_keystrokes(move |event, window, cx| {
                let key = &event.keystroke;
                let m = key.modifiers;
                if key.key != "tab" || m.control || m.alt || m.platform || m.function {
                    return;
                }
                let Some(owner) = owner.upgrade() else {
                    return;
                };
                if owner.read(cx).open && topmost(owner.entity_id(), window, cx) {
                    owner.update(cx, |state, cx| state.traverse(m.shift, window, cx));
                    cx.stop_propagation();
                }
            }),
            focus_out: None,
        }
    }
    pub fn is_open(&self) -> bool {
        self.open
    }
    pub fn set_role(&mut self, role: DialogRole, cx: &mut Context<Self>) {
        self.role = role;
        cx.notify();
    }
    pub fn set_pointer_dismissal(&mut self, allowed: bool, cx: &mut Context<Self>) {
        self.pointer_dismissal = allowed;
        cx.notify();
    }
    pub fn set_escape_dismissal(&mut self, allowed: bool, cx: &mut Context<Self>) {
        self.escape_dismissal = allowed;
        cx.notify();
    }
    /// Retain only weak application captures. Guards run on user close requests, outside rendering.
    /// Imperative set_open is authoritative and does not consult this guard.
    pub fn set_close_guard(
        &mut self,
        guard: impl Fn(DialogCloseReason, &mut Window, &mut App) -> bool + 'static,
        cx: &mut Context<Self>,
    ) {
        self.close_guard = Some(Rc::new(guard));
        cx.notify();
    }
    pub fn set_open(&mut self, open: bool, window: &mut Window, cx: &mut Context<Self>) {
        if self.open == open || (open && self.mount.as_ref().is_some_and(|m| m.upgrade().is_none()))
        {
            return;
        }
        if open {
            self.previous = window.focused(cx).map(|focus| ReturnTarget {
                opener: self
                    .openers
                    .iter()
                    .find(|opener| opener.upgrade().is_some_and(|o| o.read(cx).focus == focus))
                    .cloned(),
                focus: focus.downgrade(),
            });
            let owner = cx.entity().downgrade();
            let stack = cx
                .global_mut::<ModalStack>()
                .0
                .entry(window.window_handle())
                .or_default();
            stack.retain(|state| state.upgrade().is_some());
            stack.push(owner);
            self.opened_at = Some(cx.background_executor().now());
            self.initial_pending = self.initial_focus.is_none();
            self.initial_focus
                .as_ref()
                .unwrap_or(&self.content_focus)
                .focus(window, cx);
        } else {
            remove(cx.entity_id(), window.window_handle(), cx);
            self.opened_at = None;
            self.initial_pending = false;
            let unmounted_focus = self
                .mount
                .as_ref()
                .is_some_and(|mount| mount.upgrade().is_none())
                && self
                    .last_inside
                    .borrow()
                    .as_ref()
                    .and_then(|focus| focus.upgrade())
                    == window.focused(cx);
            if self.content_focus.contains_focused(window, cx)
                || window.focused(cx).is_none()
                || unmounted_focus
            {
                self.restore(window, cx);
            }
            self.previous = None;
            self.last_inside.borrow_mut().take();
        }
        self.open = open;
        cx.emit(DialogEvent::OpenChanged(open));
        cx.notify();
    }
    fn restore(&self, window: &mut Window, cx: &mut App) {
        if let Some(target) = &self.previous
            && target
                .opener
                .as_ref()
                .is_none_or(|opener| opener.upgrade().is_some())
            && let Some(focus) = target.focus.upgrade()
        {
            focus.focus(window, cx);
        } else {
            window.blur(cx);
        }
    }
    pub fn request_close(
        &mut self,
        reason: DialogCloseReason,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        if !self.open || !topmost(cx.entity_id(), window, cx) {
            return false;
        }
        let policy = match reason {
            DialogCloseReason::Escape => self.escape_dismissal,
            DialogCloseReason::Backdrop => {
                self.pointer_dismissal && self.role != DialogRole::AlertDialog
            }
            _ => true,
        };
        if !policy
            || self
                .close_guard
                .as_ref()
                .is_some_and(|guard| !guard(reason, window, cx))
        {
            cx.emit(DialogEvent::ClosePrevented(reason));
            return false;
        }
        self.set_open(false, window, cx);
        cx.emit(DialogEvent::Closed(reason));
        true
    }
    fn traverse(&self, reverse: bool, window: &mut Window, cx: &mut App) {
        if !self.content_focus.contains_focused(window, cx) {
            self.content_focus.focus(window, cx);
        }
        let previous = window.focused(cx);
        let mut visited = vec![];
        loop {
            if reverse {
                window.focus_prev(cx);
            } else {
                window.focus_next(cx);
            }
            if self.content_focus.contains_focused(window, cx) {
                break;
            }
            let Some(focus) = window.focused(cx) else {
                self.content_focus.focus(window, cx);
                break;
            };
            if visited.contains(&focus) {
                previous
                    .as_ref()
                    .unwrap_or(&self.content_focus)
                    .focus(window, cx);
                break;
            }
            visited.push(focus);
        }
    }
}

/// Weak capability for use in caller-provided content and later event callbacks.
#[derive(Clone)]
pub struct DialogClose {
    state: WeakEntity<DialogState>,
}
impl DialogClose {
    pub fn request(&self, reason: DialogCloseReason, window: &mut Window, cx: &mut App) -> bool {
        self.state
            .update(cx, |state, cx| state.request_close(reason, window, cx))
            .unwrap_or(false)
    }
    pub fn button(&self, button: Button) -> Button {
        let close = self.clone();
        button.after_click(move |_, window, cx| {
            close.request(DialogCloseReason::CloseButton, window, cx);
        })
    }
}

/// Kumo Button trigger; existing activation/focus/availability semantics remain authoritative.
#[derive(IntoElement)]
pub struct DialogTrigger {
    id: ElementId,
    state: Entity<DialogState>,
    button: Button,
}
impl DialogTrigger {
    pub fn new(id: impl Into<ElementId>, state: &Entity<DialogState>, button: Button) -> Self {
        Self {
            id: id.into(),
            state: state.clone(),
            button,
        }
    }
}
impl RenderOnce for DialogTrigger {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let provided = self.button.provided_focus();
        let opener = window.use_keyed_state((self.id.clone(), "opener"), cx, |_, cx| Opener {
            focus: provided.unwrap_or_else(|| cx.focus_handle()),
        });
        if let Some(provided) = self.button.provided_focus() {
            opener.update(cx, |opener, _| opener.focus = provided);
        }
        self.state.update(cx, |state, _| {
            state.openers.retain(|opener| opener.upgrade().is_some());
            if !state
                .openers
                .iter()
                .any(|existing| existing.entity_id() == opener.entity_id())
            {
                state.openers.push(opener.downgrade());
            }
        });
        let focus = opener.read(cx).focus.clone();
        let state = self.state.downgrade();
        let open = self.state.read(cx).is_open();
        div()
            .id(self.id)
            .flex()
            .flex_none()
            .child(
                self.button
                    .disclosure_trigger(&focus, false, open, move |_, window, cx| {
                        let _ = state.update(cx, |state, cx| state.set_open(true, window, cx));
                    }),
            )
    }
}

/// Mounted modal presentation. Retain form entities outside its content builder.
#[derive(IntoElement)]
pub struct Dialog {
    id: ElementId,
    state: Entity<DialogState>,
    size: DialogSize,
    description: Option<SharedString>,
    initial_focus: Option<FocusHandle>,
    content: Content,
}
impl Dialog {
    pub fn new(
        id: impl Into<ElementId>,
        state: &Entity<DialogState>,
        content: impl Fn(DialogClose, &mut Window, &mut App) -> AnyElement + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            state: state.clone(),
            size: DialogSize::Base,
            description: None,
            initial_focus: None,
            content: Rc::new(content),
        }
    }
    pub fn size(mut self, size: DialogSize) -> Self {
        self.size = size;
        self
    }
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }
    pub fn initial_focus(mut self, focus: &FocusHandle) -> Self {
        self.initial_focus = Some(focus.clone());
        self
    }
}
impl RenderOnce for Dialog {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let owner = self.state.downgrade();
        let executor = cx.foreground_executor().clone();
        let app = cx.to_async();
        let handle = window.window_handle();
        let mount = window.use_keyed_state((self.id.clone(), "mount"), cx, move |_, _| Mount {
            cleanup: Some(Box::new(move || {
                executor
                    .spawn(async move {
                        app.update(|cx| {
                            let _ = handle.update(cx, |_, window, cx| {
                                let _ = owner.update(cx, |state, cx| {
                                    if state.mount.as_ref().and_then(|m| m.upgrade()).is_some() {
                                        return;
                                    }
                                    if state.open {
                                        state.set_open(false, window, cx);
                                    }
                                    state.content = None;
                                    state.initial_focus = None;
                                });
                            });
                        });
                    })
                    .detach();
            })),
        });
        self.state.update(cx, |state, _| {
            state.size = self.size;
            state.description = self.description;
            state.initial_focus = self.initial_focus;
            state.content = Some(self.content);
            state.mount = Some(mount.downgrade());
        });
        div().id(self.id).child(self.state)
    }
}
impl Render for DialogState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.open {
            return div().into_any_element();
        }
        if self.focus_out.is_none() {
            self.focus_out = Some(cx.on_focus_out(
                &self.content_focus.clone(),
                window,
                |state, _, window, cx| {
                    if state.open
                        && topmost(cx.entity_id(), window, cx)
                        && !state.content_focus.contains_focused(window, cx)
                    {
                        let owner = cx.entity().downgrade();
                        window.defer(cx, move |window, cx| {
                            let _ = owner.update(cx, |state, cx| {
                                if state.open
                                    && topmost(cx.entity_id(), window, cx)
                                    && !state.content_focus.contains_focused(window, cx)
                                {
                                    state.traverse(false, window, cx);
                                }
                            });
                        });
                    }
                },
            ));
        }
        let t = theme(cx).clone();
        let viewport = window.viewport_size();
        let top = px(if viewport.width < px(640.) { 32. } else { 64. });
        let progress = if cx.reduce_motion() {
            1.
        } else {
            self.opened_at.map_or(1., |start| {
                (cx.background_executor()
                    .now()
                    .saturating_duration_since(start)
                    .as_secs_f32()
                    / 0.15)
                    .min(1.)
            })
        };
        if progress < 1. {
            let owner = cx.entity().downgrade();
            window.on_next_frame(move |_, cx| {
                let _ = owner.update(cx, |_, cx| cx.notify());
            });
        }
        let content = self.content.as_ref().map(|builder| {
            builder(
                DialogClose {
                    state: cx.entity().downgrade(),
                },
                window,
                cx,
            )
        });
        let shadows = vec![
            gpui_kit::BoxShadow::new(px(0.), px(0.), t.colors.line).spread_radius(px(1.)),
            gpui_kit::BoxShadow::new(px(0.), px(20.), gpui_kit::black().opacity(0.03))
                .blur_radius(px(25.))
                .spread_radius(px(-5.)),
            gpui_kit::BoxShadow::new(px(0.), px(8.), gpui_kit::black().opacity(0.03))
                .blur_radius(px(10.))
                .spread_radius(px(-6.)),
        ];
        let panel = LayerCard::new("surface")
            .w_full()
            .max_h((viewport.height - top - px(32.)).max(px(1.)))
            .rounded(px(12.))
            .shadow(shadows)
            .child(
                div()
                    .id("content-scroll")
                    .test_support()
                    .max_h((viewport.height - top - px(32.)).max(px(1.)))
                    .overflow_y_scroll()
                    .children(content),
            );
        let modal = div()
            .id("modal")
            .test_support()
            .role(match self.role {
                DialogRole::Dialog => Role::Dialog,
                DialogRole::AlertDialog => Role::AlertDialog,
            })
            .aria_label(self.name.clone())
            .when_some(self.description.clone(), |modal, description| {
                modal.aria_description(description)
            })
            .a11y_synthetic_children(|builder| builder.parent_node().set_modal())
            .track_focus(&self.content_focus)
            .tab_group()
            .w(px(self.size.width(f32::from(viewport.width))))
            .child(panel);
        let popup = base::DialogPopup::new()
            .mt(top)
            .child(modal)
            .opacity(progress);
        let backdrop = base::DialogBackdrop::new()
            .absolute()
            .inset_0()
            .bg(t.colors.recessed.opacity(0.8 * progress));
        let mut host = div()
            .id("modal-host")
            .w(viewport.width)
            .h(viewport.height)
            .absolute()
            .top_0()
            .left_0()
            .font_family(t.typography.font_family.clone())
            .text_size(t.typography.base.size)
            .line_height(t.typography.base.line_height)
            .text_color(t.text.default)
            .flex()
            .items_start()
            .justify_center()
            .occlude()
            .on_key_down(
                cx.listener(|state, event: &gpui_kit::KeyDownEvent, window, cx| {
                    if event.keystroke.key == "escape" && topmost(cx.entity_id(), window, cx) {
                        state.request_close(DialogCloseReason::Escape, window, cx);
                        cx.stop_propagation();
                    }
                }),
            )
            .child(
                div()
                    .id("backdrop")
                    .absolute()
                    .inset_0()
                    .occlude()
                    .on_any_mouse_down(cx.listener(
                        |state, event: &gpui_kit::MouseDownEvent, window, cx| {
                            if event.button == gpui_kit::MouseButton::Left {
                                state.request_close(DialogCloseReason::Backdrop, window, cx);
                            }
                            cx.stop_propagation();
                        },
                    ))
                    .child(backdrop),
            )
            .child(popup);
        let focus = self.content_focus.clone();
        let last_inside = self.last_inside.clone();
        host = host.child(
            gpui_kit::canvas(
                |_, _, _| (),
                move |_, _, window, cx| {
                    if focus.contains_focused(window, cx) {
                        *last_inside.borrow_mut() =
                            window.focused(cx).map(|focus| focus.downgrade());
                    }
                },
            )
            .absolute()
            .size_0(),
        );
        if self.initial_pending {
            self.initial_pending = false;
            let owner = cx.entity().downgrade();
            host = host.child(
                gpui_kit::canvas(
                    |_, _, _| (),
                    move |_, _, window, cx| {
                        let owner = owner.clone();
                        window.defer(cx, move |window, cx| {
                            let _ = owner.update(cx, |state, cx| {
                                if state.open
                                    && topmost(cx.entity_id(), window, cx)
                                    && state.content_focus.is_focused(window)
                                {
                                    state.traverse(false, window, cx);
                                }
                            });
                        });
                    },
                )
                .absolute()
                .size_0(),
            );
        }
        let layer = cx
            .global::<ModalStack>()
            .0
            .get(&window.window_handle())
            .and_then(|stack| {
                stack
                    .iter()
                    .position(|state| state.entity_id() == cx.entity_id())
            })
            .unwrap_or(0);
        gpui_kit::deferred(
            gpui_kit::anchored()
                .position(gpui_kit::point(px(0.), px(0.)))
                .child(host),
        )
        .with_priority(10 + layer)
        .into_any_element()
    }
}

#[cfg(test)]
mod tests;
