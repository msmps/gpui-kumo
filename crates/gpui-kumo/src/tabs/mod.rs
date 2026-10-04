//! Typed horizontal Kumo Tabs over Base Tab/TabList semantics and activation.
//! The retained state fills Base 0.7.0's compound keyboard-navigation gap.
use crate::theme;
use gpui_kit::{
    AnyElement, App, Context, ElementId, Entity, EventEmitter, FocusHandle, FontWeight,
    InteractiveElement, IntoElement, ParentElement, RenderOnce, ScrollHandle, SharedString,
    StatefulInteractiveElement, Styled, Window, base, div, prelude::FluentBuilder, px,
};

use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};
mod lifecycle;
mod motion;
mod overflow;
mod view;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
/// Closed semantic visual treatments supported by this component.
pub enum Variant {
    #[default]
    /// Joined segmented tabs.
    Segmented,
    /// Tabs with an active underline.
    Underline,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
/// Component dimensions, including coordinated spacing and typography.
pub enum Size {
    #[default]
    /// Default dimensions or treatment.
    Base,
    /// Small dimensions.
    Sm,
}
/// Localized names for segmented overflow actions.
#[derive(Clone, Debug)]
pub struct TabsLabels {
    /// Complete text for scroll start.
    pub scroll_start: SharedString,
    /// Complete text for scroll end.
    pub scroll_end: SharedString,
}
impl Default for TabsLabels {
    fn default() -> Self {
        Self {
            scroll_start: "Scroll tabs left".into(),
            scroll_end: "Scroll tabs right".into(),
        }
    }
}
type Content = Rc<dyn Fn(&mut Window, &mut App) -> AnyElement>;
/// Stable identity and a complete readable name, independent of order.
#[derive(Clone)]
pub struct TabItem<T> {
    id: ElementId,
    value: T,
    label: SharedString,
    accessible_name: Option<SharedString>,
    disabled: bool,
    content: Option<Content>,
}
impl<T> TabItem<T> {
    /// Set visible text and its default accessible name.
    ///
    /// # Panics
    /// Panics when `text` is blank.
    pub fn label(mut self, text: impl Into<SharedString>) -> Self {
        self.label = crate::name::nonblank(text);
        self
    }
    /// Override the accessible name independently of visible text and builder ordering.
    ///
    /// # Panics
    /// Panics when `name` is blank.
    pub fn accessibility_label(mut self, name: impl Into<SharedString>) -> Self {
        self.accessible_name = Some(crate::name::nonblank(name));
        self
    }

    /// Create a tab with stable identity, typed application value and readable text.
    ///
    /// # Panics
    /// Panics when the required name or label is blank.
    pub fn new(id: impl Into<ElementId>, value: T, label: impl Into<SharedString>) -> Self {
        let label = label.into();
        assert!(!label.trim().is_empty(), "Tab requires a readable name");
        Self {
            id: id.into(),
            value,
            label,
            accessible_name: None,
            disabled: false,
            content: None,
        }
    }
    /// Choose whether this control accepts user activation.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    /// Decorative, noninteractive content; retain child entities outside this factory.
    /// Capture owners weakly and do not read/update this state during its render.
    pub fn content(
        mut self,
        content: impl Fn(&mut Window, &mut App) -> AnyElement + 'static,
    ) -> Self {
        self.content = Some(Rc::new(content));
        self
    }
}
/// User selection proposal. Owner updates and repeated selection are silent.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TabsEvent<T> {
    /// Value.
    pub value: T,
}
struct Item<T> {
    tab: TabItem<T>,
    focus: FocusHandle,
}
/// One retained selection authority. Controlled mode emits proposals without applying them.
/// Kumo exposes no panel API: consumers own panel entities, mounting and focus.
pub struct TabsState<T: Clone + Eq + 'static> {
    items: Vec<Item<T>>,
    selected: Option<T>,
    controlled: bool,
    activate_on_focus: bool,
    disabled: bool,
    variant: Variant,
    size: Size,
    scroll: ScrollHandle,
    name: SharedString,
    labels: TabsLabels,
    fade_surface: Option<gpui_kit::Hsla>,
    controls: [FocusHandle; 2],
    edges: Rc<Cell<[bool; 2]>>,
    layout_valid: Rc<Cell<bool>>,
    drag: Rc<RefCell<Option<overflow::Drag>>>,
    scroll_motion: Rc<RefCell<Option<overflow::ScrollMotion>>>,
    scroll_generation: usize,
    indicator: Rc<RefCell<motion::IndicatorMotion>>,
    edge_motion: Rc<RefCell<[motion::EdgeMotion; 2]>>,
}
impl<T: Clone + Eq + 'static> EventEmitter<TabsEvent<T>> for TabsState<T> {}
impl<T: Clone + Eq + 'static> TabsState<T> {
    /// Create a retained tab collection with stable per-item focus handles. Retain with `cx.new`.
    ///
    /// # Panics
    /// Panics when tab IDs or application values are duplicated.
    pub fn new(items: Vec<TabItem<T>>, selected: Option<T>, cx: &mut Context<Self>) -> Self {
        Self::validate(&items);
        let selected = selected.or_else(|| items.first().map(|item| item.value.clone()));
        Self {
            items: items
                .into_iter()
                .map(|tab| Item {
                    tab,
                    focus: cx.focus_handle(),
                })
                .collect(),
            selected,
            controlled: false,
            activate_on_focus: false,
            disabled: false,
            variant: Variant::default(),
            size: Size::default(),
            scroll: ScrollHandle::new(),
            name: "Tabs".into(),
            labels: TabsLabels::default(),
            fade_surface: None,
            controls: [cx.focus_handle(), cx.focus_handle()],
            edges: Rc::new(Cell::new([false; 2])),
            layout_valid: Rc::new(Cell::new(false)),
            drag: Rc::new(RefCell::new(None)),
            scroll_motion: Rc::new(RefCell::new(None)),
            scroll_generation: 0,
            indicator: Rc::new(RefCell::new(motion::IndicatorMotion::default())),
            edge_motion: Rc::new(RefCell::new(Default::default())),
        }
    }
    fn validate(items: &[TabItem<T>]) {
        for (i, item) in items.iter().enumerate() {
            assert!(
                !items[..i]
                    .iter()
                    .any(|other| other.id == item.id || other.value == item.value),
                "Tab IDs and values must be unique"
            );
        }
    }
    /// Choose whether proposals wait for owner reconciliation.
    pub fn controlled(mut self, controlled: bool) -> Self {
        self.controlled = controlled;
        self
    }
    /// Choose whether keyboard focus also proposes selection.
    pub fn activate_on_focus(mut self, activate: bool) -> Self {
        self.activate_on_focus = activate;
        self
    }
    /// Read or configure the selected value.
    pub fn selected(&self) -> Option<&T> {
        self.selected.as_ref()
    }
    /// Update selected and refresh the retained component.
    pub fn set_selected(&mut self, value: Option<T>, cx: &mut Context<Self>) {
        if self.selected != value {
            self.selected = value;
            cx.notify();
        }
    }
    /// Update availability and notify presentation while retaining the value.
    pub fn set_disabled(&mut self, disabled: bool, cx: &mut Context<Self>) {
        self.disabled = disabled;
        if disabled {
            self.drag.borrow_mut().take();
            self.scroll_motion.borrow_mut().take();
        }
        cx.notify();
    }
    /// Preserve focus by stable ID through reorder. Removed/unavailable focused tabs
    /// transfer focus to the selected enabled tab, then the first enabled tab.
    /// If none remain, leave the compound with native forward traversal.
    ///
    /// # Panics
    /// Panics when tab IDs or application values are duplicated.
    pub fn set_items(
        &mut self,
        items: Vec<TabItem<T>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        Self::validate(&items);
        self.layout_valid.set(false);
        self.drag.borrow_mut().take();
        self.scroll_motion.borrow_mut().take();
        *self.indicator.borrow_mut() = motion::IndicatorMotion::default();
        let focused = self
            .items
            .iter()
            .find(|i| i.focus.is_focused(window))
            .map(|i| i.tab.id.clone());
        let mut old = std::mem::take(&mut self.items);
        self.items = items
            .into_iter()
            .map(|tab| {
                let focus = old
                    .iter()
                    .position(|i| i.tab.id == tab.id)
                    .map(|index| old.remove(index).focus)
                    .unwrap_or_else(|| cx.focus_handle());
                Item { tab, focus }
            })
            .collect();
        if !self.controlled
            && !self
                .items
                .iter()
                .any(|i| Some(&i.tab.value) == self.selected.as_ref())
        {
            self.selected = self
                .items
                .iter()
                .find(|i| !i.tab.disabled)
                .map(|i| i.tab.value.clone());
        }
        if let Some(focused) = focused
            && !self
                .items
                .iter()
                .any(|i| i.tab.id == focused && !i.tab.disabled)
        {
            if let Some(entry) = self.entry(window) {
                self.items[entry].focus.focus(window, cx);
                self.scroll.scroll_to_item(entry + 1);
            } else {
                self.leave_compound(window, cx);
            }
        }
        cx.notify();
    }
    fn entry(&self, window: &Window) -> Option<usize> {
        if self.disabled {
            return None;
        }
        self.items
            .iter()
            .position(|i| !i.tab.disabled && i.focus.is_focused(window))
            .or_else(|| {
                self.items
                    .iter()
                    .position(|i| !i.tab.disabled && Some(&i.tab.value) == self.selected.as_ref())
            })
            .or_else(|| self.items.iter().position(|i| !i.tab.disabled))
    }
    fn activate(&mut self, id: &ElementId, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }
        let Some(index) = self
            .items
            .iter()
            .position(|i| &i.tab.id == id && !i.tab.disabled)
        else {
            return;
        };
        self.scroll_motion.borrow_mut().take();
        self.items[index].focus.focus(window, cx);
        self.scroll.scroll_to_item(index + 1);
        let value = self.items[index].tab.value.clone();
        if self.selected.as_ref() != Some(&value) {
            if !self.controlled {
                self.selected = Some(value.clone());
            }
            cx.emit(TabsEvent { value });
        }
        cx.notify();
    }
    fn navigate(&mut self, key: &str, window: &mut Window, cx: &mut Context<Self>) -> bool {
        if self.disabled {
            return false;
        }
        let enabled: Vec<_> = self
            .items
            .iter()
            .enumerate()
            .filter(|(_, i)| !i.tab.disabled)
            .map(|(i, _)| i)
            .collect();
        let Some(current) = enabled
            .iter()
            .position(|i| self.items[*i].focus.is_focused(window))
        else {
            return false;
        };
        let next = match key {
            "left" => (current + enabled.len() - 1) % enabled.len(),
            "right" => (current + 1) % enabled.len(),
            "home" => 0,
            "end" => enabled.len() - 1,
            _ => return false,
        };
        let index = enabled[next];
        self.scroll_motion.borrow_mut().take();
        self.items[index].focus.focus(window, cx);
        self.scroll.scroll_to_item(index + 1);
        if self.activate_on_focus {
            let id = self.items[index].tab.id.clone();
            self.activate(&id, window, cx);
        }
        cx.notify();
        true
    }
}
/// Consumed identity and presentation over a caller-retained TabsState entity.
#[derive(IntoElement)]
pub struct Tabs<T: Clone + Eq + 'static> {
    id: ElementId,
    name: SharedString,
    state: Entity<TabsState<T>>,
    variant: Variant,
    size: Size,
    labels: TabsLabels,
    fade_surface: Option<gpui_kit::Hsla>,
}
impl<T: Clone + Eq + 'static> Tabs<T> {
    /// Present a named tab collection over its retained state.
    ///
    /// # Panics
    /// Panics when the required name or label is blank.
    pub fn new(
        id: impl Into<ElementId>,
        name: impl Into<SharedString>,
        state: &Entity<TabsState<T>>,
    ) -> Self {
        let name = name.into();
        assert!(!name.trim().is_empty(), "Tab list requires a readable name");
        Self {
            id: id.into(),
            name,
            state: state.clone(),
            variant: Variant::default(),
            size: Size::default(),
            labels: TabsLabels::default(),
            fade_surface: None,
        }
    }
    /// Opaque surface beneath native edge fades. Underline defaults to Kumo base;
    /// segmented defaults to recessed. Supply the surrounding solid color when different.
    /// Arbitrary transparent/gradient backdrop masking is not supported by this adapter.
    pub fn fade_surface(mut self, color: gpui_kit::Hsla) -> Self {
        self.fade_surface = Some(color);
        self
    }
    /// Supply complete names for the component’s secondary actions.
    ///
    /// # Panics
    /// Panics when an overflow action name is blank.
    pub fn labels(mut self, labels: TabsLabels) -> Self {
        assert!(
            !labels.scroll_start.trim().is_empty() && !labels.scroll_end.trim().is_empty(),
            "Overflow controls require readable names"
        );
        self.labels = labels;
        self
    }
    /// Select the semantic visual treatment; interaction and value state remain independent.
    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }
    /// Select the component dimensions and corresponding spacing and typography.
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
}
impl<T: Clone + Eq + 'static> RenderOnce for Tabs<T> {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        self.state.update(cx, |state, _| {
            if state.variant != self.variant || state.size != self.size {
                state.layout_valid.set(false);
                *state.indicator.borrow_mut() = motion::IndicatorMotion::default();
                state.drag.borrow_mut().take();
                state.scroll_motion.borrow_mut().take();
            }
            state.variant = self.variant;
            state.size = self.size;
            state.name = self.name.clone();
            state.labels = self.labels;
            state.fade_surface = self.fade_surface;
        });
        div()
            .id(self.id)
            .aria_label(self.name)
            .min_w_0()
            .max_w_full()
            .child(self.state)
    }
}
impl<T> std::fmt::Debug for TabItem<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TabItem")
            .field("id", &self.id)
            .field("label", &self.label)
            .field("disabled", &self.disabled)
            .finish_non_exhaustive()
    }
}

impl<T: Clone + Eq + 'static> std::fmt::Debug for TabsState<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TabsState")
            .field("items_count", &self.items.len())
            .field("controlled", &self.controlled)
            .field("activate_on_focus", &self.activate_on_focus)
            .field("disabled", &self.disabled)
            .field("variant", &self.variant)
            .field("size", &self.size)
            .field("name", &self.name)
            .field("scroll_generation", &self.scroll_generation)
            .finish_non_exhaustive()
    }
}

impl<T: Clone + Eq + 'static> std::fmt::Debug for Tabs<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tabs")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("variant", &self.variant)
            .field("size", &self.size)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests;
