//! Kumo Button: application-owned props over Base's activation and focus behavior.
//! See `docs/kumo-component-recipes.md` for the contract and native recipe policies.

use std::time::Duration;

use gpui_kit::{
    Animation, AnimationExt, AnyElement, App, Background, BoxShadow, ClickEvent, ElementId,
    FocusHandle, FontWeight, HitboxBehavior, Hsla, InteractiveElement, IntoElement, ParentElement,
    PathBuilder, Pixels, RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window,
    base, canvas, div, point, prelude::FluentBuilder, px, quad, rgb,
};

use crate::{Theme, theme};

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
    open: bool,
    focus_handle: Option<FocusHandle>,
    on_click: Option<ActivationHandler>,
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
            open: false,
            focus_handle: None,
            on_click: None,
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

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
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
        let rest = self.variant.paint(theme, unavailable, self.open, false);
        let hovered = self.variant.paint(theme, unavailable, self.open, true);
        let focus_color = self
            .variant
            .emphasis(theme)
            .map_or(theme.colors.focus.opacity(0.5), |e| e.ring);
        let keyboard_color = self
            .variant
            .emphasis(theme)
            .map_or(theme.colors.brand, |e| e.ring);
        // Supply Base with the same keyed focus handle used to resolve hover/focus
        // precedence. No component-owned copy of the application's state is needed.
        let theme = theme.clone();
        let focus_handle = self.focus_handle.unwrap_or_else(|| {
            window
                .use_keyed_state(self.id.clone(), cx, |_, cx| cx.focus_handle())
                .read(cx)
                .clone()
        });
        let opacity = if self.disabled || (self.loading && self.variant.emphasis(&theme).is_some())
        {
            0.5
        } else {
            1.
        };
        let mut button = base::Button::new(self.id)
            .accessibility_label(self.name)
            .track_focus(&focus_handle)
            .disabled(unavailable)
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
        if let Some(handler) = self.on_click {
            button = button.on_click(handler);
        }
        let leading = if self.loading {
            Some(
                Loader {
                    size: px(if self.size == Size::Lg { 16. } else { 14. }),
                    color: rest.foreground,
                    phase: 0.,
                }
                .with_animation(
                    "loader",
                    Animation::new(Duration::from_secs(1)).repeat(),
                    |mut loader, phase| {
                        loader.phase = phase;
                        loader
                    },
                )
                .into_any_element(),
            )
        } else {
            self.leading
        };
        let ring_focus = focus_handle.clone();
        let ring_width = theme.effects.control_ring_width;
        let keyboard_width = theme.effects.keyboard_focus_ring_width;
        let ring = canvas(
            |bounds, window, _| window.insert_hitbox(bounds, HitboxBehavior::Normal),
            move |bounds, hitbox, window, cx| {
                let focused = !unavailable && ring_focus.is_focused(window);
                let (color, width) = if focused && window.last_input_was_keyboard() {
                    (Some(keyboard_color), keyboard_width)
                } else if focused {
                    (Some(focus_color), ring_width)
                } else if !unavailable && !cx.has_active_drag() && hitbox.is_hovered(window) {
                    (hovered.ring, ring_width)
                } else {
                    (rest.ring, ring_width)
                };
                if let Some(color) = color {
                    window.paint_quad(quad(
                        bounds.dilate(width),
                        geometry.radius + width,
                        Hsla::transparent_black(),
                        width,
                        color,
                        Default::default(),
                    ));
                }
            },
        )
        .absolute()
        .size_full();
        div().flex().flex_shrink_0().self_start().child(
            button
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(if self.variant.emphasis(&theme).is_some() {
                            theme.spacing.six
                        } else {
                            geometry.gap
                        })
                        .children(leading)
                        .children(self.label)
                        .children(self.trailing),
                )
                .child(ring),
        )
    }
}

#[derive(IntoElement)]
struct Loader {
    size: Pixels,
    color: Hsla,
    phase: f32,
}

impl RenderOnce for Loader {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        canvas(
            |_, _, _| (),
            move |bounds, _, window, _| {
                let center = bounds.center();
                let radius = self.size / 2. - px(1.5);
                let mut path = PathBuilder::stroke(px(1.5));
                for step in 0..=32 {
                    let angle = std::f32::consts::TAU * (self.phase + step as f32 / 32. * 0.75);
                    let position = center + point(radius * angle.cos(), radius * angle.sin());
                    if step == 0 {
                        path.move_to(position);
                    } else {
                        path.line_to(position);
                    }
                }
                if let Ok(path) = path.build() {
                    window.paint_path(path, self.color);
                }
            },
        )
        .w(self.size)
        .h(self.size)
        .flex_shrink_0()
    }
}

#[cfg(test)]
#[path = "button_tests.rs"]
mod tests;
