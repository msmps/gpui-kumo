//! Kumo Button: application-owned props over Base's activation and focus behavior.
//! See `docs/kumo-component-recipes.md` for the contract and native recipe policies.

use gpui_kit::{
    AbsoluteLength, AnyElement, App, Background, Bounds, BoxShadow, ClickEvent, Corners, Edges,
    ElementId, FocusHandle, FontWeight, HitboxBehavior, Hsla, InteractiveElement, IntoElement,
    ParentElement, Pixels, Refineable, RenderOnce, SharedString, StatefulInteractiveElement,
    StyleRefinement, Styled, Window, base, canvas, div, point, prelude::FluentBuilder, px, quad,
    rgb, size,
};

use crate::{Theme, theme};
use std::{cell::RefCell, rc::Rc};
pub(crate) type JoinedRingQueue =
    Rc<RefCell<Vec<(bool, gpui_kit::PaintQuad, gpui_kit::ContentMask<Pixels>)>>>;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Variant {
    Primary,
    #[default]
    Secondary,
    Ghost,
    Destructive,
    SecondaryDestructive,
    Outline,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Size {
    Xs,
    Sm,
    #[default]
    Base,
    Lg,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Shape {
    #[default]
    Standard,
    Square,
    Circle,
}

type ActivationHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

/// A consumed, stateless component. The owner supplies loading and disabled state.
/// IDs must be stable and unique within the parent; names must be nonempty.
/// Icon slots are decorative, noninteractive content. Use `crate::Icon` for SVG
/// foreground inheritance; custom elements own their paint and dimensions.
#[derive(IntoElement)]
#[must_use]
pub struct Button {
    id: ElementId,
    name: SharedString,
    label: Option<SharedString>,
    leading: Option<AnyElement>,
    trailing: Option<AnyElement>,
    variant: Variant,
    size: Size,
    shape: Shape,
    disabled: bool,
    loading: bool,
    input_group_action: bool,
    input_group_zone: Option<Box<crate::input_group::Zone>>,
    tooltip_trigger: Option<Box<crate::tooltip::TriggerHooks>>,
    open: bool,
    expanded: Option<(bool, Option<&'static str>)>,
    focus_handle: Option<FocusHandle>,
    on_click: Option<ActivationHandler>,
    accent: Option<Box<AccentRecipe>>,
    style: StyleRefinement,
    join: Option<Box<(bool, bool, bool, JoinedRingQueue)>>,
}

impl Button {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        let label = label.into();
        Self {
            id: id.into(),
            name: label.clone(),
            label: Some(label),
            leading: None,
            trailing: None,
            variant: Variant::default(),
            size: Size::default(),
            shape: Shape::default(),
            disabled: false,
            loading: false,
            input_group_action: false,
            input_group_zone: None,
            tooltip_trigger: None,
            open: false,
            expanded: None,
            focus_handle: None,
            on_click: None,
            accent: None,
            style: Default::default(),
            join: None,
        }
    }

    /// Construct an icon-only square button with a required accessible name.
    pub fn icon(
        id: impl Into<ElementId>,
        name: impl Into<SharedString>,
        icon: impl IntoElement,
    ) -> Self {
        Self {
            label: None,
            leading: Some(icon.into_any_element()),
            shape: Shape::Square,
            ..Self::new(id, name)
        }
    }

    pub(crate) fn input_group_action(mut self, disabled: bool) -> Self {
        self.disabled |= disabled;
        self.input_group_action = true;
        self
    }
    pub(crate) fn input_group_zone(
        mut self,
        disabled: bool,
        zone: crate::input_group::Zone,
    ) -> Self {
        self.disabled |= disabled;
        self.input_group_zone = Some(Box::new(zone));
        self
    }
    pub(crate) fn tooltip_trigger(mut self, hooks: crate::tooltip::TriggerHooks) -> Self {
        self.tooltip_trigger = Some(Box::new(hooks));
        self
    }
    pub(crate) fn provided_focus(&self) -> Option<FocusHandle> {
        self.focus_handle.clone()
    }
    pub(crate) fn is_ghost(&self) -> bool {
        self.variant == Variant::Ghost
    }
    pub(crate) fn group_join(
        mut self,
        first: bool,
        last: bool,
        next_has_ring: bool,
        rings: JoinedRingQueue,
    ) -> Self {
        self.join = Some(Box::new((first, last, next_has_ring, rings)));
        self
    }
    pub(crate) fn has_group_ring(&self, theme: &Theme) -> bool {
        self.accent.as_ref().map_or_else(
            || {
                self.variant
                    .paint(theme, self.disabled || self.loading, self.open, false)
                    .ring
                    .is_some()
            },
            |accent| accent.ring.is_some() || accent.emphasis.is_some(),
        )
    }
    pub(crate) fn id(&self) -> &ElementId {
        &self.id
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub(crate) fn accent(mut self, recipe: AccentRecipe) -> Self {
        self.accent = Some(Box::new(recipe));
        self
    }
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
    pub fn shape(mut self, shape: Shape) -> Self {
        self.shape = shape;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Display a loader in place of the leading icon and prevent activation.
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// Controlled open presentation for a future overlay trigger; not toggle state.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub(crate) fn popover_expanded(mut self, expanded: bool) -> Self {
        self.expanded = Some((
            expanded,
            Some(if expanded {
                "Expanded nonmodal dialog"
            } else {
                "Collapsed nonmodal dialog"
            }),
        ));
        self
    }

    pub(crate) fn disclosure_trigger(
        mut self,
        focus: &FocusHandle,
        disabled: bool,
        expanded: bool,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.focus_handle = Some(focus.clone());
        self.disabled |= disabled;
        self.expanded = Some((expanded, None));
        let consumer = self.on_click.take();
        self.on_click = Some(Box::new(move |event, window, cx| {
            if let Some(consumer) = &consumer {
                consumer(event, window, cx);
            }
            if !window.default_prevented() {
                handler(event, window, cx);
            }
        }));
        self
    }

    pub fn leading_icon(mut self, icon: impl IntoElement) -> Self {
        self.leading = Some(icon.into_any_element());
        self
    }

    pub fn trailing_icon(mut self, icon: impl IntoElement) -> Self {
        self.trailing = Some(icon.into_any_element());
        self
    }

    pub fn accessibility_label(mut self, name: impl Into<SharedString>) -> Self {
        self.name = name.into();
        self
    }

    pub fn track_focus(mut self, handle: &FocusHandle) -> Self {
        self.focus_handle = Some(handle.clone());
        self
    }

    /// One activation path for pointer, Enter, Space and accessible click.
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl Styled for Button {
    /// Refine the interactive surface after its Kumo recipe. The surrounding
    /// native intrinsic-width wrapper remains responsible for placement.
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

struct Geometry {
    height: Pixels,
    padding: Pixels,
    gap: Pixels,
    radius: Pixels,
    text: crate::theme::TextStyle,
}

impl Size {
    fn geometry(self, shape: Shape, theme: &Theme) -> Geometry {
        let (height, padding, gap, radius, text) = match self {
            Self::Xs => (
                20.,
                theme.spacing.six,
                theme.spacing.four,
                theme.radii.sm,
                theme.typography.xs,
            ),
            Self::Sm => (
                26.,
                theme.spacing.eight,
                theme.spacing.four,
                theme.radii.md,
                theme.typography.xs,
            ),
            Self::Base => (
                36.,
                theme.spacing.twelve,
                theme.spacing.six,
                theme.radii.lg,
                theme.typography.base,
            ),
            Self::Lg => (
                40.,
                theme.spacing.sixteen,
                theme.spacing.eight,
                theme.radii.lg,
                theme.typography.base,
            ),
        };
        let height = px(if self == Self::Xs && shape != Shape::Standard {
            14.
        } else {
            height
        });
        Geometry {
            height,
            padding: if shape == Shape::Standard {
                padding
            } else {
                px(0.)
            },
            gap,
            radius: if shape == Shape::Circle {
                height / 2.
            } else {
                radius
            },
            text,
        }
    }
}

#[derive(Clone)]
struct Paint {
    background: Background,
    foreground: Hsla,
    ring: Option<Hsla>,
    inset: Option<BoxShadow>,
    drop_shadow: bool,
}

/// Presentation-only seam for compound controls that reuse Button behavior.
pub(crate) struct AccentRecipe {
    pub emphasis: Option<crate::theme::Emphasis>,
    pub foreground: Hsla,
    pub unavailable_foreground: Hsla,
    pub icon_foreground: Hsla,
    pub ring: Option<Hsla>,
    pub hover_background: Hsla,
}

impl AccentRecipe {
    fn paint(&self, hovered: bool, unavailable: bool) -> Paint {
        if let Some(emphasis) = &self.emphasis {
            Paint {
                background: emphasis.gradient(hovered && !unavailable),
                foreground: self.foreground,
                ring: Some(emphasis.ring),
                inset: Some(emphasis.inset_highlight()),
                drop_shadow: true,
            }
        } else {
            Paint {
                background: if hovered && !unavailable {
                    self.hover_background
                } else {
                    Hsla::transparent_black()
                }
                .into(),
                foreground: if unavailable {
                    self.unavailable_foreground
                } else {
                    self.foreground
                },
                ring: self.ring,
                inset: None,
                drop_shadow: false,
            }
        }
    }
}

impl Variant {
    fn emphasis(self, theme: &Theme) -> Option<&crate::theme::Emphasis> {
        match self {
            Self::Primary => Some(&theme.primary),
            Self::Destructive => Some(&theme.destructive),
            _ => None,
        }
    }

    fn paint(self, theme: &Theme, unavailable: bool, open: bool, hovered: bool) -> Paint {
        let hovered = hovered && !unavailable;
        if let Some(emphasis) = self.emphasis(theme) {
            return Paint {
                background: emphasis.gradient(hovered),
                foreground: rgb(0xffffff).into(),
                ring: Some(emphasis.ring),
                inset: Some(emphasis.inset_highlight()),
                drop_shadow: true,
            };
        }
        let base = theme.colors.base;
        let mut paint = Paint {
            background: base.into(),
            foreground: theme.text.default,
            ring: Some(theme.colors.line),
            inset: None,
            drop_shadow: true,
        };
        match self {
            Self::Secondary => {
                if unavailable {
                    paint.background = base.opacity(0.5).into();
                    paint.foreground = paint.foreground.opacity(0.7);
                } else if hovered && !open {
                    paint.background = theme.colors.tint.into();
                }
            }
            Self::SecondaryDestructive => {
                paint.foreground = theme.text.danger;
                if unavailable {
                    paint.background = base.opacity(0.5).into();
                    paint.foreground = paint.foreground.opacity(0.7);
                } else if hovered {
                    paint.ring = Some(theme.colors.danger.opacity(0.3));
                }
            }
            Self::Ghost => {
                paint.background = if hovered {
                    theme.colors.tint.into()
                } else {
                    Hsla::transparent_black().into()
                };
                paint.ring = None;
                paint.drop_shadow = false;
                if unavailable {
                    paint.foreground = theme.text.subtle;
                }
            }
            Self::Outline => {
                // GPUI drop shadows fill their interior. Preserve this variant's
                // transparent surface until hollow shadow paint is available.
                paint.drop_shadow = false;
                paint.background = Hsla::transparent_black().into();
                if unavailable {
                    paint.foreground = theme.text.subtle;
                } else if hovered {
                    paint.foreground = theme.text.strong;
                    paint.ring = Some(theme.colors.focus.opacity(0.25));
                }
            }
            Self::Primary | Self::Destructive => unreachable!("emphasis variants resolved above"),
        }
        paint
    }
}

fn shadows(paint: &Paint, theme: &Theme) -> Vec<BoxShadow> {
    let mut shadows = if paint.drop_shadow {
        theme.effects.shadow_xs.clone()
    } else {
        Vec::new()
    };
    if let Some(inset) = &paint.inset {
        shadows.push(inset.clone());
    }
    shadows
}

impl RenderOnce for Button {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = theme(cx);
        let geometry = self.size.geometry(self.shape, theme);
        let unavailable = self.disabled || self.loading;
        let mut rest = self.accent.as_ref().map_or_else(
            || self.variant.paint(theme, unavailable, self.open, false),
            |recipe| recipe.paint(false, unavailable),
        );
        if self.join.is_some() || self.input_group_action || self.input_group_zone.is_some() {
            rest.drop_shadow = false;
        }
        if self.disabled && self.input_group_zone.is_some() {
            rest.background = theme.colors.overlay.into();
            rest.foreground = theme.text.inactive;
        }
        let hovered = self.accent.as_ref().map_or_else(
            || self.variant.paint(theme, unavailable, self.open, true),
            |recipe| recipe.paint(true, unavailable),
        );
        let emphasis = self.accent.as_ref().map_or_else(
            || self.variant.emphasis(theme),
            |recipe| recipe.emphasis.as_ref(),
        );
        let is_emphasis = emphasis.is_some();
        let focus_color = emphasis.map_or(theme.colors.focus.opacity(0.5), |e| e.ring);
        let keyboard_color = if self.input_group_action {
            theme.colors.focus.opacity(0.5)
        } else {
            emphasis.map_or(theme.colors.brand, |e| e.ring)
        };
        // Supply Base with the same keyed focus handle used to resolve hover/focus
        // precedence. No component-owned copy of the application's state is needed.
        let theme = theme.clone();
        let focus_handle = self.focus_handle.unwrap_or_else(|| {
            window
                .use_keyed_state(self.id.clone(), cx, |_, cx| cx.focus_handle())
                .read(cx)
                .clone()
        });
        let opacity = if self.disabled || (self.loading && is_emphasis) {
            0.5
        } else {
            1.
        };
        let mut button = base::Button::new(self.id)
            .accessibility_label(self.name)
            .track_focus(&focus_handle)
            .disabled(unavailable)
            .a11y_synthetic_children(move |builder| {
                // Enrich Base's actual control node; do not add another button.
                let node = builder.parent_node();
                if unavailable {
                    node.set_disabled();
                }
                if self.loading {
                    node.set_busy();
                }
            })
            .when_some(self.expanded, |this, (expanded, description)| {
                this.aria_expanded(expanded)
                    .when_some(description, |this, description| {
                        this.aria_description(description)
                    })
            })
            .flex_shrink_0()
            .h(geometry.height)
            .px(geometry.padding)
            .rounded(geometry.radius)
            .font_family(theme.typography.font_family.clone())
            .text_size(geometry.text.size)
            .line_height(geometry.text.line_height)
            .font_weight(FontWeight::MEDIUM)
            .whitespace_nowrap()
            .bg(rest.background)
            .text_color(rest.foreground)
            .opacity(opacity)
            .relative()
            .shadow(shadows(&rest, &theme))
            .when(self.shape != Shape::Standard, |this| {
                this.w(geometry.height)
            })
            .when(!unavailable, |this| this.cursor_pointer())
            .when(unavailable, |this| this.cursor_not_allowed())
            .when(self.loading, |this| this.aria_description("Loading"))
            .when(!self.loading && self.disabled, |this| {
                this.aria_description("Unavailable")
            })
            .when(!unavailable, |this| {
                this.hover(|style| style.bg(hovered.background).text_color(hovered.foreground))
            });
        let tooltip_bounds = self
            .tooltip_trigger
            .as_ref()
            .map(|hooks| (hooks.bounds.clone(), hooks.moved.clone()));
        if let Some(hooks) = self.tooltip_trigger {
            let crate::tooltip::TriggerHooks { hover, press, .. } = *hooks;
            if !unavailable {
                button = button.on_hover(move |hovered, window, cx| hover(hovered, window, cx));
            }
            let handler = self.on_click;
            button = button.on_click(move |event, window, cx| {
                press(window, cx);
                if let Some(handler) = &handler {
                    handler(event, window, cx);
                }
            });
            button = button.cursor_default();
        } else if let Some(handler) = self.on_click {
            button = button.on_click(handler);
        }

        button.style().refine(&self.style);
        let joined_rings = self
            .join
            .as_ref()
            .map(|join| (join.0, join.1, join.2, join.3.clone()));
        if let Some(join) = self.join {
            let (first, last, _, _) = *join;
            if !first {
                button.style().corner_radii.top_left = Some(px(0.).into());
                button.style().corner_radii.bottom_left = Some(px(0.).into());
            }
            if !last {
                button.style().corner_radii.top_right = Some(px(0.).into());
                button.style().corner_radii.bottom_right = Some(px(0.).into());
            }
        }
        let zone = self.input_group_zone;
        if let Some(zone) = &zone {
            button = button
                .h(zone.height)
                .border_1()
                .border_color(theme.colors.line.alpha(0.));
            button.style().corner_radii.top_left =
                Some(if zone.first { zone.radius } else { px(0.) }.into());
            button.style().corner_radii.bottom_left =
                Some(if zone.first { zone.radius } else { px(0.) }.into());
            button.style().corner_radii.top_right =
                Some(if zone.last { zone.radius } else { px(0.) }.into());
            button.style().corner_radii.bottom_right =
                Some(if zone.last { zone.radius } else { px(0.) }.into());
            if self.shape != Shape::Standard {
                button = button.w(zone.height);
            }
        }
        let radii =
            Corners::<AbsoluteLength>::default().refined(button.style().corner_radii.clone());
        let borders =
            Edges::<AbsoluteLength>::default().refined(button.style().border_widths.clone());
        let leading = if self.loading {
            Some(crate::loader::indicator(
                px(if self.size == Size::Lg { 16. } else { 14. }),
                cx,
            ))
        } else {
            self.leading
        };
        let tint_icon = |icon: AnyElement| {
            if let Some(recipe) = &self.accent {
                div()
                    .text_color(recipe.icon_foreground)
                    .child(icon)
                    .into_any_element()
            } else {
                icon
            }
        };
        // Source fill utilities tint decorative icons, while Loader's stroke
        // inherits the current text color through the existing loading path.
        let leading = if self.loading {
            leading
        } else {
            leading.map(tint_icon)
        };
        let trailing = self.trailing.map(tint_icon);
        let ring_opacity = button.style().opacity.unwrap_or(1.);
        let ring_focus = focus_handle.clone();
        let ring_width = theme.effects.control_ring_width;
        let keyboard_width = if self.input_group_action {
            theme.effects.input_focus_ring_width
        } else {
            theme.effects.keyboard_focus_ring_width
        };
        let input_group_action = self.input_group_action;
        let ring = canvas(
            move |bounds, window, cx| {
                if let Some((target, moved)) = &tooltip_bounds {
                    let border = borders.to_pixels(window.rem_size());
                    let measured = Bounds::new(
                        bounds.origin - point(border.left, border.top),
                        size(
                            bounds.size.width + border.left + border.right,
                            bounds.size.height + border.top + border.bottom,
                        ),
                    );
                    if target.replace(measured) != measured {
                        moved(window, cx);
                    }
                }
                window.insert_hitbox(bounds, HitboxBehavior::Normal)
            },
            move |bounds, hitbox, window, cx| {
                let focused = !unavailable && ring_focus.is_focused(window);
                if let Some(zone) = &zone {
                    let color = if focused && window.last_input_was_keyboard() {
                        theme.colors.focus.opacity(0.5)
                    } else {
                        theme.colors.line
                    };
                    let border = quad(
                        bounds.dilate(px(1.)),
                        radii.to_pixels(window.rem_size()),
                        color.alpha(0.),
                        px(1.),
                        color,
                        Default::default(),
                    );
                    zone.borders
                        .borrow_mut()
                        .push((focused, border, window.content_mask()));
                    return;
                }
                let (color, width) = if focused && window.last_input_was_keyboard() {
                    (Some(keyboard_color), keyboard_width)
                } else if focused && input_group_action {
                    (None, px(0.))
                } else if focused {
                    (Some(focus_color), ring_width)
                } else if !unavailable && !cx.has_active_drag() && hitbox.is_hovered(window) {
                    (hovered.ring, ring_width)
                } else {
                    (rest.ring, ring_width)
                };
                if let Some(color) = color {
                    let border = borders.to_pixels(window.rem_size());
                    let root = Bounds::new(
                        bounds.origin - point(border.left, border.top),
                        size(
                            bounds.size.width + border.left + border.right,
                            bounds.size.height + border.top + border.bottom,
                        ),
                    );
                    let outer = root.dilate(width);
                    let keyboard_focus = focused && window.last_input_was_keyboard();
                    let widths = joined_rings.as_ref().filter(|_| !keyboard_focus).map_or(
                        Edges::all(width),
                        |(first, last, next_has_ring, _)| Edges {
                            top: width,
                            bottom: width,
                            left: if *first { width } else { px(0.) },
                            right: if *last || !*next_has_ring {
                                width
                            } else {
                                px(0.)
                            },
                        },
                    );
                    let mut ring_quad = quad(
                        outer,
                        radii
                            .to_pixels(window.rem_size())
                            .map(|radius| *radius + width)
                            .clamp_radii_for_quad_size(outer.size),
                        // GPUI interpolates straight RGB at the inner edge before
                        // premultiplication; transparent black creates a dark fringe.
                        color.alpha(0.),
                        widths,
                        color,
                        Default::default(),
                    );
                    if let Some((first, _, _, rings)) = &joined_rings
                        && !first
                    {
                        let divider = quad(
                            Bounds::new(root.origin, size(px(1.), root.size.height)),
                            px(0.),
                            color.opacity(ring_opacity),
                            px(0.),
                            color.alpha(0.),
                            Default::default(),
                        );
                        rings
                            .borrow_mut()
                            .push((false, divider, window.content_mask()));
                    }
                    if keyboard_focus && let Some((_, _, _, rings)) = &joined_rings {
                        ring_quad.background = ring_quad.background.opacity(ring_opacity);
                        ring_quad.border_color = ring_quad.border_color.opacity(ring_opacity);
                        rings
                            .borrow_mut()
                            .push((true, ring_quad, window.content_mask()));
                    } else {
                        window.paint_quad(ring_quad);
                    }
                }
            },
        )
        .absolute()
        .inset_0()
        .size_full();
        let surface = button
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(if is_emphasis {
                        theme.spacing.six
                    } else {
                        geometry.gap
                    })
                    .children(leading)
                    .children(self.label)
                    .children(trailing),
            )
            .child(ring);
        div().flex().flex_shrink_0().child(surface)
    }
}

#[cfg(test)]
#[path = "button_tests.rs"]
mod tests;
