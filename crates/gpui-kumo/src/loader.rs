//! Kumo's circular loading status, shared with Button's decorative indicator.

#![deny(missing_docs)]

use std::{f32::consts::TAU, time::Duration};

use gpui_kit::base::TestSupportExt;
use gpui_kit::{
    Animation, AnimationExt, App, Bounds, ElementId, Hsla, InteractiveElement, IntoElement,
    ParentElement, PathBuilder, Pixels, RenderOnce, Role, SharedString, Size as GeometrySize,
    StatefulInteractiveElement, Styled, Window, canvas, div, point, px, quad,
};

/// Preset or custom loading-indicator dimensions.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Size {
    /// 16px, suitable for inline content.
    Small,
    /// 24px default.
    #[default]
    Base,
    /// 32px prominent indicator.
    Large,
    /// Custom logical pixels. Nonpositive or nonfinite values render at zero size.
    Custom(Pixels),
}

impl Size {
    fn pixels(self) -> Pixels {
        let value = match self {
            Self::Small => px(16.),
            Self::Base => px(24.),
            Self::Large => px(32.),
            Self::Custom(value) => value,
        };
        if f32::from(value).is_finite() && value > px(0.) {
            value
        } else {
            px(0.)
        }
    }
}

/// A named, read-only loading status inheriting its parent's foreground.
#[derive(IntoElement)]
#[must_use]
pub struct Loader {
    id: ElementId,
    size: Size,
    label: SharedString,
}

impl Loader {
    /// Create a 24px indicator named "Loading" with scoped identity.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            size: Size::default(),
            label: "Loading".into(),
        }
    }

    /// Select preset or custom logical dimensions.
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// Supply a localized accessible name. Empty content remains an unnamed status.
    pub fn accessibility_label(mut self, name: impl Into<SharedString>) -> Self {
        self.label = name.into();
        self
    }
}

impl RenderOnce for Loader {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let size = self.size.pixels();
        div()
            .id(self.id)
            .test_support()
            .role(Role::Status)
            .aria_label(self.label)
            .w(size)
            .h(size)
            .flex_shrink_0()
            .child(indicator(size, cx))
    }
}

pub(crate) fn indicator(size: Pixels, cx: &App) -> gpui_kit::AnyElement {
    let graphic = Graphic { size, phase: 0. };
    if cx.reduce_motion() || size <= px(0.) {
        Graphic {
            phase: 0.125,
            ..graphic
        }
        .into_any_element()
    } else {
        graphic
            .with_animation(
                "loader-motion",
                Animation::new(Duration::from_secs(6)).repeat(),
                |mut graphic, phase| {
                    graphic.phase = phase;
                    graphic
                },
            )
            .into_any_element()
    }
}

#[derive(IntoElement)]
struct Graphic {
    size: Pixels,
    phase: f32,
}

impl RenderOnce for Graphic {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        canvas(
            |_, window, _| window.text_style().color,
            move |bounds, color, window, _| {
                if self.size <= px(0.) {
                    return;
                }
                let dash_phase = (self.phase * 4.).fract();
                let length = 42. * (dash_phase * 2.).min(1.);
                let offset = if dash_phase <= 0.5 {
                    32. * dash_phase
                } else {
                    16. + 86. * (dash_phase - 0.5)
                };
                let start = offset / 9.5 + self.phase * 3. * TAU;
                let sweep = length.min(TAU * 9.5 - offset).max(0.) / 9.5;
                paint_arc(bounds, start, sweep, color, true, window);
                let mut track = color;
                track.a *= 0.1;
                paint_arc(bounds, 0., TAU, track, false, window);
            },
        )
        .w(self.size)
        .h(self.size)
        .flex_shrink_0()
    }
}

fn paint_arc(
    bounds: Bounds<Pixels>,
    start: f32,
    sweep: f32,
    color: Hsla,
    caps: bool,
    window: &mut Window,
) {
    if sweep <= 0. {
        return;
    }
    let scale = f32::from(bounds.size.width) / 24.;
    let radius = px(9.5 * scale);
    let stroke = px(2. * scale);
    let center = bounds.center();
    let position = |angle: f32| center + point(radius * angle.cos(), radius * angle.sin());
    let mut path = PathBuilder::stroke(stroke);
    path.move_to(position(start));
    // Two SVG arcs also represent a full circle without coincident endpoints.
    for fraction in [0.5, 1.] {
        path.arc_to(
            point(radius, radius),
            px(0.),
            false,
            true,
            position(start + sweep * fraction),
        );
    }
    if !caps {
        path.close();
    }
    if let Ok(path) = path.build() {
        window.paint_path(path, color);
    }
    if caps {
        for angle in [start, start + sweep] {
            let bounds = Bounds::new(
                position(angle) - point(stroke / 2., stroke / 2.),
                GeometrySize::new(stroke, stroke),
            );
            window.paint_quad(quad(
                bounds,
                stroke / 2.,
                color,
                px(0.),
                Hsla::transparent_black(),
                Default::default(),
            ));
        }
    }
}

impl std::fmt::Debug for Loader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_struct("Loader");
        debug.field("id", &self.id);
        debug.field("size", &self.size);
        debug.field("label", &self.label);
        debug.finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui_kit::{Context, Render, Subscription, TestAppContext, test::TestWindowExt};

    struct Harness {
        _theme: Subscription,
    }
    impl Render for Harness {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            div()
                .flex()
                .text_color(crate::theme(cx).text.default)
                .gap(px(4.))
                .children([
                    Loader::new("small").size(Size::Small),
                    Loader::new("base"),
                    Loader::new("large").size(Size::Large),
                    Loader::new("custom")
                        .size(Size::Custom(px(14.)))
                        .accessibility_label("Chargement"),
                    Loader::new("zero").size(Size::Custom(px(0.))),
                    Loader::new("negative").size(Size::Custom(px(-4.))),
                    Loader::new("nan").size(Size::Custom(px(f32::NAN))),
                    Loader::new("unnamed").accessibility_label(""),
                ])
        }
    }

    #[gpui_kit::test]
    fn rendered_status_preserves_sizes_names_and_invalid_size_boundaries(cx: &mut TestAppContext) {
        cx.update(crate::init);
        let (_, cx) = cx.add_window_view(|_, cx| Harness {
            _theme: cx.observe_global::<crate::Theme>(|_, cx| cx.notify()),
        });
        cx.update(|window, cx| {
            cx.set_reduce_motion(true);
            window.render_frame(cx);
            for (id, size) in [
                ("small", 16.),
                ("base", 24.),
                ("large", 32.),
                ("custom", 14.),
                ("zero", 0.),
                ("negative", 0.),
                ("nan", 0.),
            ] {
                let element = window.find(id);
                assert_eq!(element.bounds().size, GeometrySize::new(px(size), px(size)));
                assert_eq!(element.role(), Some(Role::Status));
            }
            assert_eq!(window.find("base").label(), Some("Loading"));
            assert_eq!(window.find("custom").label(), Some("Chargement"));
            assert_eq!(window.find("unnamed").label(), Some(""));
            assert!(
                !window.painted_quads().is_empty(),
                "Static arcs retain rounded caps"
            );
        });
    }

    #[gpui_kit::test]
    fn animation_changes_paint_and_reduced_motion_stops_frame_requests(cx: &mut TestAppContext) {
        cx.update(crate::init);
        let (_, cx) = cx.add_window_view(|_, cx| Harness {
            _theme: cx.observe_global::<crate::Theme>(|_, cx| cx.notify()),
        });
        cx.update(|window, cx| {
            window.render_frame(cx);
            cx.background_executor()
                .advance_clock(Duration::from_millis(250));
            assert!(window.simulate_next_frame(cx) > 0);
            window.render_frame(cx);
            let caps: Vec<_> = window
                .painted_quads()
                .into_iter()
                .map(|quad| quad.bounds)
                .collect();
            assert!(!caps.is_empty());
            cx.background_executor()
                .advance_clock(Duration::from_millis(250));
            window.simulate_next_frame(cx);
            window.render_frame(cx);
            let moved: Vec<_> = window
                .painted_quads()
                .into_iter()
                .map(|quad| quad.bounds)
                .collect();
            assert_ne!(
                caps, moved,
                "The live animation moves its painted arc endpoints"
            );
            cx.set_reduce_motion(true);
            window.render_frame(cx);
            window.simulate_next_frame(cx);
            window.render_frame(cx);
            assert_eq!(window.simulate_next_frame(cx), 0);
            assert!(
                !window.painted_quads().is_empty(),
                "Reduced motion keeps visible arc caps"
            );
            for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
                crate::set_appearance(appearance, cx);
                window.render_frame(cx);
                let expected = crate::theme(cx).text.default;
                let caps = window.painted_quads();
                assert!(!caps.is_empty());
                assert!(
                    caps.iter()
                        .all(|quad| quad.background.as_solid() == Some(expected))
                );
            }
        });
    }
}
