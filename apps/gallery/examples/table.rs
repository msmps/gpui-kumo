//! Focused native table review: --dark and --width=520 (default1040).

use gpui_kit::{
    App, AppContext, Bounds, Context, InteractiveElement, IntoElement, ParentElement, Render,
    StatefulInteractiveElement, Styled, Window, WindowBounds, WindowOptions, div, px, size,
};
use gpui_kumo::{Appearance, set_appearance};
use kumo_gallery::tables;
gpui_kit::actions!(table_review, [Next, Previous, ToggleAppearance, Quit]);
struct Review {
    focus: gpui_kit::FocusHandle,
    tables: gpui_kit::Entity<tables::Tables>,
    _theme_subscription: gpui_kit::Subscription,
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
            .on_action(|_: &Quit, _, cx| cx.quit())
            .on_action(|_: &ToggleAppearance, _, cx| {
                set_appearance(gpui_kumo::theme(cx).appearance.opposite(), cx);
            })
            .flex()
            .flex_col()
            .gap(px(12.))
            .size_full()
            .overflow_y_scroll()
            .p(px(32.))
            .bg(theme.colors.canvas)
            .text_color(theme.text.default)
            .font_family(theme.typography.font_family.clone())
            .text_size(theme.typography.base.size)
            .line_height(theme.typography.base.line_height)
            .child(gpui_kumo::Button::new("before", "Before table"))
            .child(self.tables.clone())
            .child(gpui_kumo::Button::new("after", "After table"))
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
            gpui_kit::KeyBinding::new("cmd-l", ToggleAppearance, None),
            gpui_kit::KeyBinding::new("cmd-q", Quit, None),
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
            |_, cx| {
                cx.new(|cx| Review {
                    tables: cx.new(|_| tables::Tables::default()),
                    focus: cx.focus_handle(),
                    _theme_subscription: cx.observe_global::<gpui_kumo::Theme>(|_, cx| cx.notify()),
                })
            },
        )
        .expect("open table review window");
        cx.activate(true);
    });
}
