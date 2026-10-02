//! Native accessibility state fixture. GPUI's debug dump complements OS AX inspection.
//! That dump omits disabled/busy/read-only/invalid bits; do not infer those from descriptions.
//! Run with KUMO_A11Y_DUMP=/tmp/kumo-states.json and an accessibility client connected.

use gpui_kit::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div, px,
};
use gpui_kumo::{Button, Input, InputState};

struct States {
    mode: usize,
    input: Entity<InputState>,
}

impl Render for States {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mode = self.mode;
        self.input.update(cx, |input, cx| {
            input.set_disabled(mode == 1, cx);
            input.set_read_only(mode == 2, cx);
        });
        if let Ok(path) = std::env::var("KUMO_A11Y_DUMP") {
            window.on_next_frame(move |window, _| {
                if let Some(tree) = window.debug_a11y_tree_json() {
                    std::fs::write(&path, tree).expect("write native accessibility evidence");
                }
            });
        }
        let theme = gpui_kumo::theme(cx);
        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(32.))
            .gap(px(24.))
            .bg(theme.colors.canvas)
            .text_color(theme.text.default)
            .child(format!(
                "State {mode}: enabled / disabled / read-only / loading-invalid"
            ))
            .child(
                Button::new("subject", "State subject")
                    .disabled(mode == 1)
                    .loading(mode == 3),
            )
            .child(if mode == 3 {
                Input::new("input", &self.input).error("Invalid example")
            } else {
                Input::new("input", &self.input)
            })
            .child(
                Button::new("advance", "Next state").on_click(cx.listener(|state, _, _, cx| {
                    state.mode = (state.mode + 1) % 4;
                    cx.notify();
                })),
            )
    }
}

fn main() {
    gpui_kit::application().run(|cx: &mut App| {
        gpui_kumo::init(cx);
        cx.on_window_closed(|cx, _| cx.quit()).detach();
        gpui_kit::open_window(Default::default(), cx, |window, cx| {
            cx.new(|cx| States {
                mode: 0,
                input: cx.new(|cx| InputState::new("State input", window, cx)),
            })
        })
        .expect("open accessibility fixture");
        cx.activate(true);
    });
}
