//! Controlled checkbox selection and Kumo group presentation.
use crate::{
    Checkbox, Label,
    checkbox::{State, Variant},
    theme,
};
use gpui_kit::base::TestSupportExt;
use gpui_kit::{
    AnyElement, App, ClickEvent, ElementId, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, Role, SharedString, StatefulInteractiveElement, Styled, Window, div,
    prelude::FluentBuilder,
};
use std::{ops::ControlFlow, rc::Rc};

/// A valued checkbox item. Identity and value are stable and unique within a group.
#[must_use]
pub struct CheckboxItem {
    value: SharedString,
    label: SharedString,
    accessible_name: Option<SharedString>,
    disabled: bool,
    variant: Variant,
    on_change: Option<ItemHandler>,
}
impl CheckboxItem {
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

    /// Create a checkbox item whose stable domain value also identifies it within a group.
    ///
    /// # Panics
    /// Panics when the required name or label is blank.
    pub fn new(value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        let value = value.into();
        let label = label.into();
        assert!(!value.trim().is_empty(), "Checkbox item requires a value");
        assert!(
            !label.trim().is_empty(),
            "Checkbox item requires a readable name"
        );
        Self {
            value,
            label,
            accessible_name: None,
            disabled: false,
            variant: Variant::Default,
            on_change: None,
        }
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
    /// Observe the user's proposed checked state before the group callback.
    /// Return Break(()) to cancel the group proposal, or Continue(()) to permit it.
    /// Group rendering and select-all changes do not invoke this callback.
    pub fn on_change(
        mut self,
        handler: impl Fn(State, &ClickEvent, &mut Window, &mut App) -> ControlFlow<()> + 'static,
    ) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}
type ItemHandler = Box<dyn Fn(State, &ClickEvent, &mut Window, &mut App) -> ControlFlow<()>>;
type ValueHandler = Rc<dyn Fn(Vec<SharedString>, &mut Window, &mut App)>;
/// A stateless group. Selection is supplied by the application on every render.
/// Unknown selected values are preserved; values are never mirrored in a UI entity.
#[derive(IntoElement)]
#[must_use]
pub struct CheckboxGroup {
    id: ElementId,
    name: SharedString,
    accessible_name: Option<SharedString>,
    legend: Option<AnyElement>,
    hidden_legend: bool,
    selected: Vec<SharedString>,
    items: Vec<CheckboxItem>,
    disabled: bool,
    control_first: bool,
    description: Option<SharedString>,
    error: Option<SharedString>,
    show_error: bool,
    on_change: Option<ValueHandler>,
    select_all: Option<(SharedString, Vec<SharedString>)>,
}
impl CheckboxGroup {
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
    /// Create a named group with the owner’s current selected domain values.
    ///
    /// # Panics
    /// Panics when the required name or label is blank.
    pub fn new(
        id: impl Into<ElementId>,
        name: impl Into<SharedString>,
        selected: &[SharedString],
    ) -> Self {
        let name = name.into();
        assert!(
            !name.trim().is_empty(),
            "Checkbox group requires a readable name"
        );
        let mut values = Vec::new();
        for value in selected {
            if !values.contains(value) {
                values.push(value.clone());
            }
        }
        Self {
            id: id.into(),
            name,
            accessible_name: None,
            legend: None,
            hidden_legend: false,
            selected: values,
            items: Vec::new(),
            disabled: false,
            control_first: true,
            description: None,
            error: None,
            show_error: true,
            on_change: None,
            select_all: None,
        }
    }
    /// Append a typed item; its stable identity and value remain independent of visible text.
    ///
    /// # Panics
    /// Panics when domain values are duplicated.
    pub fn item(mut self, item: CheckboxItem) -> Self {
        assert!(
            !self.items.iter().any(|other| other.value == item.value),
            "Checkbox item values must be unique"
        );
        self.items.push(item);
        self
    }
    /// Replace the visible legend content; `new` still supplies the complete group name.
    pub fn legend(mut self, content: impl IntoElement) -> Self {
        self.legend = Some(content.into_any_element());
        self
    }
    /// Keep the group accessible name while hiding its visual legend.
    pub fn show_label(mut self, show: bool) -> Self {
        self.hidden_legend = !show;
        self
    }
    /// Choose whether this control accepts user activation.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    /// Choose whether controls precede their labels in the native layout.
    pub fn control_first(mut self, first: bool) -> Self {
        self.control_first = first;
        self
    }
    /// Supply supporting text; errors take precedence even when their message is hidden.
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }
    /// The pinned Group source renders error and description together. This differs
    /// from its documentation's replacement claim; both are preserved here.
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
        handler: impl Fn(Vec<SharedString>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
    /// Add an aggregate checkbox for an explicit participation set, replacing web
    /// `allValues` plus a parent item. Include disabled values only intentionally:
    /// this parent proposal changes every participating value, preserving others.
    ///
    /// # Panics
    /// Panics when the aggregate name or any domain value is blank.
    pub fn select_all(mut self, name: impl Into<SharedString>, values: &[SharedString]) -> Self {
        let name = name.into();
        assert!(
            !name.trim().is_empty(),
            "Select all requires a readable name"
        );
        let mut all = Vec::new();
        for value in values {
            assert!(
                !value.trim().is_empty(),
                "Select all values must be nonempty"
            );
            if !all.contains(value) {
                all.push(value.clone());
            }
        }
        self.select_all = Some((name, all));
        self
    }
}
impl RenderOnce for CheckboxGroup {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let description = self.description.filter(|_| self.error.is_none());
        let theme = theme(cx);
        let mut items = div().flex().flex_col().gap(theme.spacing.eight).min_w_0();
        if let Some((name, all)) = self.select_all {
            let count = all.iter().filter(|v| self.selected.contains(v)).count();
            let state = if count == 0 {
                State::Unchecked
            } else if count == all.len() {
                State::Checked
            } else {
                State::Indeterminate
            };
            let selected = self.selected.clone();
            let handler = self.on_change.clone();
            let parent = Checkbox::new("select-all", name)
                .state(state)
                .disabled(self.disabled || all.is_empty())
                .control_first(self.control_first)
                .when_some(handler, |this, handler| {
                    this.on_change(move |state, _, window, cx| {
                        let mut next = selected.clone();
                        if state == State::Checked {
                            for value in &all {
                                if !next.contains(value) {
                                    next.push(value.clone());
                                }
                            }
                        } else {
                            next.retain(|value| !all.contains(value));
                        }
                        handler(next, window, cx);
                    })
                });
            items = items.child(parent);
        }
        for item in self.items {
            let value = item.value;
            let selected = self.selected.clone();
            let handler = self.on_change.clone();
            let item_handler = item.on_change;
            let checked = selected.contains(&value);
            // A separate native ID namespace prevents a value "select-all" from
            // colliding with the aggregate control's retained focus state.
            let control = Checkbox::new(SharedString::from(format!("item:{value}")), item.label)
                .when_some(item.accessible_name, |control, name| {
                    control.accessibility_label(name)
                })
                .state(if checked {
                    State::Checked
                } else {
                    State::Unchecked
                })
                .group_item()
                .variant(item.variant)
                .disabled(self.disabled || item.disabled)
                .control_first(self.control_first)
                .when(handler.is_some() || item_handler.is_some(), |this| {
                    this.on_change(move |state, event, window, cx| {
                        if let Some(item_handler) = &item_handler
                            && item_handler(state, event, window, cx).is_break()
                        {
                            return;
                        }
                        let Some(handler) = &handler else { return };
                        let mut next = selected.clone();
                        if state == State::Checked {
                            if !next.contains(&value) {
                                next.push(value.clone());
                            }
                        } else {
                            next.retain(|v| v != &value);
                        }
                        handler(next, window, cx);
                    })
                });
            items = items.child(control);
        }
        div()
            .id(self.id)
            .test_support()
            .role(Role::Group)
            .aria_label(
                self.accessible_name
                    .clone()
                    .unwrap_or_else(|| self.name.clone()),
            )
            .flex()
            .flex_col()
            .gap(theme.spacing.sixteen)
            .min_w_0()
            .when(!self.hidden_legend, |this| {
                this.child(
                    Label::new("legend", self.name)
                        .when_some(self.legend, |label, content| label.content(content)),
                )
            })
            .child(items)
            .when_some(self.error.filter(|_| self.show_error), |this, error| {
                this.child(crate::field::group_message_element(
                    theme, "error", error, true,
                ))
            })
            .when_some(description, |this, description| {
                this.child(crate::field::group_message_element(
                    theme,
                    "description",
                    description,
                    false,
                ))
            })
    }
}
impl std::fmt::Debug for CheckboxItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_struct("CheckboxItem");
        debug.field("label", &self.label);
        debug.field("disabled", &self.disabled);
        debug.field("variant", &self.variant);
        debug.finish_non_exhaustive()
    }
}

impl std::fmt::Debug for CheckboxGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_struct("CheckboxGroup");
        debug.field("id", &self.id);
        debug.field("name", &self.name);
        debug.field("hidden_legend", &self.hidden_legend);
        debug.field("items_count", &self.items.len());
        debug.field("disabled", &self.disabled);
        debug.field("control_first", &self.control_first);
        debug.field("show_error", &self.show_error);
        debug.finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests;
