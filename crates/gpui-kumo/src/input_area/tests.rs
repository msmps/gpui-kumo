use super::*;
use gpui_kit::{TestAppContext, VisualTestContext, px, test::TestWindowExt};
struct Harness {
    state: Entity<InputAreaState>,
    changes: usize,
    width: f32,
    rows: usize,
    auto: Option<(usize, Option<usize>)>,
    size: Size,
    _events: Subscription,
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .w(px(self.width))
            .flex()
            .flex_col()
            .gap(px(24.))
            .child(crate::Button::new("before", "Before"))
            .child(
                InputArea::new("area", &self.state)
                    .size(self.size)
                    .rows(self.rows)
                    .show_label(true)
                    .when_some(self.auto, |area, (min, max)| area.auto_resize(min, max)),
            )
            .child(crate::Button::new("after", "After"))
    }
}
fn harness(cx: &mut TestAppContext) -> (Entity<Harness>, &mut VisualTestContext) {
    cx.update(crate::init);
    cx.add_window_view(|window, cx| {
        let state = cx.new(|cx| InputAreaState::new("Notes", window, cx));
        let events = cx.subscribe(&state, |this: &mut Harness, _, event, _| {
            if matches!(event, InputAreaEvent::Change) {
                this.changes += 1;
            }
        });
        Harness {
            state,
            changes: 0,
            width: 360.,
            rows: 2,
            auto: None,
            size: Size::Base,
            _events: events,
        }
    })
}
fn edit_key(key: &str) -> String {
    let modifier = if cfg!(target_os = "macos") {
        "cmd"
    } else {
        "ctrl"
    };
    format!("{modifier}-{key}")
}
fn settle(cx: &mut VisualTestContext) {
    cx.run_until_parked();
    // Wrap geometry is available after Base's first paint and feeds the next frame.
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.render_frame(cx);
    });
    cx.run_until_parked();
}
#[gpui_kit::test]
fn editing_retains_unicode_clipboard_history_and_tab_exit(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    let state = cx.read(|cx| view.read(cx).state.clone());
    cx.update(|window, _| window.activate_window());
    settle(cx);
    cx.update(|window, cx| state.read(cx).focus_handle(cx).focus(window, cx));
    cx.simulate_input("café 🦀");
    cx.update(|window, cx| {
        window.press("enter", cx);
    });
    cx.simulate_input("next");
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert_eq!(state.read(cx).value(cx).as_ref(), "café 🦀\nnext");
        assert_eq!(view.read(cx).changes, 11);
        assert_eq!(
            window.find("control").role(),
            Some(gpui_kit::Role::MultilineTextInput)
        );
        window.press(&edit_key("a"), cx);
        window.press(&edit_key("c"), cx);
        assert_eq!(state.read(cx).selected_value(cx).as_ref(), "café 🦀\nnext");
        assert_eq!(
            cx.read_from_clipboard().unwrap().text().unwrap(),
            "café 🦀\nnext"
        );
        window.press("backspace", cx);
        window.press(&edit_key("z"), cx);
        assert_eq!(state.read(cx).value(cx).as_ref(), "café 🦀\nnext");
        window.press("tab", cx);
        window.render_frame(cx);
        assert_eq!(window.find("after").focused(), Some(true));
        window.press("shift-tab", cx);
        window.focus_prev(cx);
        window.render_frame(cx);
        assert!(state.read(cx).focus_handle(cx).is_focused(window));
        window.press("shift-tab", cx);
        window.render_frame(cx);
        assert_eq!(window.find("before").focused(), Some(true));
    });
}
#[gpui_kit::test]
fn row_policy_grows_shrinks_rewraps_and_preserves_editor(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    let state = cx.read(|cx| view.read(cx).state.clone());
    let editor = cx.read(|cx| state.read(cx).editor.entity_id());
    for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
        for size in [Size::Xs, Size::Sm, Size::Base, Size::Lg] {
            cx.update(|_, cx| {
                crate::set_appearance(appearance, cx);
                view.update(cx, |v, _| {
                    v.size = size;
                    v.rows = 4;
                    v.auto = None;
                    v.width = 360.;
                });
            });
            settle(cx);
            let four = cx.update(|window, _| window.find("surface").bounds().size.height);
            cx.update(|window, cx| {
                view.update(cx, |v, _| v.auto = Some((1, Some(3))));
                state.update(cx, |s, cx| {
                    s.set_value("one\ntwo\nthree\nfour\nfive", window, cx)
                });
            });
            settle(cx);
            let capped = cx.update(|window, _| window.find("surface").bounds().size.height);
            assert!(capped < four);
            cx.update(|window, cx| state.update(cx, |s, cx| s.set_value("", window, cx)));
            settle(cx);
            let empty = cx.update(|window, _| window.find("surface").bounds().size.height);
            assert!(
                empty < capped,
                "empty {empty:?}, cap {capped:?}, four {four:?}, size {size:?}"
            );
            cx.update(|window, cx| {
                view.update(cx, |v, _| { v.auto = Some((1, None)); v.width = 500.; });
                state.update(cx, |s, cx| s.set_value("Long café 🦀 notes with wrapping across several words repeated across a narrow textarea to require additional visible rows.", window, cx));
            });
            settle(cx);
            let wide = cx.update(|window, _| window.find("surface").bounds().size.height);
            cx.update(|_, cx| view.update(cx, |v, _| v.width = 180.));
            settle(cx);
            let narrow = cx.update(|window, _| window.find("surface").bounds().size.height);
            assert!(
                narrow > wide,
                "narrow width must rewrap and grow: {wide:?} → {narrow:?}"
            );
            cx.read(|cx| {
                assert_eq!(state.read(cx).editor.entity_id(), editor);
                assert_eq!(view.read(cx).changes, 0);
            });
        }
    }
}
#[gpui_kit::test]
fn current_availability_rejects_edits_without_rejecting_owner_updates(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    let state = cx.read(|cx| view.read(cx).state.clone());
    cx.update(|window, cx| {
        state.update(cx, |s, cx| {
            s.set_value("café\n🦀", window, cx);
            s.set_read_only(true, cx);
        });
        window.render_frame(cx);
        state.read(cx).focus_handle(cx).focus(window, cx);
        window.press(&edit_key("a"), cx);
        window.press(&edit_key("c"), cx);
        window.press("backspace", cx);
        window.press("enter", cx);
        assert_eq!(state.read(cx).value(cx).as_ref(), "café\n🦀");
        assert_eq!(
            cx.read_from_clipboard().unwrap().text().unwrap(),
            "café\n🦀"
        );
        state.update(cx, |s, cx| {
            s.set_read_only(false, cx);
            s.set_disabled(true, cx);
        });
        window.render_frame(cx);
        window.press("backspace", cx);
        window.press("enter", cx);
        window.press(&edit_key("v"), cx);
        assert_eq!(state.read(cx).value(cx).as_ref(), "café\n🦀");
        window.click("after", cx);
        assert_eq!(window.find("after").focused(), Some(true));
        window.click("control", cx);
        assert_eq!(
            window.find("after").focused(),
            Some(true),
            "disabled pointer must not take focus"
        );
        state.update(cx, |s, cx| s.set_value("owner\nreplacement", window, cx));
        assert_eq!(state.read(cx).value(cx).as_ref(), "owner\nreplacement");
        assert_eq!(view.read(cx).changes, 0);
    });
}

#[gpui_kit::test]
fn policy_width_and_font_changes_preserve_selection_and_undo(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    let state = cx.read(|cx| view.read(cx).state.clone());
    cx.update(|window, _| window.activate_window());
    settle(cx);
    cx.update(|window, cx| state.read(cx).focus_handle(cx).focus(window, cx));
    cx.simulate_input("café 🦀");
    cx.update(|window, cx| {
        window.press(&edit_key("a"), cx);
    });
    for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
        for size in [Size::Xs, Size::Sm, Size::Base, Size::Lg] {
            cx.update(|_, cx| {
                crate::set_appearance(appearance, cx);
                view.update(cx, |v, _| {
                    v.size = size;
                    v.auto = Some((3, Some(5)));
                    v.width = 180.;
                });
            });
            settle(cx);
            cx.read(|cx| assert_eq!(state.read(cx).selected_value(cx).as_ref(), "café 🦀"));
            cx.update(|_, cx| {
                view.update(cx, |v, _| {
                    v.width = 500.;
                    v.auto = None;
                    v.rows = 2;
                })
            });
            settle(cx);
            cx.read(|cx| assert_eq!(state.read(cx).selected_value(cx).as_ref(), "café 🦀"));
        }
    }
    cx.update(|window, cx| {
        window.press("backspace", cx);
        assert_eq!(state.read(cx).value(cx).as_ref(), "");
        view.update(cx, |v, _| {
            v.auto = Some((1, Some(4)));
            v.size = Size::Xs;
        });
    });
    settle(cx);
    cx.update(|window, cx| {
        window.press(&edit_key("z"), cx);
        assert_eq!(state.read(cx).value(cx).as_ref(), "café 🦀");
    });
    cx.run_until_parked();
    cx.read(|cx| assert_eq!(view.read(cx).changes, 8));
}
