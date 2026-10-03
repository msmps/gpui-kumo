use gpui_kit::{
    Animation, AnimationExt, Context, IntoElement, ParentElement, Render, Styled, TestAppContext,
    TestSupportExt, Window, div, px, test::TestWindowExt,
};
use std::time::Duration;

struct Chained;

impl Render for Chained {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().child(
            div()
                .id("sample")
                .test_support()
                .h(px(10.))
                .with_animations(
                    "chain",
                    vec![
                        Animation::new(Duration::from_millis(100)),
                        Animation::new(Duration::from_millis(100)),
                    ],
                    |sample, step, progress| sample.w(px(10. + 20. * (step as f32 + progress))),
                ),
        )
    }
}

#[gpui_kit::test]
fn chained_duration_animation_restarts_on_the_executor_clock(cx: &mut TestAppContext) {
    let (_, cx) = cx.add_window_view(|_, _| Chained);
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert_eq!(window.find("sample").bounds().size.width, px(10.));
        cx.background_executor()
            .advance_clock(Duration::from_millis(50));
        window.render_frame(cx);
        assert_eq!(window.find("sample").bounds().size.width, px(20.));
        cx.background_executor()
            .advance_clock(Duration::from_millis(75));
        window.render_frame(cx);
        assert_eq!(window.find("sample").bounds().size.width, px(30.));
        cx.background_executor()
            .advance_clock(Duration::from_millis(50));
        window.render_frame(cx);
        assert_eq!(window.find("sample").bounds().size.width, px(40.));
        cx.background_executor()
            .advance_clock(Duration::from_millis(75));
        window.render_frame(cx);
        assert_eq!(window.find("sample").bounds().size.width, px(50.));
        cx.set_reduce_motion(true);
        window.render_frame(cx);
        assert_eq!(window.find("sample").bounds().size.width, px(50.));
        window.simulate_next_frame(cx);
        window.render_frame(cx);
        assert_eq!(window.simulate_next_frame(cx), 0);
    });
}
