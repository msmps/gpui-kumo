//! Announcement workflow preview: --dark and --width=520 (default1040).

use gpui_kit::{
    App, AppContext, Bounds, Context, InteractiveElement, IntoElement, ParentElement, Render,
    Styled, Window, WindowBounds, WindowOptions, div, px, size,
};
use gpui_kumo::{Appearance, set_appearance};
use kumo_gallery::dialogs;
gpui_kit::actions!(dialog_review, [Next, Previous]);
struct Review {
    focus: gpui_kit::FocusHandle,
    menu: gpui_kit::Entity<dialogs::Dialogs>,
}
impl Render for Review {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = gpui_kumo::theme(cx);
        div()
            .id("review")
            .track_focus(&self.focus)
            .tab_group()
            .on_action(|_: &Next, window, cx| window.focus_next(cx))
            .on_action(|_: &Previous, window, cx| window.focus_prev(cx))
            .flex()
            .flex_col()
            .gap(px(12.))
            .size_full()
            .p(px(32.))
            .bg(theme.colors.canvas)
            .text_color(theme.text.default)
            .font_family(theme.typography.font_family.clone())
            .text_size(theme.typography.base.size)
            .line_height(theme.typography.base.line_height)
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_size(px(28.))
                            .line_height(px(36.))
                            .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                            .child("Kumo for GPUI"),
                    )
                    .child(
                        gpui_kumo::Button::new("appearance", "Switch theme").on_click(
                            |_, _, cx| {
                                let appearance =
                                    if gpui_kumo::theme(cx).appearance == Appearance::Dark {
                                        Appearance::Light
                                    } else {
                                        Appearance::Dark
                                    };
                                set_appearance(appearance, cx);
                            },
                        ),
                    ),
            )
            .child(
                div()
                    .text_color(theme.text.subtle)
                    .child("Native components for Rust applications."),
            )
            .child(self.menu.clone())
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
        cx.bind_keys([
            gpui_kit::KeyBinding::new("tab", Next, None),
            gpui_kit::KeyBinding::new("shift-tab", Previous, None),
        ]);
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
                    size(px(width), px(800.)),
                    cx,
                ))),
                ..Default::default()
            },
            |_window, cx| {
                cx.new(|cx| Review {
                    menu: cx.new(|cx| dialogs::Dialogs::new(_window, cx, true)),
                    focus: cx.focus_handle(),
                })
            },
        )
        .expect("open announcement preview window");
        cx.activate(true);
    });
}
