//! Controlled checkbox selection and Kumo group presentation.
use crate::{
    Checkbox, Label,
    checkbox::{State, Variant},
    theme,
};
use gpui_kit::base::TestSupportExt;
use gpui_kit::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce, Role,
    SharedString, StatefulInteractiveElement, Styled, Window, div, prelude::FluentBuilder,
};
use std::rc::Rc;

/// A valued checkbox item. Identity and value are stable and unique within a group.
#[must_use]
pub struct CheckboxItem {
    value: SharedString,
    label: SharedString,
    disabled: bool,
    variant: Variant,
}
impl CheckboxItem {
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
            disabled: false,
            variant: Variant::Default,
        }
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }
}
type ValueHandler = Rc<dyn Fn(Vec<SharedString>, &mut Window, &mut App)>;
/// A stateless group. Selection is supplied by the application on every render.
/// Unknown selected values are preserved; values are never mirrored in a UI entity.
#[derive(IntoElement)]
#[must_use]
pub struct CheckboxGroup {
    id: ElementId,
    name: SharedString,
    legend: Option<AnyElement>,
    hidden_legend: bool,
    selected: Vec<SharedString>,
    items: Vec<CheckboxItem>,
    disabled: bool,
    control_first: bool,
    description: Option<SharedString>,
    error: Option<SharedString>,
    on_change: Option<ValueHandler>,
    select_all: Option<(SharedString, Vec<SharedString>)>,
}
impl CheckboxGroup {
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
            legend: None,
            hidden_legend: false,
            selected: values,
            items: Vec::new(),
            disabled: false,
            control_first: true,
            description: None,
            error: None,
            on_change: None,
            select_all: None,
        }
    }
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
    pub fn hide_legend(mut self, hidden: bool) -> Self {
        self.hidden_legend = hidden;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn control_first(mut self, first: bool) -> Self {
        self.control_first = first;
        self
    }
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }
    /// The pinned Group source renders error and description together. This differs
    /// from its documentation's replacement claim; both are preserved here.
    pub fn error(mut self, error: impl Into<SharedString>) -> Self {
        self.error = Some(error.into());
        self
    }
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
            let checked = selected.contains(&value);
            // A separate native ID namespace prevents a value "select-all" from
            // colliding with the aggregate control's retained focus state.
            let control = Checkbox::new(SharedString::from(format!("item:{value}")), item.label)
                .state(if checked {
                    State::Checked
                } else {
                    State::Unchecked
                })
                .group_item()
                .variant(item.variant)
                .disabled(self.disabled || item.disabled)
                .control_first(self.control_first)
                .when_some(handler, |this, handler| {
                    this.on_change(move |state, _, window, cx| {
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
            .aria_label(self.name.clone())
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
            .when_some(self.error, |this, error| {
                this.child(div().line_height(theme.typography.sm.line_height).child(
                    crate::Text::new("error", error).style(crate::text::Style::Copy {
                        tone: crate::text::Tone::Error,
                        size: crate::text::Size::Sm,
                        bold: false,
                    }),
                ))
            })
            .when_some(self.description, |this, description| {
                this.child(div().line_height(theme.typography.sm.line_height).child(
                    crate::Text::new("description", description).style(crate::text::Style::Copy {
                        tone: crate::text::Tone::Secondary,
                        size: crate::text::Size::Sm,
                        bold: false,
                    }),
                ))
            })
    }
}
#[cfg(test)]
#[path = "checkbox_group_tests.rs"]
mod tests;
