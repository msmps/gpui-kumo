use super::*;
use gpui_kit::{AppContext, TestAppContext, test::TestWindowExt};
struct Harness {
    state: Entity<TooltipState>,
    calls: usize,
    offset: f32,
    side: Side,
    align: Align,
    close_delay: Duration,
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let owner = cx.entity().downgrade();
        TooltipProvider::new(
            "provider",
            [self.state.downgrade()],
            div()
                .tab_group()
                .flex()
                .flex_col()
                .items_start()
                .pl(px(self.offset))
                .pt(px(200.))
                .child(Button::new("before", "Before"))
                .child(
                    Tooltip::new("tip", &self.state, "Helpful explanation", move |_, _| {
                        let owner = owner.clone();
                        Button::new("help", "Help").on_click(move |_, _, cx| {
                            let _ = owner.update(cx, |view, cx| {
                                view.calls += 1;
                                cx.notify();
                            });
                        })
                    })
                    .side(self.side)
                    .align(self.align)
                    .close_delay(self.close_delay),
                )
                .child(Button::new("after", "After")),
        )
    }
}
#[gpui_kit::test]
fn focus_disclosure_escape_and_activation_do_not_take_trigger_focus(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|window, cx| Harness {
        state: cx.new(|cx| TooltipState::new(window, cx)),
        calls: 0,
        offset: 200.,
        side: Side::Top,
        align: Align::Center,
        close_delay: Duration::ZERO,
    });
    let state = cx.read(|cx| view.read(cx).state.clone());
    cx.update(|window, cx| {
        window.activate_window();
        window.render_frame(cx);
        window.render_frame(cx);
        window.click("before", cx);
        window.press("tab", cx);
        window.focus_next(cx);
        window.render_frame(cx);
    });
    cx.run_until_parked();
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert!(state.read(cx).is_open());
        assert_eq!(window.find("help").focused(), Some(true));
        assert_eq!(
            window.find("tooltip-text").label(),
            Some("Helpful explanation")
        );
        window.press("escape", cx);
        window.render_frame(cx);
        assert!(!state.read(cx).is_open());
        assert!(window.try_find("tooltip-text").is_none());
        assert_eq!(window.find("help").focused(), Some(true));
        window.press("space", cx);
        assert_eq!(view.read(cx).calls, 1);
        window.press("enter", cx);
        assert_eq!(view.read(cx).calls, 2);
        window.press("tab", cx);
        window.focus_next(cx);
        window.render_frame(cx);
        assert_eq!(window.find("after").focused(), Some(true));
    });
    cx.run_until_parked();
    cx.update(|window, cx| {
        assert!(!state.read(cx).is_open());
        // Provider also dismisses an open tooltip with outside trigger focus.
        state.update(cx, |state, cx| state.set_open(true, cx));
        window.render_frame(cx);
        window.press("escape", cx);
        assert!(!state.read(cx).is_open());
    });
}
#[gpui_kit::test]
fn hover_timer_cancellation_and_disabled_disclosure(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|window, cx| Harness {
        state: cx.new(|cx| TooltipState::new(window, cx)),
        calls: 0,
        offset: 200.,
        side: Side::Top,
        align: Align::Center,
        close_delay: Duration::ZERO,
    });
    let state = cx.read(|cx| view.read(cx).state.clone());
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.render_frame(cx);
        window.hover("help", cx);
    });
    cx.run_until_parked();
    cx.background_executor
        .advance_clock(Duration::from_millis(599));
    cx.run_until_parked();
    cx.read(|cx| assert!(!state.read(cx).is_open()));
    cx.background_executor
        .advance_clock(Duration::from_millis(1));
    cx.run_until_parked();
    cx.read(|cx| assert!(state.read(cx).is_open()));
    cx.update(|window, cx| {
        window.hover("after", cx);
        assert!(!state.read(cx).is_open());
        window.hover("help", cx);
        window.hover("after", cx);
    });
    cx.run_until_parked();
    cx.background_executor
        .advance_clock(Duration::from_millis(600));
    cx.run_until_parked();
    cx.read(|cx| assert!(!state.read(cx).is_open()));
    cx.update(|window, cx| {
        window.hover("help", cx);
        state.update(cx, |state, cx| state.set_disabled(true, cx));
    });
    cx.run_until_parked();
    cx.background_executor
        .advance_clock(Duration::from_millis(600));
    cx.run_until_parked();
    cx.read(|cx| assert!(!state.read(cx).is_open()));
}

#[gpui_kit::test]
fn source_gap_alignment_and_anchor_movement(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|window, cx| Harness {
        state: cx.new(|cx| TooltipState::new(window, cx)),
        calls: 0,
        offset: 200.,
        side: Side::Top,
        align: Align::Start,
        close_delay: Duration::ZERO,
    });
    let state = cx.read(|cx| view.read(cx).state.clone());
    for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
        for side in [Side::Top, Side::Bottom, Side::Left, Side::Right] {
            for align in [Align::Start, Align::Center, Align::End] {
                cx.update(|window, cx| {
                    crate::set_appearance(appearance, cx);
                    view.update(cx, |view, cx| {
                        view.side = side;
                        view.align = align;
                        cx.notify();
                    });
                    state.update(cx, |state, cx| state.set_open(true, cx));
                    window.render_frame(cx);
                    window.render_frame(cx);
                    let trigger = window.find("help").bounds();
                    let popup = state.read(cx).resolved.get().unwrap().bounds;
                    let (a, b) = match side {
                        Side::Top | Side::Bottom => (trigger.origin.x, popup.origin.x),
                        Side::Left | Side::Right => (trigger.origin.y, popup.origin.y),
                    };
                    let (trigger_size, popup_size) = match side {
                        Side::Top | Side::Bottom => (trigger.size.width, popup.size.width),
                        Side::Left | Side::Right => (trigger.size.height, popup.size.height),
                    };
                    let expected = match align {
                        Align::Start => a,
                        Align::Center => a + (trigger_size - popup_size) / 2.,
                        Align::End => a + trigger_size - popup_size,
                    };
                    assert!(
                        (f32::from(b - expected)).abs() <= 1.,
                        "{side:?} {align:?}: {popup:?} vs {trigger:?}"
                    );
                    let gap = match side {
                        Side::Top => trigger.top() - popup.bottom(),
                        Side::Bottom => popup.top() - trigger.bottom(),
                        Side::Left => trigger.left() - popup.right(),
                        Side::Right => popup.left() - trigger.right(),
                    };
                    assert_eq!(gap, px(10.));
                });
            }
        }
    }
    cx.update(|window, cx| {
        view.update(cx, |view, cx| {
            view.side = Side::Top;
            view.align = Align::Start;
            cx.notify();
        });
        window.render_frame(cx);
        window.render_frame(cx);
    });
    let old = cx.read(|cx| state.read(cx).resolved.get().unwrap().bounds);
    cx.update(|window, cx| {
        view.update(cx, |view, cx| {
            view.offset += 80.;
            cx.notify();
        });
        window.render_frame(cx);
    });
    cx.run_until_parked();
    cx.update(|window, cx| {
        window.render_frame(cx);
        let popup = state.read(cx).resolved.get().unwrap().bounds;
        assert_eq!(popup.left() - old.left(), px(80.));
        assert_eq!(popup.left(), window.find("help").bounds().left());
    });
}

fn move_pointer(window: &mut Window, point: gpui_kit::Point<Pixels>, cx: &mut App) {
    window.dispatch_event(
        gpui_kit::PlatformInput::MouseMove(gpui_kit::MouseMoveEvent {
            position: point,
            pressed_button: None,
            modifiers: Default::default(),
        }),
        cx,
    );
}
#[gpui_kit::test]
fn popup_and_gap_are_hoverable_and_close_delay_is_cancellable(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|window, cx| Harness {
        state: cx.new(|cx| TooltipState::new(window, cx)),
        calls: 0,
        offset: 200.,
        side: Side::Top,
        align: Align::Center,
        close_delay: Duration::ZERO,
    });
    let state = cx.read(|cx| view.read(cx).state.clone());
    for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
        for side in [Side::Top, Side::Bottom, Side::Left, Side::Right] {
            cx.update(|window, cx| {
                crate::set_appearance(appearance, cx);
                view.update(cx, |view, cx| {
                    view.side = side;
                    cx.notify();
                });
                window.render_frame(cx);
                window.hover("help", cx);
                state.update(cx, |state, cx| state.set_open(true, cx));
                window.render_frame(cx);
                let trigger = window.find("help").bounds();
                let popup = state.read(cx).resolved.get().unwrap().bounds;
                let gap = match side {
                    Side::Top => {
                        gpui_kit::point(trigger.center().x, (popup.bottom() + trigger.top()) / 2.)
                    }
                    Side::Bottom => {
                        gpui_kit::point(trigger.center().x, (trigger.bottom() + popup.top()) / 2.)
                    }
                    Side::Left => {
                        gpui_kit::point((popup.right() + trigger.left()) / 2., trigger.center().y)
                    }
                    Side::Right => {
                        gpui_kit::point((trigger.right() + popup.left()) / 2., trigger.center().y)
                    }
                };
                move_pointer(window, gap, cx);
                assert!(state.read(cx).is_open(), "gap {side:?}");
                window.render_frame(cx);
                move_pointer(window, popup.center(), cx);
                assert!(state.read(cx).is_open(), "popup {side:?}");
                window.render_frame(cx);
                // Reentering an already open trigger must not start another
                // opening timer which would prevent immediate outside closure.
                window.hover("help", cx);
                assert!(state.read(cx).is_open());
                window.render_frame(cx);
                move_pointer(window, gpui_kit::point(px(10.), px(10.)), cx);
                assert!(!state.read(cx).is_open(), "outside {side:?}");
            });
        }
    }
    cx.update(|window, cx| {
        view.update(cx, |view, cx| {
            view.close_delay = Duration::from_millis(200);
            cx.notify();
        });
        window.render_frame(cx);
        window.hover("help", cx);
        state.update(cx, |state, cx| state.set_open(true, cx));
        window.render_frame(cx);
        move_pointer(window, gpui_kit::point(px(10.), px(10.)), cx);
    });
    cx.run_until_parked();
    cx.background_executor
        .advance_clock(Duration::from_millis(199));
    cx.run_until_parked();
    cx.read(|cx| assert!(state.read(cx).is_open()));
    cx.update(|window, cx| {
        let popup = state.read(cx).resolved.get().unwrap().bounds;
        move_pointer(window, popup.center(), cx);
        window.render_frame(cx);
    });
    cx.run_until_parked();
    cx.background_executor
        .advance_clock(Duration::from_millis(201));
    cx.run_until_parked();
    cx.read(|cx| assert!(state.read(cx).is_open(), "return cancels close"));
    cx.update(|window, cx| {
        move_pointer(window, gpui_kit::point(px(10.), px(10.)), cx);
    });
    cx.run_until_parked();
    cx.background_executor
        .advance_clock(Duration::from_millis(200));
    cx.run_until_parked();
    cx.read(|cx| assert!(!state.read(cx).is_open()));
}
