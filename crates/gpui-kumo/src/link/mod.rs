//! Kumo navigation links with GPUI-owned focus and activation.

#![deny(missing_docs)]

use gpui_kit::base::TestSupportExt;
use gpui_kit::{
    AbsoluteLength, AnyElement, App, Bounds, ClickEvent, Corners, Edges, ElementId,
    InteractiveElement, IntoElement, ParentElement, Refineable, RenderOnce, SharedString,
    StatefulInteractiveElement, StyleRefinement, Styled, UnderlineStyle, Window, canvas, div,
    point, quad, size, svg,
};

/// Supported link presentation variants.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Variant {
    /// Link foreground and an underline.
    #[default]
    Inline,
    /// Inherited foreground and an underline.
    Current,
    /// Link foreground without an underline; foreground fades on hover.
    Plain,
}

/// An application-owned navigation request. The activation preserves pointer,
/// keyboard and modifier information so applications can choose their routing.
#[derive(Clone, Debug)]
pub struct NavigationRequest {
    /// Destination data, using the source's href semantics.
    pub href: SharedString,
    /// The actual activation that requested navigation.
    pub activation: ClickEvent,
}

type NavigateHandler = Box<dyn Fn(&NavigationRequest, &mut Window, &mut App)>;
type ActivationHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App)>;
const BADGE_GROUP: &str = "kumo-link";

/// A consumed navigation link with a complete accessible label and destination.
/// The owner observes Theme. Navigation is injected; href alone never launches
/// a browser. Rich content and icons are decorative and inherit presentation.
///
/// ```
/// use gpui_kumo::{Link, link::Variant};
/// let link = Link::new("docs", "Documentation", "/docs")
///     .variant(Variant::Inline)
///     .on_navigate(|request, _, _| { let _target = &request.href; });
/// ```
#[derive(IntoElement)]
#[must_use]
pub struct Link {
    id: ElementId,
    label: SharedString,
    accessible_name: Option<SharedString>,
    href: SharedString,
    variant: Variant,
    content: Option<AnyElement>,
    badge_content: bool,
    breadcrumb: bool,
    external_icon: bool,
    disabled: bool,
    on_navigate: Option<NavigateHandler>,
    on_activate: Option<ActivationHandler>,
    style: StyleRefinement,
}

impl Link {
    /// Create an inline link. Supply a nonempty accessible label; destinations
    /// can be native routes or URLs interpreted by the application strategy.
    /// Set visible text and its default accessible name; explicit overrides remain authoritative.
    ///
    /// # Panics
    /// Panics when `text` is blank.
    pub fn label(mut self, text: impl Into<SharedString>) -> Self {
        self.label = crate::name::nonblank(text);
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
    /// Create a named navigation link; the owner supplies routing callbacks.
    ///
    /// # Panics
    /// Panics when the supplied name or label is blank.
    pub fn new(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        href: impl Into<SharedString>,
    ) -> Self {
        Self {
            id: id.into(),
            label: crate::name::nonblank(label),
            accessible_name: None,
            href: href.into(),
            variant: Variant::default(),
            content: None,
            badge_content: false,
            breadcrumb: false,
            external_icon: false,
            disabled: false,
            on_navigate: None,
            on_activate: None,
            style: Default::default(),
        }
    }

    pub(crate) fn breadcrumb(mut self) -> Self {
        self.breadcrumb = true;
        if self.content.is_none() {
            self.content = Some(
                div()
                    .id("label")
                    .test_support()
                    .whitespace_nowrap()
                    .child(self.label.clone())
                    .into_any_element(),
            );
        }
        self.variant = Variant::Plain;
        self
    }

    /// Select the source's inline, inherited-current or plain treatment.
    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    /// Inject the application's routing/open strategy. Dispatches this
    /// before any activation observer and preserves the actual input event.
    pub fn on_navigate(
        mut self,
        handler: impl Fn(&NavigationRequest, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_navigate = Some(Box::new(handler));
        self
    }

    /// Observe activation after the injected routing strategy, without adding
    /// a second activation engine or launching an external application.
    pub fn on_activate(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_activate = Some(Box::new(handler));
        self
    }

    /// Reject activation and remove keyboard traversal while retaining the label.
    /// Disabled links retain their appearance and expose unavailable metadata.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Replace visible text with decorative content; the supplied complete label
    /// remains the accessible name. Keep independent controls outside the link.
    pub fn rich_content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self.badge_content = false;
        self
    }

    /// Compose a typed badge before erasure, selecting the pill root and binding
    /// its hover ring to this link's scoped ancestor group.
    pub fn badge<P: 'static>(mut self, badge: crate::Badge<P>) -> Self
    where
        crate::Badge<P>: IntoElement,
    {
        self.content = Some(badge.link_hover_group(BADGE_GROUP).into_any_element());
        self.badge_content = true;
        self
    }

    /// Append Kumo's decorative external-navigation indicator. This changes
    /// presentation only; window/tab routing remains owned by the application.
    pub fn external_icon(mut self, external: bool) -> Self {
        self.external_icon = external;
        self
    }
}

impl Styled for Link {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Link {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        // One keyed handle owns activation, traversal and focus painting.
        let focus = window
            .use_keyed_state(self.id.clone(), cx, |_, cx| cx.focus_handle())
            .read(cx)
            .clone();
        let theme = crate::theme(cx);
        let inherited = window.text_style();
        let font_size = self
            .style
            .text
            .font_size
            .unwrap_or(inherited.font_size)
            .to_pixels(window.rem_size());
        let foreground = self.style.text.color.unwrap_or(match self.variant {
            Variant::Current => inherited.color,
            _ => theme.text.link,
        });
        let disabled = self.disabled;
        let mut link = control::root(
            self.id,
            self.accessible_name
                .clone()
                .unwrap_or_else(|| self.label.clone()),
            self.href.clone(),
            &focus,
            disabled,
            true,
        )
        .group(BADGE_GROUP)
        .flex()
        .items_center()
        .self_start()
        .gap(font_size * 0.1875)
        .text_color(foreground);
        if self.badge_content {
            link = link.rounded_full();
        }
        if self.variant != Variant::Plain {
            let decoration = UnderlineStyle {
                thickness: font_size * 0.0625,
                color: Some(
                    foreground.opacity(if theme.appearance == crate::Appearance::Dark {
                        0.65
                    } else {
                        0.35
                    }),
                ),
                wavy: false,
            };
            link.style().text.underline = Some(decoration);
            link = link.hover(move |mut style| {
                style.text.underline = Some(UnderlineStyle {
                    color: Some(foreground),
                    ..decoration
                });
                style
            });
        } else if !self.breadcrumb {
            link = link.hover(move |style| style.text_color(foreground.opacity(0.7)));
        }
        if !disabled {
            link = link.cursor_pointer();
        }
        link.style().refine(&self.style);
        if !disabled {
            link = link.on_click(move |event, window, cx| {
                if let Some(handler) = &self.on_navigate {
                    handler(
                        &NavigationRequest {
                            href: self.href.clone(),
                            activation: event.clone(),
                        },
                        window,
                        cx,
                    );
                }
                if let Some(handler) = &self.on_activate {
                    handler(event, window, cx);
                }
            });
        }
        let radii = Corners::<AbsoluteLength>::default().refined(link.style().corner_radii.clone());
        let borders =
            Edges::<AbsoluteLength>::default().refined(link.style().border_widths.clone());
        let focus_color = theme.colors.focus;
        let focus_width = theme.effects.keyboard_focus_ring_width;
        let inset_focus = self.breadcrumb;
        link = link
            .child(
                canvas(
                    |_, _, _| {},
                    move |bounds, _, window, _| {
                        if !disabled && focus.is_focused(window) && window.last_input_was_keyboard()
                        {
                            let border = borders.to_pixels(window.rem_size());
                            let root = Bounds::new(
                                bounds.origin - point(border.left, border.top),
                                size(
                                    bounds.size.width + border.left + border.right,
                                    bounds.size.height + border.top + border.bottom,
                                ),
                            );
                            // The source breadcrumb root clips overflow. Keep its
                            // native keyboard ring wholly inside the link bounds.
                            let outer = if inset_focus {
                                root
                            } else {
                                root.dilate(focus_width)
                            };
                            window.paint_quad(quad(
                                outer,
                                radii
                                    .to_pixels(window.rem_size())
                                    .map(|radius| *radius + focus_width)
                                    .clamp_radii_for_quad_size(outer.size),
                                focus_color.alpha(0.),
                                focus_width,
                                focus_color,
                                Default::default(),
                            ));
                        }
                    },
                )
                .absolute()
                .inset_0()
                .size_full(),
            )
            .child(self.content.unwrap_or_else(|| {
                div()
                    .id("label")
                    .test_support()
                    .min_w_0()
                    .whitespace_normal()
                    .child(self.label)
                    .into_any_element()
            }));
        if self.external_icon {
            link = link.child(ExternalIcon);
        }
        link
    }
}

/// Kumo's decorative, one-em external-navigation indicator. It inherits the
/// current foreground and uses the theme's light/dark source stroke width.
#[derive(IntoElement, Debug)]
pub struct ExternalIcon;

impl RenderOnce for ExternalIcon {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let style = window.text_style();
        let data = if crate::theme(cx).appearance == crate::Appearance::Dark {
            include_bytes!("../../assets/link-external-dark.svg").as_slice()
        } else {
            include_bytes!("../../assets/link-external-light.svg").as_slice()
        };
        div()
            .id("external-icon")
            .test_support()
            .flex_none()
            .size(style.font_size.to_pixels(window.rem_size()))
            .child(svg().data(data).size_full().text_color(style.color))
    }
}

pub(crate) mod control;

impl std::fmt::Debug for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Link")
            .field("id", &self.id)
            .field("label", &self.label)
            .field("variant", &self.variant)
            .field("badge_content", &self.badge_content)
            .field("breadcrumb", &self.breadcrumb)
            .field("external_icon", &self.external_icon)
            .field("disabled", &self.disabled)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests;
