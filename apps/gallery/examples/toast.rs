//! Native Toast review: --dark and --width=520 (default1040).
#[path = "../src/toasts.rs"]
mod toasts;
use gpui_kit::{
    App, AppContext, Bounds, Context, Entity, FocusHandle, Render, Subscription, Window,
    WindowBounds, WindowOptions, div, prelude::*, px, size,
};
use gpui_kumo::{Appearance, Button, ToastEvent, ToastState, ToastViewport, set_appearance};
gpui_kit::actions!(toast_review, [Next, Previous]);
struct Review {
    toasts: Entity<ToastState>,
    focus: FocusHandle,
    actions: usize,
    _events: Subscription,
}
impl Render for Review {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = gpui_kumo::theme(cx);
        div()
            .id("review")
            .track_focus(&self.focus)
            .tab_group()
            .size_full()
            .p(px(32.))
            .bg(theme.colors.canvas)
            .text_color(theme.text.default)
            .font_family(theme.typography.font_family.clone())
            .text_size(px(14.))
            .line_height(px(21.))
            .on_action(|_: &Next, window, cx| window.focus_next(cx))
            .on_action(|_: &Previous, window, cx| window.focus_prev(cx))
            .flex()
            .flex_col()
            .gap_4()
            .child(toasts::controls(&self.toasts))
            .child(Button::new("after", "After toast"))
            .child(gpui_kumo::Text::new(
                "actions-result",
                format!("{} toast actions", self.actions),
            ))
            .child(ToastViewport::new("feedback", &self.toasts))
    }
}
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
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
            |_, cx| {
                cx.new(|cx| {
                    let toasts = cx.new(ToastState::new);
                    let events =
                        cx.subscribe(&toasts, |state: &mut Review, _, event: &ToastEvent, cx| {
                            if matches!(event, ToastEvent::Action { .. }) {
                                state.actions += 1;
                                cx.notify();
                            }
                        });
                    Review {
                        toasts,
                        focus: cx.focus_handle(),
                        actions: 0,
                        _events: events,
                    }
                })
            },
        )
        .expect("open toast review");
        cx.activate(true);
    });
}
