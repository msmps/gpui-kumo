//! Typed Kumo Radio composition over Base Radio and semantic RadioGroup.
use crate::{Label, theme};
use gpui_kit::base::TestSupportExt;
use gpui_kit::{
    AnyElement, App, Axis, ClickEvent, ElementId, FocusHandle, FontWeight, InteractiveElement,
    IntoElement, KeyDownEvent, ParentElement, RenderOnce, SharedString, StatefulInteractiveElement,
    Styled, Window, base, canvas, div, prelude::FluentBuilder, px, quad,
};
use std::rc::Rc;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
/// Closed semantic visual treatments supported by this component.
pub enum Variant {
    #[default]
    /// Default semantic treatment.
    Default,
    /// Error treatment.
    Error,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
/// Radio-row visual treatment.
pub enum Appearance {
    #[default]
    /// Default semantic treatment.
    Default,
    /// Card-style radio row.
    Card,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
/// Layout and directional navigation axis.
pub enum Orientation {
    #[default]
    /// Arrange controls along the vertical axis.
    Vertical,
    /// Arrange controls along the horizontal axis.
    Horizontal,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Placement of the radio indicator within a row.
pub enum ControlPosition {
    /// Align with the leading edge.
    Start,
    /// Align with the trailing edge.
    End,
}
/// Native details for the actual activation or composed navigation event.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ChangeEvent {
    /// Value proposed by activation.
    Activation(ClickEvent),
    /// Value proposed while navigating the group.
    Navigation(KeyDownEvent),
}
type Handler<T> = Rc<dyn Fn(T, &ChangeEvent, &mut Window, &mut App)>;
#[must_use]
/// One controlled radio value with stable identity and readable text.
pub struct RadioItem<T: Clone + Eq + 'static> {
    id: ElementId,
    value: T,
    name: SharedString,
    accessible_name: Option<SharedString>,
    content: Option<AnyElement>,
    description: Option<SharedString>,
    disabled: bool,
    variant: Variant,
    appearance: Option<Appearance>,
}
impl<T: Clone + Eq + 'static> RadioItem<T> {
    /// Set visible text and its default accessible name; explicit overrides remain authoritative.
    ///
    /// # Panics
    /// Panics when `text` is blank.
    pub fn label(mut self, text: impl Into<SharedString>) -> Self {
        self.name = crate::name::nonblank(text);
        self
    }
    /// Override the accessible name independently of visible text and builder order.
    ///
    /// # Panics
    /// Panics when `name` is blank.
    pub fn accessibility_label(mut self, name: impl Into<SharedString>) -> Self {
        self.accessible_name = Some(crate::name::nonblank(name));
        self
    }
    /// Create a radio row with stable identity, typed value and readable text.
    ///
    /// # Panics
    /// Panics when the required name or label is blank.
    pub fn new(id: impl Into<ElementId>, value: T, name: impl Into<SharedString>) -> Self {
        let name = name.into();
        assert!(!name.trim().is_empty(), "Radio requires a readable name");
        Self {
            id: id.into(),
            value,
            name,
            accessible_name: None,
            content: None,
            description: None,
            disabled: false,
            variant: Variant::Default,
            appearance: None,
        }
    }
    /// Decorative rich label; complete readable name remains supplied by `new`.
    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }
    /// Supply supporting row description text.
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }
    /// Choose whether this control accepts user activation.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    /// Select the semantic visual treatment; interaction and value state remain independent.
    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }
    /// Choose the radio-row visual treatment.
    pub fn appearance(mut self, appearance: Appearance) -> Self {
        self.appearance = Some(appearance);
        self
    }
}
/// A typed, controlled group. Items share its value type and keep stable IDs.
///
/// ```
/// use gpui_kumo::{RadioGroup, RadioItem};
/// let _ = RadioGroup::new("page-size", "Items per page", Some(25u32))
///     .item(RadioItem::new("ten", 10u32, "10"))
///     .item(RadioItem::new("twenty-five", 25u32, "25"));
/// ```
///
/// ```compile_fail
/// use gpui_kumo::{RadioGroup, RadioItem};
/// let _ = RadioGroup::new("page-size", "Items per page", Some(25u32))
///     .item(RadioItem::new("text", "25", "25"));
/// ```
#[derive(IntoElement)]
#[must_use]
pub struct RadioGroup<T: Clone + Eq + 'static> {
    id: ElementId,
    name: SharedString,
    accessible_name: Option<SharedString>,
    selected: Option<T>,
    items: Vec<RadioItem<T>>,
    disabled: bool,
    appearance: Appearance,
    orientation: Orientation,
    position: Option<ControlPosition>,
    legend: Option<AnyElement>,
    hide_legend: bool,
    description: Option<SharedString>,
    error: Option<SharedString>,
    show_error: bool,
    on_change: Option<Handler<T>>,
}
impl<T: Clone + Eq + 'static> RadioGroup<T> {
    /// Set visible text and its default accessible name; explicit overrides remain authoritative.
    ///
    /// # Panics
    /// Panics when `text` is blank.
    pub fn label(mut self, text: impl Into<SharedString>) -> Self {
        self.name = crate::name::nonblank(text);
        self
    }
    /// Override the accessible name independently of visible text and builder order.
    ///
    /// # Panics
    /// Panics when `name` is blank.
    pub fn accessibility_label(mut self, name: impl Into<SharedString>) -> Self {
        self.accessible_name = Some(crate::name::nonblank(name));
        self
    }
    /// Create a named radio group with the current owner-controlled value.
    ///
    /// # Panics
    /// Panics when the required name or label is blank.
    pub fn new(
        id: impl Into<ElementId>,
        name: impl Into<SharedString>,
        selected: Option<T>,
    ) -> Self {
        let name = name.into();
        assert!(
            !name.trim().is_empty(),
            "Radio group requires a readable name"
        );
        Self {
            id: id.into(),
            name,
            accessible_name: None,
            selected,
            items: Vec::new(),
            disabled: false,
            appearance: Appearance::Default,
            orientation: Orientation::Vertical,
            position: None,
            legend: None,
            hide_legend: false,
            description: None,
            error: None,
            show_error: true,
            on_change: None,
        }
    }
    /// Append a typed item; its stable identity and value remain independent of visible text.
    ///
    /// # Panics
    /// Panics when item IDs or values are duplicated.
    pub fn item(mut self, item: RadioItem<T>) -> Self {
        assert!(
            !self
                .items
                .iter()
                .any(|other| other.id == item.id || other.value == item.value),
            "Radio item IDs and values must be unique"
        );
        self.items.push(item);
        self
    }
    /// Choose whether this control accepts user activation.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    /// Choose the radio-row visual treatment.
    pub fn appearance(mut self, appearance: Appearance) -> Self {
        self.appearance = appearance;
        self
    }
    /// Choose the layout and navigation axis.
    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }
    /// Place the radio indicator before or after the row content.
    pub fn control_position(mut self, position: ControlPosition) -> Self {
        self.position = Some(position);
        self
    }
    /// Supply decorative group-label content; the textual group name remains authoritative.
    pub fn legend(mut self, content: impl IntoElement) -> Self {
        self.legend = Some(content.into_any_element());
        self
    }
    /// Choose visible label presentation without changing the accessible name.
    pub fn show_label(mut self, show: bool) -> Self {
        self.hide_legend = !show;
        self
    }
    /// Supply supporting text; errors take precedence even when their message is hidden.
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }
    /// Supply an error while choosing whether its message is visible.
    /// A hidden error still suppresses helper text; validation remains application-owned.
    pub fn error_visible(mut self, text: impl Into<SharedString>, show: bool) -> Self {
        self.error = Some(text.into());
        self.show_error = show;
        self
    }
    /// Display an application-owned error; values are retained and helper text is suppressed.
    pub fn error(mut self, error: impl Into<SharedString>) -> Self {
        self.error = Some(error.into());
        self.show_error = true;
        self
    }
    /// Observe a user value-change proposal; the application owns the resulting value.
    pub fn on_change(
        mut self,
        handler: impl Fn(T, &ChangeEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}
impl<T: Clone + Eq + 'static> RenderOnce for RadioGroup<T> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let description = self.description.filter(|_| self.error.is_none());
        let theme = theme(cx).clone();
        let handles: Vec<FocusHandle> = window.with_id(self.id.clone(), |window| {
            self.items
                .iter()
                .map(|item| {
                    window.with_id(item.id.clone(), |window| {
                        window
                            .use_keyed_state("radio-focus", cx, |_, cx| cx.focus_handle())
                            .read(cx)
                            .clone()
                    })
                })
                .collect()
        });
        let enabled: Vec<usize> = self
            .items
            .iter()
            .enumerate()
            .filter(|(_, item)| !self.disabled && !item.disabled)
            .map(|(i, _)| i)
            .collect();
        let selected_index = self
            .items
            .iter()
            .position(|item| self.selected.as_ref() == Some(&item.value));
        let entry = selected_index
            .filter(|i| enabled.contains(i))
            .or_else(|| enabled.first().copied());
        let navigation: Vec<(T, FocusHandle)> = enabled
            .iter()
            .map(|i| (self.items[*i].value.clone(), handles[*i].clone()))
            .collect();
        let selected = self.selected.clone();
        let handler = self.on_change.clone();
        let id = self.id.clone();
        let mut group = base::RadioGroup::new(self.id)
            .aria_label(
                self.accessible_name
                    .clone()
                    .unwrap_or_else(|| self.name.clone()),
            )
            .axis(if self.orientation == Orientation::Vertical {
                Axis::Vertical
            } else {
                Axis::Horizontal
            })
            .flex()
            .flex_col()
            .gap(theme.spacing.sixteen)
            .min_w_0()
            .on_key_down(
                move |event: &KeyDownEvent, window: &mut Window, cx: &mut App| {
                    let modifiers = &event.keystroke.modifiers;
                    if modifiers.control
                        || modifiers.alt
                        || modifiers.platform
                        || modifiers.shift
                        || navigation.is_empty()
                    {
                        return;
                    }
                    let current = navigation
                        .iter()
                        .position(|(_, focus)| focus.is_focused(window));
                    let Some(current) = current else {
                        return;
                    };
                    let target = match event.keystroke.key.as_str() {
                        "up" | "left" => (current + navigation.len() - 1) % navigation.len(),
                        "down" | "right" => (current + 1) % navigation.len(),
                        "home" => 0,
                        "end" => navigation.len() - 1,
                        _ => return,
                    };
                    cx.stop_propagation();
                    let (value, focus) = &navigation[target];
                    focus.focus(window, cx);
                    if selected.as_ref() != Some(value)
                        && let Some(handler) = &handler
                    {
                        handler(
                            value.clone(),
                            &ChangeEvent::Navigation(event.clone()),
                            window,
                            cx,
                        );
                    }
                },
            );
        if !self.hide_legend {
            group = group.child(
                Label::new("legend", self.name)
                    .when_some(self.legend, |label, content| label.content(content)),
            );
        }
        let card_group = self.appearance == Appearance::Card;
        let mut list = div()
            .min_w_0()
            .gap(if card_group {
                theme.spacing.twelve
            } else {
                theme.spacing.eight
            })
            .when(self.orientation == Orientation::Vertical, |this| {
                this.flex().flex_col()
            })
            .when(
                self.orientation == Orientation::Horizontal && !card_group,
                |this| this.flex().flex_wrap(),
            )
            .when(
                self.orientation == Orientation::Horizontal && card_group,
                |this| this.grid().grid_cols(2),
            );
        let total = self.items.len();
        for (index, item) in self.items.into_iter().enumerate() {
            let focus = handles[index].clone();
            let checked = self.selected.as_ref() == Some(&item.value);
            let disabled = self.disabled || item.disabled;
            let appearance = item.appearance.unwrap_or(self.appearance);
            let card = appearance == Appearance::Card;
            let error = item.variant == Variant::Error;
            let position = self.position.unwrap_or(if card {
                ControlPosition::End
            } else {
                ControlPosition::Start
            });
            let ring_theme = theme.clone();
            let ring_focus = focus.clone();
            let ring = canvas(
                |_, _, _| (),
                move |bounds, _, window, _| {
                    let focused = !disabled && ring_focus.is_focused(window);
                    let color = if focused {
                        if window.last_input_was_keyboard() {
                            ring_theme.colors.brand
                        } else {
                            ring_theme.colors.focus
                        }
                    } else {
                        // Native group-hover refines the indicator foreground;
                        // paint uses that inherited ring role across the whole item.
                        window.text_style().color
                    };
                    let width = if card || focused {
                        px(2.)
                    } else {
                        ring_theme.effects.control_ring_width
                    };
                    window.paint_quad(quad(
                        bounds.dilate(width),
                        (bounds.size.width / 2.) + width,
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
            let indicator = div()
                .id("indicator")
                .test_support()
                .relative()
                .size(px(16.))
                .mt(px(2.))
                .flex_shrink_0()
                .flex()
                .items_center()
                .justify_center()
                .rounded_full()
                .text_color(if error {
                    theme.colors.danger
                } else {
                    theme.colors.line
                })
                .when(!disabled && !error, |this| {
                    this.group_hover("kumo-radio-item", |style| {
                        style.text_color(theme.colors.hairline)
                    })
                })
                .bg(if checked {
                    theme.colors.contrast
                } else {
                    theme.colors.base
                })
                .when(checked, |this| {
                    this.child(
                        div()
                            .id("dot")
                            .test_support()
                            .size(px(8.))
                            .rounded_full()
                            .bg(theme.colors.base),
                    )
                })
                .child(ring);
            let label = div()
                .id("label")
                .test_support()
                .min_w_0()
                .text_size(theme.typography.base.size)
                .line_height(theme.typography.base.line_height)
                .text_color(theme.text.default)
                .font_weight(if card {
                    FontWeight::MEDIUM
                } else {
                    FontWeight::NORMAL
                })
                .child(
                    item.content
                        .unwrap_or_else(|| item.name.clone().into_any_element()),
                );
            let semantic_description = item.description.clone().filter(|_| card);
            let content = div()
                .flex()
                .flex_col()
                .gap(px(2.))
                .min_w_0()
                .when(card, |this| this.flex_1())
                .child(label)
                .when_some(item.description.filter(|_| card), |this, description| {
                    this.child(crate::field::group_message_element(
                        &theme,
                        "item-description",
                        description,
                        false,
                    ))
                });
            let mut radio = base::Radio::new(item.id)
                .checked(checked)
                .disabled(disabled)
                .track_focus(&focus)
                .tab_stop(entry == Some(index))
                .set_position(index + 1, total)
                .accessibility_label(item.accessible_name.unwrap_or(item.name))
                .when_some(semantic_description, |this, description| {
                    this.aria_description(description)
                })
                .a11y_synthetic_children(move |builder| {
                    if disabled {
                        builder.parent_node().set_disabled();
                    }
                })
                .group("kumo-radio-item")
                .flex()
                .items_start()
                .min_w_0()
                .max_w_full()
                .gap(if card {
                    theme.spacing.twelve
                } else {
                    theme.spacing.eight
                })
                .opacity(if disabled { 0.5 } else { 1. })
                .when(!disabled, |this| this.cursor_pointer())
                .when(disabled, |this| this.cursor_not_allowed())
                .when(card, |this| {
                    this.w_full()
                        .p(theme.spacing.twelve)
                        .border_1()
                        .border_color(if error {
                            theme.colors.danger
                        } else if checked {
                            theme.colors.interact
                        } else {
                            theme.colors.hairline
                        })
                        .rounded(theme.radii.lg)
                        .bg(if checked && !error {
                            theme.colors.tint
                        } else {
                            theme.colors.base
                        })
                        .when(!disabled && !error, |this| {
                            this.hover(|style| style.bg(theme.colors.tint))
                        })
                });
            radio = if position == ControlPosition::Start {
                radio.child(indicator).child(content)
            } else {
                radio.child(content).child(indicator)
            };
            if let Some(handler) = self.on_change.clone() {
                let value = item.value;
                radio = radio.on_change(move |_, event, window, cx| {
                    handler(
                        value.clone(),
                        &ChangeEvent::Activation(event.clone()),
                        window,
                        cx,
                    )
                });
            }
            list = list.child(radio);
        }
        group = group.child(list);
        if let Some(error) = self.error.filter(|_| self.show_error) {
            group = group.child(crate::field::group_message_element(
                &theme, "error", error, true,
            ));
        }
        if let Some(description) = description {
            group = group.child(crate::field::group_message_element(
                &theme,
                "description",
                description,
                false,
            ));
        }
        // Base RadioGroup has no observation hook in 0.7.0; this identity-only
        // wrapper scopes geometry/input tests without duplicating its role/name.
        div().id(id).test_support().min_w_0().child(group)
    }
}

debug_struct!([T: Clone + Eq + 'static] RadioItem<T> { id, name, disabled, variant });

debug_struct!([T: Clone + Eq + 'static] RadioGroup<T> {
    id,
    name,
    items_count: |this| this.items.len(),
    disabled,
    appearance,
    orientation,
    hide_legend,
    show_error,
});

#[cfg(test)]
mod tests;
