use super::*;
use gpui_kit::{AppContext, Render, TestAppContext, test::TestWindowExt};
struct Harness {
    tabs: Entity<TabsState<u32>>,
    other: Entity<TabsState<u32>>,
    events: Vec<u32>,
    _subscription: gpui_kit::Subscription,
}
fn items() -> Vec<TabItem<u32>> {
    vec![
        TabItem::new("one", 1, "Overview"),
        TabItem::new("two", 2, "Unavailable").disabled(true),
        TabItem::new("three", 3, "Settings"),
    ]
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .tab_group()
            .flex()
            .flex_col()
            .w(px(500.))
            .child(crate::Button::new("before", "Before"))
            .child(Tabs::new("tabs", "Project", &self.tabs))
            .child(crate::Button::new("after", "After"))
            .child(Tabs::new("other", "Other project", &self.other))
    }
}
fn harness(
    cx: &mut TestAppContext,
    controlled: bool,
    automatic: bool,
) -> (Entity<Harness>, &mut gpui_kit::VisualTestContext) {
    cx.update(crate::init);
    cx.add_window_view(|_, cx| {
        let tabs = cx.new(|cx| {
            TabsState::new(items(), Some(1), cx)
                .controlled(controlled)
                .activate_on_focus(automatic)
        });
        let other = cx.new(|cx| TabsState::new(items(), Some(1), cx));
        let subscription = cx.subscribe(
            &tabs,
            |this: &mut Harness, _, event: &TabsEvent<u32>, cx| {
                this.events.push(event.value);
                cx.notify();
            },
        );
        Harness {
            tabs,
            other,
            events: vec![],
            _subscription: subscription,
        }
    })
}
#[gpui_kit::test]
fn manual_tabs_separate_focus_and_selection_and_exit_once(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx, false, false);
    let tabs = cx.update(|window, cx| {
        window.render_frame(cx);
        window.click("before", cx);
        window.focus_next(cx);
        window.render_frame(cx);
        assert!(
            view.read(cx).tabs.read(cx).items[0]
                .focus
                .is_focused(window),
            "composite participates in host traversal"
        );
        window.within("tabs").click("one", cx);
        let tabs = view.read(cx).tabs.clone();
        assert!(
            tabs.read(cx).items[0].focus.is_focused(window),
            "pointer activation focuses tab"
        );
        window.press("right", cx);
        window.render_frame(cx);
        assert!(tabs.read(cx).items[2].focus.is_focused(window));
        assert_eq!(tabs.read(cx).selected(), Some(&1));
        window.within("tabs").click("two", cx);
        assert_eq!(tabs.read(cx).selected(), Some(&1));
        assert!(tabs.read(cx).items[2].focus.is_focused(window));
        assert_eq!(
            window.within("tabs").find("one").role(),
            Some(gpui_kit::Role::Tab)
        );
        assert!(view.read(cx).events.is_empty());
        window.press("enter", cx);
        window.render_frame(cx);
        assert_eq!(tabs.read(cx).selected(), Some(&3));
        assert_eq!(window.within("tabs").find("three").selected(), Some(true));
        tabs
    });
    cx.update(|window, cx| {
        assert_eq!(view.read(cx).events, vec![3]);
        window.press("space", cx);
        window.render_frame(cx);
        window.press("home", cx);
        window.render_frame(cx);
        assert!(tabs.read(cx).items[0].focus.is_focused(window));
        window.press("left", cx);
        window.render_frame(cx);
        assert!(tabs.read(cx).items[2].focus.is_focused(window));
        window.press("up", cx);
        window.render_frame(cx);
        assert!(
            tabs.read(cx).items[2].focus.is_focused(window),
            "horizontal source ignores vertical arrows"
        );
        window.focus_next(cx);
        window.render_frame(cx);
        assert_eq!(
            window.find("after").focused(),
            Some(true),
            "bounds {:?}, max {:?}, offset {:?}, controls {:?}",
            tabs.read(cx).scroll.bounds(),
            tabs.read(cx).scroll.max_offset(),
            tabs.read(cx).scroll.offset(),
            tabs.read(cx)
                .controls
                .iter()
                .map(|f| f.is_focused(window))
                .collect::<Vec<_>>()
        );
        window.focus_next(cx);
        window.render_frame(cx);
        assert!(
            view.read(cx).other.read(cx).items[0]
                .focus
                .is_focused(window)
        );
        assert_eq!(view.read(cx).other.read(cx).selected(), Some(&1));
    });
    cx.update(|_, cx| assert_eq!(view.read(cx).events, vec![3]));
}
#[gpui_kit::test]
fn controlled_tabs_reject_proposals_and_read_current_availability(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx, true, true);
    let tabs = cx.update(|window, cx| {
        window.render_frame(cx);
        let tabs = view.read(cx).tabs.clone();
        tabs.update(cx, |state, cx| state.items[0].focus.focus(window, cx));
        window.press("end", cx);
        window.render_frame(cx);
        assert_eq!(tabs.read(cx).selected(), Some(&1));
        tabs
    });
    cx.update(|window, cx| {
        assert_eq!(view.read(cx).events, vec![3]);
        window.press("space", cx);
        window.render_frame(cx);
    });
    cx.update(|window, cx| {
        assert_eq!(view.read(cx).events, vec![3, 3]);
        tabs.update(cx, |state, cx| state.set_selected(Some(3), cx));
        window.render_frame(cx);
        window.press("enter", cx);
        window.render_frame(cx);
        tabs.update(cx, |state, cx| state.set_disabled(true, cx));
        window.press("home", cx);
        window.press("enter", cx);
        window.render_frame(cx);
    });
    cx.update(|_, cx| assert_eq!(view.read(cx).events, vec![3, 3]));
}
#[gpui_kit::test]
fn reorder_preserves_identity_and_removal_restores_focus(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx, false, false);
    cx.update(|window, cx| {
        window.render_frame(cx);
        let tabs = view.read(cx).tabs.clone();
        tabs.update(cx, |state, cx| state.items[2].focus.focus(window, cx));
        let id = tabs.read(cx).items[2].focus.clone();
        tabs.update(cx, |state, cx| {
            let mut reordered = items();
            reordered.reverse();
            state.set_items(reordered, window, cx);
        });
        window.render_frame(cx);
        assert!(id.is_focused(window));
        window.press("right", cx);
        window.render_frame(cx);
        assert!(tabs.read(cx).items[2].focus.is_focused(window));
        tabs.update(cx, |state, cx| {
            state.set_items(vec![TabItem::new("three", 3, "Settings")], window, cx)
        });
        window.render_frame(cx);
        assert_eq!(tabs.read(cx).selected(), Some(&3));
        assert!(tabs.read(cx).items[0].focus.is_focused(window));
        assert!(view.read(cx).events.is_empty(), "owner changes are silent");
        tabs.update(cx, |state, cx| state.set_items(vec![], window, cx));
        window.render_frame(cx);
        assert_eq!(
            window.find("after").focused(),
            Some(true),
            "bounds {:?}, max {:?}, offset {:?}, controls {:?}",
            tabs.read(cx).scroll.bounds(),
            tabs.read(cx).scroll.max_offset(),
            tabs.read(cx).scroll.offset(),
            tabs.read(cx)
                .controls
                .iter()
                .map(|f| f.is_focused(window))
                .collect::<Vec<_>>()
        );
    });
}

struct OverflowHarness {
    tabs: Entity<TabsState<u32>>,
    width: f32,
    variant: Variant,
    size: Size,
    show: bool,
    events: Vec<u32>,
    _subscription: gpui_kit::Subscription,
}
fn many_items() -> Vec<TabItem<u32>> {
    [
        "Overview",
        "Metrics",
        "Domains",
        "Settings café 🦀",
        "Access",
        "Analytics",
        "Logs",
        "Security",
        "Deployments",
        "Integrations",
    ]
    .into_iter()
    .enumerate()
    .map(|(i, label)| TabItem::new(format!("tab-{i}"), i as u32, label))
    .collect()
}
impl Render for OverflowHarness {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .tab_group()
            .flex()
            .flex_col()
            .w(px(self.width))
            .child(crate::Button::new("before", "Before"))
            .when(self.show, |v| {
                v.child(
                    Tabs::new("overflow", "Project navigation", &self.tabs)
                        .variant(self.variant)
                        .size(self.size),
                )
            })
            .child(crate::Button::new("after", "After"))
    }
}
fn overflow_harness(
    cx: &mut TestAppContext,
    width: f32,
    variant: Variant,
) -> (Entity<OverflowHarness>, &mut gpui_kit::VisualTestContext) {
    cx.update(crate::init);
    cx.add_window_view(|_, cx| {
        let tabs = cx.new(|cx| TabsState::new(many_items(), Some(0), cx));
        let subscription = cx.subscribe(
            &tabs,
            |this: &mut OverflowHarness, _, event: &TabsEvent<u32>, cx| {
                this.events.push(event.value);
                cx.notify();
            },
        );
        OverflowHarness {
            tabs,
            width,
            variant,
            size: Size::Base,
            show: true,
            events: vec![],
            _subscription: subscription,
        }
    })
}
#[gpui_kit::test]
fn overflow_actions_focus_scroll_and_recover_after_owner_shrink(cx: &mut TestAppContext) {
    let (view, cx) = overflow_harness(cx, 220., Variant::Segmented);
    cx.update(|window, cx| {
        cx.set_reduce_motion(true);
        window.render_frame(cx);
    });
    let tabs = cx.update(|window, cx| {
        window.render_frame(cx);
        assert!(window.within("overflow").try_find("scroll-start").is_none());
        window.within("overflow").click("scroll-end", cx);
        let tabs = view.read(cx).tabs.clone();
        assert!(
            tabs.read(cx).controls[1].is_focused(window),
            "pointer leaves focus on the edge action; offset {:?}, edges {:?}, valid {}, selected {:?}, focus {:?}", tabs.read(cx).scroll.offset(),tabs.read(cx).scroll_edges(),tabs.read(cx).layout_valid.get(), tabs.read(cx).selected(), window.within("overflow").find("scroll-end").focused()
        );
        window.render_frame(cx);
        assert!(tabs.read(cx).scroll.offset().x < px(0.));
        let offset = tabs.read(cx).scroll.offset().x;
        window.press("space", cx);
        window.render_frame(cx);
        assert!(
            tabs.read(cx).scroll.offset().x < offset,
            "Space continues the focused action"
        );
        tabs
    });
    cx.update(|window, cx| {
        tabs.update(cx, |s, cx| {
            s.set_items(
                vec![
                    TabItem::new("tab-0", 0, "One"),
                    TabItem::new("tab-1", 1, "Two"),
                ],
                window,
                cx,
            )
        });
        window.press("space", cx); // old mounted listener must not use invalidated geometry
        assert!(tabs.read(cx).scroll_motion.borrow().is_none());
        assert_eq!(tabs.read(cx).selected(), Some(&0));
        window.render_frame(cx);
    });
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert!(window.within("overflow").try_find("scroll-start").is_none());
        assert!(window.within("overflow").try_find("scroll-end").is_none());
        assert!(tabs.read(cx).items[0].focus.is_focused(window));
        assert_eq!(tabs.read(cx).scroll.offset().x, px(0.));
        assert!(
            view.read(cx).events.is_empty(),
            "scrolling/owner updates do not select a tab"
        );
        window.focus_next(cx);
        window.render_frame(cx);
        assert_eq!(window.find("after").focused(), Some(true));
    });
}
#[gpui_kit::test]
fn resize_hides_focused_edge_and_underline_has_no_edge_actions(cx: &mut TestAppContext) {
    let (view, cx) = overflow_harness(cx, 220., Variant::Segmented);
    cx.update(|window, cx| {
        window.render_frame(cx);
    });
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.within("overflow").click("scroll-end", cx);
        view.update(cx, |v, cx| {
            v.width = 2000.;
            cx.notify();
        });
        window.render_frame(cx);
    });
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert!(window.within("overflow").try_find("scroll-end").is_none());
        assert!(
            view.read(cx).tabs.read(cx).items[0]
                .focus
                .is_focused(window)
        );
        view.update(cx, |v, cx| {
            v.width = 220.;
            v.variant = Variant::Underline;
            cx.notify();
        });
        window.render_frame(cx);
        assert!(window.within("overflow").try_find("scroll-end").is_none());
        assert!(view.read(cx).tabs.read(cx).scroll.max_offset().x > px(1.));
    });
}
#[gpui_kit::test]
fn drag_scroll_suppresses_release_activation_and_unmount_clears_transients(
    cx: &mut TestAppContext,
) {
    let (view, cx) = overflow_harness(cx, 220., Variant::Segmented);
    cx.update(|window, cx| {
        window.render_frame(cx);
    });
    let (tabs, point) = cx.update(|window, cx| {
        window.render_frame(cx);
        (
            view.read(cx).tabs.clone(),
            window.within("overflow").find("tab-1").bounds().center(),
        )
    });
    cx.simulate_mouse_down(
        point,
        gpui_kit::MouseButton::Left,
        gpui_kit::Modifiers::default(),
    );
    cx.simulate_mouse_move(
        point - gpui_kit::point(px(2.), px(0.)),
        gpui_kit::MouseButton::Left,
        gpui_kit::Modifiers::default(),
    );
    cx.update(|_, cx| assert_eq!(tabs.read(cx).scroll.offset().x, px(0.)));
    let end = point - gpui_kit::point(px(30.), px(0.));
    cx.simulate_mouse_move(
        end,
        gpui_kit::MouseButton::Left,
        gpui_kit::Modifiers::default(),
    );
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert!(
            window
                .within("overflow")
                .find("tab-1")
                .bounds()
                .contains(&end)
        );
    });
    cx.simulate_mouse_up(
        end,
        gpui_kit::MouseButton::Left,
        gpui_kit::Modifiers::default(),
    );
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert_eq!(tabs.read(cx).selected(), Some(&0));
        assert!(view.read(cx).events.is_empty());
        assert!(tabs.read(cx).scroll.offset().x < px(0.));
    });
    cx.simulate_mouse_down(
        point,
        gpui_kit::MouseButton::Left,
        gpui_kit::Modifiers::default(),
    );
    cx.update(|window, cx| {
        view.update(cx, |v, cx| {
            v.show = false;
            cx.notify();
        });
        window.render_frame(cx);
    });
    cx.update(|window, cx| {
        assert!(tabs.read(cx).drag.borrow().is_none());
        assert!(tabs.read(cx).scroll_motion.borrow().is_none());
        view.update(cx, |v, cx| {
            v.show = true;
            cx.notify();
        });
        window.render_frame(cx);
    });
    let offset = cx.update(|_, cx| tabs.read(cx).scroll.offset());
    cx.simulate_mouse_move(
        end,
        gpui_kit::MouseButton::Left,
        gpui_kit::Modifiers::default(),
    );
    cx.update(|_, cx| assert_eq!(tabs.read(cx).scroll.offset(), offset));
    cx.simulate_mouse_up(
        end,
        gpui_kit::MouseButton::Left,
        gpui_kit::Modifiers::default(),
    );
}
#[gpui_kit::test]
fn selected_indicator_moves_resizes_reverses_and_reduced_motion_settles(cx: &mut TestAppContext) {
    let (view, cx) = overflow_harness(cx, 2000., Variant::Segmented);
    cx.update(|window, cx| {
        window.render_frame(cx);
    });
    cx.update(|window, cx| {
        let tabs = view.read(cx).tabs.clone();
        window.render_frame(cx);
        let fill = |window: &Window, cx: &App| {
            window
                .painted_quads()
                .into_iter()
                .find(|q| {
                    q.background.as_solid() == Some(crate::theme(cx).colors.base)
                        && q.bounds.size.height == px(32.).scale(window.scale_factor())
                })
                .expect("selected indicator fill")
                .bounds
        };
        let first = fill(window, cx);
        tabs.update(cx, |s, cx| s.set_selected(Some(3), cx));
        window.render_frame(cx);
        assert_eq!(
            fill(window, cx),
            first,
            "transition starts at the painted indicator"
        );
        cx.background_executor()
            .advance_clock(std::time::Duration::from_millis(80));
        window.render_frame(cx);
        let middle = fill(window, cx);
        let target = window
            .within("overflow")
            .find("tab-3")
            .bounds()
            .scale(window.scale_factor());
        assert!(middle.origin.x > first.origin.x && middle.origin.x < target.origin.x);
        assert!(middle.size.width > first.size.width && middle.size.width < target.size.width);
        tabs.update(cx, |s, cx| s.set_selected(Some(0), cx));
        window.render_frame(cx);
        assert_eq!(
            fill(window, cx),
            middle,
            "rapid reversal starts from painted position/width"
        );
        cx.background_executor()
            .advance_clock(std::time::Duration::from_millis(220));
        window.render_frame(cx);
        assert_eq!(fill(window, cx), first);
        cx.set_reduce_motion(true);
        tabs.update(cx, |s, cx| s.set_selected(Some(3), cx));
        window.render_frame(cx);
        assert_eq!(
            fill(window, cx),
            window
                .within("overflow")
                .find("tab-3")
                .bounds()
                .scale(window.scale_factor())
        );
        window.simulate_next_frame(cx);
        window.render_frame(cx);
        assert_eq!(
            window.simulate_next_frame(cx),
            0,
            "reduced motion stops requesting frames"
        );
    });
}
