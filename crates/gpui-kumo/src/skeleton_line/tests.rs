use super::*;
use gpui_kit::{Context, Render, TestAppContext, size, test::TestWindowExt};
struct Harness {
    fixed_width: Option<u8>,
    block: Pixels,
    height: Pixels,
    mounted: bool,
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let line = SkeletonLine::new("first")
            .height(self.height)
            .block_height(self.block)
            .duration_range(Duration::from_secs(1)..=Duration::from_secs(1))
            .delay_range(Duration::from_millis(200)..=Duration::from_millis(200));
        let line = if let Some(width) = self.fixed_width {
            line.width_range(width..=width)
        } else {
            line
        };
        div()
            .flex()
            .flex_col()
            .w(px(256.))
            .gap(px(16.))
            .when(self.mounted, |root| root.child(line))
            .child(
                SkeletonLine::new("second")
                    .width_range(50..=50)
                    .height(px(16.))
                    .duration_range(Duration::ZERO..=Duration::ZERO),
            )
    }
}
#[gpui_kit::test]
fn stable_percent_geometry_block_alignment_range_updates_and_themes(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|_, _| Harness {
        fixed_width: None,
        block: px(32.),
        height: px(8.),
        mounted: true,
    });
    cx.update(|window, cx| {
        cx.set_reduce_motion(true);
        window.render_frame(cx);
        let width = window.painted_quads()[0].bounds.size.width;
        assert!(
            (width.as_f32() / window.scale_factor()) >= 256. * 0.3
                && (width.as_f32() / window.scale_factor()) <= 256.
        );
        for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
            crate::set_appearance(appearance, cx);
            for _ in 0..5 {
                window.render_frame(cx);
                let root = window.find("first").bounds();
                let paint = &window.painted_quads()[0];
                assert_eq!(root.size.height, px(32.));
                assert_eq!(paint.bounds.size.width, width);
                assert_eq!(
                    (paint.bounds.size.height.as_f32() / window.scale_factor()),
                    8.
                );
                assert_eq!(
                    (paint.bounds.center().y.as_f32() / window.scale_factor()),
                    f32::from(root.center().y)
                );
                assert_eq!(paint.background.as_solid(), Some(theme(cx).skeleton.base));
                assert_eq!(
                    (paint.corner_radii.top_left.as_f32() / window.scale_factor()),
                    2.
                );
            }
            view.update(cx, |v, _| {
                v.block = px(48.);
                v.height = px(24.);
            });
            window.render_frame(cx);
            assert_eq!(window.painted_quads()[0].bounds.size.width, width);
            view.update(cx, |v, _| {
                v.block = px(32.);
                v.height = px(8.);
            });
        }
        view.update(cx, |v, _| v.fixed_width = Some(80));
        window.render_frame(cx);
        assert!(
            ((window.painted_quads()[0].bounds.size.width.as_f32() / window.scale_factor())
                - (256_f32 * 0.8).round())
            .abs()
                < 0.1
        );
        let second = window
            .painted_quads()
            .into_iter()
            .find(|q| {
                q.background.as_solid() == Some(theme(cx).skeleton.base)
                    && (q.bounds.size.height.as_f32() / window.scale_factor()) == 16.
            })
            .unwrap();
        assert_eq!(
            (second.bounds.size.width.as_f32() / window.scale_factor()),
            128.
        );
        view.update(cx, |v, _| v.fixed_width = Some(0));
        window.render_frame(cx);
        assert!(
            window
                .painted_quads()
                .iter()
                .all(|q| (q.bounds.size.height.as_f32() / window.scale_factor()) == 16.)
        );
        assert_eq!(window.find("first").role(), None);
    });
}
#[gpui_kit::test]
fn actual_shimmer_delay_translation_rounded_masks_reduced_motion_and_unmount(
    cx: &mut TestAppContext,
) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|_, _| Harness {
        fixed_width: Some(100),
        block: px(32.),
        height: px(8.),
        mounted: true,
    });
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert!(window.simulate_next_frame(cx) > 0);
        // First line remains before its delay; only its solid track is painted.
        assert_eq!(
            window
                .painted_quads()
                .iter()
                .filter(|q| (q.bounds.size.height.as_f32() / window.scale_factor()) == 8.)
                .count(),
            1
        );
    });
    cx.background_executor
        .advance_clock(Duration::from_millis(650));
    cx.update(|window, cx| {
        window.render_frame(cx);
        let before: Vec<_> = window
            .painted_quads()
            .into_iter()
            .filter(|q| {
                q.background.as_solid().is_none()
                    && (q.bounds.size.height.as_f32() / window.scale_factor()) == 8.
            })
            .map(|q| (q.content_mask.bounds, q.background))
            .collect();
        assert_eq!(before.len(), 2);
        assert_eq!(
            before[0].0.right(),
            before[1].0.left(),
            "gradient masks must meet without a doubled pixel column"
        );
        cx.background_executor()
            .advance_clock(Duration::from_millis(100));
        window.simulate_next_frame(cx);
        window.render_frame(cx);
        let after: Vec<_> = window
            .painted_quads()
            .into_iter()
            .filter(|q| {
                q.background.as_solid().is_none()
                    && (q.bounds.size.height.as_f32() / window.scale_factor()) == 8.
            })
            .map(|q| (q.content_mask.bounds, q.background))
            .collect();
        assert_ne!(
            before, after,
            "source eased translation must change actual gradient paint"
        );
        for q in window.painted_quads() {
            assert_eq!(
                (q.corner_radii.top_left.as_f32() / window.scale_factor()),
                2.
            );
            assert!(
                q.bounds.intersect(&q.content_mask.bounds).size.width
                    > gpui_kit::ScaledPixels::from(0.)
            );
        }
        cx.set_reduce_motion(true);
        window.simulate_next_frame(cx);
        window.render_frame(cx);
        assert_eq!(window.simulate_next_frame(cx), 0);
        assert!(
            window
                .painted_quads()
                .iter()
                .any(|q| q.background.as_solid().is_none())
        );
        cx.set_reduce_motion(false);
        view.update(cx, |v, _| v.mounted = false);
        window.render_frame(cx);
        window.simulate_next_frame(cx);
        window.render_frame(cx);
        assert_eq!(
            window.simulate_next_frame(cx),
            0,
            "unmounted line must stop scheduling frames"
        );
    });
}

#[test]
fn invalid_ranges_and_dimensions_rejected() {
    assert!(std::panic::catch_unwind(|| SkeletonLine::new("x").width_range(RangeInclusive::new(70, 40))).is_err());
    assert!(std::panic::catch_unwind(|| SkeletonLine::new("x").width_range(10..=101)).is_err());
    assert!(std::panic::catch_unwind(|| SkeletonLine::new("x").height(px(f32::NAN))).is_err());
    assert!(std::panic::catch_unwind(|| SkeletonLine::new("x").block_height(px(-1.))).is_err());
}

#[gpui_kit::test]
fn fully_clipped_line_does_not_paint_or_schedule_frames(cx: &mut TestAppContext) {
    cx.update(crate::init);
    struct Clipped;
    impl Render for Clipped {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .w(px(256.))
                .h(px(0.))
                .overflow_hidden()
                .child(SkeletonLine::new("offscreen").block_height(px(32.)))
        }
    }
    let (_, cx) = cx.add_window_view(|_, _| Clipped);
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert!(window.painted_quads().is_empty());
        assert_eq!(window.simulate_next_frame(cx), 0);
    });
}

#[gpui_kit::test]
fn fractional_paint_bounds_keep_gradient_masks_disjoint_across_scales(cx: &mut TestAppContext) {
    cx.update(crate::init);
    struct Fractional(f32);
    impl Render for Fractional {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let progress = self.0;
            canvas(
                |_, _, _| (),
                move |_, _, window, cx| {
                    paint_shimmer(
                        Bounds::new(point(px(31.), px(10.)), size(px(139.3), px(16.))),
                        progress,
                        theme(cx).skeleton.shimmer,
                        px(2.),
                        window,
                    );
                },
            )
            .w(px(256.))
            .h(px(32.))
        }
    }
    let (view, cx) = cx.add_window_view(|_, _| Fractional(0.51));
    for scale in [1., 1.25, 1.5, 2.] {
        cx.simulate_scale_factor_change(scale);
        cx.update(|window, cx| {
            for progress in [0.37, 0.51, 0.68] {
                view.update(cx, |v, _| v.0 = progress);
                window.render_frame(cx);
                let paint = window.painted_quads();
                assert_eq!(paint.len(), 2);
                assert_eq!(
                    paint[0].content_mask.bounds.right(),
                    paint[1].content_mask.bounds.left(),
                    "no overlap at scale {scale}, progress {progress}"
                );
            }
        });
    }
}
