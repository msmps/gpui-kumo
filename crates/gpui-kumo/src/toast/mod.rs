//! Retained Kumo notification presentation over Base's manager and stack.
use crate::{Button, Theme, theme};
use gpui_kit::base::TestSupportExt;
use gpui_kit::{
    Anchor, AnyWindowHandle, App, Context, ElementId, Entity, EventEmitter, FocusHandle,
    FontWeight, IntoElement, Render, RenderOnce, Role, SharedString, Subscription, WeakEntity,
    WeakFocusHandle, Window, base, div, prelude::*, px,
};
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    time::{Duration, Instant},
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
/// Notification semantic treatment.
pub enum ToastVariant {
    #[default]
    /// Default semantic treatment.
    Default,
    /// Success notification treatment.
    Success,
    /// Error treatment.
    Error,
    /// Warning notification treatment.
    Warning,
    /// Informational notification treatment.
    Info,
}
#[derive(Clone, Debug)]
/// Notification action with a stable routing key and readable button label.
pub struct ToastAction {
    /// Stable action routing key, independent of localised text.
    pub id: SharedString,
    /// Complete text for label.
    pub label: SharedString,
    /// Variant.
    pub variant: crate::button::Variant,
    /// Disabled.
    pub disabled: bool,
}
impl ToastAction {
    /// Create an action with a stable routing key and complete readable label.
    ///
    /// # Panics
    /// Panics when `id` or `label` is blank.
    pub fn new(id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        let id = id.into();
        assert!(!id.trim().is_empty(), "Toast action requires a stable key");
        Self {
            id,
            label: crate::name::nonblank(label),
            variant: crate::button::Variant::Secondary,
            disabled: false,
        }
    }
    /// Select the semantic visual treatment; interaction and value state remain independent.
    pub fn variant(mut self, variant: crate::button::Variant) -> Self {
        self.variant = variant;
        self
    }
    /// Choose whether this control accepts user activation.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}
#[derive(Clone, Debug)]
/// Notification text and actions, updated independently of its identity.
pub struct ToastContent {
    /// Complete text for title.
    pub title: SharedString,
    /// Description.
    pub description: Option<SharedString>,
    /// Variant.
    pub variant: ToastVariant,
    /// Actions.
    pub actions: Vec<ToastAction>,
}
/// An add request. Stable IDs deduplicate; use update to change existing content.
pub struct Toast {
    id: SharedString,
    content: ToastContent,
    timeout: Option<Duration>,
}
impl Toast {
    /// Create a notification request with a stable deduplication key and readable title.
    pub fn new(id: impl Into<SharedString>, title: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            content: ToastContent {
                title: title.into(),
                description: None,
                variant: ToastVariant::Default,
                actions: vec![],
            },
            timeout: Some(Duration::from_secs(5)),
        }
    }
    /// Supply independently readable notification description text.
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.content.description = Some(description.into());
        self
    }
    /// Select the semantic visual treatment; interaction and value state remain independent.
    pub fn variant(mut self, variant: ToastVariant) -> Self {
        self.content.variant = variant;
        self
    }
    /// Zero keeps the notification until explicitly dismissed.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = (!timeout.is_zero()).then_some(timeout);
        self
    }
    /// Append a notification action.
    pub fn action(mut self, action: ToastAction) -> Self {
        self.content.actions.push(action);
        self
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
/// Origin of a notification dismissal.
pub enum ToastDismissReason {
    /// Requested by the notification close button.
    Close,
    /// Requested through Escape.
    Escape,
    /// The configured duration elapsed.
    Timeout,
    /// Requested by application code.
    Programmatic,
}
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
/// Notification lifecycle or action routing event.
pub enum ToastEvent {
    /// A notification was added.
    Added(SharedString),
    /// An existing notification payload changed.
    Updated(SharedString),
    /// A notification began dismissal with the supplied reason.
    Dismissed(SharedString, ToastDismissReason),
    /// The dismissal transition completed.
    Removed(SharedString),
    /// Requested by an application action.
    Action {
        /// Complete text for toast.
        toast: SharedString,
        /// Complete text for action.
        action: SharedString,
    },
}
struct Entry {
    content: ToastContent,
    focus: FocusHandle,
    close: FocusHandle,
    actions: HashMap<SharedString, FocusHandle>,
    entered_at: Instant,
    closed_at: Option<Instant>,
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
fn motion() -> base::ToastMotion {
    base::ToastMotion {
        duration: Duration::from_millis(500),
        exit_duration: Duration::from_millis(500),
        collapsed_peek: px(12.),
        expanded_gap: px(12.),
        collapsed_scale_step: 0.1,
        collapsed_visible: 3,
    }
}
fn manager() -> base::ToastManager<SharedString, Rc<RefCell<Entry>>> {
    // Timeout starts at add, independently of the visual entrance transition.
    base::ToastManager::new(base::ToastMotion {
        duration: Duration::ZERO,
        ..motion()
    })
}

/// Retain one state per mounted ToastViewport. Application operations own action events.
pub struct ToastState {
    manager: base::ToastManager<SharedString, Rc<RefCell<Entry>>>,
    stack: base::ToastStackState,
    focus: FocusHandle,
    previous: Option<WeakFocusHandle>,
    last_inside: Rc<RefCell<Option<WeakFocusHandle>>>,
    mount: Option<WeakEntity<Mount>>,
    timer_running: bool,
    _theme: Subscription,
    _keys: Subscription,
}
impl EventEmitter<ToastEvent> for ToastState {}
impl ToastState {
    fn clear(&mut self) {
        self.manager = manager();
        self.stack = base::ToastStackState::default();
        self.timer_running = false;
        self.last_inside.borrow_mut().take();
    }
    /// Create an empty retained notification manager. Retain with `cx.new`.
    pub fn new(cx: &mut Context<Self>) -> Self {
        let owner = cx.entity().downgrade();
        Self {
            manager: manager(),
            stack: base::ToastStackState::default(),
            focus: cx.focus_handle().tab_stop(false),
            previous: None,
            last_inside: Rc::new(RefCell::new(None)),
            mount: None,
            timer_running: false,
            _theme: cx.observe_global::<Theme>(|_, cx| cx.notify()),
            _keys: cx.intercept_keystrokes(move |event, window, cx| {
                let key = &event.keystroke;
                let Some(owner) = owner.upgrade() else {
                    return;
                };
                if owner.read(cx).manager.is_empty()
                    || owner
                        .read(cx)
                        .mount
                        .as_ref()
                        .is_none_or(|m| m.upgrade().is_none())
                {
                    return;
                }
                if key.key == "f6" && key.modifiers == gpui_kit::Modifiers::default() {
                    owner.update(cx, |state, cx| {
                        if !state.focus.is_focused(window) {
                            state.previous = window.focused(cx).map(|f| f.downgrade());
                            state.focus.focus(window, cx);
                            cx.notify();
                        }
                    });
                    cx.stop_propagation();
                } else if key.key == "tab"
                    && !key.modifiers.control
                    && !key.modifiers.alt
                    && !key.modifiers.platform
                    && !key.modifiers.function
                    && owner.read(cx).focus.is_focused(window)
                {
                    owner.update(cx, |state, cx| {
                        if key.modifiers.shift {
                            state.restore(window, cx);
                        } else if let Some((_, entry, _)) = state
                            .manager
                            .iter()
                            .rev()
                            .find(|(_, _, phase)| *phase != base::ToastTransitionStatus::Ending)
                        {
                            entry.borrow().focus.focus(window, cx);
                        }
                    });
                    cx.stop_propagation();
                }
            }),
        }
    }
    /// Read the number of retained notifications.
    pub fn len(&self) -> usize {
        self.manager.len()
    }
    /// Return whether the notification manager is empty.
    pub fn is_empty(&self) -> bool {
        self.manager.is_empty()
    }
    /// Clone the current payload for a notification key, if it exists.
    pub fn content(&self, id: &str) -> Option<ToastContent> {
        self.manager
            .get(&SharedString::from(id.to_owned()))
            .map(|entry| entry.borrow().content.clone())
    }
    /// Add a notification; an existing stable key deduplicates the request.
    ///
    /// # Panics
    /// Panics for a blank notification key or content, blank action keys/names,
    /// or duplicate action keys.
    pub fn add(
        &mut self,
        toast: Toast,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> SharedString {
        assert!(!toast.id.trim().is_empty(), "Toast requires a stable ID");
        if self
            .mount
            .as_ref()
            .is_some_and(|mount| mount.upgrade().is_none())
        {
            return toast.id;
        }
        validate_content(&toast.content);
        if self.manager.get(&toast.id).is_some() {
            return toast.id;
        }
        if !self.focus.contains_focused(window, cx) {
            self.previous = window.focused(cx).map(|f| f.downgrade());
        }
        let now = cx.background_executor().now();
        let actions = toast
            .content
            .actions
            .iter()
            .map(|action| (action.id.clone(), cx.focus_handle()))
            .collect();
        self.manager.push(
            toast.id.clone(),
            Rc::new(RefCell::new(Entry {
                content: toast.content,
                focus: cx.focus_handle(),
                close: cx.focus_handle(),
                actions,
                entered_at: now,
                closed_at: None,
            })),
            base::ToastOptions {
                timeout: toast.timeout,
            },
            now,
        );
        self.manager.advance(now, false);
        cx.emit(ToastEvent::Added(toast.id.clone()));
        cx.notify();
        toast.id
    }
    /// Update the current payload in place. Order, focus identities and remaining timeout are retained.
    /// Returns false for missing or dismissing notifications.
    ///
    /// # Panics
    /// Panics for blank content, blank action keys/names, or duplicate action keys.
    /// Invalid proposals leave the previous payload intact.
    pub fn update(
        &mut self,
        id: &str,
        change: impl FnOnce(&mut ToastContent),
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        let id = SharedString::from(id.to_owned());
        if !self
            .manager
            .iter()
            .any(|(key, _, phase)| key == &id && phase != base::ToastTransitionStatus::Ending)
        {
            return false;
        }
        let entry = self.manager.get(&id).unwrap();
        let mut entry = entry.borrow_mut();
        let mut content = entry.content.clone();
        change(&mut content);
        validate_content(&content);
        entry.content = content;
        let keys = entry
            .content
            .actions
            .iter()
            .map(|action| action.id.clone())
            .collect::<Vec<_>>();
        let removed_focused = entry
            .actions
            .iter()
            .any(|(id, focus)| !keys.contains(id) && focus.is_focused(window));
        entry.actions.retain(|id, _| keys.contains(id));
        for id in keys {
            entry.actions.entry(id).or_insert_with(|| cx.focus_handle());
        }
        if removed_focused {
            entry.focus.focus(window, cx);
        }
        drop(entry);
        cx.emit(ToastEvent::Updated(id));
        cx.notify();
        true
    }
    /// Dismiss through the overlay’s focus-restoration lifecycle.
    pub fn dismiss(&mut self, id: &str, window: &mut Window, cx: &mut Context<Self>) -> bool {
        self.close(
            &SharedString::from(id.to_owned()),
            ToastDismissReason::Programmatic,
            window,
            cx,
        )
    }
    fn close(
        &mut self,
        id: &SharedString,
        reason: ToastDismissReason,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        let now = cx.background_executor().now();
        if !self.manager.dismiss(id, now) {
            return false;
        }
        if let Some(entry) = self.manager.get(id) {
            let mut entry = entry.borrow_mut();
            entry.closed_at = Some(now);
            if entry.focus.contains_focused(window, cx) {
                drop(entry);
                self.recover(window, cx);
            }
        }
        cx.emit(ToastEvent::Dismissed(id.clone(), reason));
        cx.notify();
        true
    }
    fn restore(&self, window: &mut Window, cx: &mut App) {
        if let Some(focus) = self.previous.as_ref().and_then(|focus| focus.upgrade()) {
            focus.focus(window, cx);
        } else {
            window.blur(cx);
        }
    }
    fn recover(&self, window: &mut Window, cx: &mut App) {
        if let Some((_, entry, _)) = self
            .manager
            .iter()
            .rev()
            .find(|(_, _, phase)| *phase != base::ToastTransitionStatus::Ending)
        {
            entry.borrow().focus.focus(window, cx);
        } else {
            self.restore(window, cx);
        }
    }
    fn advance(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let paused = self.stack.is_expanded()
            || self.focus.contains_focused(window, cx)
            || !window.is_window_active();
        let now = cx.background_executor().now();
        let changes = self.manager.advance(now, paused);
        for id in changes.ending {
            if let Some(entry) = self.manager.get(&id) {
                entry.borrow_mut().closed_at = Some(now);
            }
            cx.emit(ToastEvent::Dismissed(id, ToastDismissReason::Timeout));
        }
        for (id, _) in changes.removed {
            cx.emit(ToastEvent::Removed(id));
        }
        if self.manager.is_empty() {
            self.stack = base::ToastStackState::default();
        }
        if changes.changed {
            cx.notify();
        }
    }
    fn start_timer(&mut self, window: &Window, cx: &mut Context<Self>) {
        if self.timer_running || self.manager.is_empty() {
            return;
        }
        let Some(mount) = self.mount.clone() else {
            return;
        };
        self.timer_running = true;
        let handle = window.window_handle();
        cx.spawn(async move |owner, cx| {
            loop {
                cx.background_executor()
                    .timer(Duration::from_millis(50))
                    .await;
                let keep = handle
                    .update(cx, |_, window, cx| {
                        owner
                            .update(cx, |state, cx| {
                                if mount.upgrade().is_none()
                                    || state.mount.as_ref().is_none_or(|current| {
                                        current.entity_id() != mount.entity_id()
                                    })
                                {
                                    return false;
                                }
                                state.advance(window, cx);
                                if state.manager.is_empty() {
                                    state.timer_running = false;
                                    false
                                } else {
                                    true
                                }
                            })
                            .unwrap_or(false)
                    })
                    .unwrap_or(false);
                if !keep {
                    break;
                }
            }
        })
        .detach();
    }
}

/// One mounted notification viewport; existing application content stays outside it.
#[derive(IntoElement)]
pub struct ToastViewport {
    id: ElementId,
    state: Entity<ToastState>,
}
impl ToastViewport {
    /// Present notifications from a retained state under a stable element identity.
    pub fn new(id: impl Into<ElementId>, state: &Entity<ToastState>) -> Self {
        Self {
            id: id.into(),
            state: state.clone(),
        }
    }
}
impl RenderOnce for ToastViewport {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let owner = self.state.downgrade();
        let app = cx.to_async();
        let executor = cx.foreground_executor().clone();
        let handle: AnyWindowHandle = window.window_handle();
        let mount = window.use_keyed_state((self.id.clone(), "mount"), cx, move |_, _| Mount {
            cleanup: Some(Box::new(move || {
                executor
                    .spawn(async move {
                        app.update(|cx| {
                            let updated = handle.update(cx, |_, window, cx| {
                                let _ = owner.update(cx, |state, cx| {
                                    if state.mount.as_ref().and_then(|m| m.upgrade()).is_some() {
                                        return;
                                    }
                                    let last = state
                                        .last_inside
                                        .borrow()
                                        .as_ref()
                                        .and_then(|focus| focus.upgrade());
                                    if state.focus.contains_focused(window, cx)
                                        || (last.is_some() && last == window.focused(cx))
                                    {
                                        state.restore(window, cx);
                                    }
                                    state.clear();
                                    cx.notify();
                                });
                            });
                            if updated.is_err() {
                                let _ = owner.update(cx, |state, cx| {
                                    if state
                                        .mount
                                        .as_ref()
                                        .and_then(|mount| mount.upgrade())
                                        .is_some()
                                    {
                                        return;
                                    }
                                    state.clear();
                                    cx.notify();
                                });
                            }
                        });
                    })
                    .detach();
            })),
        });
        self.state.update(cx, |state, _| {
            if state
                .mount
                .as_ref()
                .is_some_and(|previous| previous.entity_id() != mount.entity_id())
            {
                state.timer_running = false;
            }
            state.mount = Some(mount.downgrade());
        });
        div().id(self.id).child(self.state)
    }
}
fn icon(variant: ToastVariant) -> Option<&'static [u8]> {
    match variant {
        ToastVariant::Default => None,
        ToastVariant::Success => Some(include_bytes!("../../assets/toast-success.svg")),
        ToastVariant::Error => Some(include_bytes!("../../assets/toast-error.svg")),
        ToastVariant::Warning => Some(include_bytes!("../../assets/toast-warning.svg")),
        ToastVariant::Info => Some(include_bytes!("../../assets/toast-info.svg")),
    }
}
impl Render for ToastState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.manager.is_empty() {
            return div().into_any_element();
        }
        self.start_timer(window, cx);
        let t = theme(cx).clone();
        let viewport = window.viewport_size();
        let margin = px(if viewport.width < px(640.) { 16. } else { 32. });
        let width = if viewport.width < px(640.) {
            (viewport.width - margin * 2.).max(px(1.))
        } else {
            px(340.).min((viewport.width - margin * 2.).max(px(1.)))
        };
        let expanded = self.stack.is_expanded() || self.focus.contains_focused(window, cx);
        let entries = self
            .manager
            .visible(3)
            .map(|(id, entry, phase)| (id.clone(), entry.clone(), phase))
            .collect::<Vec<_>>();
        let mut stack = base::ToastStack::new("stack", self.stack.clone())
            .placement(Anchor::BottomRight)
            .motion(motion())
            .focus_handle(self.focus.clone())
            .w(width);
        for (index, (id, entry, phase)) in entries.iter().enumerate() {
            let entry = entry.borrow();
            entry
                .focus
                .clone()
                .tab_stop(*phase != base::ToastTransitionStatus::Ending);
            let content = &entry.content;
            let color = match content.variant {
                ToastVariant::Default => t.text.default,
                ToastVariant::Success => t.text.success,
                ToastVariant::Error => t.text.danger,
                ToastVariant::Warning => t.text.warning,
                ToastVariant::Info => t.text.info,
            };
            let tint = match content.variant {
                ToastVariant::Default => t.colors.base.opacity(0.9),
                ToastVariant::Success => t.colors.success_tint.opacity(0.2),
                ToastVariant::Error => t.colors.danger_tint.opacity(0.5),
                ToastVariant::Warning => t.colors.warning_tint.opacity(0.5),
                ToastVariant::Info => t.colors.info_tint.opacity(0.5),
            };
            let now = cx.background_executor().now();
            let alpha = if cx.reduce_motion() {
                1.
            } else if let Some(closed) = entry.closed_at {
                (1. - now.saturating_duration_since(closed).as_secs_f32() / 0.5).max(0.)
            } else {
                (now.saturating_duration_since(entry.entered_at)
                    .as_secs_f32()
                    / 0.5)
                    .min(1.)
            };
            if alpha < 1. && (*phase != base::ToastTransitionStatus::Ending || alpha > 0.) {
                window.on_next_frame(|_, cx| cx.refresh_windows());
            }
            let name = if content.title.trim().is_empty() {
                content.description.clone().unwrap_or_default()
            } else {
                content.title.clone()
            };
            let owner = cx.entity().downgrade();
            let close_id = id.clone();
            let close_color = if content.variant == ToastVariant::Default {
                t.text.subtle
            } else {
                color
            };
            let close = Button::icon(
                "close",
                "Close",
                gpui_kit::svg()
                    .data(include_bytes!("../../assets/toast-close.svg"))
                    .size(px(12.))
                    .text_color(close_color),
            )
            .variant(crate::button::Variant::Ghost)
            .size(crate::button::Size::Sm)
            .track_focus(&entry.close)
            .w(px(20.))
            .h(px(20.))
            .rounded(px(4.))
            .disabled(*phase == base::ToastTransitionStatus::Ending)
            .on_click(move |_, window, cx| {
                let _ = owner.update(cx, |state, cx| {
                    state.close(&close_id, ToastDismissReason::Close, window, cx)
                });
            });
            let mut text = div()
                .flex()
                .flex_col()
                .gap(px(4.))
                .flex_1()
                .min_w_0()
                .when(!content.title.is_empty(), |text| {
                    text.child(
                        div()
                            .id("title")
                            .text_size(px(15.6))
                            .line_height(px(20.))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(color)
                            .child(content.title.clone()),
                    )
                })
                .when_some(content.description.clone(), |text, description| {
                    text.child(
                        div()
                            .id("description")
                            .text_size(px(14.8))
                            .line_height(px(20.))
                            .text_color(t.text.default.opacity(0.7))
                            .child(description),
                    )
                });
            if !content.actions.is_empty() {
                let mut actions = div()
                    .id("actions")
                    .flex()
                    .gap(px(8.))
                    .mt(px(8.))
                    .p(px(1.))
                    .min_w_0()
                    .overflow_x_scroll();
                for action in &content.actions {
                    let owner = cx.entity().downgrade();
                    let toast_id = id.clone();
                    let action_id = action.id.clone();
                    actions = actions.child(
                        Button::new(action.id.clone(), action.label.clone())
                            .variant(action.variant)
                            .track_focus(&entry.actions[&action.id])
                            .disabled(
                                action.disabled || *phase == base::ToastTransitionStatus::Ending,
                            )
                            .on_click(move |_, _, cx| {
                                let _ = owner.update(cx, |state, cx| {
                                    if state.manager.iter().any(|(id, entry, phase)| {
                                        id == &toast_id
                                            && phase != base::ToastTransitionStatus::Ending
                                            && entry.borrow().content.actions.iter().any(|action| {
                                                action.id == action_id && !action.disabled
                                            })
                                    }) {
                                        cx.emit(ToastEvent::Action {
                                            toast: toast_id.clone(),
                                            action: action_id.clone(),
                                        });
                                    }
                                });
                            }),
                    );
                }
                text = text.child(actions);
            }
            let row = div()
                .flex()
                .items_start()
                .gap(px(8.))
                .when_some(icon(content.variant), |row, icon| {
                    row.child(
                        div()
                            .mt(px(2.))
                            .flex_none()
                            .child(gpui_kit::svg().data(icon).size(px(16.)).text_color(color)),
                    )
                })
                .child(text);
            let owner = cx.entity().downgrade();
            let escape_id = id.clone();
            let ring_color = match content.variant {
                ToastVariant::Default => t.colors.line,
                ToastVariant::Success => t.colors.success,
                ToastVariant::Error => t.colors.danger,
                ToastVariant::Warning => t.colors.warning,
                ToastVariant::Info => t.colors.info,
            };
            let desired = if content.variant == ToastVariant::Default {
                1.
            } else {
                0.3
            };
            // GPUI snaps shadows and borders to physical pixels. Preserve fractional
            // ring coverage without turning Kumo's .3px accent into an opaque 1px line.
            let scale = window.scale_factor();
            let physical = (desired * scale).ceil().max(1.);
            let ring_width = px(physical / scale);
            let ring_color = ring_color.opacity(desired * scale / physical);
            let root = div()
                .id(id.clone())
                .test_support()
                .role(Role::Dialog)
                .aria_label(name)
                .when_some(content.description.clone(), |root, description| {
                    root.aria_description(description)
                })
                .track_focus(&entry.focus)
                .tab_group()
                .relative()
                .w_full()
                .p(px(16.))
                .rounded(px(12.))
                .shadow(t.effects.shadow_lg.clone())
                .bg(if content.variant == ToastVariant::Info {
                    t.colors.control
                } else {
                    t.colors.base
                })
                .opacity(alpha)
                .occlude()
                .on_key_down(move |event, window, cx| {
                    if event.keystroke.key == "escape" {
                        let _ = owner.update(cx, |state, cx| {
                            state.close(&escape_id, ToastDismissReason::Escape, window, cx)
                        });
                        cx.stop_propagation();
                    }
                })
                .child(div().absolute().inset_0().rounded(px(12.)).bg(tint))
                .child(
                    div()
                        .relative()
                        .when(!expanded && index + 1 < entries.len(), |content| {
                            content.invisible()
                        })
                        .child(row)
                        .child(div().absolute().top(px(-8.)).right(px(-8.)).child(close)),
                )
                .child(
                    gpui_kit::canvas(
                        |_, _, _| (),
                        move |bounds, _, window, _| {
                            window.paint_quad(gpui_kit::quad(
                                bounds.dilate(ring_width),
                                px(12.) + ring_width,
                                ring_color.alpha(0.),
                                ring_width,
                                ring_color,
                                Default::default(),
                            ));
                        },
                    )
                    .absolute()
                    .inset_0(),
                );
            stack = stack.item(id.clone(), root);
        }
        let focus = self.focus.clone();
        let last = self.last_inside.clone();
        let region = div()
            .id("notifications")
            .test_support()
            .role(Role::Region)
            .aria_label("Notifications")
            .track_focus(&self.focus)
            .tab_group()
            .a11y_synthetic_children(|b| {
                b.parent_node().set_live(gpui_kit::accesskit::Live::Polite);
            })
            .font_family(t.typography.font_family.clone())
            .text_color(t.text.default)
            .child(stack)
            .child(
                gpui_kit::canvas(
                    |_, _, _| (),
                    move |_, _, window, cx| {
                        if focus.contains_focused(window, cx) {
                            *last.borrow_mut() = window.focused(cx).map(|focus| focus.downgrade());
                        }
                    },
                )
                .absolute()
                .size_0(),
            );
        gpui_kit::deferred(
            gpui_kit::anchored()
                .anchor(Anchor::BottomRight)
                .position_mode(gpui_kit::AnchoredPositionMode::Window)
                .position(gpui_kit::point(
                    viewport.width - margin,
                    viewport.height - margin,
                ))
                .child(region),
        )
        .with_priority(200)
        .into_any_element()
    }
}

impl std::fmt::Debug for Toast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_struct("Toast");
        debug.field("id", &self.id);
        debug.finish_non_exhaustive()
    }
}

impl std::fmt::Debug for ToastState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_struct("ToastState");
        debug.field("timer_running", &self.timer_running);
        debug.finish_non_exhaustive()
    }
}

impl std::fmt::Debug for ToastViewport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_struct("ToastViewport");
        debug.field("id", &self.id);
        debug.finish_non_exhaustive()
    }
}

fn validate_content(content: &ToastContent) {
    assert!(
        !content.title.trim().is_empty()
            || content
                .description
                .as_ref()
                .is_some_and(|text| !text.trim().is_empty()),
        "Toast requires readable content"
    );
    let mut keys = std::collections::HashSet::new();
    for action in &content.actions {
        assert!(
            !action.id.trim().is_empty() && !action.label.trim().is_empty(),
            "Toast actions require nonblank keys and names"
        );
        assert!(keys.insert(&action.id), "Toast action keys must be unique");
    }
}

#[cfg(test)]
mod tests;
