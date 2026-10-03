//! Retained typed Kumo Select over Base disclosure and deferred positioning.
#[cfg(test)]
#[path = "select_overlay_tests.rs"]
mod overlay_tests;
#[path = "select_semantics.rs"]
mod semantics;
#[cfg(test)]
#[path = "select_tests.rs"]
mod tests;
use crate::{Theme, theme};
pub use crate::{input::Size, popover::Placement, tooltip::Align};
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
        KeyBinding::new(
            "escape",
            base::actions::Cancel,
            Some("KumoSelect && select_open"),
        ),
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
type ProposalHandler<T> = Rc<dyn Fn(&SelectValue<T>, &mut Window, &mut App)>;
/// Decorative selected content with the complete readable value on Base's control.
/// Keep interactive actions outside the trigger.
pub struct SelectValueContent {
    value_text: SharedString,
    content: AnyElement,
}
impl SelectValueContent {
    pub fn new(value_text: impl Into<SharedString>, content: impl IntoElement) -> Self {
        let value_text = value_text.into();
        assert!(
            !value_text.trim().is_empty(),
            "Select value content requires readable text"
        );
        Self {
            value_text,
            content: content.into_any_element(),
        }
    }
}
type ValueFactory<T> =
    Rc<dyn Fn(&SelectValue<T>, &mut Window, &mut App) -> Option<SelectValueContent>>;
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
    /// use it for callbacks without retaining the owner. Do not read or update this
    /// SelectState while building content: it is already rendering under a mutable borrow.
    /// Keep interactive controls outside options.
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
/// A contiguous option group. Its optional label is visible and names the native group.
#[derive(Clone)]
pub struct SelectGroup<T> {
    id: ElementId,
    label: Option<SharedString>,
    options: Vec<SelectOption<T>>,
}
impl<T> SelectGroup<T> {
    pub fn new(id: impl Into<ElementId>, options: Vec<SelectOption<T>>) -> Self {
        Self {
            id: id.into(),
            label: None,
            options,
        }
    }
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        let label = label.into();
        assert!(
            !label.trim().is_empty(),
            "Select group label requires a name"
        );
        self.label = Some(label);
        self
    }
}
/// Ordered list parts; groups own their options, so there is no second value collection.
#[derive(Clone)]
pub enum SelectPart<T> {
    Option(SelectOption<T>),
    Group(SelectGroup<T>),
    Separator(ElementId),
}
impl<T> SelectPart<T> {
    pub fn separator(id: impl Into<ElementId>) -> Self {
        Self::Separator(id.into())
    }
    fn options(&self) -> &[SelectOption<T>] {
        match self {
            Self::Option(option) => std::slice::from_ref(option),
            Self::Group(group) => &group.options,
            Self::Separator(_) => &[],
        }
    }
}
impl<T> From<SelectOption<T>> for SelectPart<T> {
    fn from(option: SelectOption<T>) -> Self {
        Self::Option(option)
    }
}
impl<T> From<SelectGroup<T>> for SelectPart<T> {
    fn from(group: SelectGroup<T>) -> Self {
        Self::Group(group)
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
    parts: Vec<SelectPart<T>>,
    value: SelectValue<T>,
    compare: Comparator<T>,
    value_content: Option<ValueFactory<T>>,
    controlled: bool,
    proposal_handler: Option<ProposalHandler<T>>,
    joined_middle: bool,
    joined_borders: Option<crate::button::JoinedRingQueue>,
    disabled: bool,
    read_only: bool,
    open: bool,
    highlighted: Option<ElementId>,
    trigger: FocusHandle,
    content: FocusHandle,
    bounds: Rc<Cell<Bounds<Pixels>>>,
    popup_bounds: Rc<Cell<Bounds<Pixels>>>,
    scroll: ScrollHandle,
    reveal_pending: Rc<Cell<bool>>,
    deferred: Option<base::DeferredPopover>,
    prefix: String,
    typed_at: Option<Instant>,
    size: Size,
    placement: Placement,
    align: Align,
    offset: Pixels,
    placeholder: SharedString,
    invalid: bool,
    description: Option<SharedString>,
    loading: bool,
    hovered: bool,
    _theme: Subscription,
    _tab_exit: Subscription,
    focus_out: Option<Subscription>,
    overlay: Rc<crate::popover::ChildOverlay>,
    parent: Option<crate::popover::PopoverClose>,
    parent_release: Option<Subscription>,
    mount: std::rc::Weak<semantics::Mount>,
    mounted_once: bool,
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
        let parts = options
            .into_iter()
            .map(SelectPart::Option)
            .collect::<Vec<_>>();
        Self::check_ids(&parts);
        let owner = cx.entity().downgrade();
        let contains_owner = owner.clone();
        let tab_owner = owner.clone();
        let overlay = Rc::new(crate::popover::ChildOverlay {
            contains: Box::new(move |point, cx| {
                contains_owner.upgrade().is_some_and(|owner| {
                    let state: &Self = owner.read(cx);
                    state.open
                        && state.mount.upgrade().is_some()
                        && state.popup_bounds.get().contains(point)
                })
            }),
            dismiss: Box::new(move |window, cx| {
                let _ = owner.update(cx, |state: &mut Self, cx| state.set_open(false, window, cx));
            }),
        });
        Self {
            name,
            parts,
            value,
            compare: Rc::new(|a, b| a == b),
            value_content: None,
            controlled: false,
            proposal_handler: None,
            joined_middle: false,
            joined_borders: None,
            disabled: false,
            read_only: false,
            open: false,
            highlighted: None,
            trigger: cx.focus_handle(),
            content: cx.focus_handle().tab_stop(false),
            bounds: Rc::new(Cell::new(Bounds::default())),
            popup_bounds: Rc::new(Cell::new(Bounds::default())),
            scroll: ScrollHandle::new(),
            reveal_pending: Rc::new(Cell::new(false)),
            deferred: None,
            prefix: String::new(),
            typed_at: None,
            size: Size::Base,
            placement: Placement::Bottom,
            align: Align::Start,
            offset: px(4.),
            placeholder: "Select…".into(),
            invalid: false,
            description: None,
            loading: false,
            hovered: false,
            _theme: cx.observe_global::<Theme>(|_, cx| cx.notify()),
            _tab_exit: cx.intercept_keystrokes({
                let owner = tab_owner;
                move |event, window, cx| {
                    let key = &event.keystroke;
                    let modifiers = key.modifiers;
                    if key.key != "tab"
                        || modifiers.control
                        || modifiers.alt
                        || modifiers.platform
                        || modifiers.function
                    {
                        return;
                    }
                    let Some(owner) = owner.upgrade() else {
                        return;
                    };
                    let state: &Self = owner.read(cx);
                    if state.open
                        && !state.unavailable()
                        && !state.is_unmounted()
                        && (state.trigger.is_focused(window) || state.content.is_focused(window))
                    {
                        owner.update(cx, |state, cx| {
                            state.exit_on_tab(modifiers.shift, window, cx);
                        });
                    }
                }
            }),
            focus_out: None,
            overlay,
            parent: None,
            parent_release: None,
            mount: std::rc::Weak::new(),
            mounted_once: false,
        }
    }
    fn check_ids(parts: &[SelectPart<T>]) {
        let mut ids = std::collections::HashSet::new();
        for part in parts {
            match part {
                SelectPart::Group(group) => {
                    assert!(ids.insert(&group.id), "Select part IDs must be unique")
                }
                SelectPart::Separator(id) => {
                    assert!(ids.insert(id), "Select part IDs must be unique")
                }
                SelectPart::Option(_) => (),
            }
            for option in part.options() {
                assert!(ids.insert(&option.id), "Select part IDs must be unique");
            }
        }
    }
    fn options(&self) -> impl DoubleEndedIterator<Item = &SelectOption<T>> {
        self.parts.iter().flat_map(|part| part.options().iter())
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
    /// Format a defined value; returning None presents the placeholder.
    /// Single(None) skips the factory; Multiple passes its array, including when empty.
    /// Retain child entities outside this factory and capture Select/owner entities
    /// weakly for returned element callbacks. Do not read or update this SelectState
    /// from the factory: it is already borrowed. Use the provided value.
    pub fn set_value_content(
        &mut self,
        content: impl Fn(&SelectValue<T>, &mut Window, &mut App) -> Option<SelectValueContent> + 'static,
        cx: &mut Context<Self>,
    ) {
        self.value_content = Some(Rc::new(content));
        cx.notify();
    }
    pub fn clear_value_content(&mut self, cx: &mut Context<Self>) {
        self.value_content = None;
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
    pub(crate) fn set_name(&mut self, name: SharedString, cx: &mut Context<Self>) {
        assert!(
            !name.trim().is_empty(),
            "Select requires an accessible name"
        );
        self.name = name;
        cx.notify();
    }
    pub fn set_controlled(&mut self, controlled: bool, cx: &mut Context<Self>) {
        self.controlled = controlled;
        cx.notify();
    }
    // Compound owners stamp proposals at activation, before deferred event
    // delivery. The handler must not borrow this Select; capture owners weakly.
    pub(crate) fn set_proposal_handler(
        &mut self,
        handler: impl Fn(&SelectValue<T>, &mut Window, &mut App) + 'static,
    ) {
        self.proposal_handler = Some(Rc::new(handler));
    }
    pub(crate) fn set_joined_middle(&mut self) {
        self.joined_middle = true;
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
        self.set_parts(options.into_iter().map(SelectPart::Option).collect(), cx);
    }
    /// Replace the authoritative list while retaining valid highlighted option identity.
    /// All option, group and separator IDs must be unique across the collection.
    pub fn set_parts(&mut self, parts: Vec<SelectPart<T>>, cx: &mut Context<Self>) {
        Self::check_ids(&parts);
        self.parts = parts;
        if !self
            .options()
            .any(|o| Some(&o.id) == self.highlighted.as_ref() && !o.disabled)
        {
            let highlighted = self.options().find(|o| !o.disabled).map(|o| o.id.clone());
            self.highlighted = highlighted;
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
        if self.open == open || (open && (self.unavailable() || self.is_unmounted())) {
            return;
        }
        if open
            && self.parent.as_ref().is_some_and(|parent| {
                parent
                    .entity()
                    .is_none_or(|parent| !parent.read(cx).is_open())
            })
        {
            return;
        }
        self.open = open;
        self.prefix.clear();
        self.typed_at = None;
        if open {
            let highlighted = self
                .options()
                .find(|o| !o.disabled && self.selected(&o.value))
                .or_else(|| self.options().find(|o| !o.disabled))
                .map(|o| o.id.clone());
            self.highlighted = highlighted;
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
    // Detachment preserves value but excludes stale popup state without restoring an unmounted trigger.
    fn discard_popup(&mut self, cx: &mut Context<Self>) {
        self.open = false;
        self.deferred = None;
        self.prefix.clear();
        self.typed_at = None;
        self.hovered = false;
        cx.notify();
    }
    fn blur_removed_focus(&self, handle: gpui_kit::AnyWindowHandle, cx: &mut App) {
        let content = self.content.clone();
        let trigger = self.trigger.clone();
        let _ = handle.update(cx, |_, window, cx| {
            if content.contains_focused(window, cx) || trigger.is_focused(window) {
                window.blur(cx);
            }
        });
    }
    fn unavailable(&self) -> bool {
        self.disabled || self.loading
    }
    fn is_unmounted(&self) -> bool {
        self.mounted_once && self.mount.upgrade().is_none()
    }
    pub(crate) fn is_mounted(&self) -> bool {
        self.mount.upgrade().is_some()
    }
    fn selected(&self, value: &T) -> bool {
        match &self.value {
            SelectValue::Single(v) => v.as_ref().is_some_and(|v| (self.compare)(v, value)),
            SelectValue::Multiple(v) => v.iter().any(|v| (self.compare)(v, value)),
        }
    }
    fn reveal(&self) {
        // ScrollHandle::scroll_to_item only understands direct children, not grouped options.
        // A highlighted row's actual prepaint bounds reconcile nearest reveal next frame.
        self.reveal_pending.set(true);
    }
    fn move_highlight(&mut self, delta: isize, cx: &mut Context<Self>) {
        let enabled: Vec<_> = self
            .options()
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
        if self.unavailable() || self.read_only || self.is_unmounted() {
            return;
        }
        let Some(option) = self.options().find(|o| &o.id == id && !o.disabled) else {
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
            if let Some(handler) = &self.proposal_handler {
                handler(&next, window, cx);
            }
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
    fn exit_on_tab(&mut self, backwards: bool, window: &mut Window, cx: &mut Context<Self>) {
        if !self.open || self.unavailable() {
            cx.propagate();
            return;
        }
        // Intercept before a consumer's global Tab action. Raw key listeners run
        // after action dispatch; later host bindings can override scoped bindings.
        self.set_open(false, window, cx);
        self.trigger.focus(window, cx);
        if backwards {
            window.focus_prev(cx);
        } else {
            window.focus_next(cx);
        }
        cx.stop_propagation();
    }
    fn keys(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        if self.unavailable() {
            return;
        }
        let key = event.keystroke.key.as_str();
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
                self.options().find(|o| !o.disabled)
            } else {
                self.options().rev().find(|o| !o.disabled)
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
            .options()
            .position(|o| Some(&o.id) == self.highlighted.as_ref())
            .map_or(0, |i| if repeated { i + 1 } else { i });
        let options: Vec<_> = self.options().collect();
        let found = (0..options.len())
            .map(|n| (start + n) % options.len())
            .find(|i| {
                !options[*i].disabled && options[*i].label.to_lowercase().starts_with(&query)
            });
        if let Some(i) = found {
            let id = options[i].id.clone();
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
        self.options()
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
    placement: Placement,
    align: Align,
    offset: Pixels,
    placeholder: SharedString,
    invalid: bool,
    loading: bool,
    label: bool,
    required: bool,
    description: Option<SharedString>,
    error: Option<(SharedString, bool)>,
    parent: Option<crate::popover::PopoverClose>,
    joined_borders: Option<crate::button::JoinedRingQueue>,
}
impl<T: Clone + PartialEq + 'static> Select<T> {
    pub fn new(id: impl Into<ElementId>, state: &Entity<SelectState<T>>) -> Self {
        Self {
            id: id.into(),
            state: state.clone(),
            size: Size::Base,
            placement: Placement::Bottom,
            align: Align::Start,
            offset: px(4.),
            placeholder: "Select…".into(),
            invalid: false,
            loading: false,
            label: false,
            required: true,
            description: None,
            error: None,
            parent: None,
            joined_borders: None,
        }
    }
    pub fn placement(mut self, placement: Placement) -> Self {
        self.placement = placement;
        self
    }
    pub(crate) fn joined_middle(mut self, borders: crate::button::JoinedRingQueue) -> Self {
        self.joined_borders = Some(borders);
        self
    }
    /// Include this child surface in the parent Popover's dismissal boundary.
    /// Parent closure dismisses this Select; reassigning or omitting detaches it.
    pub fn parent(mut self, parent: &crate::popover::PopoverClose) -> Self {
        self.parent = Some(parent.clone());
        self
    }
    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }
    /// Main-axis separation from the trigger, including collision fitting.
    pub fn offset(mut self, offset: Pixels) -> Self {
        assert!(
            f32::from(offset).is_finite() && offset >= px(0.),
            "Select offset must be finite and nonnegative"
        );
        self.offset = offset;
        self
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
        let previous = self.state.read(cx).parent.clone();
        let same_parent = match (&previous, &self.parent) {
            (Some(previous), Some(parent)) => previous.same_parent(parent),
            (None, None) => true,
            _ => false,
        };
        if !same_parent {
            let overlay = self.state.read(cx).overlay.clone();
            if let Some(previous) = &previous {
                previous.detach_overlay(&overlay, cx);
            }
            if let Some(parent) = &self.parent {
                parent.attach_overlay(&overlay, cx);
            }
            self.state.update(cx, |state, cx| {
                state.parent_release =
                    self.parent
                        .as_ref()
                        .and_then(|parent| parent.entity())
                        .map(|parent| {
                            let window_handle = window.window_handle();
                            cx.observe_release(&parent, move |state, _, cx| {
                                // Release state even if its original window is already gone.
                                state.discard_popup(cx);
                                state.blur_removed_focus(window_handle, cx);
                            })
                        });
                state.parent = self.parent;
                if state.open
                    && state.parent.as_ref().is_some_and(|parent| {
                        parent
                            .entity()
                            .is_none_or(|parent| !parent.read(cx).is_open())
                    })
                {
                    state.set_open(false, window, cx);
                }
            });
        }
        self.state.update(cx, |v, cx| {
            v.joined_borders = self.joined_borders;
            v.size = self.size;
            v.placement = self.placement;
            v.align = self.align;
            v.offset = self.offset;
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
        let radius = if self.joined_middle { px(0.) } else { radius };
        let display = self.display();
        let has_value = match &self.value {
            SelectValue::Single(value) => value.is_some(),
            SelectValue::Multiple(_) => true,
        };
        let custom = if has_value && !self.loading {
            self.value_content
                .as_ref()
                .and_then(|factory| factory(&self.value, window, cx))
        } else {
            None
        };
        let empty = if has_value && self.value_content.is_some() && !self.loading {
            custom.is_none()
        } else {
            display.is_empty()
        };
        let readable = custom
            .as_ref()
            .map_or_else(|| display.clone(), |value| value.value_text.clone());
        let value_content = custom.map(|value| value.content).unwrap_or_else(|| {
            div()
                .child(if empty {
                    self.placeholder.clone()
                } else {
                    display
                })
                .into_any_element()
        });
        let open = self.open;
        let disabled = self.unavailable();
        let bounds = self.bounds.clone();
        let owner = cx.entity().downgrade();
        let open_owner = cx.entity().downgrade();
        let mount_owner = open_owner.clone();
        let confirm_owner = open_owner.clone();
        let ring_focus = self.trigger.clone();
        let invalid = self.invalid;
        let colors = t.colors.clone();
        let joined_middle = self.joined_middle;
        let joined_borders = self.joined_borders.clone();
        let trigger = base::Select::new("trigger")
            .open(open)
            .disabled(disabled)
            .focus_handle(&self.trigger)
            .content_focus_handle(&self.content)
            .key_context(if open {
                "KumoSelect select_open"
            } else {
                "KumoSelect"
            })
            .accessibility_label(self.name.clone())
            .accessibility_value(readable)
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
            } else if self.hovered {
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
                    .gap(px(match self.size {
                        Size::Xs | Size::Sm => 4.,
                        Size::Base => 6.,
                        Size::Lg => 8.,
                    }))
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
                            .when(!self.loading, |v| v.child(value_content))
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
                        } else if joined_middle {
                            colors.hairline
                        } else {
                            colors.line
                        };
                        let border = gpui_kit::quad(
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
                        );
                        if let Some(borders) = &joined_borders {
                            borders
                                .borrow_mut()
                                .push((focused, border, window.content_mask()));
                        } else {
                            window.paint_quad(border);
                        }
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
            mount: Box::new({
                let owner = mount_owner;
                move |window, cx| {
                    let executor = cx.foreground_executor().clone();
                    let app = cx.to_async();
                    let handle = window.window_handle();
                    let cleanup_owner = owner.clone();
                    let mount = Rc::new(semantics::Mount::new(move || {
                        // Element-state GC occurs during a frame; defer entity updates
                        // until the app/window borrow has ended.
                        executor
                            .spawn(async move {
                                if cleanup_owner.upgrade().is_none() {
                                    return;
                                }
                                app.update(|cx| {
                                    let _ = cleanup_owner.update(cx, |state: &mut Self, cx| {
                                        // A new identity may already have remounted this state.
                                        if state.mount.upgrade().is_some() {
                                            return;
                                        }
                                        if let Some(parent) = state.parent.take() {
                                            parent.detach_overlay(&state.overlay, cx);
                                        }
                                        state.parent_release = None;
                                        state.discard_popup(cx);
                                        state.blur_removed_focus(handle, cx);
                                    });
                                });
                            })
                            .detach();
                    }));
                    let _ = owner.update(cx, |state: &mut Self, _| {
                        state.mount = Rc::downgrade(&mount);
                        state.mounted_once = true;
                    });
                    mount
                }
            }),
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
            .key_context(if open {
                "KumoSelect select_open"
            } else {
                "KumoSelect"
            })
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
        let available = match self.placement {
            Placement::Top | Placement::Bottom => {
                (window.viewport_size().height - trigger_bounds.bottom() - self.offset - px(8.))
                    .max(trigger_bounds.top() - self.offset - px(8.))
            }
            Placement::Left | Placement::Right => window.viewport_size().height - px(16.),
        }
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
            // CSS overflow-y:auto also contains horizontal overflow. In GPUI
            // the axes are independent; contain source negative-margin separators.
            .overflow_x_hidden()
            .flex()
            .flex_col()
            .max_h((available - px(14.)).max(px(1.)));
        for part in &self.parts {
            list = list.child(match part {
                SelectPart::Option(option) => self
                    .option_element(option, &t, window, cx)
                    .into_any_element(),
                SelectPart::Group(group) => {
                    let mut element = div()
                        .id(group.id.clone())
                        .test_support()
                        .role(Role::Group)
                        .flex()
                        .flex_col()
                        .flex_shrink_0()
                        .when_some(group.label.clone(), |v, label| {
                            v.aria_label(label.clone()).child(
                                div()
                                    .px(px(14.))
                                    .py(px(6.))
                                    .text_size(t.typography.sm.size)
                                    .line_height(t.typography.sm.line_height)
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(t.text.subtle)
                                    .child(label),
                            )
                        });
                    for option in &group.options {
                        element = element.child(self.option_element(option, &t, window, cx));
                    }
                    element.into_any_element()
                }
                SelectPart::Separator(id) => div()
                    .id(id.clone())
                    .test_support()
                    .role(Role::Splitter)
                    .flex_shrink_0()
                    .mx(px(-4.))
                    .my(px(4.))
                    .h(px(1.))
                    .bg(t.colors.hairline)
                    .into_any_element(),
            });
        }
        // CSS gives the source popup an anchor minimum, not a fixed anchor
        // width. Plain option words need their intrinsic text space plus the
        // actual mx6/px8/check14/gap8/border1 recipe. Allow long sentences to
        // wrap while keeping compact numeric options on one line. Rich option
        // content keeps its existing caller-owned layout contract.
        let word_width = self
            .options()
            .filter(|option| option.content.is_none())
            .flat_map(|option| option.label.split_whitespace())
            .map(|word| {
                let text: SharedString = word.to_owned().into();
                let run = gpui_kit::TextRun {
                    len: text.len(),
                    font: gpui_kit::font(t.typography.font_family.clone()),
                    color: t.text.default,
                    background_color: None,
                    underline: None,
                    strikethrough: None,
                };
                window
                    .text_system()
                    .shape_line(text, px(14.), &[run], None)
                    .width
                    .ceil()
            })
            .fold(px(0.), |width, next| width.max(next));
        let option_chrome = px(2. * 6. + 2. * 8. + 14. + 8. + 2. * 1.);
        let popup_width = trigger_bounds
            .size
            .width
            .max(word_width + option_chrome)
            .min((window.viewport_size().width - px(16.)).max(px(1.)));
        let surface = div()
            .id("surface")
            .test_support()
            .occlude()
            .relative()
            .w(popup_width)
            .py(px(6.))
            .rounded(px(8.))
            .bg(t.colors.base)
            .border_1()
            .border_color(t.colors.line)
            .shadow(t.effects.shadow_lg.clone())
            .text_color(t.text.default)
            .font_family(t.typography.font_family.clone())
            .text_size(t.typography.base.size)
            .line_height(t.typography.base.line_height)
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
                    match self.placement {
                        Placement::Top | Placement::Bottom => {
                            b.origin.y -= self.offset;
                            b.size.height += self.offset * 2.;
                        }
                        Placement::Left | Placement::Right => {
                            b.origin.x -= self.offset;
                            b.size.width += self.offset * 2.;
                        }
                    }
                    b
                })
                .placement(match self.placement {
                    Placement::Top => base::Placement::Top,
                    Placement::Bottom => base::Placement::Bottom,
                    Placement::Left => base::Placement::Left,
                    Placement::Right => base::Placement::Right,
                })
                .align(match self.align {
                    Align::Start => base::Align::Start,
                    Align::Center => base::Align::Center,
                    Align::End => base::Align::End,
                })
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

impl<T: Clone + PartialEq + 'static> SelectState<T> {
    fn option_element(
        &self,
        option: &SelectOption<T>,
        t: &Theme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let id = option.id.clone();
        let select_id = id.clone();
        let hover_id = id.clone();
        let accessible_id = id.clone();
        let selected = self.selected(&option.value);
        let unavailable = option.disabled;
        let highlighted = self.highlighted.as_ref() == Some(&id);
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
            })
            .when(highlighted, |v| {
                let scroll = self.scroll.clone();
                let pending = self.reveal_pending.clone();
                let owner = cx.entity().downgrade();
                v.child(
                    canvas(
                        move |bounds, window, cx| {
                            if pending.replace(false) {
                                let viewport = scroll.bounds();
                                let padding = if viewport.size.height > px(16.) {
                                    px(8.)
                                } else {
                                    px(0.)
                                };
                                let top = viewport.top() + padding;
                                let bottom = viewport.bottom() - padding;
                                let delta =
                                    if bounds.size.height > bottom - top || bounds.top() < top {
                                        top - bounds.top()
                                    } else if bounds.bottom() > bottom {
                                        bottom - bounds.bottom()
                                    } else {
                                        px(0.)
                                    };
                                let mut offset = scroll.offset();
                                let next = (offset.y + delta).clamp(-scroll.max_offset().y, px(0.));
                                if next != offset.y {
                                    offset.y = next;
                                    scroll.set_offset(offset);
                                    let _ = owner.update(cx, |_, cx| cx.notify());
                                    window.request_animation_frame();
                                }
                            }
                        },
                        |_, _, _, _| {},
                    )
                    .absolute()
                    .inset_0()
                    .size_full(),
                )
            })
    }
}
