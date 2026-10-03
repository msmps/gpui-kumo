use gpui_kit::{Context, IntoElement, ParentElement, Render, Styled, Window, div, px};
use gpui_kumo::{Button, SkeletonLine, theme};
use std::time::Duration;

pub struct Skeletons;
impl Render for Skeletons {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = theme(cx);
        crate::panel(
            theme,
            "SkeletonLine · stable widths, shimmer and loading composition",
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(theme.spacing.twelve)
                .child(SkeletonLine::new("skeleton-default-a"))
                .child(SkeletonLine::new("skeleton-default-b"))
                .child(SkeletonLine::new("skeleton-default-c")),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(theme.spacing.twelve)
                .child(SkeletonLine::new("skeleton-wide").width_range(80..=100))
                .child(
                    SkeletonLine::new("skeleton-middle")
                        .width_range(60..=80)
                        .height(px(16.)),
                )
                .child(
                    SkeletonLine::new("skeleton-short")
                        .width_range(40..=60)
                        .height(px(24.)),
                ),
        )
        .child(div().flex().flex_col().gap(theme.spacing.four).children(
            [32., 48., 64.].into_iter().enumerate().map(|(i, height)| {
                div()
                    .flex()
                    .items_center()
                    .gap(theme.spacing.eight)
                    .child(format!("{height}px"))
                    .child(
                        SkeletonLine::new(("skeleton-block", i))
                            .width_range(50..=50)
                            .block_height(px(height)),
                    )
            }),
        ))
        .child(
            div()
                .max_w_full()
                .w(px(256.))
                .rounded(theme.radii.lg)
                .border_1()
                .border_color(theme.colors.line)
                .p(theme.spacing.sixteen)
                .flex()
                .flex_col()
                .gap(theme.spacing.eight)
                .child(
                    SkeletonLine::new("skeleton-card-heading")
                        .width_range(40..=60)
                        .block_height(px(32.)),
                )
                .child(SkeletonLine::new("skeleton-card-a"))
                .child(SkeletonLine::new("skeleton-card-b"))
                .child(SkeletonLine::new("skeleton-card-c").width_range(50..=70)),
        )
        .child(
            SkeletonLine::new("skeleton-static")
                .width_range(100..=100)
                .height(px(32.))
                .duration_range(Duration::ZERO..=Duration::ZERO),
        )
        .child(
            Button::new("skeleton-reduced", "Toggle reduced motion")
                .on_click(|_, _, cx| cx.set_reduce_motion(!cx.reduce_motion())),
        )
    }
}
