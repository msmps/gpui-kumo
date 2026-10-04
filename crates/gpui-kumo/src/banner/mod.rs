//! Structured Kumo messages with typed, context-resolved actions.

#![deny(missing_docs)]

use crate::{Button, Link, Theme, button};
use gpui_kit::base::TestSupportExt;
use gpui_kit::{
    AnyElement, App, ClickEvent, ElementId, FocusHandle, FontWeight, Hsla, InteractiveElement,
    IntoElement, ParentElement, Refineable, RenderOnce, Role, SharedString,
    StatefulInteractiveElement, StyleRefinement, Styled, Window, div, prelude::FluentBuilder, px,
    relative,
};

/// Supported contextual-message treatments.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Variant {
    /// General informational message.
    #[default]
    Info,
    /// Caution or warning.
    Alert,
    /// Critical error.
    Error,
    /// Secondary neutral message.
    Secondary,
}

/// Message spacing and action dimensions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Size {
    /// Stacked content, with small actions.
    #[default]
    Base,
    /// Compact wrapping content, with extra-small actions.
    Sm,
}

/// Accent-aware action treatment.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ActionVariant {
    /// Filled accent gradient.
    #[default]
    Primary,
    /// Transparent accent outline.
    Secondary,
    /// Transparent text action.
    Ghost,
}

/// A Banner-owned action specification. The Banner supplies its size and accent;
/// Button retains activation, focus, accessibility and availability behavior.
#[must_use]
pub struct Action {
    button: Button,
    variant: ActionVariant,
}

impl Styled for Action {
    /// Refine the Button surface after the resolved banner recipe.
    fn style(&mut self) -> &mut StyleRefinement {
        self.button.style()
    }
}

impl Action {
    /// Set visible text and its default accessible name.
    ///
    /// # Panics
    /// Panics when `text` is blank.
    pub fn label(mut self, text: impl Into<SharedString>) -> Self {
        self.button = self.button.label(text);
        self
    }
    /// Choose visible text presentation while retaining the action's name.
    pub fn show_label(mut self, show: bool) -> Self {
        self.button = self.button.show_label(show);
        self
    }
    /// Create a named primary action; the Banner supplies its size and accent.
    ///
    /// # Panics
    /// Panics when `label` is blank.
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            button: Button::new(id, label),
            variant: ActionVariant::default(),
        }
    }
    /// Create an icon-only action with a complete accessible name.
    ///
    /// # Panics
    /// Panics when the required accessible name is blank.
    pub fn icon(
        id: impl Into<ElementId>,
        name: impl Into<SharedString>,
        icon: impl IntoElement,
    ) -> Self {
        Self {
            button: Button::icon(id, name, icon),
            variant: ActionVariant::default(),
        }
    }
    /// Select primary, secondary or ghost presentation.
    pub fn variant(mut self, variant: ActionVariant) -> Self {
        self.variant = variant;
        self
    }
    /// Reject activation and keyboard traversal.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.button = self.button.disabled(disabled);
        self
    }
    /// Show the shared Loader and reject activation while loading.
    pub fn loading(mut self, loading: bool) -> Self {
        self.button = self.button.loading(loading);
        self
    }
    /// Add a decorative leading icon.
    pub fn leading_icon(mut self, icon: impl IntoElement) -> Self {
        self.button = self.button.leading_icon(icon);
        self
    }
    /// Add a decorative trailing icon.
    pub fn trailing_icon(mut self, icon: impl IntoElement) -> Self {
        self.button = self.button.trailing_icon(icon);
        self
    }
    /// Refine the Button shape; size remains owned by Banner.
    pub fn shape(mut self, shape: button::Shape) -> Self {
        self.button = self.button.shape(shape);
        self
    }
    /// Override the complete accessible name.
    ///
    /// # Panics
    /// Panics when the name is blank.
    pub fn accessibility_label(mut self, name: impl Into<SharedString>) -> Self {
        self.button = self.button.accessibility_label(name);
        self
    }
    /// Bind an application-retained focus handle.
    pub fn track_focus(mut self, focus: &FocusHandle) -> Self {
        self.button = self.button.track_focus(focus);
        self
    }
    /// Handle one Base activation through pointer, keyboard or accessible input.
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.button = self.button.on_click(handler);
        self
    }

    fn resolve(self, variant: Variant, size: Size, foreground: Hsla, theme: &Theme) -> Button {
        let accent = variant.icon_color(theme);
        let neutral = variant == Variant::Secondary;
        let emphasis = match variant {
            Variant::Info => &theme.banner.info,
            Variant::Alert => &theme.banner.warning,
            Variant::Error => &theme.destructive,
            Variant::Secondary => &theme.banner.secondary,
        };
        let primary = self.variant == ActionVariant::Primary;
        self.button
            .variant(match self.variant {
                ActionVariant::Primary => button::Variant::Primary,
                ActionVariant::Secondary => button::Variant::Outline,
                ActionVariant::Ghost => button::Variant::Ghost,
            })
            .size(if size == Size::Sm {
                button::Size::Xs
            } else {
                button::Size::Sm
            })
            .accent(button::AccentRecipe {
                emphasis: primary.then(|| emphasis.clone()),
                foreground: if primary {
                    gpui_kit::white()
                } else {
                    foreground
                },
                unavailable_foreground: theme.text.subtle,
                icon_foreground: if primary {
                    gpui_kit::white()
                } else if neutral {
                    theme.text.subtle
                } else {
                    accent
                },
                ring: (self.variant == ActionVariant::Secondary).then(|| {
                    if neutral {
                        theme.colors.focus.opacity(0.2)
                    } else {
                        accent.opacity(0.5)
                    }
                }),
                hover_background: if neutral {
                    theme.colors.contrast
                } else {
                    accent
                }
                .opacity(0.1),
            })
    }
}

impl Variant {
    fn foreground(self, theme: &Theme) -> Hsla {
        match self {
            Self::Info => theme.text.info,
            Self::Alert => theme.text.warning,
            Self::Error => theme.text.danger,
            Self::Secondary => theme.text.default.opacity(0.7),
        }
    }
    fn background(self, theme: &Theme) -> Hsla {
        match self {
            Self::Info => theme.colors.info_tint,
            Self::Alert => theme.colors.warning_tint,
            Self::Error => theme.colors.danger_tint,
            Self::Secondary => theme.colors.contrast.opacity(0.05),
        }
    }
    fn icon_color(self, theme: &Theme) -> Hsla {
        match self {
            Self::Info => theme.colors.info,
            Self::Alert => theme.colors.warning,
            Self::Error => theme.colors.danger,
            Self::Secondary => theme.colors.interact,
        }
    }
}

enum Description {
    Text(SharedString),
    Content(AnyElement),
}
enum ActionContent {
    Accent(Action),
    Link(Link),
    Custom(AnyElement),
}

/// A static contextual message. Actions retain their own semantics; the root
/// adds no live-region role, focus target, dismissal state or activation.
/// The owner observes Theme and owns any callback-driven state.
///
/// ```
/// use gpui_kumo::{Banner, banner::{Action, Variant}};
/// let banner = Banner::new("update")
///     .variant(Variant::Info)
///     .title("Update available")
///     .description("A new version is ready to install.")
///     .action(Action::new("install", "Install").on_click(|_, _, _| {}));
/// ```
#[derive(IntoElement)]
#[must_use]
pub struct Banner {
    id: ElementId,
    variant: Variant,
    size: Size,
    title: Option<SharedString>,
    description: Option<Description>,
    icon: Option<AnyElement>,
    actions: Vec<ActionContent>,
    style: StyleRefinement,
}

impl Banner {
    /// Create an empty structured message; populate title and/or description.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            variant: Variant::default(),
            size: Size::default(),
            title: None,
            description: None,
            icon: None,
            actions: Vec::new(),
            style: Default::default(),
        }
    }
    /// Select the semantic treatment.
    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }
    /// Select base or compact layout; actions inherit the corresponding size.
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
    /// Set the primary message text. It is a paragraph, without heading semantics.
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        let title = title.into();
        self.title = (!title.is_empty()).then_some(title);
        self
    }
    /// Set plain description text with its complete accessible name.
    pub fn description(mut self, text: impl Into<SharedString>) -> Self {
        let text = text.into();
        self.description = (!text.is_empty()).then_some(Description::Text(text));
        self
    }
    /// Set rich description content; the caller owns its semantics and layout.
    pub fn description_content(mut self, content: impl IntoElement) -> Self {
        self.description = Some(Description::Content(content.into_any_element()));
        self
    }
    /// Set a decorative icon. Its content owns width; the wrapper supplies height
    /// and the general semantic fill foreground, distinct from message text.
    pub fn icon(mut self, icon: impl IntoElement) -> Self {
        self.icon = Some(icon.into_any_element());
        self
    }
    /// Append an accent-aware action, resolving its context before erasure.
    pub fn action(mut self, action: Action) -> Self {
        self.actions.push(ActionContent::Accent(action));
        self
    }
    /// Append a typed Link action. A sole Link is inline in compact mode.
    pub fn link_action(mut self, link: Link) -> Self {
        self.actions.push(ActionContent::Link(link));
        self
    }
    /// Append caller-owned trailing content without imposing Button recipes.
    pub fn custom_action(mut self, action: impl IntoElement) -> Self {
        self.actions
            .push(ActionContent::Custom(action.into_any_element()));
        self
    }
}

impl Styled for Banner {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Banner {
    fn render(mut self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = crate::theme(cx);
        let compact = self.size == Size::Sm;
        let typography = if compact {
            theme.typography.sm
        } else {
            theme.typography.base
        };
        let foreground = self
            .style
            .text
            .color
            .unwrap_or(self.variant.foreground(theme));
        let font_size = self
            .style
            .text
            .font_size
            .unwrap_or(typography.size.into())
            .to_pixels(window.rem_size());
        let mut root = div()
            .id(self.id)
            .test_support()
            .flex()
            .w_full()
            .gap(if compact {
                theme.spacing.eight
            } else {
                theme.spacing.twelve
            })
            .px(if compact {
                theme.spacing.twelve
            } else {
                theme.spacing.sixteen
            })
            .py(if compact {
                theme.spacing.eight
            } else {
                theme.spacing.twelve
            })
            .rounded(if compact {
                theme.radii.md
            } else {
                theme.radii.lg
            })
            .text_size(typography.size)
            .line_height(typography.line_height)
            .text_color(foreground)
            .bg(self.variant.background(theme))
            .when(compact, |root| root.items_center())
            .when(!compact, |root| root.items_start());
        root.style().refine(&self.style);
        if self.title.is_none() && self.description.is_none() {
            return root;
        }
        if let Some(icon) = self.icon {
            root = root.child(
                div()
                    .id("icon")
                    .test_support()
                    .flex()
                    .flex_none()
                    .items_center()
                    .h(font_size * if compact { 1.25 } else { 1.375 })
                    .text_color(self.variant.icon_color(theme))
                    .child(icon),
            );
        }
        let inline =
            compact && self.actions.len() == 1 && matches!(self.actions[0], ActionContent::Link(_));
        let mut content = div()
            .id("content")
            .test_support()
            .min_w_0()
            .line_height(relative(1.375));
        if compact {
            content = content
                .flex()
                .flex_wrap()
                .items_baseline()
                .gap_x(theme.spacing.six);
        } else {
            content = content.flex().flex_col().gap(px(2.));
        }
        let has_title = self.title.is_some();
        if let Some(title) = self.title {
            content = content.child(
                div()
                    .id("title")
                    .test_support()
                    .role(Role::Label)
                    .aria_label(title.clone())
                    .aria_value(title.clone())
                    .min_w_0()
                    .whitespace_normal()
                    .font_weight(FontWeight::MEDIUM)
                    .child(title),
            );
        }
        if let Some(description) = self.description {
            let mut slot = div()
                .id("description")
                .test_support()
                .min_w_0()
                .text_size(theme.typography.sm.size)
                .line_height(relative(1.375))
                .whitespace_normal();
            slot = match description {
                Description::Text(text) => slot
                    .role(Role::Label)
                    .aria_label(text.clone())
                    .aria_value(text.clone())
                    .child(text),
                Description::Content(content) => slot.child(content),
            };
            content = content.child(slot);
        }
        if inline && let ActionContent::Link(link) = self.actions.remove(0) {
            content = content.child(link);
        }
        let mut row = div()
            .id("row")
            .test_support()
            .flex()
            .flex_auto()
            .min_w_0()
            .items_center()
            .justify_between()
            .gap(if compact {
                theme.spacing.eight
            } else {
                theme.spacing.twelve
            })
            .when(!has_title, |row| row.pt(px(1.)))
            .child(content);
        if !self.actions.is_empty() {
            row = row.child(
                div()
                    .id("actions")
                    .test_support()
                    .flex()
                    .flex_none()
                    .items_center()
                    .gap(theme.spacing.eight)
                    .children(self.actions.into_iter().map(|action| {
                        match action {
                            ActionContent::Accent(action) => action
                                .resolve(self.variant, self.size, foreground, theme)
                                .into_any_element(),
                            ActionContent::Link(link) => link.into_any_element(),
                            ActionContent::Custom(content) => content,
                        }
                    })),
            );
        }
        root.child(row)
    }
}

debug_struct!(Action { button, variant });

debug_struct!(Banner {
    id,
    variant,
    size,
    actions_count: |this| this.actions.len()
});

#[cfg(test)]
mod tests;
