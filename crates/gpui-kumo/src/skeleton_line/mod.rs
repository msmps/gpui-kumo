//! Decorative loading lines with retained sampling and Base CSS timing.
use crate::theme;
use base::TestSupportExt;
use gpui_kit::InteractiveElement;
use gpui_kit::{
    App, Bounds, ContentMask, ElementId, IntoElement, ParentElement, Pixels, RenderOnce, Styled,
    Window, base, canvas, div, linear_color_stop, linear_gradient, point, prelude::FluentBuilder,
    px, quad, relative,
};
use std::{
    collections::hash_map::RandomState,
    hash::BuildHasher,
    ops::RangeInclusive,
    time::{Duration, Instant},
};

#[derive(Clone, PartialEq, Eq)]
struct Ranges {
    width: RangeInclusive<u8>,
    duration: RangeInclusive<Duration>,
    delay: RangeInclusive<Duration>,
}
impl Default for Ranges {
    fn default() -> Self {
        Self {
            width: 30..=100,
            duration: Duration::from_millis(1300)..=Duration::from_millis(1700),
            delay: Duration::ZERO..=Duration::from_millis(500),
        }
    }
}
#[derive(Clone)]
struct Sample {
    ranges: Ranges,
    width: f32,
    duration: Duration,
    delay: Duration,
    started: Instant,
}
impl Sample {
    fn new(ranges: Ranges, cx: &App) -> Self {
        let random = RandomState::new();
        let unit = |key| (random.hash_one(key) >> 11) as f64 / ((1u64 << 53) as f64);
        let width = (*ranges.width.start() as f64
            + (unit(0u8)
                * (u16::from(*ranges.width.end()) - u16::from(*ranges.width.start()) + 1) as f64)
                .floor()) as f32
            / 100.;
        let sample_time = |range: &RangeInclusive<Duration>, key| {
            let min = range.start().as_secs_f64();
            let max = range.end().as_secs_f64();
            let rounded = ((min + unit(key) * (max - min)) * 100.).round() / 100.;
            Duration::try_from_secs_f64(rounded)
                .unwrap_or(*range.end())
                .clamp(*range.start(), *range.end())
        };
        Self {
            duration: sample_time(&ranges.duration, 1),
            delay: sample_time(&ranges.delay, 2),
            ranges,
            width,
            started: cx.background_executor().now(),
        }
    }
}

/// Stable unique IDs retain randomized presentation while mounted. Theme and
/// dimensions do not resample; changing any range resamples all source values.
/// Decorative only: the owner provides a loading status for its content region.
#[derive(IntoElement)]
#[must_use]
pub struct SkeletonLine {
    id: ElementId,
    ranges: Ranges,
    height: Pixels,
    block_height: Option<Pixels>,
}
impl SkeletonLine {
    /// Create a decorative loading line with stable animation identity.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            ranges: Ranges::default(),
            height: px(8.),
            block_height: None,
        }
    }
    /// Integer percent width, inclusive; finite source percentage range0–100.
    ///
    /// # Panics
    /// Panics when the width range is reversed or outside 0–100%.
    pub fn width_range(mut self, range: RangeInclusive<u8>) -> Self {
        assert!(
            range.start() <= range.end() && *range.end() <= 100,
            "SkeletonLine width must be ordered within 0–100%"
        );
        self.ranges.width = range;
        self
    }
    /// Duration range under the SkeletonLine contract.
    ///
    /// # Panics
    /// Panics when the duration range is reversed.
    pub fn duration_range(mut self, range: RangeInclusive<Duration>) -> Self {
        assert!(
            range.start() <= range.end(),
            "SkeletonLine duration range must be ordered"
        );
        self.ranges.duration = range;
        self
    }
    /// Delay range under the SkeletonLine contract.
    ///
    /// # Panics
    /// Panics when the delay range is reversed.
    pub fn delay_range(mut self, range: RangeInclusive<Duration>) -> Self {
        assert!(
            range.start() <= range.end(),
            "SkeletonLine delay range must be ordered"
        );
        self.ranges.delay = range;
        self
    }
    /// Set the skeleton line height.
    pub fn height(mut self, height: Pixels) -> Self {
        self.height = dimension(height);
        self
    }
    /// Native logical-pixel counterpart of CSS blockHeight.
    pub fn block_height(mut self, height: Pixels) -> Self {
        self.block_height = Some(dimension(height));
        self
    }
}
fn dimension(value: Pixels) -> Pixels {
    assert!(
        f32::from(value).is_finite() && value >= px(0.),
        "SkeletonLine dimensions must be finite and nonnegative"
    );
    value
}
impl RenderOnce for SkeletonLine {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let sample = window.with_id(self.id.clone(), |window| {
            window.use_keyed_state("sample", cx, |_, cx| Sample::new(self.ranges.clone(), cx))
        });
        sample.update(cx, |sample, cx| {
            if sample.ranges != self.ranges {
                *sample = Sample::new(self.ranges, cx);
            }
        });
        let sample = sample.read(cx).clone();
        let theme = theme(cx).clone();
        let reduced = cx.reduce_motion();
        let width = sample.width;
        let graphic = canvas(
            |_, _, _| (),
            move |bounds, _, window, cx| {
                if bounds.size.width <= px(0.)
                    || bounds.size.height <= px(0.)
                    || !bounds.intersects(&window.content_mask().bounds)
                {
                    return;
                }
                window.paint_quad(quad(
                    bounds,
                    theme.radii.xs,
                    theme.skeleton.base,
                    px(0.),
                    theme.skeleton.base.alpha(0.),
                    Default::default(),
                ));
                let timing = base::Timing::new(sample.duration)
                    .delay(base::SignedDuration::positive(sample.delay))
                    .iterations(base::IterationCount::Infinite)
                    .ease(base::Easing::EaseInOut);
                let current = timing.sample(
                    cx.background_executor()
                        .now()
                        .saturating_duration_since(sample.started),
                );
                let progress = if reduced || sample.duration.is_zero() {
                    0.5
                } else {
                    current.directed_progress
                };
                if !reduced && !sample.duration.is_zero() {
                    window.request_animation_frame();
                }
                paint_shimmer(
                    bounds,
                    progress,
                    theme.skeleton.shimmer,
                    theme.radii.xs,
                    window,
                );
            },
        )
        .w(relative(width))
        .h(self.height)
        .flex_shrink_0();
        div()
            .id(self.id)
            .test_support()
            .flex()
            .items_center()
            .min_w_0()
            .w_full()
            .when_some(self.block_height, |this, height| this.h(height))
            .child(graphic)
    }
}

// GPUI gradients have two stops. Split the source three-stop gradient at its
// peak, using rectangular masks over the SAME rounded outer quad. Each half
// retains the true outer corner shape, including during translation.
fn paint_shimmer(
    bounds: Bounds<Pixels>,
    progress: f32,
    color: gpui_kit::Hsla,
    radius: Pixels,
    window: &mut Window,
) {
    let shift = progress * 2. - 1.;
    // paint_quad covers content masks outward to device pixels. A fractional
    // shared split would therefore shade one column twice. Snap the common
    // peak once so both halves meet on the same device-pixel boundary.
    let scale = window.scale_factor();
    let peak_x =
        (f32::from(bounds.left() + bounds.size.width * (shift + 0.5)) * scale).round() / scale;
    let peak = (peak_x - f32::from(bounds.left())) / f32::from(bounds.size.width);
    for (start, end, rising) in [(shift, peak, true), (peak, shift + 1., false)] {
        let left = start.max(0.);
        let right = end.min(1.);
        if left >= right {
            continue;
        }
        let alpha = |x: f32| {
            let p = ((x - start) / (end - start)).clamp(0., 1.);
            color.alpha(color.a * if rising { p } else { 1. - p })
        };
        // Keep the shared mask edge in absolute coordinates. Reconstructing
        // it via width * normalized_peak can reintroduce float rounding and
        // undo the device-pixel snap.
        let mask_left = if rising {
            bounds.left()
        } else {
            px(peak_x).max(bounds.left())
        };
        let mask_right = if rising {
            px(peak_x).min(bounds.right())
        } else {
            bounds.right()
        };
        let mask = Bounds::from_corners(
            point(mask_left, bounds.top()),
            point(mask_right, bounds.bottom()),
        );
        window.with_content_mask(Some(ContentMask { bounds: mask }), |window| {
            window.paint_quad(quad(
                bounds,
                radius,
                linear_gradient(
                    90.,
                    linear_color_stop(alpha(left), left),
                    linear_color_stop(alpha(right), right),
                ),
                px(0.),
                color.alpha(0.),
                Default::default(),
            ));
        });
    }
}

debug_struct!(SkeletonLine { id });

#[cfg(test)]
mod tests;
