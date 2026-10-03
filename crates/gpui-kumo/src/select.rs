//! Retained typed Kumo Select over Base disclosure and deferred positioning.
#[path = "select_semantics.rs"]
mod semantics;
#[cfg(test)]
#[path = "select_tests.rs"]
mod tests;
pub use crate::input::Size;
use crate::{Theme, theme};
use gpui_kit::base::TestSupportExt;
use gpui_kit::{
    AnyElement, App, Bounds, Context, ElementId, Entity, EventEmitter, FocusHandle, FontWeight,
    InteractiveElement, IntoElement, KeyBinding, KeyDownEvent, MouseButton, ParentElement, Pixels,
    Render, RenderOnce, Role, ScrollHandle, SharedString, StatefulInteractiveElement, Styled,
    Subscription, Window, base, canvas, deferred, div, prelude::FluentBuilder, px, svg,
};
use std::{
    cell::Cell,
    rc::Rc,
    time::{Duration, Instant},
};

pub(crate) fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("up", base::actions::SelectUp, Some("KumoSelect")),
        KeyBinding::new("down", base::actions::SelectDown, Some("KumoSelect")),
        KeyBinding::new(
            "enter",
            base::actions::Confirm { secondary: false },
            Some("KumoSelect"),
        ),
        KeyBinding::new("escape", base::actions::Cancel, Some("KumoSelect")),
    ]);
}

/// Single and multiple values have explicit, immutable selection modes.
#[derive(Clone, Debug, PartialEq)]
pub enum SelectValue<T> {
    Single(Option<T>),
    Multiple(Vec<T>),
}
type ContentFactory = Rc<dyn Fn(&mut Window, &mut App) -> AnyElement>;
type Comparator<T> = Rc<dyn Fn(&T, &T) -> bool>;
/// A named option with identity independent of its value or current order.
#[derive(Clone)]
pub struct SelectOption<T> {
    id: ElementId,
    value: T,
    label: SharedString,
    disabled: bool,
    content: Option<ContentFactory>,
}
impl<T> SelectOption<T> {
    pub fn new(id: impl Into<ElementId>, value: T, label: impl Into<SharedString>) -> Self {
        let label = label.into();
        assert!(!label.trim().is_empty(), "Select option requires a name");
        Self {
            id: id.into(),
            value,
            label,
            disabled: false,
            content: None,
        }
    }
    /// Decorative option content; name stays complete. Retain child entities outside this factory.
    /// Capture Select/owner entities weakly: the state retains this closure, so
    /// capturing either strongly can form a reference cycle. For example,
    /// create `let owner = cx.entity().downgrade()` before the factory and
    /// upgrade it only while building content. Keep interactive controls outside options.
    pub fn content(
        mut self,
        content: impl Fn(&mut Window, &mut App) -> AnyElement + 'static,
    ) -> Self {
        self.content = Some(Rc::new(content));
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}
/// Value proposals emitted after activation; programmatic setters emit nothing.
#[derive(Clone, Debug)]
pub struct SelectEvent<T> {
    pub value: SelectValue<T>,
}
/// One retained owner for selection, collection, highlight and popup lifecycle.
/// Subscribe to `SelectEvent` to accept controlled proposals with `set_value`.
pub struct SelectState<T: Clone + PartialEq + 'static> {
    name: SharedString,
    options: Vec<SelectOption<T>>,
    value: SelectValue<T>,
    compare: Comparator<T>,
    controlled: bool,
    disabled: bool,
    read_only: bool,
    open: bool,
    highlighted: Option<ElementId>,
    trigger: FocusHandle,
    content: FocusHandle,
    bounds: Rc<Cell<Bounds<Pixels>>>,
    popup_bounds: Rc<Cell<Bounds<Pixels>>>,
    scroll: ScrollHandle,
    deferred: Option<base::DeferredPopover>,
    prefix: String,
    typed_at: Option<Instant>,
    size: Size,
    placeholder: SharedString,
    invalid: bool,
    description: Option<SharedString>,
    loading: bool,
    hovered: bool,
    _theme: Subscription,
    focus_out: Option<Subscription>,
}
impl<T: Clone + PartialEq + 'static> EventEmitter<SelectEvent<T>> for SelectState<T> {}
impl<T: Clone + PartialEq + 'static> SelectState<T> {
    pub fn new(
        name: impl Into<SharedString>,
        value: SelectValue<T>,
        options: Vec<SelectOption<T>>,
        cx: &mut Context<Self>,
    ) -> Self {
        let name = name.into();
        assert!(
            !name.trim().is_empty(),
            "Select requires an accessible name"
        );
        Self::check_ids(&options);
        Self {
            name,
            options,
            value,
            compare: Rc::new(|a, b| a == b),
            controlled: false,
            disabled: false,
            read_only: false,
            open: false,
            highlighted: None,
            trigger: cx.focus_handle(),
            content: cx.focus_handle().tab_stop(false),
            bounds: Rc::new(Cell::new(Bounds::default())),
            popup_bounds: Rc::new(Cell::new(Bounds::default())),
            scroll: ScrollHandle::new(),
            deferred: None,
            prefix: String::new(),
            typed_at: None,
            size: Size::Base,
            placeholder: "Select…".into(),
            invalid: false,
            description: None,
            loading: false,
            hovered: false,
            _theme: cx.observe_global::<Theme>(|_, cx| cx.notify()),
            focus_out: None,
        }
    }
    fn check_ids(options: &[SelectOption<T>]) {
        for (i, o) in options.iter().enumerate() {
            assert!(
                !options[..i].iter().any(|p| p.id == o.id),
                "Select option IDs must be unique"
            );
        }
    }
    /// Override logical equality for object values without relying on allocation identity.
    /// Prefer a pure comparator. This retained closure must capture Select/owner
    /// entities weakly if it needs application context, to avoid reference cycles.
    pub fn set_comparator(
        &mut self,
        compare: impl Fn(&T, &T) -> bool + 'static,
        cx: &mut Context<Self>,
    ) {
        self.compare = Rc::new(compare);
        cx.notify();
    }
    pub fn value(&self) -> &SelectValue<T> {
        &self.value
    }
    pub fn is_open(&self) -> bool {
        self.open
    }
    pub fn is_disabled(&self) -> bool {
        self.disabled
    }
    pub fn is_read_only(&self) -> bool {
        self.read_only
    }
    pub fn focus_handle(&self) -> FocusHandle {
        self.trigger.clone()
    }
    pub fn set_controlled(&mut self, controlled: bool, cx: &mut Context<Self>) {
        self.controlled = controlled;
        cx.notify();
    }
    pub fn set_value(&mut self, value: SelectValue<T>, cx: &mut Context<Self>) {
        assert!(
            matches!(
                (&self.value, &value),
                (SelectValue::Single(_), SelectValue::Single(_))
                    | (SelectValue::Multiple(_), SelectValue::Multiple(_))
            ),
            "Select mode cannot change"
        );
        self.value = value;
        cx.notify();
    }
    pub fn set_options(&mut self, options: Vec<SelectOption<T>>, cx: &mut Context<Self>) {
        Self::check_ids(&options);
        self.options = options;
        if !self
            .options
            .iter()
            .any(|o| Some(&o.id) == self.highlighted.as_ref() && !o.disabled)
        {
            self.highlighted = self
                .options
                .iter()
                .find(|o| !o.disabled)
                .map(|o| o.id.clone());
        }
        self.reveal();
        cx.notify();
    }
    pub fn set_disabled(&mut self, disabled: bool, window: &mut Window, cx: &mut Context<Self>) {
        self.disabled = disabled;
        self.trigger.clone().tab_stop(!disabled);
        if disabled {
            self.set_open(false, window, cx);
        }
        cx.notify();
    }
    /// Read-only permits browsing and dismissal while rejecting value changes.
    pub fn set_read_only(&mut self, read_only: bool, cx: &mut Context<Self>) {
        self.read_only = read_only;
        cx.notify();
    }
    pub fn set_open(&mut self, open: bool, window: &mut Window, cx: &mut Context<Self>) {
        if self.open == open || (open && self.unavailable()) {
            return;
        }
        self.open = open;
        self.prefix.clear();
        self.typed_at = None;
        if open {
            self.highlighted = self
                .options
                .iter()
                .find(|o| !o.disabled && self.selected(&o.value))
                .or_else(|| self.options.iter().find(|o| !o.disabled))
                .map(|o| o.id.clone());
            self.deferred = Some(base::GlobalState::register_deferred_popover(cx));
            self.content.focus(window, cx);
            self.reveal();
        } else {
            self.deferred = None;
            if self.content.contains_focused(window, cx) {
                self.trigger.focus(window, cx);
            }
        }
        cx.notify();
    }
    fn unavailable(&self) -> bool {
        self.disabled || self.loading
    }
    fn selected(&self, value: &T) -> bool {
        match &self.value {
            SelectValue::Single(v) => v.as_ref().is_some_and(|v| (self.compare)(v, value)),
            SelectValue::Multiple(v) => v.iter().any(|v| (self.compare)(v, value)),
        }
    }
    fn reveal(&self) {
        if let Some(i) = self
            .options
            .iter()
            .position(|o| Some(&o.id) == self.highlighted.as_ref())
        {
            self.scroll.scroll_to_item(i);
        }
    }
    fn move_highlight(&mut self, delta: isize, cx: &mut Context<Self>) {
        let enabled: Vec<_> = self
            .options
            .iter()
            .filter(|o| !o.disabled)
            .map(|o| o.id.clone())
            .collect();
        if enabled.is_empty() {
            return;
        }
        let current = enabled
            .iter()
            .position(|id| Some(id) == self.highlighted.as_ref())
            .unwrap_or(0);
        let next = (current as isize + delta).clamp(0, enabled.len() as isize - 1) as usize;
        self.highlighted = Some(enabled[next].clone());
        self.reveal();
        cx.notify();
    }
    fn commit(&mut self, id: &ElementId, window: &mut Window, cx: &mut Context<Self>) {
        if !self.open {
            return;
        }
        self.propose(id, window, cx);
    }
    fn propose(&mut self, id: &ElementId, window: &mut Window, cx: &mut Context<Self>) {
        if self.unavailable() || self.read_only {
            return;
        }
        let Some(option) = self.options.iter().find(|o| &o.id == id && !o.disabled) else {
            return;
        };
        let next = match &self.value {
            SelectValue::Single(_) => SelectValue::Single(Some(option.value.clone())),
            SelectValue::Multiple(values) => {
                let mut values = values.clone();
                if let Some(i) = values.iter().position(|v| (self.compare)(v, &option.value)) {
                    values.remove(i);
                } else {
                    values.push(option.value.clone());
                }
                SelectValue::Multiple(values)
            }
        };
        self.highlighted = Some(id.clone());
        let changed = !match (&next, &self.value) {
            (SelectValue::Single(None), SelectValue::Single(None)) => true,
            (SelectValue::Single(Some(a)), SelectValue::Single(Some(b))) => (self.compare)(a, b),
            (SelectValue::Multiple(a), SelectValue::Multiple(b)) => {
                a.len() == b.len() && a.iter().zip(b).all(|(a, b)| (self.compare)(a, b))
            }
            _ => false,
        };
        if !self.controlled && changed {
            self.value = next.clone();
        }
        if matches!(next, SelectValue::Single(_)) {
            self.set_open(false, window, cx);
        }
        if changed {
            cx.emit(SelectEvent { value: next });
        }
        cx.notify();
    }
    fn confirm(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(id) = self.highlighted.clone() {
            self.commit(&id, window, cx);
        }
        cx.stop_propagation();
    }
    fn keys(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        if self.unavailable() {
            return;
        }
        let key = event.keystroke.key.as_str();
        if key == "tab" && self.open {
            self.set_open(false, window, cx);
            self.trigger.focus(window, cx);
            if event.keystroke.modifiers.shift {
                window.focus_prev(cx);
            } else {
                window.focus_next(cx);
            }
            cx.stop_propagation();
            return;
        }
        if key == "space" {
            if self.open
                && self.typed_at.is_some_and(|t| {
                    cx.background_executor().now().saturating_duration_since(t)
                        < Duration::from_millis(500)
                })
            {
                self.typeahead(" ", window, cx);
                cx.stop_propagation();
                return;
            }
            if !event.is_held {
                if self.open {
                    self.confirm(window, cx);
                } else {
                    self.set_open(true, window, cx);
                }
            }
            cx.stop_propagation();
            return;
        }
        if self.open && (key == "home" || key == "end") {
            self.highlighted = if key == "home" {
                self.options.iter().find(|o| !o.disabled)
            } else {
                self.options.iter().rev().find(|o| !o.disabled)
            }
            .map(|o| o.id.clone());
            self.reveal();
            cx.notify();
            cx.stop_propagation();
            return;
        }
        if event.keystroke.modifiers.control
            || event.keystroke.modifiers.alt
            || event.keystroke.modifiers.platform
        {
            return;
        }
        let typed = event.keystroke.key_char.as_deref().unwrap_or(key);
        if typed.chars().count() != 1 || typed.chars().any(char::is_control) {
            return;
        }
        if !self.open && (self.read_only || matches!(self.value, SelectValue::Multiple(_))) {
            return;
        }
        self.typeahead(typed, window, cx);
        cx.stop_propagation();
    }
    fn typeahead(&mut self, typed: &str, window: &mut Window, cx: &mut Context<Self>) {
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
        let start = self
            .options
            .iter()
            .position(|o| Some(&o.id) == self.highlighted.as_ref())
            .map_or(0, |i| if repeated { i + 1 } else { i });
        let found = (0..self.options.len())
            .map(|n| (start + n) % self.options.len())
            .find(|i| {
                !self.options[*i].disabled
                    && self.options[*i].label.to_lowercase().starts_with(&query)
            });
        if let Some(i) = found {
            let id = self.options[i].id.clone();
            if !self.open {
                self.highlighted = Some(id.clone());
                self.propose(&id, window, cx);
            } else {
                self.highlighted = Some(id);
                self.reveal();
                cx.notify();
            }
        }
        cx.stop_propagation();
    }
    fn display(&self) -> SharedString {
        self.options
            .iter()
            .filter(|o| self.selected(&o.value))
            .map(|o| o.label.as_ref())
            .collect::<Vec<_>>()
            .join(", ")
            .into()
    }
}

/// Mount a retained state once; configuration owns presentation only.
#[derive(IntoElement)]
pub struct Select<T: Clone + PartialEq + 'static> {
    id: ElementId,
    state: Entity<SelectState<T>>,
    size: Size,
    placeholder: SharedString,
    invalid: bool,
    loading: bool,
    label: bool,
    required: bool,
    description: Option<SharedString>,
    error: Option<(SharedString, bool)>,
}
impl<T: Clone + PartialEq + 'static> Select<T> {
    pub fn new(id: impl Into<ElementId>, state: &Entity<SelectState<T>>) -> Self {
        Self {
            id: id.into(),
            state: state.clone(),
            size: Size::Base,
            placeholder: "Select…".into(),
            invalid: false,
            loading: false,
            label: false,
            required: true,
            description: None,
            error: None,
        }
    }
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }
    pub fn label(mut self, label: bool) -> Self {
        self.label = label;
        self
    }
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }
    pub fn error(mut self, error: impl Into<SharedString>, show: bool) -> Self {
        self.error = Some((error.into(), show));
        self
    }
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }
}
impl<T: Clone + PartialEq + 'static> RenderOnce for Select<T> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        self.state.update(cx, |v, cx| {
            v.size = self.size;
            v.placeholder = self.placeholder;
            v.invalid = self.invalid || self.error.is_some();
            v.description =
                crate::field::resolve_message(self.description.clone(), self.error.clone())
                    .map(|(text, _)| text);
            v.loading = self.loading;
            if v.loading {
                v.set_open(false, window, cx);
            }
        });
        let name = self.state.read(cx).name.clone();
        crate::Field::new(self.id, name, self.state)
            .hide_label(!self.label)
            .required(self.required)
            .when_some(self.description, |v, d| v.description(d))
            .when_some(self.error, |v, (e, show)| v.error(e, show))
    }
}
impl<T: Clone + PartialEq + 'static> SelectState<T> {
    fn trigger_element(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl gpui_kit::Element {
        let t = theme(cx).clone();
        let (height, padding, radius, style) = crate::input::metrics(self.size, &t);
        let display = self.display();
        let empty = display.is_empty();
        let open = self.open;
        let disabled = self.unavailable();
        let bounds = self.bounds.clone();
        let owner = cx.entity().downgrade();
        let open_owner = cx.entity().downgrade();
        let confirm_owner = open_owner.clone();
        let ring_focus = self.trigger.clone();
        let invalid = self.invalid;
        let colors = t.colors.clone();
        let trigger = base::Select::new("trigger")
            .open(open)
            .disabled(disabled)
            .focus_handle(&self.trigger)
            .content_focus_handle(&self.content)
            .key_context("KumoSelect")
            .accessibility_label(self.name.clone())
            .accessibility_value(display.clone())
            .on_open_change(move |open, window, cx| {
                let _ = open_owner.update(cx, |v, cx| v.set_open(open, window, cx));
            })
            .on_confirm(move |window, cx| {
                let _ = confirm_owner.update(cx, |v, cx| v.confirm(window, cx));
            })
            .relative()
            .w_full()
            .h(px(height))
            .rounded(radius)
            .bg(if disabled {
                t.colors.control.opacity(0.5)
            } else if self.hovered && !open {
                t.colors.tint
            } else {
                t.colors.control
            })
            .opacity(if self.disabled { 0.5 } else { 1. })
            .shadow(t.effects.shadow_xs.clone())
            .text_color(if disabled {
                t.text.default.opacity(0.7)
            } else if empty {
                t.text.placeholder
            } else {
                t.text.default
            })
            .font_family(t.typography.font_family.clone())
            .font_weight(FontWeight::NORMAL)
            .text_size(style.size)
            .line_height(style.line_height)
            .child(
                div()
                    .id("hit")
                    .test_support()
                    .flex()
                    .items_center()
                    .justify_between()
                    .size_full()
                    .px(padding)
                    .gap(px(8.))
                    .when(!disabled, |v| v.cursor_pointer())
                    .on_hover(cx.listener(|v, hovered, _, cx| {
                        v.hovered = *hovered;
                        cx.notify();
                    }))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|v, _, window, cx| {
                            if !v.unavailable() {
                                v.set_open(!v.open, window, cx);
                                if !v.open {
                                    v.trigger.focus(window, cx);
                                }
                                cx.stop_propagation();
                            }
                        }),
                    )
                    .child(
                        div()
                            .id("value")
                            .min_w_0()
                            .flex_1()
                            .text_ellipsis()
                            .when(!self.loading, |v| {
                                v.child(if empty {
                                    self.placeholder.clone()
                                } else {
                                    display
                                })
                            })
                            .when(self.loading, |v| {
                                v.child(div().w(px(128.)).max_w_full().child(
                                    crate::SkeletonLine::new("loading").width_range(100..=100),
                                ))
                            }),
                    )
                    .child(
                        svg()
                            .data(include_bytes!("../assets/caret-up-down.svg").as_slice())
                            .size(px(match self.size {
                                Size::Xs => 12.,
                                Size::Sm => 14.,
                                Size::Base => 16.,
                                Size::Lg => 18.,
                            }))
                            .flex_shrink_0()
                            .text_color(t.text.subtle),
                    ),
            )
            .child(
                canvas(
                    move |b, window, cx| {
                        if bounds.replace(b) != b {
                            let _ = owner.update(cx, |_, cx| cx.notify());
                            window.request_animation_frame();
                        }
                    },
                    move |bounds, _, window, _| {
                        let focused = !disabled && ring_focus.is_focused(window);
                        let keyboard = focused && window.last_input_was_keyboard();
                        let width = if focused && invalid {
                            px(1.5)
                        } else if keyboard {
                            px(2.)
                        } else {
                            px(1.)
                        };
                        let color = if invalid {
                            if focused {
                                colors.danger.opacity(0.5)
                            } else {
                                colors.danger
                            }
                        } else if focused {
                            colors.focus.opacity(0.5)
                        } else {
                            colors.line
                        };
                        window.paint_quad(gpui_kit::quad(
                            if keyboard {
                                bounds
                            } else {
                                bounds.dilate(width)
                            },
                            if keyboard { radius } else { radius + width },
                            color.alpha(0.),
                            width,
                            color,
                            Default::default(),
                        ));
                    },
                )
                .absolute()
                .inset_0()
                .size_full(),
            );
        semantics::Control {
            inner: trigger.render(window, cx).into_element(),
            disabled,
            read_only: self.read_only,
            invalid: self.invalid,
            description: self.description.clone(),
        }
    }
}
impl<T: Clone + PartialEq + 'static> Render for SelectState<T> {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.focus_out.is_none() {
            self.focus_out =
                Some(
                    cx.on_focus_out(&self.content.clone(), window, |v, _, window, cx| {
                        if v.open
                            && !v.trigger.is_focused(window)
                            && !v.content.contains_focused(window, cx)
                        {
                            v.set_open(false, window, cx);
                        }
                    }),
                );
        }
        let t = theme(cx).clone();
        let open = self.open;
        let trigger = self.trigger_element(window, cx);
        let root = div()
            .relative()
            .w_full()
            .min_w_0()
            .key_context("KumoSelect")
            .capture_any_mouse_down(cx.listener(|v, _, window, cx| {
                if v.unavailable() {
                    window.prevent_default();
                    cx.stop_propagation();
                }
            }))
            .on_key_down(cx.listener(Self::keys))
            .on_action(cx.listener(move |v, _: &base::actions::SelectDown, _, cx| {
                if open && !v.unavailable() {
                    v.move_highlight(1, cx);
                }
                cx.stop_propagation();
            }))
            .on_action(cx.listener(move |v, _: &base::actions::SelectUp, _, cx| {
                if open && !v.unavailable() {
                    v.move_highlight(-1, cx);
                }
                cx.stop_propagation();
            }))
            .on_action(
                cx.listener(move |v, _: &base::actions::Confirm, window, cx| {
                    if open {
                        v.confirm(window, cx);
                    } else {
                        cx.stop_propagation();
                    }
                }),
            )
            .on_action(cx.listener(|v, _: &base::actions::Cancel, window, cx| {
                if v.open {
                    v.set_open(false, window, cx);
                    cx.stop_propagation();
                }
            }))
            .child(trigger);
        if !open || self.bounds.get().size.width == px(0.) {
            return root.into_any_element();
        }
        let popup_bounds = self.popup_bounds.clone();
        let trigger_bounds = self.bounds.get();
        let available = (window.viewport_size().height - trigger_bounds.bottom() - px(12.))
            .max(trigger_bounds.top() - px(12.))
            .max(px(1.));
        let mut list = div()
            .id("list")
            .test_support()
            .role(Role::ListBox)
            .aria_label(self.name.clone())
            .when(matches!(self.value, SelectValue::Multiple(_)), |v| {
                v.a11y_synthetic_children(|b| b.parent_node().set_multiselectable())
            })
            .track_focus(&self.content)
            .track_scroll(&self.scroll)
            .overflow_y_scroll()
            .flex()
            .flex_col()
            .max_h((available - px(14.)).max(px(1.)));
        for option in &self.options {
            let id = option.id.clone();
            let select_id = id.clone();
            let hover_id = id.clone();
            let accessible_id = id.clone();
            let selected = self.selected(&option.value);
            let unavailable = option.disabled;
            let highlighted = self.highlighted.as_ref() == Some(&id);
            list = list.child(
                div()
                    .id(id)
                    .test_support()
                    .role(Role::ListBoxOption)
                    .aria_label(option.label.clone())
                    .aria_selected(selected)
                    .a11y_synthetic_children(move |builder| {
                        if unavailable {
                            builder.parent_node().set_disabled();
                        }
                    })
                    .when(highlighted, |v| v.aria_active_descendant())
                    .relative()
                    .flex()
                    .items_center()
                    .gap(px(8.))
                    .mx(px(6.))
                    .px(px(8.))
                    .py(px(6.))
                    .rounded(px(4.))
                    .bg(if highlighted {
                        t.colors.tint
                    } else {
                        t.colors.base
                    })
                    .opacity(if unavailable { 0.5 } else { 1. })
                    .on_hover(cx.listener(move |v, hovered, _, cx| {
                        if *hovered && !unavailable {
                            v.highlighted = Some(hover_id.clone());
                            cx.notify();
                        }
                    }))
                    .when(!unavailable, |v| {
                        let owner = cx.entity().downgrade();
                        v.on_a11y_action(gpui_kit::AccessibleAction::Click, move |_, window, cx| {
                            let _ = owner.update(cx, |v, cx| v.commit(&accessible_id, window, cx));
                        })
                    })
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |v, _, window, cx| {
                            v.commit(&select_id, window, cx);
                            cx.stop_propagation();
                        }),
                    )
                    .child(
                        div()
                            .min_w_0()
                            .flex_1()
                            .child(if let Some(factory) = &option.content {
                                factory(window, cx)
                            } else {
                                div().child(option.label.clone()).into_any_element()
                            }),
                    )
                    .child(div().size(px(14.)).flex_shrink_0().when(selected, |v| {
                        v.child(
                            svg()
                                .data(include_bytes!("../assets/empty-check.svg").as_slice())
                                .size_full()
                                .text_color(t.text.default),
                        )
                    }))
                    .when(highlighted, |v| {
                        let brand = t.colors.brand;
                        v.child(
                            canvas(
                                |_, _, _| (),
                                move |bounds, _, window, _| {
                                    if window.last_input_was_keyboard() {
                                        window.paint_quad(gpui_kit::quad(
                                            bounds,
                                            px(4.),
                                            brand.alpha(0.),
                                            px(2.),
                                            brand,
                                            Default::default(),
                                        ));
                                    }
                                },
                            )
                            .absolute()
                            .inset_0()
                            .size_full(),
                        )
                    }),
            );
        }
        let surface = div()
            .id("surface")
            .test_support()
            .occlude()
            .relative()
            .w(self
                .bounds
                .get()
                .size
                .width
                .min(window.viewport_size().width - px(16.)))
            .py(px(6.))
            .rounded(px(8.))
            .bg(t.colors.base)
            .border_1()
            .border_color(t.colors.line)
            .shadow(t.effects.shadow_lg.clone())
            .text_color(t.text.default)
            .font_family(t.typography.font_family.clone())
            .text_size(px(14.))
            .line_height(px(20.))
            .child(list)
            .on_mouse_down_out(
                cx.listener(|v, event: &gpui_kit::MouseDownEvent, window, cx| {
                    if !v.bounds.get().contains(&event.position) {
                        v.set_open(false, window, cx);
                    }
                }),
            )
            .child(
                canvas(move |b, _, _| popup_bounds.set(b), |_, _, _, _| {})
                    .absolute()
                    .inset_0()
                    .size_full(),
            );
        root.child(
            deferred(
                base::Positioner::side({
                    let mut b = self.bounds.get();
                    b.origin.y -= px(4.);
                    b.size.height += px(8.);
                    b
                })
                .placement(base::Placement::Bottom)
                .align(base::Align::Start)
                .offset(px(0.))
                .margin(px(8.))
                .occlude()
                .child(surface),
            )
            .with_priority(base::POPUP_PRIORITY),
        )
        .into_any_element()
    }
}
