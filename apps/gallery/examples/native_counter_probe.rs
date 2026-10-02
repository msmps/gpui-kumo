//! KUMO-020 diagnostic: compare retained state, platform visibility and pixels.
//! Return/Space increment the counter. macOS may suspend presentation while
//! this background-launched window is fully occluded; see the issue evidence.

use gpui_kit::{
    App, AppContext, Bounds, Context, FocusHandle, InteractiveElement, IntoElement, Menu, MenuItem,
    ParentElement, Render, Role, StatefulInteractiveElement, Styled, Window, WindowBounds,
    WindowOptions, div, px, size,
};

gpui_kit::actions!(native_counter_probe, [Quit]);

struct Counter {
    count: usize,
    focus: FocusHandle,
}

impl Render for Counter {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let label = format!(
            "Counter {}; visibility {:?}",
            self.count,
            window.visibility()
        );
        let counter = div()
            .id("counter")
            .role(Role::Label)
            .aria_label(label.clone())
            .child(label);
        div()
            .id("probe")
            .size_full()
            .bg(gpui_kit::white())
            .text_color(gpui_kit::black())
            .track_focus(&self.focus)
            .on_key_down(cx.listener(|this, event: &gpui_kit::KeyDownEvent, _, cx| {
                if matches!(event.keystroke.key.as_str(), "enter" | "space") {
                    this.count += 1;
                    cx.notify();
                }
            }))
            .child(counter)
    }
}

fn main() {
    gpui_kit::application().run(|cx: &mut App| {
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.set_menus([Menu::new("Counter Probe").items([MenuItem::action("Quit", Quit)])]);
        cx.on_window_closed(|cx, _| cx.quit()).detach();
        let bounds = Bounds::centered(None, size(px(600.), px(320.)), cx);
        if let Err(error) = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| {
                window.set_window_title("Counter Probe");
                cx.new(|cx| {
                    let focus = cx.focus_handle();
                    focus.focus(window, cx);
                    Counter { count: 0, focus }
                })
            },
        ) {
            eprintln!("Failed to open counter probe: {error:#}");
            cx.quit();
            return;
        }
        cx.activate(true);
    });
}
