use super::*;
use gpui_kit::{
    AppContext, TestAppContext, VisualTestContext, prelude::FluentBuilder, test::TestWindowExt,
};

struct Harness {
    popup: Entity<PopoverState>,
    inside: FocusHandle,
    outside: FocusHandle,
    placement: Placement,
    initial_focus: bool,
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let inside = self.inside.clone();
        div()
            .flex()
            .flex_col()
            .gap(px(24.))
            .p(px(32.))
            .child(
                Popover::new("popover", &self.popup, "Settings")
                    .placement(self.placement)
                    .when(self.initial_focus, |this| this.initial_focus(&self.inside))
                    .content(move |close, _, _| {
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(12.))
                            .child(Button::new("inside", "Inside").track_focus(&inside))
                            .child(
                                Button::new("close", "Close").on_click(move |_, window, cx| {
                                    close.dismiss(window, cx);
                                }),
                            )
                    }),
            )
            .child(
                div()
                    .ml(px(400.))
                    .child(Button::new("outside", "Outside").track_focus(&self.outside)),
            )
    }
}
fn harness(cx: &mut TestAppContext) -> (Entity<Harness>, &mut VisualTestContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|_, cx| Harness {
        popup: cx.new(|cx| PopoverState::new("Settings dialog", cx)),
        inside: cx.focus_handle(),
        outside: cx.focus_handle(),
        placement: Placement::Bottom,
        initial_focus: false,
    });
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.render_frame(cx);
    });
    (view, cx)
}

#[gpui_kit::test]
fn pointer_and_keyboard_toggle_once_and_escape_restores_focus(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    let changes = Rc::new(std::cell::RefCell::new(Vec::new()));
    let _subscription = cx.update(|_, cx| {
        let changes = changes.clone();
        cx.subscribe(
            &view.read(cx).popup.clone(),
            move |_, event: &PopoverEvent, _| changes.borrow_mut().push(event.open),
        )
    });
    cx.update(|window, cx| {
        let state = view.read(cx).popup.clone();
        assert_eq!(window.find("trigger").expanded(), Some(false));
        window.click("trigger", cx);
        assert!(state.read(cx).is_open());
        assert_eq!(window.find("trigger").expanded(), Some(true));
        assert_eq!(window.find("surface").role(), Some(Role::Dialog));
        assert_eq!(window.find("surface").label(), Some("Settings dialog"));
        window.focus_next(cx);
        assert!(view.read(cx).inside.is_focused(window));
        window.press("escape", cx);
        assert!(!state.read(cx).is_open());
        assert!(state.read(cx).trigger_focus().is_focused(window));
        assert!(window.try_find("surface").is_none());
        for key in ["enter", "space"] {
            window.press(key, cx);
            assert!(state.read(cx).is_open());
            window.press("escape", cx);
            assert!(!state.read(cx).is_open());
        }
        window.click("trigger", cx);
        window.click("trigger", cx);
        assert!(!state.read(cx).is_open());
    });
    assert_eq!(
        &*changes.borrow(),
        &[true, false, true, false, true, false, true, false]
    );
}

#[gpui_kit::test]
fn inside_click_keeps_open_close_and_outside_click_dismiss(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    cx.update(|window, cx| {
        let state = view.read(cx).popup.clone();
        window.click("trigger", cx);
        window.click("inside", cx);
        assert!(state.read(cx).is_open());
        window.click("close", cx);
        assert!(!state.read(cx).is_open());
        assert!(state.read(cx).trigger_focus().is_focused(window));
        window.click("trigger", cx);
        window.click("outside", cx);
        assert!(!state.read(cx).is_open());
        assert!(view.read(cx).outside.is_focused(window));
    });
}

#[gpui_kit::test]
fn disable_dismisses_and_rejects_all_trigger_paths(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    cx.update(|window, cx| {
        let state = view.read(cx).popup.clone();
        window.click("trigger", cx);
        state.update(cx, |state, cx| state.set_disabled(true, window, cx));
        window.render_frame(cx);
        assert!(!state.read(cx).is_open());
        for key in ["enter", "space"] {
            window.press(key, cx);
            assert!(!state.read(cx).is_open());
        }
        window.click("trigger", cx);
        assert!(!state.read(cx).is_open());
        state.update(cx, |state, cx| state.set_open(true, window, cx));
        assert!(!state.read(cx).is_open());
        state.update(cx, |state, cx| state.set_disabled(false, window, cx));
        window.render_frame(cx);
        window.click("trigger", cx);
        assert!(state.read(cx).is_open());
    });
}

#[gpui_kit::test]
fn placement_geometry_and_theme_update_keep_open(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    for placement in [
        Placement::Bottom,
        Placement::Top,
        Placement::Left,
        Placement::Right,
    ] {
        cx.update(|window, cx| {
            let state = view.read(cx).popup.clone();
            state.update(cx, |state, cx| state.dismiss(window, cx));
            view.update(cx, |view, cx| {
                view.placement = placement;
                cx.notify();
            });
            window.render_frame(cx);
            window.click("trigger", cx);
            let bounds = window.find("surface").bounds();
            assert!(bounds.origin.x >= px(8.) && bounds.origin.y >= px(8.));
            assert!(bounds.right() <= window.viewport_size().width - px(8.));
            assert!(bounds.bottom() <= window.viewport_size().height - px(8.));
            if placement == Placement::Bottom {
                assert_eq!(
                    bounds.origin.y,
                    window.find("trigger").bounds().bottom() + px(8.)
                );
            }
            crate::set_appearance(crate::Appearance::Dark, cx);
            window.render_frame(cx);
            assert!(state.read(cx).is_open());
            assert_eq!(window.find("surface").bounds(), bounds);
        });
    }
}

#[gpui_kit::test]
fn nested_resize_keeps_focus_and_escape_closes_nearest_panel_then_descendants(
    cx: &mut TestAppContext,
) {
    struct Nested {
        parent: Entity<PopoverState>,
        child: Entity<PopoverState>,
    }
    impl Render for Nested {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let child = self.child.clone();
            div()
                .p(px(32.))
                .child(Popover::new("parent", &self.parent, "Parent").content(
                    move |parent, _, _| {
                        Popover::new("child", &child, "Child")
                            .parent(&parent)
                            .placement(Placement::Right)
                            .content(|close, _, _| {
                                Button::new("nested-close", "Close child")
                                    .on_click(move |_, window, cx| close.dismiss(window, cx))
                            })
                    },
                ))
        }
    }
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|_, cx| Nested {
        parent: cx.new(|cx| PopoverState::new("Parent dialog", cx)),
        child: cx.new(|cx| PopoverState::new("Child dialog", cx)),
    });
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.render_frame(cx);
        window.within("parent").click("trigger", cx);
        cx.background_executor()
            .advance_clock(Duration::from_millis(75));
        window.simulate_next_frame(cx);
        window.render_frame(cx);
        window.within("child").click("trigger", cx);
        cx.background_executor()
            .advance_clock(Duration::from_millis(75));
        window.simulate_next_frame(cx);
        window.render_frame(cx);
        let child_bounds = window
            .within("child")
            .find("surface")
            .bounds()
            .scale(window.scale_factor());
        let child_alpha = window
            .painted_quads()
            .iter()
            .find_map(|quad| {
                (quad.bounds == child_bounds)
                    .then(|| quad.background.as_solid().map(|color| color.a))
                    .flatten()
            })
            .expect("nested popup paints its own fade");
        assert!((child_alpha - 0.5).abs() < 0.01);
        cx.background_executor()
            .advance_clock(Duration::from_millis(75));
        window.simulate_next_frame(cx);
        window.render_frame(cx);
        let child_alpha = window
            .painted_quads()
            .iter()
            .find_map(|quad| {
                (quad.bounds == child_bounds)
                    .then(|| quad.background.as_solid().map(|color| color.a))
                    .flatten()
            })
            .expect("nested popup settles opaque");
        assert_eq!(child_alpha, 1.);
    });
    cx.simulate_resize(gpui_kit::size(px(320.), px(240.)));
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.render_frame(cx);
        let parent = view.read(cx).parent.clone();
        let child = view.read(cx).child.clone();
        assert!(parent.read(cx).is_open() && child.read(cx).is_open());
        assert!(child.read(cx).content_focus.contains_focused(window, cx));
        let parent_bounds = parent.read(cx).resolved_position.get().unwrap().bounds;
        let child_bounds = window.within("child").find("surface").bounds();
        assert!(window.painted_quads().iter().any(|quad| {
            quad.bounds == parent_bounds.scale(window.scale_factor())
                && quad
                    .background
                    .as_solid()
                    .is_some_and(|color| color.a == 1.)
        }));
        for bounds in [parent_bounds, child_bounds] {
            assert!(bounds.left() >= px(8.) && bounds.top() >= px(8.));
            assert!(bounds.right() <= px(312.) && bounds.bottom() <= px(232.));
        }
        window.press("escape", cx);
        assert!(parent.read(cx).is_open());
        assert!(!child.read(cx).is_open());
        assert!(child.read(cx).trigger_focus().is_focused(window));
        window.within("child").click("trigger", cx);
        parent.update(cx, |parent, cx| parent.dismiss(window, cx));
        window.render_frame(cx);
        assert!(!parent.read(cx).is_open() && !child.read(cx).is_open());
        assert!(parent.read(cx).trigger_focus().is_focused(window));
        assert!(!base::GlobalState::is_in_deferred_context(cx));
    });
}

#[gpui_kit::test]
fn removing_an_open_owner_releases_registration_without_content_cycles(cx: &mut TestAppContext) {
    struct Removable {
        state: Option<Entity<PopoverState>>,
    }
    impl Render for Removable {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div().children(self.state.as_ref().map(|state| {
                Popover::new("removable", state, "Open").content(|close, _, _| {
                    Button::new("done", "Done")
                        .on_click(move |_, window, cx| close.dismiss(window, cx))
                })
            }))
        }
    }
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|_, cx| Removable {
        state: Some(cx.new(|cx| PopoverState::new("Dialog", cx))),
    });
    let weak = cx.read(|cx| view.read(cx).state.as_ref().unwrap().downgrade());
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.render_frame(cx);
        window.click("trigger", cx);
        assert!(base::GlobalState::is_in_deferred_context(cx));
        view.update(cx, |view, cx| {
            view.state = None;
            cx.notify();
        });
        window.render_frame(cx);
        window.render_frame(cx);
    });
    cx.run_until_parked();
    cx.read(|cx| {
        assert!(weak.upgrade().is_none());
        assert!(!base::GlobalState::is_in_deferred_context(cx));
    });
}

#[gpui_kit::test]
fn initial_focus_and_programmatic_dismiss_preserve_outside_focus(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    cx.update(|window, cx| {
        let state = view.read(cx).popup.clone();
        let outside = view.read(cx).outside.clone();
        window.click("trigger", cx);
        outside.focus(window, cx);
        state.update(cx, |state, cx| state.dismiss(window, cx));
        assert!(outside.is_focused(window));
        let inside = view.read(cx).inside.clone();
        view.update(cx, |view, cx| {
            view.initial_focus = true;
            cx.notify();
        });
        window.render_frame(cx);
        window.click("trigger", cx);
        assert!(inside.is_focused(window));
    });
}

#[gpui_kit::test]
fn first_open_initial_focus_accepts_text_and_reopening_preserves_edits(cx: &mut TestAppContext) {
    use gpui_kit::Focusable;
    struct EditorPopup {
        popup: Entity<PopoverState>,
        input: Entity<crate::InputState>,
    }
    impl Render for EditorPopup {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            let input = self.input.clone();
            Popover::new("editor-popup", &self.popup, "Edit")
                .initial_focus(&input.read(cx).focus_handle(cx))
                .content(move |_, _, _| crate::Input::new("editor", &input).show_label(true))
        }
    }
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|window, cx| EditorPopup {
        popup: cx.new(|cx| PopoverState::new("Edit dialog", cx)),
        input: cx.new(|cx| crate::InputState::new("Name", window, cx)),
    });
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.render_frame(cx);
        window.click("trigger", cx);
        assert!(
            view.read(cx)
                .input
                .read(cx)
                .focus_handle(cx)
                .is_focused(window)
        );
        window.input("draft", cx);
        assert_eq!(view.read(cx).input.read(cx).value(cx).as_ref(), "draft");
        window.press("escape", cx);
        assert!(!view.read(cx).popup.read(cx).is_open());
        window.click("trigger", cx);
        window.input("!", cx);
        assert_eq!(view.read(cx).input.read(cx).value(cx).as_ref(), "draft!");
    });
}

#[gpui_kit::test]
fn destroyed_previous_focus_target_is_not_restored(cx: &mut TestAppContext) {
    struct RemovableFocus {
        popup: Entity<PopoverState>,
        opener: Option<FocusHandle>,
    }
    impl Render for RemovableFocus {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .flex()
                .flex_col()
                .children(
                    self.opener
                        .as_ref()
                        .map(|focus| Button::new("opener", "Open").track_focus(focus)),
                )
                .child(
                    Popover::new("popup", &self.popup, "Settings")
                        .content(|_, _, _| div().child("Settings content")),
                )
        }
    }
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|_, cx| RemovableFocus {
        popup: cx.new(|cx| PopoverState::new("Settings", cx)),
        opener: Some(cx.focus_handle()),
    });
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.render_frame(cx);
        view.read(cx)
            .opener
            .as_ref()
            .unwrap()
            .clone()
            .focus(window, cx);
        let popup = view.read(cx).popup.clone();
        popup.update(cx, |popup, cx| popup.set_open(true, window, cx));
        window.render_frame(cx);
        view.update(cx, |view, cx| {
            view.opener = None;
            cx.notify();
        });
        window.render_frame(cx);
        window.render_frame(cx);
        popup.update(cx, |popup, cx| popup.dismiss(window, cx));
        assert!(window.focused(cx).is_none());
    });
}

#[gpui_kit::test]
fn side_collisions_flip_and_follow_a_moving_trigger(cx: &mut TestAppContext) {
    struct Edge {
        popup: Entity<PopoverState>,
        origin: gpui_kit::Point<Pixels>,
        placement: Placement,
    }
    impl Render for Edge {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div().size_full().child(
                div()
                    .absolute()
                    .left(self.origin.x)
                    .top(self.origin.y)
                    .child(
                        Popover::new("edge", &self.popup, "Settings")
                            .placement(self.placement)
                            .arrow(true)
                            .content(|_, _, _| div().h(px(80.)).child("Edge content")),
                    ),
            )
        }
    }
    cx.update(|cx| {
        crate::init(cx);
        cx.set_reduce_motion(true);
    });
    let (view, cx) = cx.add_window_view(|_, cx| Edge {
        popup: cx.new(|cx| PopoverState::new("Edge dialog", cx)),
        origin: gpui_kit::point(px(300.), px(300.)),
        placement: Placement::Bottom,
    });
    cx.update(|window, cx| {
        let viewport = window.viewport_size();
        for (requested, origin, expected) in [
            (
                Placement::Bottom,
                gpui_kit::point(px(300.), viewport.height - px(50.)),
                base::Placement::Top,
            ),
            (
                Placement::Top,
                gpui_kit::point(px(300.), px(8.)),
                base::Placement::Bottom,
            ),
            (
                Placement::Left,
                gpui_kit::point(px(8.), px(300.)),
                base::Placement::Right,
            ),
            (
                Placement::Right,
                gpui_kit::point(viewport.width - px(150.), px(300.)),
                base::Placement::Left,
            ),
        ] {
            view.update(cx, |view, cx| {
                view.origin = origin;
                view.placement = requested;
                cx.notify();
            });
            window.render_frame(cx);
            window.render_frame(cx);
            let state = view.read(cx).popup.clone();
            state.update(cx, |state, cx| state.set_open(true, window, cx));
            window.render_frame(cx);
            assert_eq!(
                state.read(cx).resolved_position.get().unwrap().placement,
                Some(expected)
            );
            let bounds = window.find("surface").bounds();
            let trigger = window.find("trigger").bounds();
            match expected {
                base::Placement::Top => assert_eq!(bounds.bottom(), trigger.top() - px(8.)),
                base::Placement::Bottom => assert_eq!(bounds.top(), trigger.bottom() + px(8.)),
                base::Placement::Left => assert_eq!(bounds.right(), trigger.left() - px(8.)),
                base::Placement::Right => assert_eq!(bounds.left(), trigger.right() + px(8.)),
            }
        }
        view.update(cx, |view, cx| {
            view.origin = gpui_kit::point(px(350.), px(250.));
            view.placement = Placement::Bottom;
            cx.notify();
        });
        window.render_frame(cx);
        window.render_frame(cx);
        assert_eq!(
            window.find("surface").bounds().top(),
            window.find("trigger").bounds().bottom() + px(8.)
        );
    });
}

#[gpui_kit::test]
fn opening_fade_settles_and_reduced_motion_opens_opaque(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    cx.update(|window, cx| {
        let state = view.read(cx).popup.clone();
        let open = |window: &mut Window, cx: &mut App| {
            state.update(cx, |state, cx| state.set_open(true, window, cx));
            window.render_frame(cx);
        };
        let opacity = |window: &Window| {
            let bounds = window.find("surface").bounds().scale(window.scale_factor());
            window
                .painted_quads()
                .iter()
                .find_map(|quad| {
                    (quad.bounds == bounds)
                        .then(|| quad.background.as_solid().map(|color| color.a))
                        .flatten()
                })
                .unwrap_or(0.)
        };
        open(window, cx);
        assert_eq!(opacity(window), 0.);
        cx.background_executor()
            .advance_clock(Duration::from_millis(75));
        window.render_frame(cx);
        assert!((opacity(window) - 0.5).abs() < 0.01);
        cx.background_executor()
            .advance_clock(Duration::from_millis(75));
        window.render_frame(cx);
        assert_eq!(opacity(window), 1.);
        state.update(cx, |state, cx| state.dismiss(window, cx));
        window.render_frame(cx);
        assert!(window.try_find("surface").is_none());
        cx.set_reduce_motion(true);
        open(window, cx);
        assert_eq!(opacity(window), 1.);
    });
}

struct StressHarness {
    popup: Entity<PopoverState>,
    activated: Rc<Cell<usize>>,
}

#[gpui_kit::test]
fn corner_clamping_and_neither_side_fitting_preserve_input_boundaries(cx: &mut TestAppContext) {
    struct Collision {
        state: Entity<PopoverState>,
        origin: gpui_kit::Point<Pixels>,
        placement: Placement,
        tall: bool,
        activations: Rc<Cell<usize>>,
    }
    impl Render for Collision {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let tall = self.tall;
            let activations = self.activations.clone();
            div().size_full().child(
                div()
                    .absolute()
                    .left(self.origin.x)
                    .top(self.origin.y)
                    .child(
                        Popover::new("collision", &self.state, "Open")
                            .placement(self.placement)
                            .width(px(200.))
                            .arrow(true)
                            .content(move |_, _, _| {
                                let activations = activations.clone();
                                div().h(px(if tall { 200. } else { 60. })).child(
                                    Button::new("inside", "Inside").on_click(move |_, _, _| {
                                        activations.set(activations.get() + 1);
                                    }),
                                )
                            }),
                    ),
            )
        }
    }
    cx.update(|cx| {
        crate::init(cx);
        cx.set_reduce_motion(true);
    });
    let (view, cx) = cx.add_window_view(|_, cx| Collision {
        state: cx.new(|cx| PopoverState::new("Collision dialog", cx)),
        origin: gpui_kit::point(px(8.), px(8.)),
        placement: Placement::Bottom,
        tall: false,
        activations: Rc::new(Cell::new(0)),
    });
    cx.simulate_resize(gpui_kit::size(px(400.), px(300.)));
    for (x, y, placement) in [
        (8., 8., Placement::Bottom),
        (330., 8., Placement::Bottom),
        (8., 256., Placement::Top),
        (330., 256., Placement::Top),
        (8., 8., Placement::Right),
        (330., 8., Placement::Left),
        (8., 256., Placement::Right),
        (330., 256., Placement::Left),
    ] {
        cx.update(|window, cx| {
            let state = view.read(cx).state.clone();
            state.update(cx, |state, cx| state.dismiss(window, cx));
            view.update(cx, |view, cx| {
                view.origin = gpui_kit::point(px(x), px(y));
                view.placement = placement;
                cx.notify();
            });
            window.render_frame(cx);
            window.render_frame(cx);
            window.click("trigger", cx);
            let bounds = window.find("surface").bounds();
            assert!(bounds.left() >= px(8.) && bounds.top() >= px(8.));
            assert!(bounds.right() <= px(392.) && bounds.bottom() <= px(292.));
            window.click("inside", cx);
            assert!(state.read(cx).is_open());
        });
    }
    cx.update(|window, cx| {
        let state = view.read(cx).state.clone();
        state.update(cx, |state, cx| state.dismiss(window, cx));
        view.update(cx, |view, cx| {
            view.origin = gpui_kit::point(px(180.), px(140.));
            view.placement = Placement::Bottom;
            view.tall = true;
            cx.notify();
        });
        window.render_frame(cx);
        window.render_frame(cx);
        window.click("trigger", cx);
        let surface = window.find("surface").bounds();
        let trigger = window.find("trigger").bounds();
        assert!(
            surface.contains(&trigger.center()),
            "{surface:?}, {trigger:?}"
        );
        // Neither side fits: Base clamps the popup across its trigger. The overlay
        // must consume this pointer event rather than activating the covered trigger.
        window.click("trigger", cx);
        assert!(state.read(cx).is_open());
        assert_eq!(view.read(cx).activations.get(), 8);
        window.press("escape", cx);
        assert!(!state.read(cx).is_open());
        assert!(state.read(cx).trigger_focus().is_focused(window));
    });
}

#[gpui_kit::test]
fn resized_popup_dismissal_removes_painted_surface_without_forcing_refresh(
    cx: &mut TestAppContext,
) {
    use gpui_kit::{InputEvent, KeyDownEvent, Keystroke};

    cx.update(|cx| {
        crate::init(cx);
        cx.set_reduce_motion(true);
    });
    let (view, cx) = cx.add_window_view(|_, cx| StressHarness {
        popup: cx.new(|cx| PopoverState::new("Stress dialog", cx)),
        activated: Rc::new(Cell::new(0)),
    });
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.render_frame(cx);
        window.click("trigger", cx);
    });
    cx.simulate_resize(gpui_kit::size(px(400.), px(280.)));
    cx.update(|window, cx| {
        window.draw(cx).clear(cx);
        window.simulate_next_frame(cx);
        window.draw(cx).clear(cx);
        let surface = window.find("surface").bounds().scale(window.scale_factor());
        assert!(
            window
                .painted_quads()
                .iter()
                .any(|quad| quad.bounds == surface)
        );
        window.dispatch_event(
            KeyDownEvent {
                keystroke: Keystroke::parse("escape").unwrap(),
                is_held: false,
                prefer_character_input: false,
            }
            .to_platform_input(),
            cx,
        );
        assert!(!view.read(cx).popup.read(cx).is_open());
        // TestWindowExt::render_frame forces a full refresh and can hide cache defects.
        window.draw(cx).clear(cx);
        assert!(window.try_find("surface").is_none());
        assert!(
            !window
                .painted_quads()
                .iter()
                .any(|quad| quad.bounds == surface)
        );
    });
}

#[gpui_kit::test]
fn collision_fitting_includes_the_requested_gap(cx: &mut TestAppContext) {
    struct NearFit(Entity<PopoverState>);
    impl Render for NearFit {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div().size_full().child(
                div().absolute().left(px(120.)).top(px(220.)).child(
                    Popover::new("near-fit", &self.0, "Open")
                        .width(px(200.))
                        .content(|_, _, _| div().h(px(108.)).child("Near-fit content")),
                ),
            )
        }
    }
    cx.update(crate::init);
    let (view, cx) =
        cx.add_window_view(|_, cx| NearFit(cx.new(|cx| PopoverState::new("Near-fit dialog", cx))));
    cx.simulate_resize(gpui_kit::size(px(400.), px(400.)));
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.render_frame(cx);
        window.click("trigger", cx);
        let surface = window.find("surface").bounds();
        let trigger = window.find("trigger").bounds();
        assert_eq!(surface.size.height, px(132.));
        assert_eq!(
            view.read(cx)
                .0
                .read(cx)
                .resolved_position
                .get()
                .unwrap()
                .placement,
            Some(base::Placement::Top)
        );
        assert_eq!(surface.bottom(), trigger.top() - px(8.));
    });
}

#[gpui_kit::test]
fn open_popup_follows_trigger_after_real_ancestor_scrolling(cx: &mut TestAppContext) {
    struct Scrolling(Entity<PopoverState>);
    impl Render for Scrolling {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .id("ancestor")
                .w(px(400.))
                .h(px(180.))
                .flex()
                .flex_col()
                .overflow_y_scroll()
                .child(div().h(px(80.)).flex_shrink_0())
                .child(
                    Popover::new("scrolled", &self.0, "Open")
                        .content(|_, _, _| div().h(px(40.)).child("Scrolling trigger")),
                )
                .child(div().h(px(400.)).flex_shrink_0())
        }
    }
    cx.update(crate::init);
    let (view, cx) = cx
        .add_window_view(|_, cx| Scrolling(cx.new(|cx| PopoverState::new("Scrolling popup", cx))));
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.render_frame(cx);
        window.click("trigger", cx);
        let before = window.find("trigger").bounds();
        use gpui_kit::InputEvent;
        window.dispatch_event(
            gpui_kit::ScrollWheelEvent {
                position: gpui_kit::point(px(20.), px(20.)),
                delta: gpui_kit::ScrollDelta::Pixels(gpui_kit::point(px(0.), -px(40.))),
                ..Default::default()
            }
            .to_platform_input(),
            cx,
        );
        window.render_frame(cx);
        window.render_frame(cx);
        let trigger = window.find("trigger").bounds();
        assert_eq!(trigger.top(), before.top() - px(40.));
        assert_eq!(
            window.find("surface").bounds().top(),
            trigger.bottom() + px(8.)
        );
        assert!(view.read(cx).0.read(cx).is_open());
        window.click("trigger", cx);
        assert!(!view.read(cx).0.read(cx).is_open());
    });
}

impl Render for StressHarness {
    fn render(&mut self, window: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let viewport = window.viewport_size();
        let activated = self.activated.clone();
        div().size_full().child(
            div()
                .absolute()
                .left(viewport.width / 2. - px(40.))
                .top(viewport.height / 2. - px(18.))
                .child(
                    Popover::new("stress", &self.popup, "Open")
                        .width(px(600.))
                        .arrow(true)
                        .content(move |_, _, _| {
                            let activated = activated.clone();
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(8.))
                                .children((0..30).map(|index| {
                                    div()
                                        .h(px(36.))
                                        .flex_shrink_0()
                                        .child(format!("Content row {index}"))
                                }))
                                .child(
                                    Button::new("last", "Last action").on_click(move |_, _, _| {
                                        activated.set(activated.get() + 1)
                                    }),
                                )
                        }),
                ),
        )
    }
}

#[gpui_kit::test]
fn resizing_open_oversized_content_preserves_fitting_and_scroll_reachability(
    cx: &mut TestAppContext,
) {
    cx.update(|cx| {
        crate::init(cx);
        cx.set_reduce_motion(true);
    });
    let (view, cx) = cx.add_window_view(|_, cx| StressHarness {
        popup: cx.new(|cx| PopoverState::new("Stress dialog", cx)),
        activated: Rc::new(Cell::new(0)),
    });
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.render_frame(cx);
        window.click("trigger", cx);
    });
    for (width, height) in [(640., 480.), (320., 240.), (160., 120.), (800., 600.)] {
        cx.simulate_resize(gpui_kit::size(px(width), px(height)));
        cx.update(|window, cx| {
            window.render_frame(cx);
            window.render_frame(cx);
            let bounds = window.find("surface").bounds();
            assert!(view.read(cx).popup.read(cx).is_open());
            assert!(bounds.left() >= px(8.) && bounds.top() >= px(8.));
            assert!(bounds.right() <= px(width - 8.), "{bounds:?}");
            assert!(bounds.bottom() <= px(height - 8.), "{bounds:?}");
            let before = window.find("last").bounds();
            use gpui_kit::InputEvent;
            window.dispatch_event(
                gpui_kit::ScrollWheelEvent {
                    position: bounds.center(),
                    delta: gpui_kit::ScrollDelta::Pixels(gpui_kit::point(px(0.), -px(2000.))),
                    ..Default::default()
                }
                .to_platform_input(),
                cx,
            );
            window.render_frame(cx);
            let last = window.find("last").bounds();
            assert!(last.top() < before.top() || bounds.contains(&last.center()));
            assert!(
                bounds.contains(&last.center()),
                "last action is unreachable: {last:?}, {bounds:?}"
            );
            window.click("last", cx);
            assert!(view.read(cx).popup.read(cx).is_open());
        });
    }
    cx.read(|cx| assert_eq!(view.read(cx).activated.get(), 4));
}

struct Reparenting {
    parents: [Entity<PopoverState>; 2],
    child: Entity<PopoverState>,
    selected_parent: Option<usize>,
}

impl Render for Reparenting {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let child = Popover::new("child", &self.child, "Child")
            .when_some(self.selected_parent, |child, index| {
                child.parent(&PopoverClose {
                    state: self.parents[index].downgrade(),
                })
            })
            .content(|_, _, _| "Child content");
        div()
            .flex()
            .gap(px(100.))
            .p(px(32.))
            .children(self.parents.iter().enumerate().map(|(index, parent)| {
                Popover::new(("parent", index), parent, "Parent")
                    .content(|_, _, _| "Parent content")
            }))
            .child(child)
    }
}

fn reparenting_harness(cx: &mut TestAppContext) -> (Entity<Reparenting>, &mut VisualTestContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|_, cx| Reparenting {
        parents: std::array::from_fn(|_| cx.new(|cx| PopoverState::new("Parent dialog", cx))),
        child: cx.new(|cx| PopoverState::new("Child dialog", cx)),
        selected_parent: Some(0),
    });
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.render_frame(cx);
    });
    (view, cx)
}

#[gpui_kit::test]
fn reassigned_child_only_dismisses_with_its_current_parent(cx: &mut TestAppContext) {
    let (view, cx) = reparenting_harness(cx);
    cx.update(|window, cx| {
        let parents = view.read(cx).parents.clone();
        let child = view.read(cx).child.clone();
        for state in parents.iter().chain(std::iter::once(&child)) {
            state.update(cx, |state, cx| state.set_open(true, window, cx));
        }
        window.render_frame(cx);
        view.update(cx, |view, cx| {
            view.selected_parent = Some(1);
            cx.notify();
        });
        window.render_frame(cx);
        parents[0].update(cx, |state, cx| state.dismiss(window, cx));
        window.render_frame(cx);
        assert!(child.read(cx).is_open());
        assert!(window.within("child").try_find("surface").is_some());
        parents[1].update(cx, |state, cx| state.dismiss(window, cx));
        window.render_frame(cx);
        assert!(!child.read(cx).is_open());
        assert!(window.within("child").try_find("surface").is_none());
    });
}

#[gpui_kit::test]
fn detached_child_stays_open_when_its_previous_parent_dismisses(cx: &mut TestAppContext) {
    let (view, cx) = reparenting_harness(cx);
    cx.update(|window, cx| {
        let parent = view.read(cx).parents[0].clone();
        let child = view.read(cx).child.clone();
        for state in [&parent, &child] {
            state.update(cx, |state, cx| state.set_open(true, window, cx));
        }
        window.render_frame(cx);
        view.update(cx, |view, cx| {
            view.selected_parent = None;
            cx.notify();
        });
        window.render_frame(cx);
        parent.update(cx, |state, cx| state.dismiss(window, cx));
        window.render_frame(cx);
        assert!(child.read(cx).is_open());
        assert!(window.within("child").try_find("surface").is_some());
    });
}

#[gpui_kit::test]
fn ancestry_cycle_is_rejected_before_mutation_or_dismissal_borrowing(cx: &mut TestAppContext) {
    let (view, cx) = reparenting_harness(cx);
    cx.update(|window, cx| {
        let parents = view.read(cx).parents.clone();
        let child = view.read(cx).child.clone();
        // Existing chain: child -> first parent -> second parent.
        let _ = Popover::new("association", &parents[0], "First")
            .parent(&PopoverClose {
                state: parents[1].downgrade(),
            })
            .render(window, cx);
        let rejection = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = Popover::new("cycle", &parents[1], "Second")
                .parent(&PopoverClose {
                    state: child.downgrade(),
                })
                .render(window, cx);
        }))
        .expect_err("a three-node ancestry cycle must be rejected");
        let message = rejection
            .downcast_ref::<String>()
            .map(String::as_str)
            .or_else(|| rejection.downcast_ref::<&str>().copied())
            .unwrap();
        assert!(message.contains("cannot create an ancestry cycle"));
        for state in parents.iter().chain(std::iter::once(&child)) {
            state.update(cx, |state, cx| state.set_open(true, window, cx));
        }
        // The rejected relation left the original tree intact; recursive close
        // succeeds without trying to borrow an ancestor that is already updating.
        parents[1].update(cx, |state, cx| state.dismiss(window, cx));
        for state in parents.iter().chain(std::iter::once(&child)) {
            assert!(!state.read(cx).is_open());
        }
        window.render_frame(cx);
        assert!(window.within("child").try_find("surface").is_none());
    });
}

#[gpui_kit::test]
fn clicking_nested_content_preserves_ancestors(cx: &mut TestAppContext) {
    struct Nested {
        parent: Entity<PopoverState>,
        child: Entity<PopoverState>,
        clicks: Rc<Cell<usize>>,
    }
    impl Render for Nested {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let child = self.child.clone();
            let clicks = self.clicks.clone();
            div()
                .p(px(32.))
                .child(Popover::new("parent", &self.parent, "Parent").content(
                    move |parent, _, _| {
                        let clicks = clicks.clone();
                        Popover::new("child", &child, "Child")
                            .parent(&parent)
                            .placement(Placement::Right)
                            .content(move |_, _, _| {
                                let clicks = clicks.clone();
                                div().ml(px(160.)).child(
                                    Button::new("nested-action", "Act")
                                        .on_click(move |_, _, _| clicks.set(clicks.get() + 1)),
                                )
                            })
                    },
                ))
        }
    }
    cx.update(|cx| {
        crate::init(cx);
        cx.set_reduce_motion(true);
    });
    let (view, cx) = cx.add_window_view(|_, cx| Nested {
        parent: cx.new(|cx| PopoverState::new("Parent dialog", cx)),
        child: cx.new(|cx| PopoverState::new("Child dialog", cx)),
        clicks: Rc::new(Cell::new(0)),
    });
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.render_frame(cx);
        window.within("parent").click("trigger", cx);
        window.within("child").click("trigger", cx);
        window.render_frame(cx);
        window.render_frame(cx);
        let parent_bounds = view
            .read(cx)
            .parent
            .read(cx)
            .resolved_position
            .get()
            .unwrap()
            .bounds;
        let target = window.within("child").find("nested-action").bounds();
        assert!(
            !parent_bounds.contains(&target.center()),
            "parent={parent_bounds:?} target={target:?} child={:?}",
            view.read(cx).child.read(cx).resolved_position.get()
        );
        window.within("child").click("nested-action", cx);
        assert!(view.read(cx).parent.read(cx).is_open());
        assert!(view.read(cx).child.read(cx).is_open());
        assert_eq!(view.read(cx).clicks.get(), 1);
        window.press("escape", cx);
        assert!(view.read(cx).parent.read(cx).is_open());
        assert!(!view.read(cx).child.read(cx).is_open());
    });
}

#[gpui_kit::test]
fn rename_open_popover_preserves_focus_and_disclosure(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    cx.update(|window, cx| {
        window.click("trigger", cx);
        window.render_frame(cx);
        window.click("inside", cx);
        let popup = view.read(cx).popup.clone();
        popup.update(cx, |popup, cx| popup.set_name("Paramètres du dossier", cx));
        window.render_frame(cx);
        assert_eq!(
            window.find("surface").label(),
            Some("Paramètres du dossier")
        );
        assert!(popup.read(cx).is_open());
        assert!(view.read(cx).inside.is_focused(window));
        window.press("escape", cx);
        window.render_frame(cx);
        assert!(!popup.read(cx).is_open());
    });
}
