use super::*;
use gpui_kit::{AppContext, TestAppContext, VisualTestContext, test::TestWindowExt};
struct Harness {
    toasts: Entity<ToastState>,
    events: Vec<ToastEvent>,
    mounted: bool,
    _events: Subscription,
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .tab_group()
            .child(
                Button::new("producer", "Save document").on_click(cx.listener(
                    |state, _, window, cx| {
                        state.toasts.update(cx, |state, cx| {
                            state.add(
                                Toast::new("saved", "Document saved")
                                    .description("Your changes are ready to share.")
                                    .variant(ToastVariant::Success)
                                    .timeout(Duration::ZERO)
                                    .action(ToastAction::new("undo", "Undo")),
                                window,
                                cx,
                            );
                        });
                    },
                )),
            )
            .child(Button::new("after", "After toast"))
            .when(self.mounted, |root| {
                root.child(ToastViewport::new("toasts", &self.toasts))
            })
    }
}
fn harness(cx: &mut TestAppContext) -> (Entity<Harness>, &mut VisualTestContext) {
    cx.update(crate::init);
    cx.add_window_view(|_, cx| {
        let toasts = cx.new(ToastState::new);
        let events = cx.subscribe(&toasts, |state: &mut Harness, _, event: &ToastEvent, cx| {
            state.events.push(event.clone());
            cx.notify();
        });
        Harness {
            toasts,
            events: vec![],
            mounted: true,
            _events: events,
        }
    })
}
fn frames(window: &mut Window, cx: &mut App) {
    for _ in 0..4 {
        window.render_frame(cx);
    }
}

#[gpui_kit::test]
fn toast_source_width_edges_focus_entry_action_and_close(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    cx.simulate_resize(gpui_kit::size(px(1040.), px(800.)));
    cx.update(|window, cx| {
        window.activate_window();
        frames(window, cx);
        window.click("producer", cx);
        frames(window, cx);
        window.press("f6", cx);
        frames(window, cx);
    });
    cx.executor().advance_clock(Duration::from_secs(1));
    cx.run_until_parked();
    cx.update(|window, cx| {
        frames(window, cx);
        let toast = view.read(cx).toasts.clone();
        assert!(toast.read(cx).focus.is_focused(window));
        let bounds = window.find("saved").bounds();
        assert_eq!(bounds.right(), px(1008.));
        assert_eq!(bounds.size.width, px(340.));
        assert_eq!(bounds.bottom(), px(768.));
        window.click("undo", cx);
        frames(window, cx);
        assert!(!toast.read(cx).is_empty());
        window.click("close", cx);
        frames(window, cx);
        assert_eq!(window.find("producer").focused(), Some(true));
        assert!(!toast.read(cx).is_empty()); // exits stay mounted
    });
    cx.executor().advance_clock(Duration::from_secs(1));
    cx.run_until_parked();
    cx.read(|cx| {
        assert!(view.read(cx).toasts.read(cx).is_empty());
        assert_eq!(
            view.read(cx)
                .events
                .iter()
                .filter(|event| matches!(event, ToastEvent::Action { .. }))
                .count(),
            1
        );
        assert_eq!(
            view.read(cx)
                .events
                .iter()
                .filter(|event| matches!(
                    event,
                    ToastEvent::Dismissed(_, ToastDismissReason::Close)
                ))
                .count(),
            1
        );
    });
}

#[gpui_kit::test]
fn toast_dedupe_functional_update_retains_order_focus_and_ending_guard(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    cx.update(|window, cx| {
        frames(window, cx);
        window.click("producer", cx);
        frames(window, cx);
        let toasts = view.read(cx).toasts.clone();
        let focus = toasts
            .read(cx)
            .manager
            .get(&"saved".into())
            .unwrap()
            .borrow()
            .actions["undo"]
            .clone();
        focus.focus(window, cx);
        toasts.update(cx, |state, cx| {
            state.add(Toast::new("saved", "Ignored duplicate"), window, cx);
            assert_eq!(state.len(), 1);
            state.add(
                Toast::new("second", "Second").timeout(Duration::ZERO),
                window,
                cx,
            );
            state.update(
                "saved",
                |content| content.title = format!("Updated {}", content.title).into(),
                window,
                cx,
            );
            assert_eq!(
                state
                    .manager
                    .iter()
                    .map(|(id, _, _)| id.as_ref())
                    .collect::<Vec<_>>(),
                vec!["saved", "second"]
            );
        });
        frames(window, cx);
        assert!(focus.is_focused(window));
        assert_eq!(
            toasts.read(cx).content("saved").unwrap().title.as_ref(),
            "Updated Document saved"
        );
        toasts.update(cx, |state, cx| {
            state.update("saved", |content| content.actions.clear(), window, cx);
        });
        frames(window, cx);
        assert!(
            toasts
                .read(cx)
                .manager
                .get(&"saved".into())
                .unwrap()
                .borrow()
                .focus
                .is_focused(window)
        );
        toasts.update(cx, |state, cx| {
            assert!(state.dismiss("saved", window, cx));
            assert!(!state.dismiss("saved", window, cx));
            assert!(!state.update("saved", |_| panic!("ending update called"), window, cx));
        });
        frames(window, cx);
        assert!(
            toasts
                .read(cx)
                .manager
                .get(&"second".into())
                .unwrap()
                .borrow()
                .focus
                .is_focused(window)
        );
    });
}

#[gpui_kit::test]
fn toast_timeout_pauses_for_focus_and_resumes_after_exit(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    cx.update(|window, cx| {
        window.activate_window();
        frames(window, cx);
        view.read(cx).toasts.clone().update(cx, |state, cx| {
            state.add(
                Toast::new("timer", "Timed notification").timeout(Duration::from_secs(1)),
                window,
                cx,
            );
        });
        frames(window, cx);
        window.press("f6", cx);
        frames(window, cx);
    });
    cx.executor().advance_clock(Duration::from_secs(3));
    cx.run_until_parked();
    cx.update(|window, cx| {
        frames(window, cx);
        let toasts = view.read(cx).toasts.clone();
        assert_eq!(
            toasts.read(cx).manager.iter().next().unwrap().2,
            base::ToastTransitionStatus::Present
        );
        window.press("shift-tab", cx);
        frames(window, cx);
    });
    cx.executor().advance_clock(Duration::from_secs(2));
    cx.run_until_parked();
    cx.executor().advance_clock(Duration::from_secs(1));
    cx.run_until_parked();
    cx.read(|cx| {
        assert!(view.read(cx).toasts.read(cx).is_empty());
        assert!(view.read(cx).events.contains(&ToastEvent::Dismissed(
            "timer".into(),
            ToastDismissReason::Timeout
        )));
    });
}

#[gpui_kit::test]
fn toast_unmount_cancels_retained_timer_and_remount_starts_clean(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    cx.update(|window, cx| {
        frames(window, cx);
        window.click("producer", cx);
        frames(window, cx);
        view.update(cx, |state, cx| {
            state.mounted = false;
            cx.notify();
        });
        frames(window, cx);
    });
    cx.run_until_parked();
    cx.update(|window, cx| {
        assert!(view.read(cx).toasts.read(cx).is_empty());
        view.update(cx, |state, cx| {
            state.mounted = true;
            cx.notify();
        });
        frames(window, cx);
        window.click("producer", cx);
        frames(window, cx);
        assert_eq!(view.read(cx).toasts.read(cx).len(), 1);
    });
}

#[gpui_kit::test]
fn toast_hover_pauses_timer_and_narrow_surface_matches_source(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    cx.update(|_, cx| cx.set_reduce_motion(true));
    cx.simulate_resize(gpui_kit::size(px(520.), px(800.)));
    cx.update(|window, cx| {
        window.activate_window();
        frames(window, cx);
        view.read(cx).toasts.clone().update(cx, |state, cx| {
            state.add(
                Toast::new("hovered", "Document saved")
                    .description("Your changes are ready to share.")
                    .variant(ToastVariant::Success)
                    .timeout(Duration::from_secs(1)),
                window,
                cx,
            );
        });
        frames(window, cx);
        window.hover("hovered", cx);
        frames(window, cx);
    });
    cx.executor().advance_clock(Duration::from_secs(3));
    cx.run_until_parked();
    cx.update(|window, cx| {
        frames(window, cx);
        let bounds = window.find("hovered").bounds();
        assert_eq!(
            bounds,
            gpui_kit::Bounds::new(
                gpui_kit::point(px(16.), px(708.)),
                gpui_kit::size(px(488.), px(76.))
            )
        );
        window.hover("after", cx);
        frames(window, cx);
    });
    cx.executor().advance_clock(Duration::from_secs(2));
    cx.run_until_parked();
    cx.executor().advance_clock(Duration::from_secs(1));
    cx.run_until_parked();
    cx.read(|cx| assert!(view.read(cx).toasts.read(cx).is_empty()));
}

#[gpui_kit::test]
fn toast_action_reads_current_availability_before_a_rerender(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    cx.update(|_, cx| cx.set_reduce_motion(true));
    cx.update(|window, cx| {
        frames(window, cx);
        window.click("producer", cx);
        frames(window, cx);
        let toasts = view.read(cx).toasts.clone();
        toasts.update(cx, |state, cx| {
            state.update(
                "saved",
                |content| content.actions[0].disabled = true,
                window,
                cx,
            );
        });
        window.click("undo", cx);
        frames(window, cx);
    });
    cx.read(|cx| {
        assert!(
            !view
                .read(cx)
                .events
                .iter()
                .any(|event| matches!(event, ToastEvent::Action { .. }))
        )
    });
}

#[gpui_kit::test]
fn toast_f6_then_tab_enters_newest_and_escape_recovers_to_remaining_toast(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    cx.update(|window, cx| {
        frames(window, cx);
        window.click("producer", cx);
        frames(window, cx);
        let toasts = view.read(cx).toasts.clone();
        toasts.update(cx, |state, cx| {
            state.add(
                Toast::new("newest", "Newest").timeout(Duration::ZERO),
                window,
                cx,
            );
        });
        frames(window, cx);
        window.press("f6", cx);
        frames(window, cx);
        window.press("tab", cx);
        frames(window, cx);
        assert!(
            toasts
                .read(cx)
                .manager
                .get(&"newest".into())
                .unwrap()
                .borrow()
                .focus
                .is_focused(window)
        );
        window.press("escape", cx);
        frames(window, cx);
        assert!(
            toasts
                .read(cx)
                .manager
                .get(&"saved".into())
                .unwrap()
                .borrow()
                .focus
                .is_focused(window)
        );
    });
}

#[gpui_kit::test]
fn toast_window_close_releases_rows_with_application_retaining_state(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    cx.update(|window, cx| {
        frames(window, cx);
        window.click("producer", cx);
        frames(window, cx);
        window.remove_window();
    });
    cx.run_until_parked();
    cx.read(|cx| {
        assert!(view.read(cx).toasts.read(cx).is_empty());
        assert!(!view.read(cx).toasts.read(cx).timer_running);
    });
}
