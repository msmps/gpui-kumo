//! Diagnostic fixture for KUMO-020: the gallery's retained Banner panel without
//! its other panels. Compare visible and accessible activation counts.
//! Platform visibility is shown to distinguish occlusion from stale rendering.
//! Gallery SVG assets are deliberately omitted; icons remain empty.

use gpui_kit::{
    App, AppContext, Context, InteractiveElement, IntoElement, KeyBinding, Menu, MenuItem,
    ParentElement, Render, StatefulInteractiveElement, Styled, Window, div, px,
};

gpui_kit::actions!(native_frame_probe, [Quit]);

#[path = "../src/banners.rs"]
mod banners;

struct Root {
    banners: gpui_kit::Entity<banners::Banners>,
}

impl Render for Root {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = gpui_kumo::theme(cx);
        div()
            .id("root")
            .size_full()
            .overflow_y_scroll()
            .flex()
            .flex_col()
            .p(px(32.))
            .bg(theme.colors.canvas)
            .text_color(theme.text.default)
            .child(format!("Platform visibility: {:?}", window.visibility()))
            .child(self.banners.clone())
    }
}

fn panel(theme: &gpui_kumo::Theme, title: &'static str) -> gpui_kit::Div {
    div()
        .flex()
        .flex_col()
        .flex_shrink_0()
        .gap(theme.spacing.sixteen)
        .p(px(24.))
        .bg(theme.colors.base)
        .rounded(theme.radii.lg)
        .border_1()
        .border_color(theme.colors.hairline)
        .child(div().font_weight(gpui_kit::FontWeight::MEDIUM).child(title))
}

fn main() {
    gpui_kit::application().run(|cx: &mut App| {
        gpui_kumo::init(cx);
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.set_menus([Menu::new("Frame Probe").items([MenuItem::action("Quit", Quit)])]);
        cx.on_window_closed(|cx, _| cx.quit()).detach();
        if let Err(error) = gpui_kit::open_window(
            gpui_kit::WindowOptions {
                window_bounds: Some(gpui_kit::WindowBounds::Windowed(
                    gpui_kit::Bounds::centered(None, gpui_kit::size(px(1040.), px(800.)), cx),
                )),
                ..Default::default()
            },
            cx,
            |window, cx| {
                window.set_window_title("Frame Probe");
                cx.new(|cx| Root {
                    banners: cx.new(banners::Banners::new),
                })
            },
        ) {
            eprintln!("Failed to open frame probe: {error:#}");
            cx.quit();
            return;
        }
        cx.activate(true);
    });
}
