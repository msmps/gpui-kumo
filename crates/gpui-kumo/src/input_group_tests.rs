use super::*;
use gpui_kit::AppContext;
use gpui_kit::{Context, Focusable, Render, TestAppContext, test::TestWindowExt};
struct Harness {
    state: Entity<InputState>,
    size: Size,
    narrow: bool,
}

struct SuffixHarness {
    state: Entity<InputState>,
}
impl Render for SuffixHarness {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .w(px(180.))
            .child(InputGroup::new("domain", &self.state).suffix(".workers.dev"))
    }
}
#[gpui_kit::test]
fn suffix_follows_value_width_without_owning_selection_or_overflow(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|window, cx| SuffixHarness {
        state: cx.new(|cx| InputState::new("Subdomain", window, cx)),
    });
    let state = cx.read(|cx| view.read(cx).state.clone());
    cx.update(|window, cx| {
        window.activate_window();
        window.render_frame(cx);
        state.read(cx).focus_handle(cx).focus(window, cx);
    });
    cx.simulate_input("my-worker");
    cx.update(|window, cx| {
        assert_eq!(state.read(cx).value(cx).as_ref(), "my-worker");
        state.update(cx, |state, cx| state.set_value("", window, cx));
        window.render_frame(cx);
        let empty = window.find("editor-zone").bounds();
        assert_eq!(
            window.find("suffix").bounds().left(),
            empty.right() - SUFFIX_OVERLAP
        );
        state.update(cx, |state, cx| {
            state.set_placeholder("placeholder", window, cx)
        });
        window.render_frame(cx);
        let placeholder = window.find("editor-zone").bounds().size.width;
        assert!(placeholder > empty.size.width);
        state.update(cx, |state, cx| state.set_value("é🦀", window, cx));
        window.render_frame(cx);
        let short = window.find("editor-zone").bounds();
        assert_eq!(
            window.find("suffix").bounds().left(),
            short.right() - SUFFIX_OVERLAP
        );
        assert!(short.size.width > empty.size.width);
        state.read(cx).focus_handle(cx).focus(window, cx);
        window.press("end", cx);
        window.render_frame(cx);
        assert_visible_caret(window, cx);
        #[cfg(target_os = "macos")]
        window.press("cmd-a", cx);
        #[cfg(not(target_os = "macos"))]
        window.press("ctrl-a", cx);
        assert_eq!(state.read(cx).selected_value(cx).as_ref(), "é🦀");
        state.update(cx, |state, cx| {
            state.set_value("long-domain".repeat(30), window, cx)
        });
        for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
            crate::set_appearance(appearance, cx);
            window.render_frame(cx);
            let control = window.find("surface").bounds();
            let editor = window.find("editor-zone").bounds();
            let suffix = window.find("suffix").bounds();
            assert!(editor.right() <= control.right());
            assert!(suffix.right() <= control.right());
            assert!(
                suffix.size.width > px(12.),
                "natural suffix basis must retain content room"
            );
            assert_eq!(suffix.left(), editor.right() - SUFFIX_OVERLAP);
            assert_eq!(window.find("editor-clip").bounds().right(), suffix.left());
            window.press("end", cx);
            window.render_frame(cx);
            assert_visible_caret(window, cx);
            window.press("home", cx);
            window.render_frame(cx);
            assert_eq!(
                window.find("editor-clip").bounds().right(),
                window.find("suffix").bounds().left()
            );
            #[cfg(target_os = "macos")]
            window.press("cmd-a", cx);
            #[cfg(not(target_os = "macos"))]
            window.press("ctrl-a", cx);
            window.render_frame(cx);
            assert_eq!(
                state.read(cx).selected_value(cx).as_ref(),
                "long-domain".repeat(30)
            );
            #[cfg(target_os = "macos")]
            window.press("cmd-c", cx);
            #[cfg(not(target_os = "macos"))]
            window.press("ctrl-c", cx);
            assert_eq!(
                cx.read_from_clipboard().unwrap().text().unwrap(),
                "long-domain".repeat(30)
            );
            assert!(state.read(cx).focus_handle(cx).is_focused(window));
        }
    });
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
                let control = window.find("surface").bounds();
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
        assert_eq!(window.find("surface").bounds().size.width, px(52.));
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

fn assert_visible_caret(window: &Window, cx: &gpui_kit::App) {
    let width = (EDITOR_CARET_MARGIN - SUFFIX_OVERLAP).scale(window.scale_factor());
    let caret = window
        .painted_quads()
        .into_iter()
        .find(|q| {
            q.background.as_solid() == Some(crate::theme(cx).text.default)
                && q.bounds.size.width == width
        })
        .expect("focused End caret must paint");
    assert!(caret.content_mask.bounds.contains(&caret.bounds.origin));
    assert!(
        caret.bounds.right() <= caret.content_mask.bounds.right(),
        "whole caret stays visible before suffix"
    );
}
