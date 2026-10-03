//! Focused native foundation review: --dark and --width=520 (default1040).
use gpui_kit::{
    App, AppContext, Bounds, Context, IntoElement, ParentElement, Render, Styled, Window,
    WindowBounds, WindowOptions, div, px, size,
};
use gpui_kumo::{Appearance, set_appearance};
use kumo_gallery::foundations;
struct Review;
impl Render for Review {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = gpui_kumo::theme(cx);
        div()
            .size_full()
            .p(px(32.))
            .bg(theme.colors.canvas)
            .text_color(theme.text.default)
            .font_family(theme.typography.font_family.clone())
            .text_size(theme.typography.base.size)
            .line_height(theme.typography.base.line_height)
            .child(foundations::panels(theme, window.viewport_size().width))
    }
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let width = args
        .iter()
        .find_map(|arg| {
            arg.strip_prefix("--width=")
                .and_then(|value| value.parse::<f32>().ok())
        })
        .unwrap_or(1040.);
    let dark = args.iter().any(|arg| arg == "--dark");
    gpui_kit::application().run(move |cx: &mut App| {
        gpui_kumo::init(cx);
        set_appearance(
            if dark {
                Appearance::Dark
            } else {
                Appearance::Light
            },
            cx,
        );
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(width), px(1200.)),
                    cx,
                ))),
                ..Default::default()
            },
            |_, cx| cx.new(|_| Review),
        )
        .expect("open foundation review window");
        cx.activate(true);
    });
}
