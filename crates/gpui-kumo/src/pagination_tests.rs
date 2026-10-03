use super::*;
use gpui_kit::{
    Focusable, InteractiveElement, Role, TestAppContext, VisualTestContext, test::TestWindowExt,
};
gpui_kit::actions!(pagination_test, [FocusNext, FocusPrevious]);
struct Host {
    state: Entity<PaginationState>,
    events: Vec<usize>,
    sizes: Vec<usize>,
    show_size: bool,
    size_label: Option<&'static str>,
    accept: bool,
    simple: bool,
    dropdown: bool,
    mounted: bool,
    info_text: Option<&'static str>,
    _events: Subscription,
}
impl Render for Host {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .on_action(|_: &FocusNext, window, cx| window.focus_next(cx))
            .on_action(|_: &FocusPrevious, window, cx| window.focus_prev(cx))
            .tab_group()
            .w(px(360.))
            .flex()
            .flex_col()
            .child(crate::Button::new("before", "Before"))
            .when(self.mounted, |this| {
                this.child(
                    Pagination::new("pages", &self.state)
                        .page_size(self.show_size)
                        .page_selector(if self.dropdown {
                            PageSelector::Dropdown
                        } else {
                            PageSelector::Input
                        })
                        .controls(if self.simple {
                            Controls::Simple
                        } else {
                            Controls::Full
                        })
                        .when_some(self.size_label, |this, label| {
                            this.content(move |parts, _, _| {
                                div()
                                    .flex()
                                    .flex_wrap()
                                    .items_center()
                                    .gap(px(8.))
                                    .child(parts.info)
                                    .child(parts.page_size.label(label))
                                    .child(parts.controls)
                                    .into_any_element()
                            })
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
            })
            .child(crate::Button::new("after", "After"))
    }
}
fn host(
    cx: &mut TestAppContext,
    page: usize,
    total: PaginationTotal,
    accept: bool,
) -> (Entity<Host>, &mut VisualTestContext) {
    cx.update(|cx| {
        crate::init(cx);
        // Match the gallery's global traversal path. Select handles Tab itself
        // while open; surviving ordinary controls use the host's actions.
        cx.bind_keys([
            gpui_kit::KeyBinding::new("tab", FocusNext, None),
            gpui_kit::KeyBinding::new("shift-tab", FocusPrevious, None),
        ]);
    });
    let (view, cx) = cx.add_window_view(|window, cx| {
        let state = cx.new(|cx| PaginationState::new(page, 10, total, window, cx));
        let events = cx.subscribe_in(&state, window, |h: &mut Host, state, event, window, cx| {
            let PaginationEvent::Page(page) = *event else {
                if let PaginationEvent::PageSize(size) = *event {
                    h.sizes.push(size);
                    if h.accept {
                        state.update(cx, |s, cx| s.set_per_page(size, window, cx));
                    }
                    cx.notify();
                }
                return;
            };
            h.events.push(page);
            if h.accept {
                state.update(cx, |s, cx| s.set_page(page, window, cx));
            }
            cx.notify();
        });
        Host {
            state,
            events: vec![],
            sizes: vec![],
            show_size: false,
            size_label: None,
            accept,
            simple: false,
            dropdown: false,
            mounted: true,
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
            assert_eq!(first.size, gpui_kit::size(px(42.), px(36.)));
            assert_eq!(input.size.width, px(50.));
            assert_eq!(last.right() - first.left(), px(214.));
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

#[gpui_kit::test]
fn page_size_controlled_proposals_owner_sync_and_source_labels(cx: &mut TestAppContext) {
    let (view, cx) = host(cx, 5, PaginationTotal::Known(95), false);
    let state = cx.read(|cx| view.read(cx).state.clone());
    let select = cx.read(|cx| state.read(cx).page_size.clone());
    cx.update(|window, cx| {
        view.update(cx, |v, cx| {
            v.show_size = true;
            cx.notify();
        });
        state.update(cx, |s, cx| {
            s.set_labels(
                PaginationLabels {
                    page_size: "Résultats par page café 🦀".into(),
                    ..Default::default()
                },
                cx,
            )
        });
        window.render_frame(cx);
        assert_eq!(window.find("trigger").role(), Some(Role::ComboBox));
        assert_eq!(
            window.find("trigger").label(),
            Some("Résultats par page café 🦀")
        );
        assert_eq!(window.find("trigger").value(), Some("10"));
        assert_eq!(
            window.find("pagination-page-size-label").value(),
            Some("Per page:")
        );
        window.click("hit", cx);
        for _ in 0..3 {
            window.render_frame(cx);
        }
        assert!(select.read(cx).is_open());
        for value in [25usize, 50, 100, 250] {
            assert_eq!(
                window.find(("pagination-size", value)).bounds().size.height,
                px(33.),
                "numeric option must stay on one source line"
            );
            assert_eq!(
                window.find(("pagination-size", value)).label(),
                Some(value.to_string().as_str())
            );
        }
        window.click(("pagination-size", 25usize), cx);
    });
    cx.run_until_parked();
    assert_eq!(view.read_with(cx, |v, _| v.sizes.clone()), vec![25]);
    assert_eq!(
        state.read_with(cx, |s, _| (s.page(), s.per_page())),
        (5, 10)
    );
    assert_eq!(
        select.read_with(cx, |s, _| s.value().clone()),
        SelectValue::Single(Some(10))
    );
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert!(!select.read(cx).is_open());
        assert!(select.read(cx).focus_handle().is_focused(window));
        view.update(cx, |v, _| v.accept = true);
        window.press("space", cx);
        for _ in 0..3 {
            window.render_frame(cx);
        }
        window.press("down", cx);
        window.render_frame(cx);
        window.press("enter", cx);
    });
    cx.run_until_parked();
    assert_eq!(view.read_with(cx, |v, _| v.sizes.clone()), vec![25, 50]);
    assert_eq!(
        state.read_with(cx, |s, _| (s.page(), s.per_page())),
        (2, 50)
    );
    assert_eq!(
        state
            .read_with(cx, |s, cx| s.input.read(cx).value(cx))
            .as_ref(),
        "2"
    );
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert_eq!(window.find("trigger").value(), Some("50"));
        window.press("enter", cx);
        window.render_frame(cx);
        window.press("enter", cx); // Same selected size closes, no duplicate proposal.
        state.update(cx, |s, cx| {
            s.set_per_page(25, window, cx);
            s.set_page(1, window, cx);
        });
    });
    cx.run_until_parked();
    assert_eq!(view.read_with(cx, |v, _| v.sizes.len()), 2);
    assert!(view.read_with(cx, |v, _| v.events.is_empty()));
    assert_eq!(
        select.read_with(cx, |s, _| s.value().clone()),
        SelectValue::Single(Some(25))
    );
}

#[gpui_kit::test]
fn page_size_changed_options_owner_updates_empty_and_disabled_paths(cx: &mut TestAppContext) {
    let (view, cx) = host(
        cx,
        1,
        PaginationTotal::Unknown {
            has_next_page: true,
        },
        true,
    );
    let state = cx.read(|cx| view.read(cx).state.clone());
    let select = cx.read(|cx| state.read(cx).page_size.clone());
    cx.update(|window, cx| {
        view.update(cx, |v, cx| {
            v.show_size = true;
            cx.notify();
        });
        state.update(cx, |s, cx| s.set_page_size_options(vec![10, 20, 50], cx));
        window.render_frame(cx);
        window.click("hit", cx);
        for _ in 0..3 {
            window.render_frame(cx);
        }
        // Replace/reorder while open; removed highlighted option cannot submit a stale value.
        window.press("down", cx);
        window.render_frame(cx);
        state.update(cx, |s, cx| {
            s.set_page_size_options(vec![50, 10], cx);
            s.set_per_page(250, window, cx);
        });
        window.render_frame(cx);
        assert_eq!(window.find("trigger").value(), Some("250"));
        window.press("enter", cx);
    });
    cx.run_until_parked();
    assert_eq!(view.read_with(cx, |v, _| v.sizes.clone()), vec![50]);
    assert_eq!(state.read_with(cx, |s, _| s.per_page()), 50);
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.press("enter", cx);
        window.render_frame(cx);
        state.update(cx, |s, cx| s.set_disabled(true, window, cx));
        window.render_frame(cx);
        assert!(!select.read(cx).is_open());
        // Synthetic Select disabled metadata is verified by the actual native probe.
        window.click("hit", cx);
        window.press("space", cx);
        window.press("enter", cx);
        state.update(cx, |s, cx| {
            s.set_page_size_options(vec![], cx);
            s.set_disabled(false, window, cx);
            s.set_total(PaginationTotal::Known(0), window, cx);
        });
        window.render_frame(cx);
        window.click("hit", cx);
        for _ in 0..3 {
            window.render_frame(cx);
        }
        assert!(select.read(cx).is_open());
        assert_eq!(window.find("list").role(), Some(Role::ListBox));
        window.press("enter", cx);
        window.press("escape", cx);
        window.render_frame(cx);
        assert!(select.read(cx).focus_handle().is_focused(window));
        assert_eq!(window.find("trigger").value(), Some("50"));
    });
    cx.run_until_parked();
    assert_eq!(view.read_with(cx, |v, _| v.sizes.clone()), vec![50]);
}

#[gpui_kit::test]
fn page_size_unmount_remount_focus_and_theme_geometry(cx: &mut TestAppContext) {
    let (view, cx) = host(cx, 1, PaginationTotal::Known(usize::MAX), true);
    let state = cx.read(|cx| view.read(cx).state.clone());
    let select = cx.read(|cx| state.read(cx).page_size.clone());
    cx.update(|window, cx| {
        for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
            crate::set_appearance(appearance, cx);
            view.update(cx, |v, cx| {
                v.show_size = true;
                v.size_label = Some("Par page café 🦀");
                cx.notify();
            });
            state.update(cx, |s, cx| s.set_per_page(usize::MAX, window, cx));
            for _ in 0..3 {
                window.render_frame(cx);
            }
            let bounds = window.find("trigger").bounds();
            assert_eq!(bounds.size.height, px(36.));
            assert!(bounds.left() >= px(0.) && bounds.right() <= px(360.));
            assert_eq!(
                window.find("trigger").value(),
                Some(usize::MAX.to_string().as_str())
            );
            assert_eq!(
                window.find("pagination-page-size-label").value(),
                Some("Par page café 🦀")
            );
            window.click("hit", cx);
            for _ in 0..3 {
                window.render_frame(cx);
            }
            assert!(select.read(cx).is_open());
            window.press("escape", cx);
            window.render_frame(cx);
            assert!(select.read(cx).focus_handle().is_focused(window));
        }
        view.update(cx, |v, cx| {
            v.size_label = None;
            cx.notify();
        });
        window.render_frame(cx);
        window.click("hit", cx);
        window.render_frame(cx);
        assert!(select.read(cx).is_open());
        view.update(cx, |v, cx| {
            v.show_size = false;
            cx.notify();
        });
        window.render_frame(cx);
    });
    cx.run_until_parked();
    assert!(!select.read_with(cx, |s, _| s.is_open()));
    cx.update(|window, cx| {
        assert!(!select.read(cx).focus_handle().is_focused(window));
        select.update(cx, |s, cx| s.set_open(true, window, cx));
        assert!(!select.read(cx).is_open());
        view.update(cx, |v, cx| {
            v.show_size = true;
            v.size_label = Some("");
            cx.notify();
        });
        for _ in 0..3 {
            window.render_frame(cx);
        }
        assert_eq!(
            select.read(cx).value(),
            &SelectValue::Single(Some(usize::MAX))
        );
        window.click("hit", cx);
        window.render_frame(cx);
        assert!(select.read(cx).is_open());
        window.press("tab", cx);
        window.render_frame(cx);
        assert!(!select.read(cx).is_open());
        assert!(!select.read(cx).focus_handle().is_focused(window));
    });
}

fn mount_dropdown(view: &Entity<Host>, window: &mut Window, cx: &mut App) {
    view.update(cx, |v, cx| {
        v.dropdown = true;
        cx.notify();
    });
    for _ in 0..3 {
        window.render_frame(cx);
    }
}
fn open_page_select(window: &mut Window, cx: &mut App) {
    window.click("hit", cx);
    for _ in 0..3 {
        window.render_frame(cx);
    }
}

#[gpui_kit::test]
fn dropdown_proposes_once_rejects_accepts_and_synchronizes_native_draft(cx: &mut TestAppContext) {
    let (view, cx) = host(cx, 3, PaginationTotal::Known(95), false);
    let state = cx.read(|cx| view.read(cx).state.clone());
    let select = cx.read(|cx| state.read(cx).page_select.clone());
    cx.update(|window, cx| {
        mount_dropdown(&view, window, cx);
        assert_eq!(window.find("trigger").value(), Some("3"));
        open_page_select(window, cx);
        assert!(select.read(cx).is_open());
        window.click(("pagination-page", 7usize), cx);
    });
    cx.run_until_parked();
    assert_eq!(view.read_with(cx, |v, _| v.events.clone()), vec![7]);
    assert_eq!(state.read_with(cx, |s, _| s.page()), 3);
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert_eq!(window.find("trigger").value(), Some("3"));
        assert!(select.read(cx).focus_handle().is_focused(window));
        view.update(cx, |v, _| v.accept = true);
        window.press("space", cx);
        for _ in 0..3 {
            window.render_frame(cx);
        }
        window.press("down", cx);
        window.render_frame(cx);
        window.press("enter", cx);
    });
    cx.run_until_parked();
    assert_eq!(view.read_with(cx, |v, _| v.events.clone()), vec![7, 4]);
    assert_eq!(state.read_with(cx, |s, _| s.page()), 4);
    assert_eq!(
        state
            .read_with(cx, |s, cx| s.input.read(cx).value(cx))
            .as_ref(),
        "4"
    );
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert_eq!(window.find("trigger").value(), Some("4"));
        window.press("enter", cx);
        window.render_frame(cx);
        window.press("enter", cx); // Same selected page closes without a proposal.
    });
    cx.run_until_parked();
    assert_eq!(view.read_with(cx, |v, _| v.events.len()), 2);
}

#[gpui_kit::test]
fn dropdown_and_page_size_activation_cannot_override_a_newer_owner(cx: &mut TestAppContext) {
    let (view, cx) = host(cx, 3, PaginationTotal::Known(95), true);
    let state = cx.read(|cx| view.read(cx).state.clone());
    cx.update(|window, cx| {
        mount_dropdown(&view, window, cx);
        open_page_select(window, cx);
        window.click(("pagination-page", 7usize), cx);
        state.update(cx, |s, cx| s.set_page(2, window, cx));
    });
    cx.run_until_parked();
    assert_eq!(state.read_with(cx, |s, _| s.page()), 2);
    assert!(view.read_with(cx, |v, _| v.events.is_empty()));
    cx.update(|window, cx| {
        view.update(cx, |v, cx| {
            v.dropdown = false;
            v.show_size = true;
            cx.notify();
        });
        for _ in 0..3 {
            window.render_frame(cx);
        }
        open_page_select(window, cx);
        window.click(("pagination-size", 25usize), cx);
        state.update(cx, |s, cx| s.set_per_page(50, window, cx));
    });
    cx.run_until_parked();
    assert_eq!(state.read_with(cx, |s, _| s.per_page()), 50);
    assert!(view.read_with(cx, |v, _| v.sizes.is_empty()));
    // An option-list replacement also invalidates the pending old proposal.
    cx.update(|window, cx| {
        window.render_frame(cx);
        open_page_select(window, cx);
        window.click(("pagination-size", 25usize), cx);
        state.update(cx, |s, cx| s.set_page_size_options(vec![50, 100], cx));
    });
    cx.run_until_parked();
    assert!(view.read_with(cx, |v, _| v.sizes.is_empty()));
}

#[gpui_kit::test]
fn dropdown_allocates_only_full_known_pages_and_reconciles_open_owner_changes(
    cx: &mut TestAppContext,
) {
    let (view, cx) = host(cx, 1, PaginationTotal::Known(usize::MAX), true);
    let state = cx.read(|cx| view.read(cx).state.clone());
    cx.update(|window, cx| {
        assert_eq!(state.read(cx).page_options, None);
        // Dropdown requested on Simple must not enumerate usize::MAX pages.
        view.update(cx, |v, cx| {
            v.simple = true;
            v.dropdown = true;
            cx.notify();
        });
        window.render_frame(cx);
        assert_eq!(state.read(cx).page_options, None);
        state.update(cx, |s, cx| {
            s.set_total(
                PaginationTotal::Unknown {
                    has_next_page: true,
                },
                window,
                cx,
            )
        });
        view.update(cx, |v, cx| {
            v.simple = false;
            cx.notify();
        });
        window.render_frame(cx);
        assert_eq!(state.read(cx).page_options, None);
        state.update(cx, |s, cx| {
            s.set_total(PaginationTotal::Known(95), window, cx)
        });
        for _ in 0..3 {
            window.render_frame(cx);
        }
        assert_eq!(state.read(cx).page_options, Some(10));
        open_page_select(window, cx);
        state.update(cx, |s, cx| {
            s.set_per_page(25, window, cx);
            s.set_page(4, window, cx);
        });
        for _ in 0..3 {
            window.render_frame(cx);
        }
        assert_eq!(state.read(cx).page_options, Some(4));
        let options: Vec<_> = base::test_support::snapshots(window)
            .into_iter()
            .filter(|n| n.role() == Some(Role::ListBoxOption))
            .collect();
        assert_eq!(options.len(), 4);
        assert_eq!(window.find("trigger").value(), Some("4"));
        window.press("escape", cx);
        window.render_frame(cx);
        view.update(cx, |v, cx| {
            v.dropdown = false;
            cx.notify();
        });
        window.render_frame(cx);
        assert_eq!(state.read(cx).page_options, None);
        state.update(cx, |s, cx| {
            s.set_total(PaginationTotal::Known(usize::MAX), window, cx)
        });
        window.render_frame(cx);
        assert_eq!(state.read(cx).page_options, None);
    });
    cx.run_until_parked();
    assert!(view.read_with(cx, |v, _| v.events.is_empty()));
}

#[gpui_kit::test]
fn dropdown_focus_mode_removal_availability_and_unmount_cleanup(cx: &mut TestAppContext) {
    let (view, cx) = host(cx, 3, PaginationTotal::Known(95), true);
    let state = cx.read(|cx| view.read(cx).state.clone());
    let select = cx.read(|cx| state.read(cx).page_select.clone());
    cx.update(|window, cx| {
        mount_dropdown(&view, window, cx);
        open_page_select(window, cx);
        state.update(cx, |s, cx| s.set_disabled(true, window, cx));
        window.render_frame(cx);
        assert!(!select.read(cx).is_open());
        window.click("hit", cx);
        window.press("space", cx);
        window.press("enter", cx);
        assert!(!select.read(cx).is_open());
        state.update(cx, |s, cx| s.set_disabled(false, window, cx));
        window.render_frame(cx);
        open_page_select(window, cx);
        view.update(cx, |v, cx| {
            v.simple = true;
            cx.notify();
        });
        window.render_frame(cx);
        assert!(!select.read(cx).is_open());
        assert!(state.read(cx).focuses[1].is_focused(window));
        window.press("tab", cx);
        window.render_frame(cx);
        assert!(state.read(cx).focuses[2].is_focused(window));
        window.press("tab", cx);
        window.render_frame(cx);
        assert_eq!(window.find("after").focused(), Some(true));
        view.update(cx, |v, cx| {
            v.simple = false;
            cx.notify();
        });
        for _ in 0..3 {
            window.render_frame(cx);
        }
        open_page_select(window, cx);
        view.update(cx, |v, cx| {
            v.mounted = false;
            cx.notify();
        });
        window.render_frame(cx);
    });
    cx.run_until_parked();
    assert!(!select.read_with(cx, |s, _| s.is_open()));
    cx.update(|window, cx| {
        assert!(!select.read(cx).focus_handle().is_focused(window));
        select.update(cx, |s, cx| s.set_open(true, window, cx));
        assert!(!select.read(cx).is_open());
        view.update(cx, |v, cx| {
            v.mounted = true;
            cx.notify();
        });
        for _ in 0..3 {
            window.render_frame(cx);
        }
        open_page_select(window, cx);
        assert!(select.read(cx).is_open());
        window.press("escape", cx);
        window.render_frame(cx);
        assert!(select.read(cx).focus_handle().is_focused(window));
    });
    cx.run_until_parked();
    assert!(view.read_with(cx, |v, _| v.events.is_empty()));
}

#[gpui_kit::test]
fn dropdown_source_joins_and_disabled_border_paint_in_both_themes(cx: &mut TestAppContext) {
    let (view, cx) = host(cx, 1, PaginationTotal::Known(95), true);
    cx.update(|window, cx| {
        mount_dropdown(&view, window, cx);
        for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
            crate::set_appearance(appearance, cx);
            for _ in 0..3 {
                window.render_frame(cx);
            }
            let first = window.find("pagination-first").bounds();
            let prev = window.find("pagination-previous").bounds();
            let trigger = window.find("trigger").bounds();
            let next = window.find("pagination-next").bounds();
            let last = window.find("pagination-last").bounds();
            assert_eq!(first.size, gpui_kit::size(px(42.), px(36.)));
            for (a, b, overlap) in [
                (first, prev, 1.),
                (prev, trigger, 0.),
                (trigger, next, 1.),
                (next, last, 1.),
            ] {
                assert_eq!(a.right() - b.left(), px(overlap));
                assert_eq!(a.center().y, b.center().y);
            }
            assert!(last.right() <= px(360.));
            let t = theme(cx);
            assert!(
                window
                    .painted_quads()
                    .iter()
                    .any(|q| q.bounds == first.scale(window.scale_factor())
                        && q.border_color == t.colors.line),
                "disabled joined border must remain full-opacity source line"
            );
            assert!(
                window.painted_quads().iter().any(|q| q.bounds
                    == trigger.dilate(px(1.)).scale(window.scale_factor())
                    && q.border_color == t.colors.hairline),
                "flat middle Select ring must paint source hairline"
            );
        }
    });
}
