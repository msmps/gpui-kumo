//! Kumo Checkbox presentation over Base's controlled toggle and focus behavior.
use crate::{Label, theme};
use gpui_kit::{
    AnyElement, App, ClickEvent, ElementId, FocusHandle, FontWeight, HitboxBehavior, IntoElement,
    ParentElement, RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window, base,
    canvas, div, prelude::FluentBuilder, px, quad, svg,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
/// Three-state checkbox value.
pub enum State {
    #[default]
    /// No selection.
    Unchecked,
    /// Selected.
    Checked,
    /// Mixed selection.
    Indeterminate,
}
impl State {
    fn base(self) -> base::CheckboxState {
        match self {
            Self::Unchecked => base::CheckboxState::Unchecked,
            Self::Checked => base::CheckboxState::Checked,
            Self::Indeterminate => base::CheckboxState::Indeterminate,
        }
    }
    fn from_base(state: base::CheckboxState) -> Self {
        match state {
            base::CheckboxState::Unchecked => Self::Unchecked,
            base::CheckboxState::Checked => Self::Checked,
            base::CheckboxState::Indeterminate => Self::Indeterminate,
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
/// Closed semantic visual treatments supported by this component.
pub enum Variant {
    #[default]
    /// Default semantic treatment.
    Default,
    /// Error treatment.
    Error,
}
type ChangeHandler = Box<dyn Fn(State, &ClickEvent, &mut Window, &mut App)>;
/// A controlled checkbox. Its complete readable name is required even when bare.
/// Rich labels are decorative; interactive content belongs outside this control.
#[derive(IntoElement)]
#[must_use]
pub struct Checkbox {
    id: ElementId,
    name: SharedString,
    accessible_name: Option<SharedString>,
    state: State,
    variant: Variant,
    disabled: bool,
    show_label: bool,
    control_first: bool,
    optional: bool,
    content: Option<AnyElement>,
    focus: Option<FocusHandle>,
    group_item: bool,
    on_change: Option<ChangeHandler>,
}
impl Checkbox {
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
    /// Create a named controlled checkbox, initially unchecked.
    ///
    /// # Panics
    /// Panics when the required name or label is blank.
    pub fn new(id: impl Into<ElementId>, name: impl Into<SharedString>) -> Self {
        let name = name.into();
        assert!(!name.trim().is_empty(), "Checkbox requires a readable name");
        Self {
            id: id.into(),
            name,
            accessible_name: None,
            state: State::default(),
            variant: Variant::default(),
            disabled: false,
            show_label: true,
            control_first: true,
            optional: false,
            content: None,
            focus: None,
            group_item: false,
            on_change: None,
        }
    }
    pub(crate) fn group_item(mut self) -> Self {
        self.group_item = true;
        self
    }
    /// Supply the controlled value state; the owner commits user proposals.
    pub fn state(mut self, state: State) -> Self {
        self.state = state;
        self
    }
    /// Select the semantic visual treatment; interaction and value state remain independent.
    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }
    /// Choose whether this control accepts user activation.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    /// Keep the accessible name while omitting visible label presentation.
    pub fn show_label(mut self, show: bool) -> Self {
        self.show_label = show;
        self
    }
    /// Choose whether controls precede their labels in the native layout.
    pub fn control_first(mut self, first: bool) -> Self {
        self.control_first = first;
        self
    }
    /// Show or hide “(optional)”; validation remains caller-owned.
    pub fn optional_indicator(mut self, optional: bool) -> Self {
        self.optional = optional;
        self
    }
    /// Decorative rich label; the constructor's string remains its complete
    /// accessible name. Keep interactive accessories outside the control.
    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }
    /// Use the supplied retained focus handle instead of keyed default focus.
    pub fn track_focus(mut self, focus: &FocusHandle) -> Self {
        self.focus = Some(focus.clone());
        self
    }
    /// Proposes the next state once through Base; the owner must apply it and redraw.
    pub fn on_change(
        mut self,
        handler: impl Fn(State, &ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}
impl RenderOnce for Checkbox {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = theme(cx).clone();
        let focus = self.focus.unwrap_or_else(|| {
            window
                .use_keyed_state(self.id.clone(), cx, |_, cx| cx.focus_handle())
                .read(cx)
                .clone()
        });
        let selected = self.state != State::Unchecked;
        let fill = if selected {
            theme.colors.contrast
        } else {
            theme.colors.base
        };
        let rest_ring = if selected {
            theme.colors.contrast
        } else if self.variant == Variant::Error {
            theme.colors.danger
        } else {
            theme.colors.hairline
        };
        let disabled = self.disabled;
        let ring_focus = focus.clone();
        let ring = canvas(
            |bounds, window, _| window.insert_hitbox(bounds, HitboxBehavior::Normal),
            move |bounds, hitbox, window, cx| {
                let focused = !disabled && ring_focus.is_focused(window);
                let color = if focused {
                    if window.last_input_was_keyboard() {
                        theme.colors.brand
                    } else {
                        theme.colors.focus
                    }
                } else if !disabled
                    && !selected
                    && !cx.has_active_drag()
                    && hitbox.is_hovered(window)
                {
                    theme.colors.hairline
                } else {
                    rest_ring
                };
                let width = if focused {
                    theme.effects.keyboard_focus_ring_width
                } else {
                    theme.effects.control_ring_width
                };
                window.paint_quad(quad(
                    bounds.dilate(width),
                    theme.radii.sm + width,
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
        let icon = match self.state {
            State::Unchecked => None,
            State::Checked => Some(include_bytes!("../../assets/checkbox-check.svg").as_slice()),
            State::Indeterminate => {
                Some(include_bytes!("../../assets/checkbox-minus.svg").as_slice())
            }
        };
        let indicator = base::CheckboxIndicator::new()
            .state(self.state.base())
            .disabled(disabled)
            .size(px(16.))
            .flex_shrink_0()
            .relative()
            .flex()
            .items_center()
            .justify_center()
            .rounded(theme.radii.sm)
            .bg(fill)
            .text_color(theme.text.inverse)
            .opacity(if disabled && !self.group_item {
                0.5
            } else {
                1.
            })
            .when(self.show_label, |this| this.mt(px(2.)))
            .when_some(icon, |this, icon| {
                this.child(
                    svg()
                        .data(icon)
                        .size(px(12.))
                        .text_color(theme.text.inverse),
                )
            })
            .child(ring);
        let name = if self.optional {
            format!("{} (optional)", self.name).into()
        } else {
            self.name.clone()
        };
        let mut root = base::Checkbox::new(self.id)
            .state(self.state.base())
            .disabled(disabled)
            .track_focus(&focus)
            .accessibility_label(self.accessible_name.unwrap_or(name))
            .a11y_synthetic_children(move |builder| {
                if disabled {
                    builder.parent_node().set_disabled();
                }
            })
            .flex()
            .opacity(if disabled && self.group_item { 0.5 } else { 1. })
            .items_start()
            .text_size(theme.typography.base.size)
            .line_height(theme.typography.base.line_height)
            .font_weight(FontWeight::NORMAL)
            .text_color(theme.text.default)
            .gap(theme.spacing.eight)
            .min_w_0()
            .max_w_full()
            .self_start()
            .when(!self.control_first, |this| this.flex_row_reverse())
            .when(disabled, |this| this.cursor_not_allowed())
            .when(!disabled, |this| this.cursor_pointer())
            .child(indicator)
            .when(self.show_label, |this| {
                this.child(
                    Label::new("label", self.name)
                        .as_content()
                        .optional(self.optional)
                        .when_some(self.content, |label, content| label.content(content)),
                )
            });
        if let Some(handler) = self.on_change {
            root = root.on_change(move |state, event, window, cx| {
                handler(State::from_base(state), event, window, cx)
            });
        }
        div().flex().min_w_0().max_w_full().child(root)
    }
}
impl std::fmt::Debug for Checkbox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Checkbox")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("variant", &self.variant)
            .field("disabled", &self.disabled)
            .field("show_label", &self.show_label)
            .field("control_first", &self.control_first)
            .field("optional", &self.optional)
            .field("group_item", &self.group_item)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests;
