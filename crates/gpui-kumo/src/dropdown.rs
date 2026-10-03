//! Retained action menus over Base's unstyled popup positioning.
use crate::{Button, Theme, theme};
use gpui_kit::base::TestSupportExt;
use gpui_kit::{
    App, Context, ElementId, Entity, EventEmitter, FocusHandle, IntoElement, KeyDownEvent, Render,
    RenderOnce, Role, SharedString, Subscription, Window, base, div, prelude::*, px,
};
use std::{
    cell::Cell,
    collections::HashMap,
    rc::Rc,
    time::{Duration, Instant},
};

/// Kumo's two supported action-row treatments.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum DropdownVariant {
    #[default]
    Default,
    Danger,
}

/// Stable action identity is separate from its visible/typeahead label.
#[derive(Clone)]
pub struct DropdownItem {
    id: SharedString,
    label: SharedString,
    disabled: bool,
    variant: DropdownVariant,
    icon: Option<SharedString>,
    inset: bool,
    shortcut: Option<SharedString>,
    selected: bool,
}
impl DropdownItem {
    pub fn new(id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            disabled: false,
            variant: DropdownVariant::Default,
            icon: None,
            inset: false,
            shortcut: None,
            selected: false,
        }
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn variant(mut self, variant: DropdownVariant) -> Self {
        self.variant = variant;
        self
    }
    pub fn icon(mut self, path: impl Into<SharedString>) -> Self {
        self.icon = Some(path.into());
        self
    }
    pub fn inset(mut self, inset: bool) -> Self {
        self.inset = inset;
        self
    }
    /// Informational shortcut; binding the corresponding application action is the caller's responsibility.
    pub fn shortcut(mut self, shortcut: impl Into<SharedString>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

#[derive(Clone)]
pub enum DropdownPart {
    Item(DropdownItem),
    Label(SharedString),
    Separator,
}
impl From<DropdownItem> for DropdownPart {
    fn from(item: DropdownItem) -> Self {
        Self::Item(item)
    }
}

/// Notifications. Activation carries identity; the application owns the operation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DropdownEvent {
    OpenChanged(bool),
    Activated(SharedString),
}

/// Retain one state per mounted menu. No callback retains the application's owner.
pub struct DropdownState {
    name: SharedString,
    parts: Vec<DropdownPart>,
    handles: HashMap<SharedString, FocusHandle>,
    trigger: FocusHandle,
    content: FocusHandle,
    open: bool,
    disabled: bool,
    looping: bool,
    prefix: String,
    typed_at: Option<Instant>,
    deferred: Option<base::DeferredPopover>,
    label: SharedString,
    width: f32,
    _theme: Subscription,
    _tab_exit: Subscription,
    focus_out: Option<Subscription>,
    trigger_bounds: Rc<Cell<gpui_kit::Bounds<gpui_kit::Pixels>>>,
    mount: Option<gpui_kit::WeakEntity<Mount>>,
    scroll: gpui_kit::ScrollHandle,
}
impl EventEmitter<DropdownEvent> for DropdownState {}
impl DropdownState {
    pub fn new(
        name: impl Into<SharedString>,
        parts: Vec<DropdownPart>,
        cx: &mut Context<Self>,
    ) -> Self {
        let name = name.into();
        assert!(
            !name.trim().is_empty(),
            "Dropdown requires an accessible name"
        );
        let owner = cx.entity().downgrade();
        let mut state = Self {
            name,
            parts: vec![],
            handles: HashMap::new(),
            trigger: cx.focus_handle(),
            content: cx.focus_handle().tab_stop(false),
            open: false,
            disabled: false,
            looping: true,
            prefix: String::new(),
            typed_at: None,
            deferred: None,
            label: "Actions".into(),
            width: 220.,
            _theme: cx.observe_global::<Theme>(|_, cx| cx.notify()),
            _tab_exit: cx.intercept_keystrokes(move |event, window, cx| {
                let key = &event.keystroke;
                let m = key.modifiers;
                if key.key != "tab" || m.control || m.alt || m.platform || m.function {
                    return;
                }
                let Some(owner) = owner.upgrade() else {
                    return;
                };
                let state: &Self = owner.read(cx);
                if state.open
                    && (state.content.contains_focused(window, cx)
                        || state.handles.values().any(|h| h.is_focused(window)))
                {
                    owner.update(cx, |state, cx| {
                        state.set_open(false, window, cx);
                        state.trigger.focus(window, cx);
                        if !m.shift {
                            window.focus_next(cx);
                        }
                        cx.stop_propagation();
                    });
                }
            }),
            focus_out: None,
            trigger_bounds: Rc::new(Cell::new(Default::default())),
            mount: None,
            scroll: gpui_kit::ScrollHandle::new(),
        };
        state.replace(parts, cx);
        state
    }
    fn items(&self) -> impl DoubleEndedIterator<Item = &DropdownItem> {
        self.parts.iter().filter_map(|p| match p {
            DropdownPart::Item(i) => Some(i),
            _ => None,
        })
    }
    fn replace(&mut self, parts: Vec<DropdownPart>, cx: &mut Context<Self>) {
        let mut ids = std::collections::HashSet::new();
        for part in &parts {
            if let DropdownPart::Item(item) = part {
                assert!(
                    !item.id.is_empty() && !item.label.trim().is_empty(),
                    "Dropdown items require nonempty identity and label"
                );
                assert!(
                    ids.insert(item.id.clone()),
                    "Dropdown item IDs must be unique"
                );
                self.handles
                    .entry(item.id.clone())
                    .or_insert_with(|| cx.focus_handle().tab_stop(false));
            }
        }
        self.handles.retain(|id, _| ids.contains(id));
        self.parts = parts;
    }
    pub fn is_open(&self) -> bool {
        self.open
    }
    pub fn trigger_focus(&self) -> FocusHandle {
        self.trigger.clone()
    }
    pub fn set_parts(
        &mut self,
        parts: Vec<DropdownPart>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let focused = self
            .handles
            .iter()
            .find(|(_, h)| h.is_focused(window))
            .map(|(id, _)| id.clone());
        self.replace(parts, cx);
        if self.open && focused.is_some_and(|id| !self.items().any(|i| i.id == id)) {
            self.focus_edge(false, window, cx);
        }
        cx.notify();
    }
    pub fn set_disabled(&mut self, disabled: bool, window: &mut Window, cx: &mut Context<Self>) {
        self.disabled = disabled;
        if disabled {
            self.set_open(false, window, cx);
        }
        cx.notify();
    }
    pub fn set_looping(&mut self, looping: bool, cx: &mut Context<Self>) {
        self.looping = looping;
        cx.notify();
    }
    pub fn set_open(&mut self, open: bool, window: &mut Window, cx: &mut Context<Self>) {
        if open == self.open
            || (open
                && (self.disabled || self.mount.as_ref().is_some_and(|m| m.upgrade().is_none())))
        {
            return;
        }
        self.open = open;
        self.prefix.clear();
        self.typed_at = None;
        if open {
            self.deferred = Some(base::GlobalState::register_deferred_popover(cx));
            self.focus_edge(false, window, cx);
        } else {
            self.deferred = None;
            if self.content.contains_focused(window, cx)
                || self.handles.values().any(|h| h.is_focused(window))
            {
                self.trigger.focus(window, cx);
            }
        }
        cx.emit(DropdownEvent::OpenChanged(open));
        cx.notify();
    }
    fn focus_edge(&self, last: bool, window: &mut Window, cx: &mut Context<Self>) {
        let item = if last {
            self.items().next_back()
        } else {
            self.items().next()
        };
        item.map(|i| &self.handles[&i.id])
            .unwrap_or(&self.content)
            .focus(window, cx);
        self.reveal(window);
        cx.notify();
    }
    fn reveal(&self, window: &Window) {
        if let Some(index) = self.parts.iter().position(|part| match part {
            DropdownPart::Item(item) => self.handles[&item.id].is_focused(window),
            _ => false,
        }) {
            self.scroll.scroll_to_item(index);
        }
    }
    fn activate(&mut self, id: &SharedString, window: &mut Window, cx: &mut Context<Self>) {
        if !self.open || self.disabled || !self.items().any(|i| &i.id == id && !i.disabled) {
            return;
        }
        self.set_open(false, window, cx);
        cx.emit(DropdownEvent::Activated(id.clone()));
    }
    fn keys(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }
        let key = event.keystroke.key.as_str();
        let modifiers = event.keystroke.modifiers;
        if modifiers.control || modifiers.alt || modifiers.platform || modifiers.function {
            return;
        }
        if !self.open {
            if matches!(key, "down" | "up") {
                self.set_open(true, window, cx);
                self.focus_edge(key == "up", window, cx);
                cx.stop_propagation();
            }
            return;
        }
        if key == "escape" {
            self.set_open(false, window, cx);
            cx.stop_propagation();
            return;
        }
        if key == "tab" {
            self.set_open(false, window, cx);
            self.trigger.focus(window, cx);
            if !modifiers.shift {
                window.focus_next(cx);
            }
            cx.stop_propagation();
            return;
        }
        if matches!(key, "up" | "down" | "home" | "end") {
            if matches!(key, "home" | "end") {
                self.focus_edge(key == "end", window, cx);
            } else {
                let items: Vec<_> = self.items().collect();
                if !items.is_empty() {
                    let current = items
                        .iter()
                        .position(|i| self.handles[&i.id].is_focused(window));
                    let n = items.len();
                    let index = match (current, key == "down") {
                        (None, true) => 0,
                        (None, false) => n - 1,
                        (Some(i), true) => {
                            if self.looping {
                                (i + 1) % n
                            } else {
                                (i + 1).min(n - 1)
                            }
                        }
                        (Some(i), false) => {
                            if self.looping {
                                (i + n - 1) % n
                            } else {
                                i.saturating_sub(1)
                            }
                        }
                    };
                    self.handles[&items[index].id].focus(window, cx);
                    self.reveal(window);
                    cx.notify();
                }
            }
            cx.stop_propagation();
            return;
        }
        // Enter/Space activation is supplied once by GPUI's click path.
        let typing = self.typed_at.is_some_and(|t| {
            cx.background_executor().now().saturating_duration_since(t) < Duration::from_millis(500)
        });
        if key == "enter" || (key == "space" && !typing) {
            return;
        }
        let typed = if key == "space" {
            " "
        } else {
            event.keystroke.key_char.as_deref().unwrap_or(key)
        };
        if typed.chars().count() != 1 || typed.chars().any(char::is_control) {
            return;
        }
        let now = cx.background_executor().now();
        if self
            .typed_at
            .is_none_or(|t| now.saturating_duration_since(t) > Duration::from_millis(500))
        {
            self.prefix.clear();
        }
        self.typed_at = Some(now);
        self.prefix.push_str(&typed.to_lowercase());
        let repeated = self.prefix.chars().all(|c| self.prefix.starts_with(c));
        let query = if repeated {
            typed.to_lowercase()
        } else {
            self.prefix.clone()
        };
        let items: Vec<_> = self.items().collect();
        let start = items
            .iter()
            .position(|i| self.handles[&i.id].is_focused(window))
            .map_or(0, |i| if repeated { i + 1 } else { i });
        if let Some(item) = (0..items.len())
            .map(|n| items[(start + n) % items.len()])
            .find(|i| i.label.to_lowercase().starts_with(&query))
        {
            self.handles[&item.id].focus(window, cx);
            self.reveal(window);
            cx.notify();
        }
        cx.stop_propagation();
    }
}

#[derive(IntoElement)]
pub struct Dropdown {
    id: ElementId,
    state: Entity<DropdownState>,
    label: SharedString,
    width: f32,
}
impl Dropdown {
    pub fn new(
        id: impl Into<ElementId>,
        state: &Entity<DropdownState>,
        label: impl Into<SharedString>,
    ) -> Self {
        Self {
            id: id.into(),
            state: state.clone(),
            label: label.into(),
            width: 220.,
        }
    }
    /// Native content width, clamped to the viewport; Kumo minimum is 144px.
    pub fn width(mut self, width: gpui_kit::Pixels) -> Self {
        let width = f32::from(width);
        assert!(width.is_finite() && width >= 144.);
        self.width = width;
        self
    }
}
impl RenderOnce for Dropdown {
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
                            let mut removed_focus = vec![];
                            let _ = owner.update(cx, |state, cx| {
                                if state.mount.as_ref().and_then(|m| m.upgrade()).is_some() {
                                    return;
                                }
                                removed_focus.extend(state.handles.values().cloned());
                                removed_focus.push(state.trigger.clone());
                                removed_focus.push(state.content.clone());
                                state.open = false;
                                state.deferred = None;
                                state.prefix.clear();
                                state.typed_at = None;
                                cx.notify();
                            });
                            let _ = handle.update(cx, |_, window, cx| {
                                if removed_focus.iter().any(|h| h.is_focused(window)) {
                                    window.blur(cx);
                                }
                            });
                        });
                    })
                    .detach();
            })),
        });
        self.state.update(cx, |state, _| {
            state.label = self.label;
            state.width = self.width;
            state.mount = Some(mount.downgrade());
        });
        div().id(self.id).flex().flex_none().child(self.state)
    }
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
impl Render for DropdownState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.focus_out.is_none() {
            self.focus_out =
                Some(
                    cx.on_focus_out(&self.content.clone(), window, |state, _, window, cx| {
                        if state.open
                            && !state.content.contains_focused(window, cx)
                            && !state.handles.values().any(|h| h.is_focused(window))
                        {
                            state.set_open(false, window, cx);
                        }
                    }),
                );
        }
        let t = theme(cx).clone();
        let trigger = Button::new("trigger", self.label.clone())
            .track_focus(&self.trigger)
            .disabled(self.disabled)
            .open(self.open)
            .menu_expanded(self.open)
            .on_click(
                cx.listener(|state, event: &gpui_kit::ClickEvent, window, cx| {
                    state.set_open(!state.open, window, cx);
                    if state.open && !matches!(event, gpui_kit::ClickEvent::Keyboard(_)) {
                        state.content.focus(window, cx);
                    }
                }),
            );
        let bounds = self.trigger_bounds.clone();
        let root = base::Popup::new("popup", trigger)
            .anchor(gpui_kit::Anchor::TopCenter)
            .offset(px(8.))
            .on_position(move |_, trigger| bounds.set(trigger))
            .on_key_down(cx.listener(Self::keys));
        if !self.open {
            return root.into_any_element();
        }
        let mut surface = div()
            .id("menu")
            .test_support()
            .role(Role::Menu)
            .aria_label(self.name.clone())
            .track_focus(&self.content)
            .on_key_down(cx.listener(Self::keys))
            .on_mouse_down_out(cx.listener(
                |state, event: &gpui_kit::MouseDownEvent, window, cx| {
                    if !state.trigger_bounds.get().contains(&event.position) {
                        state.set_open(false, window, cx);
                    }
                },
            ))
            .relative()
            .flex()
            .flex_col()
            .w(px(self.width).min((window.viewport_size().width - px(16.)).max(px(1.))))
            .max_h((window.viewport_size().height - px(32.)).max(px(1.)))
            .overflow_y_scroll()
            .track_scroll(&self.scroll)
            .p(px(6.))
            .rounded(t.radii.lg)
            .bg(t.colors.control)
            .shadow(t.effects.shadow_lg.clone())
            .text_color(t.text.default)
            .font_family(t.typography.font_family.clone())
            .text_size(t.typography.base.size)
            .line_height(t.typography.base.line_height);
        for (index, part) in self.parts.iter().enumerate() {
            surface = match part {
                DropdownPart::Separator => surface.child(
                    div()
                        .id(("separator", index))
                        .role(Role::Splitter)
                        .mx(-px(4.))
                        .my(px(4.))
                        .h(px(1.))
                        .flex_shrink_0()
                        .bg(t.colors.hairline),
                ),
                DropdownPart::Label(label) => surface.child(
                    div()
                        .id(("label", index))
                        .px(px(8.))
                        .py(px(6.))
                        .flex_shrink_0()
                        .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                        .font_family(t.typography.font_family.clone())
                        .role(Role::Label)
                        .aria_label(label.clone())
                        .aria_value(label.clone())
                        .child(label.clone()),
                ),
                DropdownPart::Item(item) => {
                    let handle = &self.handles[&item.id];
                    let focused = handle.is_focused(window);
                    let id = item.id.clone();
                    let hover_id = id.clone();
                    let danger = item.variant == DropdownVariant::Danger;
                    let foreground = if danger {
                        t.text.danger
                    } else {
                        t.text.default
                    };
                    let background = if danger {
                        t.colors.danger.opacity(0.05)
                    } else {
                        t.colors.overlay
                    };
                    let row = div()
                        .id(item.id.clone())
                        .test_support()
                        .role(Role::MenuItem)
                        .aria_label(item.label.clone())
                        .track_focus(handle)
                        .a11y_synthetic_children({
                            let disabled = item.disabled;
                            move |builder| {
                                if disabled {
                                    builder.parent_node().set_disabled();
                                }
                            }
                        })
                        .on_mouse_down(
                            gpui_kit::MouseButton::Left,
                            cx.listener({
                                let id = item.id.clone();
                                move |state, _, window, cx| {
                                    if state.items().any(|i| i.id == id && !i.disabled) {
                                        state.handles[&id].focus(window, cx);
                                        cx.notify();
                                    }
                                }
                            }),
                        )
                        .on_click(cx.listener(move |state, event: &gpui_kit::ClickEvent, window, cx| {
                            if matches!(event, gpui_kit::ClickEvent::Keyboard(e) if e.button == gpui_kit::KeyboardButton::Space)
                                && state.typed_at.is_some_and(|t| cx.background_executor().now().saturating_duration_since(t) < Duration::from_millis(500)) {
                                cx.stop_propagation(); return;
                            }
                            state.activate(&id, window, cx);
                            cx.stop_propagation();
                        }))
                        .on_hover(cx.listener(move |state, hovered, window, cx| {
                            if *hovered
                                && state.open
                                && state.items().any(|i| i.id == hover_id && !i.disabled)
                            {
                                state.handles[&hover_id].focus(window, cx);
                                cx.notify();
                            }
                        }))
                        .flex()
                        .flex_shrink_0()
                        .items_center()
                        .px(px(8.))
                        .py(px(6.))
                        .rounded(t.radii.md)
                        .text_color(foreground)
                        .when(item.disabled, |row| row.opacity(0.5))
                        .when(focused, |row| row.bg(background))
                        .when(focused && window.last_input_was_keyboard(), |row| {
                            row.border_2()
                                .border_color(t.colors.brand)
                                .px(px(6.))
                                .py(px(4.))
                        })
                        .when(item.inset && item.icon.is_none(), |row| row.pl(px(32.)))
                        .when_some(item.icon.clone(), |row, icon| {
                            row.child(div().mr(px(8.)).child(crate::Icon::new(icon)))
                        })
                        .child(item.label.clone())
                        .when(item.selected, |row| row.child("✓"))
                        .when_some(item.shortcut.clone(), |row, shortcut| {
                            row.child(
                                div()
                                    .ml_auto()
                                    .pl(px(12.))
                                    .text_size(px(12.))
                                    .opacity(0.6)
                                    .child(shortcut),
                            )
                        });
                    surface.child(row)
                }
            };
        }
        let line = t.colors.line;
        let radius = t.radii.lg;
        surface = surface.child(
            gpui_kit::canvas(
                |_, _, _| (),
                move |bounds, _, window, _| {
                    window.paint_quad(gpui_kit::quad(
                        bounds.dilate(px(1.)),
                        radius + px(1.),
                        line.opacity(0.),
                        px(1.),
                        line,
                        Default::default(),
                    ));
                },
            )
            .absolute()
            .top_0()
            .left_0()
            .size_full(),
        );
        let backdrop = gpui_kit::deferred(
            gpui_kit::anchored()
                .position(gpui_kit::point(px(0.), px(0.)))
                .child(
                    div()
                        .id("dismissal-backdrop")
                        .w(window.viewport_size().width)
                        .h(window.viewport_size().height)
                        .occlude()
                        .on_mouse_down(
                            gpui_kit::MouseButton::Left,
                            cx.listener(|state, _, window, cx| {
                                state.set_open(false, window, cx);
                                cx.stop_propagation();
                            }),
                        ),
                ),
        )
        .with_priority(base::POPUP_PRIORITY - 1);
        div()
            .flex()
            .flex_none()
            .child(backdrop)
            .child(root.content(surface))
            .into_any_element()
    }
}

#[cfg(test)]
mod tests;
