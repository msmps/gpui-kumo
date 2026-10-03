use super::*;
use gpui_kit::{
    Focusable, InteractiveElement, Role, TestAppContext, VisualTestContext, test::TestWindowExt,
};
struct Host {
    state: Entity<PaginationState>,
    events: Vec<usize>,
    accept: bool,
    simple: bool,
    info_text: Option<&'static str>,
    _events: Subscription,
}
impl Render for Host {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .tab_group()
            .w(px(360.))
            .flex()
            .flex_col()
            .child(crate::Button::new("before", "Before"))
            .child(
                Pagination::new("pages", &self.state)
                    .controls(if self.simple {
                        Controls::Simple
                    } else {
                        Controls::Full
                    })
                    .when_some(self.info_text, |this, text| {
                        this.content(move |parts, _, _| {
                            div()
                                .flex()
                                .items_center()
                                .child(parts.info.text(text))
                                .child(parts.controls)
                                .into_any_element()
                        })
                    }),
            )
            .child(crate::Button::new("after", "After"))
    }
}
fn host(
    cx: &mut TestAppContext,
    page: usize,
    total: PaginationTotal,
    accept: bool,
) -> (Entity<Host>, &mut VisualTestContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|window, cx| {
        let state = cx.new(|cx| PaginationState::new(page, 10, total, window, cx));
        let events = cx.subscribe_in(&state, window, |h: &mut Host, state, event, window, cx| {
            let PaginationEvent::Page(page) = *event;
            h.events.push(page);
            if h.accept {
                state.update(cx, |s, cx| s.set_page(page, window, cx));
            }
            cx.notify();
        });
        Host {
            state,
            events: vec![],
            accept,
            simple: false,
            info_text: None,
            _events: events,
        }
    });
    cx.update(|window, cx| {
        window.activate_window();
        window.render_frame(cx);
        window.render_frame(cx);
    });
    cx.run_until_parked();
    (view, cx)
}

#[gpui_kit::test]
fn info_exposes_default_and_localized_text_without_empty_fallback(cx: &mut TestAppContext) {
    let (view, cx) = host(cx, 1, PaginationTotal::Known(95), true);
    let state = cx.read(|cx| view.read(cx).state.clone());
    cx.update(|window, cx| {
        assert_eq!(window.find("pagination-info").role(), Some(Role::Label));
        assert_eq!(
            window.find("pagination-info").value(),
            Some("Showing 1-10 of 95")
        );
        assert_eq!(
            window.find("pagination-info").label(),
            Some("Showing 1-10 of 95")
        );
        view.update(cx, |v, cx| {
            v.info_text = Some("Résultats café 🦀 · localized long information");
            cx.notify();
        });
        window.render_frame(cx);
        assert_eq!(
            window.find("pagination-info").label(),
            Some("Résultats café 🦀 · localized long information")
        );
        view.update(cx, |v, cx| {
            v.info_text = None;
            cx.notify();
        });
        state.update(cx, |s, cx| {
            s.set_total(PaginationTotal::Known(0), window, cx)
        });
        window.render_frame(cx);
        assert_eq!(window.find("pagination-info").role(), None);
        assert_eq!(window.find("pagination-info").label(), None);
    });
}
#[gpui_kit::test]
fn controlled_navigation_rejects_or_accepts_once_through_real_input(cx: &mut TestAppContext) {
    let (view, cx) = host(cx, 1, PaginationTotal::Known(95), false);
    let state = cx.read(|cx| view.read(cx).state.clone());
    cx.update(|window, cx| {
        assert_eq!(window.find("control").role(), Some(Role::TextInput));
        assert_eq!(
            window.find("pagination-info").label(),
            Some("Showing 1-10 of 95")
        );
        window.click("pagination-first", cx);
        window.click("pagination-previous", cx);
        window.click("pagination-next", cx);
    });
    cx.run_until_parked();
    assert_eq!(view.read_with(cx, |v, _| v.events.clone()), vec![2]);
    assert_eq!(state.read_with(cx, |s, _| s.page()), 1);
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert!(state.read(cx).focuses[2].is_focused(window));
        window.press("space", cx);
    });
    cx.run_until_parked();
    assert_eq!(view.read_with(cx, |v, _| v.events.clone()), vec![2, 2]);
    cx.update(|window, cx| {
        view.update(cx, |v, _| v.accept = true);
        window.press("enter", cx);
    });
    cx.run_until_parked();
    assert_eq!(state.read_with(cx, |s, _| s.page()), 2);
    assert_eq!(view.read_with(cx, |v, _| v.events.len()), 3);
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.click("pagination-last", cx);
    });
    cx.run_until_parked();
    assert_eq!(state.read_with(cx, |s, _| s.page()), 10);
    assert_eq!(
        state.read_with(cx, |s, _| s.info().page_showing_range()),
        "91-95"
    );
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.click("pagination-next", cx);
        window.press("space", cx);
        window.press("enter", cx);
    });
    cx.run_until_parked();
    assert_eq!(view.read_with(cx, |v, _| v.events.len()), 4);
    cx.update(|window, cx| {
        window.click("pagination-first", cx);
    });
    cx.run_until_parked();
    assert_eq!(state.read_with(cx, |s, _| s.page()), 1);
}
#[gpui_kit::test]
fn page_draft_enter_blur_clamp_malformed_and_owner_changes(cx: &mut TestAppContext) {
    let (view, cx) = host(cx, 1, PaginationTotal::Known(100), true);
    let state = cx.read(|cx| view.read(cx).state.clone());
    let input = cx.read(|cx| state.read(cx).input.clone());
    cx.update(|window, cx| {
        input.read(cx).focus_handle(cx).focus(window, cx);
    });
    cx.run_until_parked();
    for (text, expected) in [
        ("5", 5),
        ("999999999999999999999999999999999999999", 10),
        ("-8", 1),
        ("7", 7),
        ("café 🦀", 7),
        ("3.2", 7),
        ("", 1),
    ] {
        cx.update(|window, cx| {
            input.update(cx, |s, cx| s.set_value("", window, cx));
            input.read(cx).focus_handle(cx).focus(window, cx);
        });
        cx.run_until_parked();
        // Empty text uses an actual deletion, since setters deliberately emit no Change.
        if text.is_empty() {
            cx.simulate_input("0");
            cx.simulate_keystrokes("backspace");
        } else {
            cx.simulate_input(text);
        }
        cx.simulate_keystrokes("enter");
        cx.run_until_parked();
        assert_eq!(state.read_with(cx, |s, _| s.page()), expected, "{text}");
        assert_eq!(
            input.read_with(cx, |s, cx| s.value(cx)).as_ref(),
            expected.to_string()
        );
        let count = view.read_with(cx, |v, _| v.events.len());
        cx.update(|window, cx| {
            window.focus_next(cx);
        });
        cx.run_until_parked();
        assert_eq!(
            view.read_with(cx, |v, _| v.events.len()),
            count,
            "blur after Enter repeats proposal"
        );
    }
    cx.update(|window, cx| {
        input.update(cx, |s, cx| s.set_value("", window, cx));
        input.read(cx).focus_handle(cx).focus(window, cx);
    });
    cx.run_until_parked();
    cx.simulate_input("3");
    cx.update(|window, cx| window.focus_next(cx));
    cx.run_until_parked();
    assert_eq!(state.read_with(cx, |s, _| s.page()), 3);
    let count = view.read_with(cx, |v, _| v.events.len());
    cx.update(|window, cx| {
        state.update(cx, |s, cx| {
            s.set_page(8, window, cx);
            s.set_total(PaginationTotal::Known(15), window, cx);
        });
    });
    cx.run_until_parked();
    assert_eq!(state.read_with(cx, |s, _| s.page()), 2);
    assert_eq!(input.read_with(cx, |s, cx| s.value(cx)).as_ref(), "2");
    assert_eq!(view.read_with(cx, |v, _| v.events.len()), count);
}
#[gpui_kit::test]
fn unknown_empty_disabled_and_hidden_focus_use_current_boundaries(cx: &mut TestAppContext) {
    let (view, cx) = host(
        cx,
        3,
        PaginationTotal::Unknown {
            has_next_page: true,
        },
        true,
    );
    let state = cx.read(|cx| view.read(cx).state.clone());
    cx.update(|window, cx| {
        assert!(
            !base::test_support::snapshots(window)
                .iter()
                .any(|n| n.role() == Some(Role::TextInput))
        );
        window.click("pagination-next", cx);
    });
    cx.run_until_parked();
    assert_eq!(state.read_with(cx, |s, _| s.page()), 4);
    assert_eq!(state.read_with(cx, |s, _| s.max_page()), None);
    cx.update(|window, cx| {
        state.update(cx, |s, cx| {
            s.set_total(
                PaginationTotal::Unknown {
                    has_next_page: false,
                },
                window,
                cx,
            )
        });
        window.render_frame(cx);
        window.click("pagination-next", cx);
        window.press("space", cx);
        window.press("enter", cx);
    });
    cx.run_until_parked();
    assert_eq!(view.read_with(cx, |v, _| v.events.len()), 1);
    cx.update(|window, cx| {
        state.update(cx, |s, cx| {
            s.set_total(PaginationTotal::Known(100), window, cx)
        });
        window.render_frame(cx);
        state
            .read(cx)
            .input
            .read(cx)
            .focus_handle(cx)
            .focus(window, cx);
        view.update(cx, |v, cx| {
            v.simple = true;
            cx.notify();
        });
        window.render_frame(cx);
        assert!(state.read(cx).focuses[1].is_focused(window));
        state.update(cx, |s, cx| s.set_disabled(true, window, cx));
        window.render_frame(cx);
        window.click("pagination-previous", cx);
        window.press("space", cx);
        window.press("enter", cx);
    });
    cx.run_until_parked();
    assert_eq!(view.read_with(cx, |v, _| v.events.len()), 1);
    cx.update(|window, cx| {
        window.click("before", cx);
        window.focus_next(cx);
        window.render_frame(cx);
        assert_eq!(window.find("after").focused(), Some(true));
        state.update(cx, |s, cx| {
            s.set_disabled(false, window, cx);
            s.set_total(PaginationTotal::Known(0), window, cx);
        });
        window.render_frame(cx);
        assert_eq!(state.read(cx).page(), 1);
        assert_eq!(state.read(cx).info().page_showing_range(), "0-0");
        assert_eq!(state.read(cx).model.next_page(), None);
    });
}
#[gpui_kit::test]
fn pagination_geometry_theme_localization_large_counts_and_repeated_draft_render(
    cx: &mut TestAppContext,
) {
    let (view, cx) = host(cx, 5, PaginationTotal::Known(usize::MAX), false);
    let state = cx.read(|cx| view.read(cx).state.clone());
    cx.update(|window, cx| {
        for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
            crate::set_appearance(appearance, cx);
            window.render_frame(cx);
            let first = window.find("pagination-first").bounds();
            let prev = window.find("pagination-previous").bounds();
            let input = window.find("surface").bounds();
            let next = window.find("pagination-next").bounds();
            let last = window.find("pagination-last").bounds();
            assert_eq!(first.size, gpui_kit::size(px(36.), px(36.)));
            assert_eq!(input.size.width, px(50.));
            assert_eq!(last.right() - first.left(), px(190.));
            for (a, b) in [(first, prev), (prev, input), (input, next), (next, last)] {
                assert_eq!(a.right() - b.left(), px(1.));
                assert_eq!(a.center().y, b.center().y);
            }
        }
        state.update(cx, |s, cx| {
            s.set_labels(
                PaginationLabels {
                    navigation: "Pages des résultats".into(),
                    next_page: "Page suivante".into(),
                    page_number: "Numéro de page".into(),
                    ..Default::default()
                },
                cx,
            )
        });
        window.render_frame(cx);
        assert_eq!(
            state.read(cx).labels.navigation.as_ref(),
            "Pages des résultats"
        );
        assert_eq!(window.find("control").label(), Some("Numéro de page"));
        state
            .read(cx)
            .input
            .read(cx)
            .focus_handle(cx)
            .focus(window, cx);
    });
    cx.run_until_parked();
    cx.simulate_input("99");
    cx.update(|window, cx| {
        for _ in 0..3 {
            window.render_frame(cx);
        }
    });
    assert_eq!(
        state
            .read_with(cx, |s, cx| s.input.read(cx).value(cx))
            .as_ref(),
        "599"
    );
}

#[gpui_kit::test]
fn owner_update_cancels_deferred_draft_and_preserves_owner_value(cx: &mut TestAppContext) {
    let (view, cx) = host(cx, 1, PaginationTotal::Known(100), true);
    let state = cx.read(|cx| view.read(cx).state.clone());
    let input = cx.read(|cx| state.read(cx).input.clone());
    cx.update(|window, cx| {
        input.update(cx, |s, cx| s.set_value("", window, cx));
        input.read(cx).focus_handle(cx).focus(window, cx);
    });
    cx.run_until_parked();
    cx.simulate_input("5");
    cx.update(|window, cx| {
        window.press("enter", cx);
        state.update(cx, |s, cx| s.set_page(2, window, cx));
    });
    cx.run_until_parked();
    assert_eq!(state.read_with(cx, |s, _| s.page()), 2);
    assert!(view.read_with(cx, |v, _| v.events.is_empty()));
    assert_eq!(input.read_with(cx, |s, cx| s.value(cx)).as_ref(), "2");
    // Reproduce the narrower interval after the subscribed commit queues its work.
    cx.update(|window, cx| {
        input.update(cx, |s, cx| s.set_value("", window, cx));
        input.read(cx).focus_handle(cx).focus(window, cx);
    });
    cx.run_until_parked();
    cx.simulate_input("6");
    cx.update(|window, cx| {
        state.update(cx, |s, cx| s.commit_draft(window, cx));
        state.update(cx, |s, cx| s.set_page(3, window, cx));
    });
    cx.run_until_parked();
    assert_eq!(state.read_with(cx, |s, _| s.page()), 3);
    assert!(view.read_with(cx, |v, _| v.events.is_empty()));
}
