use super::*;
use gpui_kit::{Animation, AnimationExt, Bounds, Corners, Pixels, canvas, quad, size};
use std::time::Duration;

#[derive(Default)]
pub(super) struct IndicatorMotion {
    pub target: Option<ElementId>,
    pub from: Option<Bounds<Pixels>>,
    pub current: Option<Bounds<Pixels>>,
    pub generation: usize,
}
/// Tailwind's default transition timing: cubic-bezier(.4,0,.2,1).
pub(super) fn easing(progress: f32) -> f32 {
    let t = progress.clamp(0., 1.);
    let mut low: f32 = 0.;
    let mut high = 1.;
    for _ in 0..16 {
        let u = (low + high) / 2.;
        let x = 3. * (1. - u).powi(2) * u * 0.4 + 3. * (1. - u) * u * u * 0.2 + u * u * u;
        if x < t {
            low = u;
        } else {
            high = u;
        }
    }
    let u = (low + high) / 2.;
    3. * (1. - u) * u * u + u * u * u
}
#[derive(IntoElement)]
pub(super) struct Indicator {
    pub scroll: ScrollHandle,
    pub index: Option<usize>,
    pub motion: Rc<RefCell<IndicatorMotion>>,
    pub from: Option<Bounds<Pixels>>,
    pub theme: crate::Theme,
    pub variant: Variant,
    pub size: Size,
    pub progress: f32,
}
impl Indicator {
    pub fn element(self, animate: bool, generation: usize) -> AnyElement {
        if animate {
            self.with_animation(
                ("tabs-indicator", generation),
                Animation::new(Duration::from_millis(200)).with_easing(easing),
                |mut graphic, progress| {
                    graphic.progress = progress;
                    graphic
                },
            )
            .into_any_element()
        } else {
            self.into_any_element()
        }
    }
}
impl RenderOnce for Indicator {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        canvas(
            |_, _, _| (),
            move |_, _, window, _| {
                let Some(target) = self.index.and_then(|i| self.scroll.bounds_for_item(i + 1))
                else {
                    self.motion.borrow_mut().current = None;
                    return;
                };
                let viewport = self.scroll.bounds();
                let mut target = Bounds {
                    origin: target.origin - viewport.origin,
                    size: target.size,
                };
                let segmented = self.variant == Variant::Segmented;
                if !segmented {
                    target.origin.y = viewport.size.height - px(2.);
                    target.size.height = px(2.);
                }
                let relative = if let Some(from) = self.from {
                    Bounds {
                        origin: from.origin + (target.origin - from.origin) * self.progress,
                        size: size(
                            from.size.width + (target.size.width - from.size.width) * self.progress,
                            from.size.height
                                + (target.size.height - from.size.height) * self.progress,
                        ),
                    }
                } else {
                    target
                };
                self.motion.borrow_mut().current = Some(relative);
                let bounds = Bounds {
                    origin: viewport.origin + self.scroll.offset() + relative.origin,
                    size: relative.size,
                };
                if segmented {
                    let radius = px(if self.size == Size::Sm { 4. } else { 6. });
                    window.paint_drop_shadows(
                        bounds,
                        Corners::all(radius),
                        &self.theme.effects.shadow_sm,
                    );
                    let color = self.theme.colors.line;
                    window.paint_quad(quad(
                        bounds.dilate(px(1.)),
                        radius + px(1.),
                        color.alpha(0.),
                        px(1.),
                        color,
                        Default::default(),
                    ));
                    window.paint_quad(quad(
                        bounds,
                        radius,
                        self.theme.colors.base,
                        px(0.),
                        color,
                        Default::default(),
                    ));
                } else {
                    window.paint_quad(quad(
                        bounds,
                        px(0.),
                        self.theme.colors.brand,
                        px(0.),
                        self.theme.colors.brand,
                        Default::default(),
                    ));
                }
            },
        )
        .absolute()
        .left(px(if self.variant == Variant::Segmented {
            2.
        } else {
            0.
        }))
        .top_0()
        .w(px(0.))
        .h(px(0.))
    }
}
