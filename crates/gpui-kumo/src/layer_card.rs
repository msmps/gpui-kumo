//! Simple and layered Kumo card containers with typed section composition.

#![deny(missing_docs)]

use std::{cell::Cell, rc::Rc};

use gpui_kit::base::TestSupportExt;
use gpui_kit::{
    AbsoluteLength, AnyElement, App, Bounds, BoxShadow, ContentMask, Corners, ElementId,
    FontWeight, Hsla, InteractiveElement, IntoElement, Overflow, ParentElement, Pixels, Refineable,
    RenderOnce, StyleRefinement, Styled, Window, canvas, div, px, quad,
};

/// A consumed card container. Direct children form a simple surface; adding a
/// typed section selects the layered recipe. Caller styles override the recipe.
/// The owning view observes Theme to refresh retained presentation. Sections
/// preserve their descendants' focus and accessibility; the card adds no role.
///
/// ```
/// use gpui_kit::{ParentElement, Styled, px};
/// use gpui_kumo::{LayerCard, Text, layer_card::Section};
/// let card = LayerCard::new("guide").w(px(250.))
///     .section(Section::secondary("header").child(Text::new("title", "Next steps")))
///     .section(Section::primary("body").child(Text::new("description", "Get started")));
/// ```
#[derive(IntoElement)]
#[must_use]
pub struct LayerCard {
    id: ElementId,
    children: Vec<CardContent>,
    layered: bool,
    style: StyleRefinement,
}

#[cfg(test)]
#[path = "layer_card_tests.rs"]
mod tests;

impl LayerCard {
    /// Create an empty simple card with scoped identity and no default padding.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            children: Vec::new(),
            layered: false,
            style: Default::default(),
        }
    }

    /// Append a typed primary or secondary section and select the layered recipe.
    /// Use this method before type erasure; `child` treats content as unstructured.
    pub fn section(mut self, section: Section) -> Self {
        self.layered = true;
        self.children.push(CardContent::Section(Box::new(section)));
        self
    }
}

impl Styled for LayerCard {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for LayerCard {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children
            .extend(elements.into_iter().map(CardContent::Element));
    }
}

impl RenderOnce for LayerCard {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = crate::theme(cx);
        let ring = BoxShadow::new(
            px(0.),
            px(0.),
            if self.layered {
                theme.colors.hairline
            } else {
                theme.colors.line
            },
        )
        .spread_radius(px(1.));
        let mut shadows = vec![ring];
        let mut element = div()
            .id(self.id)
            .test_support()
            .overflow_hidden()
            .rounded(theme.radii.lg);
        if self.layered {
            element = element
                .flex()
                .flex_col()
                .w_full()
                .bg(theme.colors.elevated)
                .text_size(theme.typography.base.size)
                .line_height(theme.typography.base.line_height);
        } else {
            shadows.extend(theme.effects.shadow_xs.iter().cloned());
            element = element.bg(theme.colors.base);
        }
        element = element.shadow(shadows);
        element.style().refine(&self.style);
        // GPUI's overflow mask is rectangular. Preserve the root geometry so
        // secondary fills can intersect its rounded shape despite negative margins.
        let rounded_overflow = self.layered
            && element.style().overflow.x != Some(Overflow::Visible)
            && element.style().overflow.y != Some(Overflow::Visible);
        let clip = rounded_overflow.then(|| RootClip {
            bounds: Rc::new(Cell::new(Bounds::default())),
            radii: Corners::<AbsoluteLength>::default()
                .refined(element.style().corner_radii.clone()),
        });
        if let Some(clip) = &clip {
            let bounds = Rc::clone(&clip.bounds);
            element = element.child(
                canvas(move |rect, _, _| bounds.set(rect), |_, _, _, _| {})
                    .absolute()
                    .inset_0()
                    .size_full(),
            );
        }
        element.children(self.children.into_iter().map(|child| match child {
            CardContent::Element(element) => element,
            CardContent::Section(mut section) => {
                section.root_clip = clip.clone();
                section.into_any_element()
            }
        }))
    }
}

enum CardContent {
    Element(AnyElement),
    Section(Box<Section>),
}

#[derive(Clone)]
struct RootClip {
    bounds: Rc<Cell<Bounds<Pixels>>>,
    radii: Corners<AbsoluteLength>,
}

#[derive(Clone, Copy)]
enum Kind {
    Primary,
    Secondary,
}

/// A card section with caller-owned content and styling refinements.
#[derive(IntoElement)]
#[must_use]
pub struct Section {
    id: ElementId,
    kind: Kind,
    root_clip: Option<RootClip>,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl Section {
    /// Primary content: base surface, clipped rounded edges and vertical layout.
    pub fn primary(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            kind: Kind::Primary,
            root_clip: None,
            children: Vec::new(),
            style: Default::default(),
        }
    }

    /// Secondary header/footer: elevated surface and supporting medium text.
    pub fn secondary(id: impl Into<ElementId>) -> Self {
        Self {
            kind: Kind::Secondary,
            ..Self::primary(id)
        }
    }
}

impl Styled for Section {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Section {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Section {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = crate::theme(cx);
        let mut element = div()
            .id(self.id)
            .test_support()
            .flex()
            .gap(theme.spacing.eight)
            .p(theme.spacing.sixteen);
        element = match self.kind {
            Kind::Primary => element
                .relative()
                .flex_col()
                .overflow_hidden()
                .rounded(theme.radii.lg)
                .bg(theme.colors.base)
                .pr(theme.spacing.twelve)
                .shadow(vec![
                    BoxShadow::new(px(0.), px(0.), theme.colors.fill).spread_radius(px(1.)),
                ]),
            Kind::Secondary => element
                .items_center()
                .my(-theme.spacing.eight)
                .bg(theme.colors.elevated)
                .text_size(theme.typography.base.size)
                .line_height(theme.typography.base.line_height)
                .font_weight(FontWeight::MEDIUM)
                .text_color(theme.text.subtle),
        };
        element.style().refine(&self.style);
        if let (Kind::Secondary, Some(clip)) = (self.kind, self.root_clip) {
            let radii = Corners::<AbsoluteLength>::default()
                .refined(element.style().corner_radii.clone())
                .to_pixels(window.rem_size());
            let solid = element
                .style()
                .background
                .as_ref()
                .and_then(|fill| fill.color())
                .and_then(|color| color.as_solid());
            // Keep custom section rounding and gradients on their normal path;
            // general rounded subtree clipping is tracked in KUMO-019.
            if radii == Corners::all(px(0.))
                && let Some(color) = solid
            {
                element.style().background = None;
                element = element.child(
                    canvas(
                        |_, _, _| {},
                        move |bounds, _, window, _| {
                            let root = clip.bounds.get();
                            window.with_content_mask(Some(ContentMask { bounds }), |window| {
                                window.paint_quad(quad(
                                    root,
                                    clip.radii.to_pixels(window.rem_size()),
                                    color,
                                    px(0.),
                                    Hsla::transparent_black(),
                                    Default::default(),
                                ));
                            });
                        },
                    )
                    .absolute()
                    .inset_0()
                    .size_full(),
                );
            }
        }
        element.children(self.children)
    }
}
