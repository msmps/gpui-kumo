use super::*;
use gpui_kit::{
    AppContext, Entity, Modifiers, TestAppContext, VisualTestContext, size, test::TestWindowExt,
};
struct Harness {
    state: Entity<SelectState<u32>>,
    proposals: Vec<SelectValue<u32>>,
    _subscription: Subscription,
    loading: bool,
    size: Size,
    offset: f32,
    after: FocusHandle,
    description: Option<SharedString>,
    error: Option<(SharedString, bool)>,
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .w(px(230.))
            .pl(px(16.))
            .pt(px(self.offset))
            .flex()
            .flex_col()
            .gap(px(8.))
            .child(
                Select::new("select", &self.state)
                    .loading(self.loading)
                    .size(self.size)
                    .when_some(self.description.clone(), |v, d| v.description(d))
                    .when_some(self.error.clone(), |v, (e, show)| v.error(e, show)),
            )
            .child(crate::Button::new("after", "After").track_focus(&self.after))
    }
}
fn options() -> Vec<SelectOption<u32>> {
    vec![
        SelectOption::new("one", 1, "Apple"),
        SelectOption::new("two", 2, "Banana").disabled(true),
        SelectOption::new("three", 3, "Cherry café 🦀"),
        SelectOption::new("four", 4, "Date"),
    ]
}
fn harness(cx: &mut TestAppContext, multiple: bool) -> (Entity<Harness>, &mut VisualTestContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|_, cx| {
        let state = cx.new(|cx| {
            SelectState::new(
                "Fruit",
                if multiple {
                    SelectValue::Multiple(vec![])
                } else {
                    SelectValue::Single(Some(1))
                },
                options(),
                cx,
            )
        });
        let subscription = cx.subscribe(&state, |v: &mut Harness, _, e: &SelectEvent<u32>, _| {
            v.proposals.push(e.value.clone())
        });
        Harness {
            state,
            proposals: vec![],
            _subscription: subscription,
            loading: false,
            size: Size::Base,
            offset: 0.,
            after: cx.focus_handle(),
            description: None,
            error: None,
        }
    });
    cx.update(|window, _| window.activate_window());
    cx.run_until_parked();
    cx.simulate_resize(size(px(600.), px(480.)));
    cx.update(|window, cx| {
        let focus = view.read(cx).state.read(cx).trigger.clone();
        focus.focus(window, cx);
        window.render_frame(cx);
        window.render_frame(cx);
    });
    cx.run_until_parked();
    (view, cx)
}
#[gpui_kit::test]
fn select_keyboard_skip_confirmation_focus_and_rejection(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx, false);
    let state = view.read_with(cx, |v, _| v.state.clone());
    cx.update(|window, cx| {
        window.press("enter", cx);
        window.render_frame(cx);
        window.render_frame(cx);
        assert!(state.read(cx).open);
        assert!(state.read(cx).content.is_focused(window));
        window.press("down", cx);
        window.render_frame(cx);
        assert_eq!(state.read(cx).highlighted, Some("three".into()));
        window.press("enter", cx);
        window.render_frame(cx);
        assert_eq!(state.read(cx).value, SelectValue::Single(Some(3)));
        assert!(!state.read(cx).open);
        assert!(state.read(cx).trigger.is_focused(window));
    });
    cx.run_until_parked();
    assert_eq!(
        view.read_with(cx, |v, _| v.proposals.clone()),
        vec![SelectValue::Single(Some(3))]
    );
    state.update(cx, |s, cx| s.set_controlled(true, cx));
    cx.update(|window, cx| {
        window.press("space", cx);
        window.render_frame(cx);
        window.press("end", cx);
        window.render_frame(cx);
        window.press("enter", cx);
        window.render_frame(cx);
        assert_eq!(state.read(cx).value, SelectValue::Single(Some(3)));
    });
    cx.run_until_parked();
    assert_eq!(
        view.read_with(cx, |v, _| v.proposals.clone()),
        vec![SelectValue::Single(Some(3)), SelectValue::Single(Some(4))]
    );
    state.update(cx, |s, cx| s.set_value(SelectValue::Single(Some(4)), cx));
    cx.run_until_parked();
    assert_eq!(view.read_with(cx, |v, _| v.proposals.len()), 2);
}
#[gpui_kit::test]
fn select_multiple_pointer_toggle_readonly_and_disable(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx, true);
    let state = view.read_with(cx, |v, _| v.state.clone());
    cx.update(|window, cx| {
        window.press("enter", cx);
        window.render_frame(cx);
        window.render_frame(cx);
    });
    let cherry = cx.update(|window, cx| {
        window.render_frame(cx);
        window.find("three").bounds().center()
    });
    cx.simulate_click(cherry, Modifiers::default());
    cx.run_until_parked();
    assert_eq!(
        state.read_with(cx, |s, _| s.value.clone()),
        SelectValue::Multiple(vec![3])
    );
    assert!(state.read_with(cx, |s, _| s.open));
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.press("space", cx);
        window.render_frame(cx);
    });
    cx.run_until_parked();
    assert_eq!(
        state.read_with(cx, |s, _| s.value.clone()),
        SelectValue::Multiple(vec![])
    );
    assert_eq!(view.read_with(cx, |v, _| v.proposals.len()), 2);
    state.update(cx, |s, cx| s.set_read_only(true, cx));
    cx.simulate_click(cherry, Modifiers::default());
    cx.update(|window, cx| {
        window.press("enter", cx);
        window.render_frame(cx);
    });
    cx.run_until_parked();
    assert_eq!(view.read_with(cx, |v, _| v.proposals.len()), 2);
    cx.update(|window, cx| state.update(cx, |s, cx| s.set_disabled(true, window, cx)));
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.press("enter", cx);
        window.press("space", cx);
        window.render_frame(cx);
        assert!(!state.read(cx).open);
    });
    cx.run_until_parked();
    assert_eq!(view.read_with(cx, |v, _| v.proposals.len()), 2);
}
#[gpui_kit::test]
fn select_reorder_preserves_identity_and_empty_navigation(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx, false);
    let state = view.read_with(cx, |v, _| v.state.clone());
    cx.update(|window, cx| {
        window.press("enter", cx);
        window.render_frame(cx);
        window.press("down", cx);
        window.render_frame(cx);
    });
    state.update(cx, |s, cx| {
        let mut next = options();
        next.reverse();
        s.set_options(next, cx);
        assert_eq!(s.highlighted, Some("three".into()));
    });
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.press("enter", cx);
        window.render_frame(cx);
        assert_eq!(state.read(cx).value, SelectValue::Single(Some(3)));
    });
    state.update(cx, |s, cx| s.set_options(vec![], cx));
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.press("enter", cx);
        window.render_frame(cx);
        window.press("down", cx);
        window.press("home", cx);
        window.press("c", cx);
        window.press("enter", cx);
        window.render_frame(cx);
        assert!(state.read(cx).open);
        window.press("escape", cx);
        window.render_frame(cx);
        assert!(!state.read(cx).open);
        assert!(state.read(cx).trigger.is_focused(window));
    });
}

#[gpui_kit::test]
fn select_closed_typeahead_accumulates_and_cycles_without_opening(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx, false);
    let state = view.read_with(cx, |v, _| v.state.clone());
    state.update(cx, |s, cx| {
        s.set_options(
            vec![
                SelectOption::new("g", 1, "Gala"),
                SelectOption::new("gr", 3, "Grape"),
                SelectOption::new("grey", 4, "Grey").disabled(true),
                SelectOption::new("r", 5, "Raisin"),
            ],
            cx,
        )
    });
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.press("g", cx);
        window.render_frame(cx);
        window.press("r", cx);
        window.render_frame(cx);
        assert_eq!(state.read(cx).value, SelectValue::Single(Some(3)));
        assert!(!state.read(cx).open);
        assert!(state.read(cx).trigger.is_focused(window));
    });
    cx.executor().advance_clock(Duration::from_millis(501));
    cx.update(|window, cx| {
        window.press("g", cx);
        window.render_frame(cx);
        assert_eq!(state.read(cx).value, SelectValue::Single(Some(1)));
        window.press("g", cx);
        window.render_frame(cx);
        assert_eq!(state.read(cx).value, SelectValue::Single(Some(3)));
    });
    cx.run_until_parked();
    assert_eq!(view.read_with(cx, |v, _| v.proposals.len()), 3);
}

#[gpui_kit::test]
fn select_disabled_pointer_preserves_outside_focus_and_loading_closes(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx, false);
    let state = view.read_with(cx, |v, _| v.state.clone());
    let hit = cx.update(|window, cx| {
        window.render_frame(cx);
        window.find("hit").bounds().center()
    });
    cx.update(|window, cx| state.update(cx, |s, cx| s.set_disabled(true, window, cx)));
    let after = cx.update(|window, cx| {
        window.render_frame(cx);
        window.find("after").bounds().center()
    });
    cx.simulate_click(after, Modifiers::default());
    let focus = cx.update(|window, cx| window.focused(cx).unwrap());
    cx.simulate_click(hit, Modifiers::default());
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert!(focus.is_focused(window));
        assert!(!state.read(cx).open);
        state.update(cx, |s, cx| s.set_disabled(false, window, cx));
    });
    cx.simulate_click(hit, Modifiers::default());
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert!(state.read(cx).open);
    });
    view.update(cx, |v, cx| {
        v.loading = true;
        cx.notify();
    });
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.render_frame(cx);
        assert!(!state.read(cx).open);
        window.press("enter", cx);
        window.press("space", cx);
        window.render_frame(cx);
        assert!(!state.read(cx).open);
    });
    cx.run_until_parked();
    assert_eq!(view.read_with(cx, |v, _| v.proposals.len()), 0);
}

#[gpui_kit::test]
fn select_both_theme_sizes_popup_alignment_flip_and_tab_exit(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx, false);
    let state = view.read_with(cx, |v, _| v.state.clone());
    cx.update(|window, cx| {
        for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
            crate::set_appearance(appearance, cx);
            for (size, height) in [
                (Size::Xs, 20.),
                (Size::Sm, 26.),
                (Size::Base, 36.),
                (Size::Lg, 40.),
            ] {
                view.update(cx, |v, cx| {
                    v.size = size;
                    cx.notify();
                });
                window.render_frame(cx);
                window.render_frame(cx);
                let trigger = window.find("trigger");
                assert_eq!(trigger.role(), Some(Role::ComboBox));
                assert_eq!(trigger.label(), Some("Fruit"));
                assert_eq!(trigger.value(), Some("Apple"));
                assert_eq!(trigger.bounds().size.height, px(height));
                window.press("enter", cx);
                window.render_frame(cx);
                window.render_frame(cx);
                let trigger = window.find("trigger").bounds();
                let surface = window.find("surface").bounds();
                assert_eq!(surface.left(), trigger.left());
                assert_eq!(surface.top(), trigger.bottom() + px(4.));
                assert_eq!(surface.size.width, trigger.size.width);
                assert_eq!(window.find("list").role(), Some(Role::ListBox));
                assert_eq!(window.find("one").selected(), Some(true));
                assert_eq!(window.find("three").selected(), Some(false));
                window.press("escape", cx);
                window.render_frame(cx);
                assert!(state.read(cx).trigger.is_focused(window));
            }
        }
        window.press("enter", cx);
        window.render_frame(cx);
        window.press("tab", cx);
        window.render_frame(cx);
        assert!(!state.read(cx).open);
        assert!(!state.read(cx).trigger.is_focused(window));
    });
    view.update(cx, |v, cx| {
        v.offset = 110.;
        cx.notify();
    });
    cx.simulate_resize(size(px(600.), px(160.)));
    cx.update(|window, cx| {
        let focus = state.read(cx).trigger.clone();
        focus.focus(window, cx);
        window.render_frame(cx);
        window.render_frame(cx);
        window.press("enter", cx);
        window.render_frame(cx);
        window.render_frame(cx);
        let trigger = window.find("trigger").bounds();
        let popup = window.find("surface").bounds();
        assert_eq!(popup.bottom(), trigger.top() - px(4.));
        assert!(popup.top() >= px(8.));
    });
}

#[gpui_kit::test]
fn select_open_word_typeahead_space_and_programmatic_focus_dismiss(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx, false);
    let state = view.read_with(cx, |v, _| v.state.clone());
    state.update(cx, |s, cx| {
        s.set_options(
            vec![
                SelectOption::new("ny", 1, "New York"),
                SelectOption::new("nj", 3, "New Jersey"),
            ],
            cx,
        )
    });
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.press("enter", cx);
        window.render_frame(cx);
        for key in ["n", "e", "w", "space", "j"] {
            window.press(key, cx);
            window.render_frame(cx);
        }
        assert!(state.read(cx).open);
        assert_eq!(state.read(cx).highlighted, Some("nj".into()));
        assert_eq!(state.read(cx).value, SelectValue::Single(Some(1)));
    });
    cx.executor().advance_clock(Duration::from_millis(501));
    cx.update(|window, cx| {
        window.press("space", cx);
        window.render_frame(cx);
        assert!(!state.read(cx).open);
        assert_eq!(state.read(cx).value, SelectValue::Single(Some(3)));
    });
    cx.run_until_parked();
    assert_eq!(view.read_with(cx, |v, _| v.proposals.len()), 1);
    cx.update(|window, cx| {
        window.press("enter", cx);
        window.render_frame(cx);
        window.render_frame(cx);
    });
    cx.update(|window, cx| {
        let after = view.read(cx).after.clone();
        after.focus(window, cx);
    });
    cx.run_until_parked();
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert!(!state.read(cx).open);
        assert!(state.read(cx).deferred.is_none());
        assert!(!state.read(cx).trigger.is_focused(window));
    });
}

#[gpui_kit::test]
fn select_custom_equality_rich_name_and_controlled_acceptance(cx: &mut TestAppContext) {
    #[derive(Clone, Debug, PartialEq)]
    struct Region {
        id: u32,
        version: u32,
    }
    struct Owner {
        state: Entity<SelectState<Region>>,
        changes: usize,
        _subscription: Subscription,
    }
    impl Render for Owner {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div().w(px(240.)).child(Select::new("select", &self.state))
        }
    }
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|_, cx| {
        let state = cx.new(|cx| {
            let mut s = SelectState::new(
                "Region",
                SelectValue::Single(Some(Region { id: 1, version: 7 })),
                vec![
                    SelectOption::new("one", Region { id: 1, version: 8 }, "Primary café 🦀")
                        .content(|_, _| {
                            div()
                                .font_weight(FontWeight::BOLD)
                                .child("Primary café 🦀")
                                .into_any_element()
                        }),
                    SelectOption::new("two", Region { id: 2, version: 8 }, "Secondary"),
                ],
                cx,
            );
            s.set_comparator(|a, b| a.id == b.id, cx);
            s.set_controlled(true, cx);
            s
        });
        let subscription = cx.subscribe(
            &state,
            |v: &mut Owner, state, e: &SelectEvent<Region>, cx| {
                v.changes += 1;
                state.update(cx, |s, cx| s.set_value(e.value.clone(), cx));
            },
        );
        Owner {
            state,
            changes: 0,
            _subscription: subscription,
        }
    });
    let state = view.read_with(cx, |v, _| v.state.clone());
    cx.update(|window, _| window.activate_window());
    cx.run_until_parked();
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.render_frame(cx);
        let focus = state.read(cx).trigger.clone();
        focus.focus(window, cx);
        assert_eq!(window.find("trigger").value(), Some("Primary café 🦀"));
        window.press("enter", cx);
        window.render_frame(cx);
        assert_eq!(window.find("one").label(), Some("Primary café 🦀"));
        assert_eq!(window.find("one").selected(), Some(true));
        window.press("enter", cx);
        window.render_frame(cx);
    });
    cx.run_until_parked();
    assert_eq!(view.read_with(cx, |v, _| v.changes), 0);
    assert_eq!(
        state.read_with(cx, |s, _| s.value.clone()),
        SelectValue::Single(Some(Region { id: 1, version: 7 }))
    );
    cx.update(|window, cx| {
        window.press("enter", cx);
        window.render_frame(cx);
        window.press("down", cx);
        window.render_frame(cx);
        window.press("enter", cx);
        window.render_frame(cx);
    });
    cx.run_until_parked();
    assert_eq!(view.read_with(cx, |v, _| v.changes), 1);
    assert_eq!(
        state.read_with(cx, |s, _| s.value.clone()),
        SelectValue::Single(Some(Region { id: 2, version: 8 }))
    );
}

#[gpui_kit::test]
fn select_rich_option_unmount_releases_open_entity(cx: &mut TestAppContext) {
    struct Host {
        child: Option<Entity<SelectState<u32>>>,
    }
    impl Render for Host {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .w(px(240.))
                .children(self.child.as_ref().map(|s| Select::new("select", s)))
        }
    }
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|_, cx| {
        let owner = cx.entity().downgrade();
        Host {
            child: Some(cx.new(|cx| {
                SelectState::new(
                    "Region",
                    SelectValue::Single(Some(1)),
                    vec![SelectOption::new("one", 1, "Primary").content(move |_, _| {
                        assert!(owner.upgrade().is_some());
                        div()
                            .font_weight(FontWeight::BOLD)
                            .child("Primary")
                            .into_any_element()
                    })],
                    cx,
                )
            })),
        }
    });
    let weak = view.read_with(cx, |v, _| v.child.as_ref().unwrap().downgrade());
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.render_frame(cx);
        weak.update(cx, |s, cx| s.set_open(true, window, cx))
            .unwrap();
        window.render_frame(cx);
        window.render_frame(cx);
    });
    view.update(cx, |v, cx| {
        v.child = None;
        cx.notify();
    });
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.render_frame(cx);
    });
    cx.run_until_parked();
    assert!(
        weak.upgrade().is_none(),
        "Rich option factories and open overlay listeners must not retain an unmounted state"
    );
}

#[gpui_kit::test]
fn select_actual_base_node_enrichment_and_message_precedence(cx: &mut TestAppContext) {
    use gpui_kit::{Element, accesskit};
    let (view, cx) = harness(cx, false);
    let state = view.read_with(cx, |v, _| v.state.clone());
    for (read_only, disabled, error, expected_description) in [
        (false, false, None, Some("Choose a region")),
        (
            true,
            false,
            Some(("Region required", true)),
            Some("Region required"),
        ),
        (false, true, Some(("Region required", false)), None),
        (false, false, None, Some("Choose a region")),
    ] {
        view.update(cx, |v, cx| {
            v.description = Some("Choose a region".into());
            v.error = error.map(|(e, show)| (e.into(), show));
            cx.notify();
        });
        cx.update(|window, cx| {
            state.update(cx, |s, cx| {
                s.set_read_only(read_only, cx);
                s.set_disabled(disabled, window, cx);
            });
            window.render_frame(cx);
            let node = state.update(cx, |s, cx| {
                let element = s.trigger_element(window, cx);
                assert_eq!(element.a11y_role(), Some(Role::ComboBox));
                let mut node = accesskit::Node::new(Role::ComboBox);
                element.write_a11y_info(&mut node);
                node
            });
            assert_eq!(node.supports_action(accesskit::Action::Click), !disabled);
            assert_eq!(node.supports_action(accesskit::Action::Focus), !disabled);
            assert_eq!(node.label(), Some("Fruit"));
            assert_eq!(node.value(), Some("Apple"));
            assert_eq!(node.is_expanded(), Some(false));
            assert_eq!(node.is_read_only(), read_only);
            assert_eq!(node.is_disabled(), disabled);
            assert_eq!(node.invalid(), error.map(|_| accesskit::Invalid::True));
            assert_eq!(node.description(), expected_description);
        });
    }
    view.update(cx, |v, cx| {
        v.loading = true;
        cx.notify();
    });
    cx.update(|window, cx| {
        window.render_frame(cx);
        let node = state.update(cx, |s, cx| {
            let element = s.trigger_element(window, cx);
            let mut node = accesskit::Node::new(Role::ComboBox);
            element.write_a11y_info(&mut node);
            node
        });
        assert!(node.is_disabled());
    });
}
