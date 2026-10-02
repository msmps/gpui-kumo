use super::*;
use gpui_kit::{TestAppContext, VisualTestContext, test::TestWindowExt};

struct Harness {
    secret: Entity<SensitiveInputState>,
    copies: usize,
    changes: usize,
    _events: Subscription,
    size: Size,
    error: bool,
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .w(px(240.))
            .flex()
            .flex_col()
            .gap(px(24.))
            .child(
                SensitiveInput::new("secret", &self.secret)
                    .label(true)
                    .size(self.size)
                    .when(self.error, |input| input.error("Bad key", true)),
            )
            .child(crate::Button::new("outside", "Outside"))
    }
}

#[gpui_kit::test]
fn modes_preserve_value_and_reject_disabled_activation(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx, "café 🦀");
    let secret = cx.read(|cx| view.read(cx).secret.clone());
    cx.run_until_parked();
    cx.update(|window, _| window.activate_window());
    cx.run_until_parked();
    modes(&view, &secret, cx);
}
fn harness<'a>(
    cx: &'a mut TestAppContext,
    value: &str,
) -> (Entity<Harness>, &'a mut VisualTestContext) {
    cx.update(crate::init);
    let value: SharedString = value.to_owned().into();
    cx.add_window_view(move |window, cx| {
        let secret = cx.new(|cx| SensitiveInputState::new("API key", value, window, cx));
        let events = cx.subscribe(&secret, |view: &mut Harness, _, event, _| match event {
            SensitiveInputEvent::Copy => view.copies += 1,
            SensitiveInputEvent::Change => view.changes += 1,
            _ => {}
        });
        Harness {
            secret,
            copies: 0,
            changes: 0,
            _events: events,
            size: Size::Base,
            error: false,
        }
    })
}
fn modes(view: &Entity<Harness>, secret: &Entity<SensitiveInputState>, cx: &mut VisualTestContext) {
    for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
        cx.update(|window, cx| {
            crate::set_appearance(appearance, cx);
            window.render_frame(cx);
            assert_eq!(window.find("masked").role(), Some(gpui_kit::Role::Button));
            assert_eq!(window.find("masked").value(), None);
            assert_eq!(window.find("masked").label(), Some("API key, masked."));
            window.click("masked", cx);
            window.render_frame(cx);
            assert_eq!(secret.read(cx).mode(), Mode::Revealed);
            assert_eq!(window.find("control").value(), Some("café 🦀"));
        });
        cx.update(|window, cx| {
            window.click("copy", cx);
            window.render_frame(cx);
            assert_eq!(secret.read(cx).mode(), Mode::Revealed);
            assert_eq!(cx.read_from_clipboard().unwrap().text().unwrap(), "café 🦀");
            window.click("visibility", cx);
            window.render_frame(cx);
            assert_eq!(secret.read(cx).mode(), Mode::Masked);
            assert!(secret.read(cx).focus_handle(cx).is_focused(window));
        });
        cx.update(|window, cx| {
            window.press("space", cx);
            window.render_frame(cx);
            assert_eq!(secret.read(cx).mode(), Mode::Revealed);
            window.press("escape", cx);
            window.render_frame(cx);
            assert_eq!(secret.read(cx).mode(), Mode::Masked);
            window.press("enter", cx);
            window.render_frame(cx);
            assert_eq!(secret.read(cx).mode(), Mode::Revealed);
        });
        cx.update(|window, cx| {
            assert!(
                secret.read(cx).scope.contains_focused(window, cx),
                "editor must belong to secret scope"
            );
            window.click("outside", cx);
            assert_eq!(window.find("outside").focused(), Some(true));
        });
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.render_frame(cx);
            assert_eq!(secret.read(cx).mode(), Mode::Masked);
            secret.read(cx).focus_handle(cx).focus(window, cx);
            window.render_frame(cx);
            secret.update(cx, |state, cx| state.set_disabled(true, cx));
            window.render_frame(cx);
            window.click("masked", cx);
            window.press("space", cx);
            window.press("enter", cx);
            assert_eq!(secret.read(cx).mode(), Mode::Masked);
            secret.update(cx, |state, cx| state.set_disabled(false, cx));
        });
    }
    cx.update(|_, cx| {
        assert_eq!(view.read(cx).copies, 2);
        assert_eq!(view.read(cx).changes, 0);
        assert_eq!(secret.read(cx).value(cx).as_ref(), "café 🦀");
    });
}

#[gpui_kit::test]
fn source_sizes_and_error_height_are_stable_through_read_only_reveal(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx, "a long café 🦀 secret that exceeds its narrow field");
    let secret = cx.read(|cx| view.read(cx).secret.clone());
    cx.update(|window, cx| {
        secret.update(cx, |state, cx| state.set_read_only(true, cx));
        for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
            crate::set_appearance(appearance, cx);
            for (size, height, padding, icon) in [
                (Size::Xs, 20., 6., 12.),
                (Size::Sm, 26., 8., 12.),
                (Size::Base, 36., 12., 16.),
                (Size::Lg, 40., 16., 16.),
            ] {
                view.update(cx, |view, cx| {
                    view.size = size;
                    view.error = true;
                    cx.notify();
                });
                window.render_frame(cx);
                let masked = window.find("masked").bounds();
                assert_eq!(masked.size.height, px(height));
                window.click("masked", cx);
                window.render_frame(cx);
                assert_eq!(secret.read(cx).mode(), Mode::Revealed);
                assert!(secret.read(cx).eye_focus.is_focused(window));
                let body = window.find("sensitive-controls").bounds();
                assert_eq!(
                    body.size.height,
                    px(height),
                    "outer Field owns the only error message"
                );
                let eye = window.find("visibility").bounds();
                assert_eq!(eye.size.width, px(icon));
                assert_eq!(eye.size.height, px(icon));
                assert_eq!(eye.center().y, body.center().y);
                assert_eq!(body.right() - eye.right(), px(padding));
                secret
                    .read(cx)
                    .input
                    .read(cx)
                    .focus_handle(cx)
                    .focus(window, cx);
                window.press("backspace", cx);
                assert_eq!(
                    secret.read(cx).value(cx).as_ref(),
                    "a long café 🦀 secret that exceeds its narrow field"
                );
                window.click("visibility", cx);
                window.render_frame(cx);
                assert_eq!(
                    window.find("sensitive-controls").bounds().size.height,
                    px(height)
                );
            }
        }
    });
}

#[gpui_kit::test]
fn empty_editing_external_updates_and_copy_feedback_keep_one_owner(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx, "");
    let secret = cx.read(|cx| view.read(cx).secret.clone());
    let editor_id = cx.read(|cx| secret.read(cx).input.entity_id());
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert_eq!(secret.read(cx).mode(), Mode::Empty);
        assert_eq!(
            window.find("control").role(),
            Some(gpui_kit::Role::PasswordInput)
        );
        secret.read(cx).focus_handle(cx).focus(window, cx);
    });
    cx.simulate_input("café 🦀");
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert_eq!(secret.read(cx).mode(), Mode::Revealed);
        assert_eq!(secret.read(cx).value(cx).as_ref(), "café 🦀");
        assert_eq!(
            view.read(cx).changes,
            6,
            "one notification per simulated character edit"
        );
        window.click("copy", cx);
        assert!(secret.read(cx).copy_focus.is_focused(window));
    });
    cx.run_until_parked();
    cx.background_executor
        .advance_clock(Duration::from_millis(1500));
    cx.run_until_parked();
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.press("space", cx);
    });
    cx.run_until_parked();
    cx.background_executor
        .advance_clock(Duration::from_millis(500));
    cx.run_until_parked();
    cx.read(|cx| {
        assert!(
            secret.read(cx).copied,
            "older reset cannot erase latest feedback"
        )
    });
    cx.background_executor
        .advance_clock(Duration::from_millis(1500));
    cx.run_until_parked();
    cx.read(|cx| assert!(!secret.read(cx).copied));
    cx.update(|window, cx| window.press("enter", cx));
    cx.update(|window, cx| {
        secret.update(cx, |state, cx| {
            state.hide(false, window, cx);
            state.set_value("", window, cx);
        });
        assert_eq!(secret.read(cx).mode(), Mode::Empty);
        secret.update(cx, |state, cx| {
            state.set_value("external secret", window, cx)
        });
        window.render_frame(cx);
        assert_eq!(
            secret.read(cx).mode(),
            Mode::Empty,
            "source keeps empty mode for programmatic insertion"
        );
        assert_eq!(window.find("control").value(), Some("••••••••"));
        window.click("visibility", cx);
        window.render_frame(cx);
        assert_eq!(window.find("control").value(), Some("external secret"));
        assert_eq!(secret.read(cx).input.entity_id(), editor_id);
        assert_eq!(view.read(cx).changes, 6);
        assert_eq!(view.read(cx).copies, 3);
    });
}

#[gpui_kit::test]
fn clearing_a_disappearing_eye_restores_editor_and_tab_exit(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx, "");
    let secret = cx.read(|cx| view.read(cx).secret.clone());
    cx.run_until_parked();
    cx.update(|window, _| window.activate_window());
    cx.run_until_parked();
    for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
        cx.update(|window, cx| {
            crate::set_appearance(appearance, cx);
            secret.update(cx, |state, cx| {
                state.set_value("external secret", window, cx)
            });
            window.render_frame(cx);
            assert_eq!(secret.read(cx).mode(), Mode::Empty);
            secret.read(cx).focus_handle(cx).focus(window, cx);
            window.press("tab", cx);
            window.focus_next(cx);
            window.render_frame(cx);
            assert!(secret.read(cx).eye_focus.is_focused(window));
            secret.update(cx, |state, cx| state.set_value("", window, cx));
            window.render_frame(cx);
            assert!(secret.read(cx).focus_handle(cx).is_focused(window));
            window.press("tab", cx);
            window.focus_next(cx);
            window.render_frame(cx);
            assert_eq!(window.find("outside").focused(), Some(true));
            assert_eq!(view.read(cx).changes, 0);
        });
    }
}
