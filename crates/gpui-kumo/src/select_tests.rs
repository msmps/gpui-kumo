use super::*;
use gpui_kit::{
    AppContext, Entity, Modifiers, TestAppContext, VisualTestContext, size, test::TestWindowExt,
};
gpui_kit::actions!(select_traversal_test, [FocusNext, FocusPrevious]);

struct TraversalHost {
    state: Entity<SelectState<u32>>,
    before: FocusHandle,
    after: FocusHandle,
}
impl Render for TraversalHost {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        // Match the gallery: global action bindings and no enclosing tab group.
        div()
            .on_action(|_: &FocusNext, window, cx| window.focus_next(cx))
            .on_action(|_: &FocusPrevious, window, cx| window.focus_prev(cx))
            .flex()
            .flex_col()
            .w(px(230.))
            .child(crate::Button::new("before", "Before").track_focus(&self.before))
            .child(Select::new("select", &self.state))
            .child(crate::Button::new("after", "After").track_focus(&self.after))
    }
}

#[gpui_kit::test]
fn open_select_tab_exits_once_with_consuming_host_bindings(cx: &mut TestAppContext) {
    cx.update(|cx| {
        crate::init(cx);
        cx.bind_keys([
            KeyBinding::new("tab", FocusNext, None),
            KeyBinding::new("shift-tab", FocusPrevious, None),
        ]);
    });
    let (view, cx) = cx.add_window_view(|_, cx| TraversalHost {
        state: cx.new(|cx| SelectState::new("Fruit", SelectValue::Single(Some(1)), options(), cx)),
        before: cx.focus_handle(),
        after: cx.focus_handle(),
    });
    let state = view.read_with(cx, |v, _| v.state.clone());
    cx.update(|window, cx| {
        for key in ["tab", "shift-tab"] {
            window.render_frame(cx);
            state.update(cx, |s, cx| s.set_open(true, window, cx));
            window.render_frame(cx);
            assert!(state.read(cx).content.is_focused(window));
            window.press("down", cx);
            assert_eq!(state.read(cx).highlighted, Some("three".into()));
            window.press(key, cx);
            window.render_frame(cx);
            assert!(!state.read(cx).is_open(), "{key} must dismiss in one press");
            let expected = view.read(cx);
            assert!(if key == "tab" {
                expected.after.is_focused(window)
            } else {
                expected.before.is_focused(window)
            });
            assert_eq!(state.read(cx).value(), &SelectValue::Single(Some(1)));
        }
        state.update(cx, |s, cx| s.set_open(true, window, cx));
        window.press("ctrl-tab", cx);
        assert!(state.read(cx).is_open(), "modified Tab belongs to the host");
        window.press("escape", cx);
    });
}

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
    placement: Placement,
    align: Align,
    gap: Pixels,
    left: f32,
}

#[gpui_kit::test]
fn open_hovered_trigger_paints_observed_source_tint(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx, false);
    let state = view.read_with(cx, |v, _| v.state.clone());
    cx.update(|window, cx| {
        for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
            crate::set_appearance(appearance, cx);
            window.hover("after", cx);
            window.render_frame(cx);
            let control = theme(cx).colors.control;
            let tint = theme(cx).colors.tint;
            let paints = |window: &mut Window, color| {
                let bounds = window.find("trigger").bounds().scale(window.scale_factor());
                window
                    .painted_quads()
                    .iter()
                    .any(|q| q.bounds == bounds && q.background.as_solid() == Some(color))
            };
            assert!(paints(window, control));
            window.hover("hit", cx);
            window.render_frame(cx);
            assert!(paints(window, tint));
            window.click("hit", cx);
            window.render_frame(cx);
            assert!(state.read(cx).is_open());
            // Base UI authors data-popup-open, so Kumo's data-state=open CSS
            // override does not replace the trigger's ordinary hover paint.
            assert!(
                paints(window, tint),
                "open hovered trigger lost source tint"
            );
            window.hover("one", cx);
            window.render_frame(cx);
            assert!(paints(window, control));
            window.press("escape", cx);
        }
    });
}

impl Render for Harness {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().child(
            div()
                .w(px(230.))
                .ml(px(self.left))
                .pl(px(16.))
                .pt(px(self.offset))
                .flex()
                .flex_col()
                .gap(px(8.))
                .child(
                    Select::new("select", &self.state)
                        .loading(self.loading)
                        .size(self.size)
                        .placement(self.placement)
                        .align(self.align)
                        .offset(self.gap)
                        .when_some(self.description.clone(), |v, d| v.description(d))
                        .when_some(self.error.clone(), |v, (e, show)| v.error(e, show)),
                )
                .child(crate::Button::new("after", "After").track_focus(&self.after)),
        )
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
fn grouped_options() -> Vec<SelectPart<u32>> {
    vec![
        SelectGroup::new("early", options())
            .label("Early fruit")
            .into(),
        SelectPart::separator("divider"),
        SelectGroup::new(
            "later",
            (5..35)
                .map(|i| SelectOption::new(format!("fruit-{i}"), i, format!("Fruit {i}")))
                .collect(),
        )
        .label("Later fruit")
        .into(),
    ]
}
#[gpui_kit::test]
fn select_grouped_keyboard_scroll_semantics_and_reorder(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx, false);
    let state = view.read_with(cx, |v, _| v.state.clone());
    state.update(cx, |s, cx| s.set_parts(grouped_options(), cx));
    cx.simulate_resize(size(px(270.), px(260.)));
    cx.update(|window, cx| {
        for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
            crate::set_appearance(appearance, cx);
            window.press("enter", cx);
            for _ in 0..3 {
                window.render_frame(cx);
            }
            assert_eq!(window.find("early").role(), Some(Role::Group));
            assert_eq!(window.find("early").label(), Some("Early fruit"));
            assert_eq!(window.find("later").label(), Some("Later fruit"));
            assert_eq!(window.find("divider").role(), Some(Role::Splitter));
            assert_eq!(window.find("divider").bounds().size.height, px(1.));
            window.press("down", cx);
            window.render_frame(cx);
            assert_eq!(state.read(cx).highlighted, Some("three".into()));
            window.press("end", cx);
            for _ in 0..3 {
                window.render_frame(cx);
            }
            assert_eq!(state.read(cx).highlighted, Some("fruit-34".into()));
            let list = window.find("list").bounds();
            let last = window.find("fruit-34").bounds();
            assert!(last.top() >= list.top());
            assert!(last.bottom() <= list.bottom());
            // Nearest reveal targets the nested row rather than aligning its whole group.
            assert!(state.read(cx).scroll.offset().y < px(0.));
            window.press("home", cx);
            for _ in 0..3 {
                window.render_frame(cx);
            }
            let first = window.find("one").bounds();
            assert!(first.top() >= list.top());
            assert!(first.bottom() <= list.bottom());
            window.press("escape", cx);
            window.render_frame(cx);
        }
        window.press("enter", cx);
        window.render_frame(cx);
        window.press("end", cx);
        for _ in 0..3 {
            window.render_frame(cx);
        }
        state.update(cx, |s, cx| {
            let mut parts = grouped_options();
            parts.reverse();
            s.set_parts(parts, cx);
        });
        for _ in 0..3 {
            window.render_frame(cx);
        }
        assert_eq!(state.read(cx).highlighted, Some("fruit-34".into()));
        window.press("enter", cx);
        window.render_frame(cx);
        assert_eq!(state.read(cx).value(), &SelectValue::Single(Some(34)));
        assert!(state.read(cx).trigger.is_focused(window));
    });
    cx.run_until_parked();
    assert_eq!(
        view.read_with(cx, |v, _| v.proposals.clone()),
        vec![SelectValue::Single(Some(34))]
    );
}
#[gpui_kit::test]
fn select_deep_initial_selection_manual_scroll_and_oversized_group_row(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx, false);
    let state = view.read_with(cx, |v, _| v.state.clone());
    state.update(cx, |s, cx| {
        s.set_parts(grouped_options(), cx);
        s.set_value(SelectValue::Single(Some(34)), cx);
    });
    cx.simulate_resize(size(px(270.), px(260.)));
    cx.update(|window, cx| {
        window.press("enter", cx);
        for _ in 0..3 {
            window.render_frame(cx);
        }
        let viewport = window.find("list").bounds();
        let last = window.within("later").find("fruit-34").bounds();
        assert!(last.top() >= viewport.top() && last.bottom() <= viewport.bottom());
        assert_eq!(window.find("fruit-34").selected(), Some(true));
        let scroll = state.read(cx).scroll.clone();
        scroll.set_offset(Default::default());
        for _ in 0..3 {
            window.render_frame(cx);
        }
        // A settled highlight must not force a manually scrolled viewport back to its row.
        assert_eq!(scroll.offset().y, px(0.));
        window.press("escape", cx);
        state.update(cx, |s, cx| {
            s.set_parts(
                vec![
                    SelectGroup::new(
                        "tall-group",
                        vec![
                            SelectOption::new("tall", 1, "Tall decorative option").content(
                                |_, _| {
                                    div()
                                        .h(px(400.))
                                        .child("Tall decorative option")
                                        .into_any_element()
                                },
                            ),
                            SelectOption::new("short", 3, "Short option"),
                        ],
                    )
                    .label("Tall options")
                    .into(),
                ],
                cx,
            );
            s.set_value(SelectValue::Single(Some(1)), cx);
        });
        window.press("enter", cx);
        for _ in 0..3 {
            window.render_frame(cx);
        }
        let viewport = window.find("list").bounds();
        let tall = window.find("tall").bounds();
        assert!(tall.size.height > viewport.size.height);
        assert_eq!(tall.top(), viewport.top() + px(8.));
        let settled = scroll.offset();
        for _ in 0..3 {
            window.render_frame(cx);
        }
        assert_eq!(scroll.offset(), settled);
        window.press("end", cx);
        for _ in 0..3 {
            window.render_frame(cx);
        }
        let short = window.find("short").bounds();
        assert!(short.top() >= viewport.top() && short.bottom() <= viewport.bottom());
        window.press("enter", cx);
        window.render_frame(cx);
        assert_eq!(state.read(cx).value(), &SelectValue::Single(Some(3)));
    });
}
#[gpui_kit::test]
fn select_custom_value_readable_text_null_fallback_and_current_value(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx, false);
    let state = view.read_with(cx, |v, _| v.state.clone());
    let calls = Rc::new(Cell::new(0));
    let observed = calls.clone();
    state.update(cx, |s, cx| {
        s.set_value_content(
            move |value, _, _| {
                observed.set(observed.get() + 1);
                let SelectValue::Single(Some(value)) = value else {
                    panic!("null must skip factory")
                };
                Some(SelectValueContent::new(
                    format!("Complete formatted fruit {value}"),
                    div()
                        .id("formatted")
                        .test_support()
                        .child(format!("Fruit {value}")),
                ))
            },
            cx,
        )
    });
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert_eq!(
            window.find("trigger").value(),
            Some("Complete formatted fruit 1")
        );
        assert!(window.find("formatted").bounds().size.width > px(0.));
        state.update(cx, |s, cx| s.set_value(SelectValue::Single(Some(3)), cx));
        window.render_frame(cx);
        assert_eq!(
            window.find("trigger").value(),
            Some("Complete formatted fruit 3")
        );
        assert!(window.find("formatted").bounds().size.width > px(0.));
        state.update(cx, |s, cx| s.set_value(SelectValue::Single(None), cx));
        let before = calls.get();
        for _ in 0..2 {
            window.render_frame(cx);
        }
        assert_eq!(calls.get(), before);
        assert_eq!(window.find("trigger").value(), Some(""));
        state.update(cx, |s, cx| {
            s.set_value(SelectValue::Single(Some(1)), cx);
            s.set_value_content(|_, _, _| None, cx);
        });
        window.render_frame(cx);
        // Returning no decoration does not erase the actual current selection metadata.
        assert_eq!(window.find("trigger").value(), Some("Apple"));
        state.update(cx, |s, cx| s.clear_value_content(cx));
        window.render_frame(cx);
        assert_eq!(window.find("trigger").value(), Some("Apple"));
    });
    cx.run_until_parked();
    assert!(view.read_with(cx, |v, _| v.proposals.is_empty()));
}
#[gpui_kit::test]
fn select_placement_alignment_gap_and_edge_flip(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx, false);
    let state = view.read_with(cx, |v, _| v.state.clone());
    cx.simulate_resize(size(px(1000.), px(800.)));
    view.update(cx, |v, cx| {
        v.offset = 300.;
        v.left = 250.;
        v.gap = px(12.);
        cx.notify();
    });
    cx.update(|window, cx| {
        for placement in [
            Placement::Top,
            Placement::Bottom,
            Placement::Left,
            Placement::Right,
        ] {
            for align in [Align::Start, Align::Center, Align::End] {
                view.update(cx, |v, cx| {
                    v.placement = placement;
                    v.align = align;
                    cx.notify();
                });
                window.render_frame(cx);
                window.press("enter", cx);
                for _ in 0..3 {
                    window.render_frame(cx);
                }
                let trigger = window.find("trigger").bounds();
                let popup = window.find("surface").bounds();
                match placement {
                    Placement::Top => assert_eq!(popup.bottom(), trigger.top() - px(12.)),
                    Placement::Bottom => assert_eq!(popup.top(), trigger.bottom() + px(12.)),
                    Placement::Left => assert_eq!(popup.right(), trigger.left() - px(12.)),
                    Placement::Right => assert_eq!(popup.left(), trigger.right() + px(12.)),
                }
                if matches!(placement, Placement::Left | Placement::Right) {
                    match align {
                        Align::Start => assert_eq!(popup.top(), trigger.top()),
                        Align::Center => assert_eq!(popup.center().y, trigger.center().y),
                        Align::End => assert_eq!(popup.bottom(), trigger.bottom()),
                    }
                } else {
                    assert_eq!(popup.left(), trigger.left());
                }
                window.press("escape", cx);
                window.render_frame(cx);
                assert!(state.read(cx).trigger.is_focused(window));
            }
        }
        view.update(cx, |v, cx| {
            v.left = 0.;
            v.placement = Placement::Left;
            v.align = Align::Start;
            cx.notify();
        });
        window.render_frame(cx);
        window.press("enter", cx);
        for _ in 0..3 {
            window.render_frame(cx);
        }
        assert_eq!(
            window.find("surface").bounds().left(),
            window.find("trigger").bounds().right() + px(12.)
        );
    });
}
#[gpui_kit::test]
fn select_multiple_factory_receives_empty_and_loading_skips_presentation(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx, true);
    let state = view.read_with(cx, |v, _| v.state.clone());
    let calls = Rc::new(Cell::new(0));
    let observed = calls.clone();
    state.update(cx, |s, cx| {
        s.set_value_content(
            move |value, _, _| {
                let SelectValue::Multiple(values) = value else {
                    panic!("immutable multiple mode")
                };
                observed.set(observed.get() + 1);
                Some(SelectValueContent::new(
                    format!("{} fruits selected", values.len()),
                    div().child(format!("{} fruits", values.len())),
                ))
            },
            cx,
        )
    });
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert_eq!(window.find("trigger").value(), Some("0 fruits selected"));
        window.press("enter", cx);
        window.render_frame(cx);
        window.press("space", cx);
        window.render_frame(cx);
        assert_eq!(window.find("trigger").value(), Some("1 fruits selected"));
        assert_eq!(state.read(cx).value(), &SelectValue::Multiple(vec![1]));
        view.update(cx, |v, cx| {
            v.loading = true;
            cx.notify();
        });
        let before = calls.get();
        for _ in 0..2 {
            window.render_frame(cx);
        }
        assert_eq!(calls.get(), before);
        assert!(!state.read(cx).is_open());
        assert_eq!(window.find("trigger").value(), Some("Apple"));
        view.update(cx, |v, cx| {
            v.loading = false;
            cx.notify();
        });
        window.render_frame(cx);
        assert_eq!(window.find("trigger").value(), Some("1 fruits selected"));
    });
    cx.run_until_parked();
    assert_eq!(view.read_with(cx, |v, _| v.proposals.len()), 1);
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
            placement: Placement::Bottom,
            align: Align::Start,
            gap: px(4.),
            left: 0.,
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
    let value_owner = weak.clone();
    weak.update(cx, |s, cx| {
        s.set_value_content(
            move |_, _, _| {
                assert!(value_owner.upgrade().is_some());
                Some(SelectValueContent::new("Primary", div().child("Primary")))
            },
            cx,
        )
    })
    .unwrap();
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

#[gpui_kit::test]
fn compact_numeric_popup_reserves_check_space_only_for_selected_options(cx: &mut TestAppContext) {
    struct Numeric(Entity<SelectState<u32>>);
    impl Render for Numeric {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div().w(px(40.)).child(Select::new("numeric", &self.0))
        }
    }
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|_, cx| {
        Numeric(cx.new(|cx| {
            SelectState::new(
                "Page",
                SelectValue::Single(Some(3)),
                (1..=10)
                    .map(|n| SelectOption::new(format!("number-{n}"), n, n.to_string()))
                    .collect(),
                cx,
            )
        }))
    });
    let state = view.read_with(cx, |v, _| v.0.clone());
    cx.update(|window, cx| {
        for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
            crate::set_appearance(appearance, cx);
            let mut widths = Vec::new();
            for selected in [3, 10, 3] {
                state.update(cx, |s, cx| {
                    s.set_value(SelectValue::Single(Some(selected)), cx);
                    s.set_open(true, window, cx);
                });
                window.render_frame(cx);
                window.render_frame(cx);
                let surface = window.find("surface").bounds();
                let text: SharedString = selected.to_string().into();
                let run = gpui_kit::TextRun {
                    len: text.len(),
                    font: gpui_kit::font(theme(cx).typography.font_family.clone()),
                    color: theme(cx).text.default,
                    background_color: None,
                    underline: None,
                    strikethrough: None,
                };
                let expected = window
                    .text_system()
                    .shape_line(text, px(14.), &[run], None)
                    .width
                    + px(52.);
                assert!(
                    (surface.size.width - expected).abs() <= px(1.),
                    "selected {selected}: {:?} vs {expected:?}",
                    surface.size.width
                );
                widths.push(surface.size.width);
                let row = window.find(format!("number-{selected}")).bounds();
                assert!(row.right() <= surface.right());
                assert_eq!(row.size.height, px(33.));
                state.update(cx, |s, cx| s.set_open(false, window, cx));
            }
            assert!(widths[1] > widths[0]);
            assert_eq!(widths[0], widths[2]);
        }
    });
}
