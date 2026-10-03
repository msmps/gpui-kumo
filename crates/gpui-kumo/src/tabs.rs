//! Typed horizontal Kumo Tabs over Base Tab/TabList semantics and activation.
//! The retained state fills Base 0.7.0's compound keyboard-navigation gap.
use crate::theme;
use gpui_kit::{
    AnyElement, App, Context, ElementId, Entity, EventEmitter, FocusHandle, FontWeight,
    InteractiveElement, IntoElement, ParentElement, Render, RenderOnce, ScrollHandle, SharedString,
    StatefulInteractiveElement, Styled, Window, base, div, prelude::FluentBuilder, px,
};
use gpui_kit::{canvas, quad};
use std::rc::Rc;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Variant {
    #[default]
    Segmented,
    Underline,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Size {
    #[default]
    Base,
    Sm,
}
type Content = Rc<dyn Fn(&mut Window, &mut App) -> AnyElement>;
/// Stable identity and a complete readable name, independent of order.
#[derive(Clone)]
pub struct TabItem<T> {
    id: ElementId,
    value: T,
    label: SharedString,
    disabled: bool,
    content: Option<Content>,
}
impl<T> TabItem<T> {
    pub fn new(id: impl Into<ElementId>, value: T, label: impl Into<SharedString>) -> Self {
        let label = label.into();
        assert!(!label.trim().is_empty(), "Tab requires a readable name");
        Self {
            id: id.into(),
            value,
            label,
            disabled: false,
            content: None,
        }
    }
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
}
impl<T: Clone + Eq + 'static> EventEmitter<TabsEvent<T>> for TabsState<T> {}
impl<T: Clone + Eq + 'static> TabsState<T> {
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
    pub fn controlled(mut self, controlled: bool) -> Self {
        self.controlled = controlled;
        self
    }
    pub fn activate_on_focus(mut self, activate: bool) -> Self {
        self.activate_on_focus = activate;
        self
    }
    pub fn selected(&self) -> Option<&T> {
        self.selected.as_ref()
    }
    pub fn set_selected(&mut self, value: Option<T>, cx: &mut Context<Self>) {
        if self.selected != value {
            self.selected = value;
            cx.notify();
        }
    }
    pub fn set_disabled(&mut self, disabled: bool, cx: &mut Context<Self>) {
        self.disabled = disabled;
        cx.notify();
    }
    /// Preserve focus by stable ID through reorder. Removed/unavailable focused tabs
    /// transfer focus to the selected enabled tab, then the first enabled tab.
    /// If none remain, leave the compound with native forward traversal.
    pub fn set_items(
        &mut self,
        items: Vec<TabItem<T>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        Self::validate(&items);
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
            } else {
                window.focus_next(cx);
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
        self.items[index].focus.focus(window, cx);
        self.scroll.scroll_to_item(index);
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
        self.items[index].focus.focus(window, cx);
        self.scroll.scroll_to_item(index);
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
}
impl<T: Clone + Eq + 'static> Tabs<T> {
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
        }
    }
    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
}
impl<T: Clone + Eq + 'static> RenderOnce for Tabs<T> {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        self.state.update(cx, |state, _| {
            state.variant = self.variant;
            state.size = self.size;
            state.name = self.name.clone();
        });
        div()
            .id(self.id)
            .aria_label(self.name)
            .min_w_0()
            .max_w_full()
            .child(self.state)
    }
}
impl<T: Clone + Eq + 'static> Render for TabsState<T> {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = theme(cx).clone();
        let sm = self.size == Size::Sm;
        let segmented = self.variant == Variant::Segmented;
        let height = if self.items.is_empty() {
            0.
        } else if segmented {
            if sm { 26. } else { 36. }
        } else if sm {
            26.
        } else {
            30.
        };
        let entry = self.entry(window);
        let mut list = base::Tabs::new("list")
            .aria_label(self.name.clone())
            .flex()
            .relative()
            .min_w_0()
            .max_w_full()
            .h(px(height))
            .font_weight(FontWeight::MEDIUM)
            .overflow_x_scroll()
            .track_scroll(&self.scroll)
            .on_key_down(
                cx.listener(|state, event: &gpui_kit::KeyDownEvent, window, cx| {
                    let m = &event.keystroke.modifiers;
                    if m.control || m.alt || m.platform || m.shift || m.function {
                        return;
                    }
                    if state.navigate(&event.keystroke.key, window, cx) {
                        cx.stop_propagation();
                    }
                }),
            )
            .when(segmented, |list| {
                list.bg(theme.colors.recessed)
                    .px(px(2.))
                    .rounded(px(if sm { 6. } else { 8. }))
            })
            .when(!segmented, |list| {
                list.gap(px(16.))
                    .pb(px(8.))
                    .border_b_1()
                    .border_color(theme.colors.hairline)
            });
        for (index, item) in self.items.iter().enumerate() {
            let selected = self.selected.as_ref() == Some(&item.tab.value);
            let disabled = self.disabled || item.tab.disabled;
            let owner = cx.entity().downgrade();
            let id = item.tab.id.clone();
            let content = item
                .tab
                .content
                .as_ref()
                .map(|f| f(window, cx))
                .unwrap_or_else(|| div().child(item.tab.label.clone()).into_any_element());
            let focus = item.focus.clone();
            let ring_color = theme.colors.brand;
            let selected_ring = theme.colors.line;
            let decoration = canvas(
                |_, _, _| (),
                move |bounds, _, window, _| {
                    if selected && segmented {
                        window.paint_quad(quad(
                            bounds,
                            px(if sm { 4. } else { 6. }),
                            selected_ring.alpha(0.),
                            px(1.),
                            selected_ring,
                            Default::default(),
                        ));
                    }
                    if selected && !segmented {
                        let mut indicator = bounds;
                        indicator.origin.y = bounds.bottom() + px(7.);
                        indicator.size.height = px(2.);
                        window.paint_quad(quad(
                            indicator,
                            px(0.),
                            ring_color,
                            px(0.),
                            ring_color,
                            Default::default(),
                        ));
                    }
                    if !disabled && focus.is_focused(window) && window.last_input_was_keyboard() {
                        window.paint_quad(quad(
                            bounds,
                            px(if sm { 2. } else { 6. }),
                            ring_color.alpha(0.),
                            px(2.),
                            ring_color,
                            Default::default(),
                        ));
                    }
                },
            )
            .absolute()
            .inset_0()
            .size_full();
            let mut tab = base::Tab::new(item.tab.id.clone())
                .selected(selected)
                .disabled(disabled)
                .set_position(index + 1, self.items.len())
                .accessibility_label(item.tab.label.clone())
                .track_focus(
                    &item
                        .focus
                        .clone()
                        .tab_index(0)
                        .tab_stop(entry == Some(index)),
                )
                .flex_shrink_0()
                .a11y_synthetic_children(move |builder| {
                    if disabled {
                        builder.parent_node().set_disabled();
                    }
                })
                .text_size(px(if sm { 12. } else { 14. }))
                .line_height(px(if sm { 16. } else { 21. }))
                .text_color(if selected {
                    theme.text.default
                } else {
                    theme.text.subtle
                })
                .relative()
                .rounded(px(if sm {
                    if selected && segmented { 4. } else { 2. }
                } else {
                    6.
                }))
                .px(px(if segmented {
                    if sm { 8. } else { 10. }
                } else if sm {
                    6.
                } else {
                    8.
                }))
                .when(segmented, |tab| tab.my(px(2.)))
                .when(!disabled, |tab| {
                    tab.cursor_pointer().hover(|style| {
                        let style = style.text_color(theme.text.default);
                        if segmented {
                            style
                        } else {
                            style.bg(theme.colors.tint)
                        }
                    })
                })
                .when(disabled, |tab| tab.opacity(0.5).cursor_not_allowed())
                .on_click(move |_, window, cx| {
                    let _ = owner.update(cx, |state, cx| state.activate(&id, window, cx));
                })
                .child(content)
                .child(decoration);
            if selected {
                tab = if segmented {
                    tab.bg(theme.colors.base)
                        .shadow(theme.effects.shadow_xs.clone())
                } else {
                    tab
                };
            }
            list = list.child(tab);
        }
        if segmented && !self.items.is_empty() {
            let color = theme.colors.hairline.alpha(0.7);
            list = list.child(
                canvas(
                    |_, _, _| (),
                    move |bounds, _, window, _| {
                        window.paint_quad(quad(
                            bounds,
                            px(if sm { 6. } else { 8. }),
                            color.alpha(0.),
                            px(1.),
                            color,
                            Default::default(),
                        ));
                    },
                )
                .absolute()
                .inset_0()
                .size_full(),
            );
        }
        list
    }
}
#[cfg(test)]
#[path = "tabs_tests.rs"]
mod tests;
