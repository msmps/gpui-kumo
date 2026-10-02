use super::*;
use gpui_kit::AppContext;
use gpui_kit::{Context, Focusable, Render, TestAppContext, test::TestWindowExt};
struct Harness {
    state: Entity<InputState>,
    size: Size,
    narrow: bool,
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().w(px(if self.narrow { 52. } else { 220. })).child(
            InputGroup::new("group", &self.state)
                .size(self.size)
                .start(InputGroupAddon::text(if self.narrow {
                    "A very long addon that must be clipped"
                } else {
                    "/api/"
                }))
                .end(InputGroupAddon::text(".json")),
        )
    }
}
#[gpui_kit::test]
fn container_addons_preserve_editor_selection_and_source_geometry(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|window, cx| Harness {
        state: cx.new(|cx| InputState::new("Endpoint", window, cx)),
        size: Size::Base,
        narrow: false,
    });
    let state = cx.read(|cx| view.read(cx).state.clone());
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.click("addon-start", cx);
        assert!(state.read(cx).focus_handle(cx).is_focused(window));
    });
    cx.simulate_input("café 🦀");
    cx.update(|window, cx| {
        for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
            crate::set_appearance(appearance, cx);
            for (size, height) in [
                (Size::Xs, 24.),
                (Size::Sm, 28.),
                (Size::Base, 36.),
                (Size::Lg, 44.),
            ] {
                view.update(cx, |view, cx| {
                    view.size = size;
                    cx.notify();
                });
                window.render_frame(cx);
                let control = window.find("control").bounds();
                assert_eq!(control.size.height, px(height));
                assert_eq!(control.size.width, px(220.));
                assert!(window.find("editor-zone").bounds().size.width > px(0.));
                assert!(window.find("addon-end").bounds().right() <= control.right());
                assert_eq!(state.read(cx).value(cx).as_ref(), "café 🦀");
            }
        }
        view.update(cx, |view, cx| {
            view.narrow = true;
            cx.notify();
        });
        window.render_frame(cx);
        assert_eq!(window.find("control").bounds().size.width, px(52.));
        assert_eq!(window.find("content-row").bounds().size.width, px(52.));
        assert!(window.find("editor-zone").bounds().size.width >= px(0.));
        window.press("backspace", cx);
        assert_eq!(state.read(cx).value(cx).as_ref(), "café ");
        state.update(cx, |state, cx| state.set_disabled(true, cx));
        window.render_frame(cx);
        window.press("backspace", cx);
        assert_eq!(state.read(cx).value(cx).as_ref(), "café ");
    });
}
