//! Supported Kumo status/category labels with typed filled and dot composition.

#![deny(missing_docs)]

use gpui_kit::base::TestSupportExt;
use gpui_kit::{
    AbsoluteLength, AnyElement, App, Bounds, Corners, Edges, ElementId, FontWeight,
    InteractiveElement, IntoElement, ParentElement, Refineable, RenderOnce, Role, SharedString,
    StatefulInteractiveElement, StyleRefinement, Styled, Window, canvas, div, point, px, quad,
    size,
};

/// Supported source variants; deprecated compatibility variants are excluded.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Variant {
    /// Inverted emphasis.
    #[default]
    Primary,
    /// Neutral supporting category.
    Secondary,
    /// Semantic error tint.
    Error,
    /// Semantic warning tint.
    Warning,
    /// Semantic success tint.
    Success,
    /// Semantic information tint.
    Info,
    /// Dashed experimental marker.
    Beta,
    /// Base surface with a solid border.
    Outline,
    /// Solid red.
    Red,
    /// Solid green.
    Green,
    /// Solid neutral.
    Neutral,
    /// Solid orange with dark text.
    Orange,
    /// Solid purple.
    Purple,
    /// Solid teal.
    Teal,
    /// Subtle teal text; the pinned background token is absent.
    TealSubtle,
    /// Solid blue.
    Blue,
}

/// Filled presentation, optionally containing a decorative icon.
#[derive(Default)]
pub struct Filled {
    icon: Option<AnyElement>,
}

/// Dot presentation; icon composition is unavailable on this type.
pub struct Dot;

/// A consumed, read-only label. The owner observes Theme when caching it.
/// Rich content and icons are decorative; the complete label names the badge.
///
/// ```
/// use gpui_kumo::{Badge, badge::Variant};
/// let category = Badge::new("category", "New").variant(Variant::Secondary);
/// let status = Badge::dot("health", "Healthy", Variant::Success);
/// ```
///
/// Dot badges intentionally cannot accept icons:
/// ```compile_fail
/// use gpui_kumo::{Badge, badge::Variant};
/// Badge::dot("health", "Healthy", Variant::Success).icon(gpui_kit::div());
/// ```
#[must_use]
pub struct Badge<P = Filled> {
    id: ElementId,
    label: SharedString,
    variant: Variant,
    presentation: P,
    rich_content: Option<AnyElement>,
    link_hover_group: Option<SharedString>,
    style: StyleRefinement,
}

impl Badge<Filled> {
    /// Create a primary filled badge with identity scoped to its ancestor.
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self::build(id, label, Variant::Primary, Filled::default())
    }

    /// Add a decorative icon in the source's 12px slot; SVGs should fill the slot.
    pub fn icon(mut self, icon: impl IntoElement) -> Self {
        self.presentation.icon = Some(icon.into_any_element());
        self
    }
}

impl Badge<Dot> {
    /// Create a dot appearance. Only Success, Warning, Error and Neutral show a
    /// colored dot; other supported variants preserve the outlined label alone.
    pub fn dot(id: impl Into<ElementId>, label: impl Into<SharedString>, variant: Variant) -> Self {
        Self::build(id, label, variant, Dot)
    }
}

impl<P> Badge<P> {
    fn build(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        variant: Variant,
        presentation: P,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            variant,
            presentation,
            rich_content: None,
            link_hover_group: None,
            style: Default::default(),
        }
    }

    /// Select a supported color variant without changing the presentation type.
    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    /// Replace visible text with decorative inline composition. The original
    /// label remains the full accessible name; keep interactive content outside.
    pub fn rich_content(mut self, content: impl IntoElement) -> Self {
        self.rich_content = Some(content.into_any_element());
        self
    }

    /// Opt into the source's ancestor-link hover ring using the named GPUI
    /// group on the caller's link. Navigation/focus remain owned by that link.
    pub fn link_hover_group(mut self, group: impl Into<SharedString>) -> Self {
        self.link_hover_group = Some(group.into());
        self
    }

    fn render_badge(self, icon: Option<AnyElement>, dot: bool, cx: &mut App) -> impl IntoElement {
        let theme = crate::theme(cx);
        let badge = &theme.badge;
        let (background, foreground) = match self.variant {
            Variant::Primary => (Some(badge.inverted), badge.inverted_foreground),
            Variant::Secondary => (Some(theme.colors.fill), badge.neutral_subtle_foreground),
            Variant::Error => (Some(theme.colors.danger_tint), theme.text.danger),
            Variant::Warning => (Some(theme.colors.warning_tint), theme.text.warning),
            Variant::Success => (Some(theme.colors.success_tint), theme.text.success),
            Variant::Info => (Some(theme.colors.info_tint), theme.text.info),
            Variant::Beta => (None, theme.text.link),
            Variant::Outline => (Some(theme.colors.base), theme.text.default),
            Variant::Red => (Some(badge.red), badge.solid_foreground),
            Variant::Green => (Some(badge.green), badge.solid_foreground),
            Variant::Neutral => (Some(badge.neutral), badge.solid_foreground),
            Variant::Orange => (Some(badge.orange), badge.orange_foreground),
            Variant::Purple => (Some(badge.purple), badge.solid_foreground),
            Variant::Teal => (Some(badge.teal), badge.solid_foreground),
            Variant::TealSubtle => (None, badge.teal_subtle_foreground),
            Variant::Blue => (Some(badge.blue), badge.solid_foreground),
        };
        let foreground = if dot { theme.text.default } else { foreground };
        let mut element = div()
            .id(self.id)
            .test_support()
            .role(Role::Label)
            .aria_label(self.label.clone())
            .aria_value(self.label.clone())
            .flex()
            .flex_none()
            .self_start()
            .items_center()
            .gap(theme.spacing.four)
            .rounded_full()
            .px(theme.spacing.eight)
            .py(px(2.))
            .text_size(theme.typography.xs.size)
            .line_height(theme.typography.xs.line_height)
            .font_weight(FontWeight::MEDIUM)
            .text_color(foreground)
            .whitespace_nowrap();
        if dot {
            element = element.gap(theme.spacing.six);
            let color = match self.variant {
                Variant::Success => Some(theme.colors.success),
                Variant::Warning => Some(badge.orange),
                Variant::Error => Some(badge.red),
                Variant::Neutral => Some(badge.neutral),
                _ => None,
            };
            if let Some(color) = color {
                element = element.child(
                    div()
                        .id("dot")
                        .test_support()
                        .size(px(7.))
                        .flex_none()
                        .rounded_full()
                        .bg(color),
                );
            }
        } else {
            if let Some(background) = background {
                element = element.bg(background);
            }
            match self.variant {
                Variant::Beta => {
                    element = element
                        .border_1()
                        .border_dashed()
                        .border_color(theme.colors.brand)
                }
                Variant::Outline => element = element.border_1().border_color(theme.colors.fill),
                _ => {}
            }
            if let Some(icon) = icon {
                element = element.pl(theme.spacing.six).child(
                    div()
                        .id("icon")
                        .test_support()
                        .w(theme.spacing.twelve)
                        .h(theme.typography.xs.line_height)
                        .flex_none()
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(div().size(theme.spacing.twelve).child(icon)),
                );
            }
        }
        element.style().refine(&self.style);
        let foreground = element.style().text.color.unwrap_or(foreground);
        if dot || self.link_hover_group.is_some() {
            let radii =
                Corners::<AbsoluteLength>::default().refined(element.style().corner_radii.clone());
            let borders =
                Edges::<AbsoluteLength>::default().refined(element.style().border_widths.clone());
            let color = if dot {
                theme.colors.hairline
            } else {
                foreground
            };
            let mut ring = div().absolute().inset_0().size_full().text_color(color);
            if !dot {
                ring = ring.invisible();
            }
            if let Some(group) = self.link_hover_group {
                ring = ring.group_hover(group, |style| style.visible().text_color(foreground));
            }
            element = element.child(
                ring.child(
                    canvas(
                        |_, window, _| window.text_style().color,
                        move |bounds, color, window, _| {
                            let border = borders.to_pixels(window.rem_size());
                            let root = Bounds::new(
                                bounds.origin - point(border.left, border.top),
                                size(
                                    bounds.size.width + border.left + border.right,
                                    bounds.size.height + border.top + border.bottom,
                                ),
                            );
                            // A shadow fills transparent interiors. A border-only quad
                            // preserves the source ring without consuming layout space.
                            let outer = root.dilate(px(1.));
                            window.paint_quad(quad(
                                outer,
                                radii
                                    .to_pixels(window.rem_size())
                                    .map(|radius| *radius + px(1.))
                                    .clamp_radii_for_quad_size(outer.size),
                                color.alpha(0.),
                                px(1.),
                                color,
                                Default::default(),
                            ));
                        },
                    )
                    .size_full(),
                ),
            );
        }
        element.child(
            self.rich_content
                .unwrap_or_else(|| self.label.into_any_element()),
        )
    }
}

impl<P> Styled for Badge<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Badge<Filled> {
    fn render(mut self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let icon = self.presentation.icon.take();
        self.render_badge(icon, false, cx)
    }
}

impl RenderOnce for Badge<Dot> {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        self.render_badge(None, true, cx)
    }
}

impl<P: 'static> IntoElement for Badge<P>
where
    Self: RenderOnce,
{
    type Element = gpui_kit::ViewElement<Self>;

    fn into_element(self) -> Self::Element {
        gpui_kit::ViewElement::new(self)
    }
}

#[cfg(test)]
#[path = "badge_tests.rs"]
mod tests;
