use super::*;
use gpui_kit::{AppContext, TestAppContext, VisualTestContext, test::TestWindowExt};
struct Harness {
    toolbar: Entity<ToolbarState>,
    other: Entity<ToolbarState>,
    events: Vec<ToolbarEvent>,
    orientation: Orientation,
    looping: bool,
    width: f32,
    _subscription: gpui_kit::Subscription,
}
fn items() -> Vec<ToolbarItem> {
    vec![
        ToolbarItem::button("first", "Refresh"),
        ToolbarItem::button("paused", "Paused").disabled(true),
        ToolbarItem::button("skipped", "Skipped")
            .disabled(true)
            .focusable_when_disabled(false),
        ToolbarItem::button("saving", "Saving").loading(true),
        ToolbarItem::link("docs", "Documentation", "/docs"),
        ToolbarItem::button("last", "Deploy"),
    ]
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .tab_group()
            .flex()
            .flex_col()
            .w(px(self.width))
            .child(Button::new("before", "Before"))
            .child(
                Toolbar::new("primary", "Actions", &self.toolbar)
                    .orientation(self.orientation)
                    .loop_focus(self.looping),
            )
            .child(Button::new("after", "After"))
            .child(Toolbar::new("other", "Other actions", &self.other))
    }
}
fn harness(cx: &mut TestAppContext) -> (Entity<Harness>, &mut VisualTestContext) {
    cx.update(crate::init);
    cx.add_window_view(|_, cx| {
        let toolbar = cx.new(|cx| ToolbarState::new(items(), cx));
        let other = cx.new(|cx| ToolbarState::new(items(), cx));
        let subscription =
            cx.subscribe(&toolbar, |s: &mut Harness, _, event: &ToolbarEvent, cx| {
                s.events.push(event.clone());
                cx.notify();
            });
        Harness {
            toolbar,
            other,
            events: vec![],
            orientation: Orientation::Horizontal,
            looping: true,
            width: 900.,
            _subscription: subscription,
        }
    })
}
#[gpui_kit::test]
fn roving_entry_exit_and_unavailable_controls_use_base_activation(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.click("before", cx);
        window.focus_next(cx);
        window.render_frame(cx);
        let tabs = view.read(cx).toolbar.clone();
        assert!(tabs.read(cx).items[0].focus.is_focused(window));
        window.within("primary").click("first", cx);
        assert!(tabs.read(cx).items[0].focus.is_focused(window));
        window.press("space", cx);
        window.render_frame(cx);
        window.press("right", cx);
        window.render_frame(cx);
        assert!(tabs.read(cx).items[1].focus.is_focused(window));
        window.press("space", cx);
        window.press("enter", cx);
        window.render_frame(cx);
        window.press("right", cx);
        window.render_frame(cx);
        assert!(tabs.read(cx).items[3].focus.is_focused(window));
        window.press("enter", cx);
        window.press("right", cx);
        window.render_frame(cx);
        assert!(tabs.read(cx).items[4].focus.is_focused(window));
        window.press("enter", cx);
        window.render_frame(cx);
        window.focus_next(cx);
        window.render_frame(cx);
        assert_eq!(
            window.find("after").focused(),
            Some(true),
            "focused {:?}, own {:?}, other {:?}",
            window.focused(cx),
            view.read(cx)
                .toolbar
                .read(cx)
                .items
                .iter()
                .map(|i| i.focus.is_focused(window))
                .collect::<Vec<_>>(),
            view.read(cx)
                .other
                .read(cx)
                .items
                .iter()
                .map(|i| i.focus.is_focused(window))
                .collect::<Vec<_>>()
        );
        window.focus_prev(cx);
        window.render_frame(cx);
        assert!(tabs.read(cx).items[4].focus.is_focused(window));
        window.press("right", cx);
        window.render_frame(cx);
        window.press("right", cx);
        window.render_frame(cx);
        assert!(tabs.read(cx).items[0].focus.is_focused(window));
        window.press("home", cx);
        window.press("end", cx);
        assert!(tabs.read(cx).items[0].focus.is_focused(window));
    });
    cx.update(|_,cx|{
        let events=&view.read(cx).events;assert_eq!(events.len(),3);
        assert!(matches!(&events[0],ToolbarEvent::Activate{id,..} if id==&ElementId::from("first")));
        assert!(matches!(&events[2],ToolbarEvent::Navigate{request,..} if request.href.as_ref()=="/docs"));
    });
}
#[gpui_kit::test]
fn owner_changes_cancel_stale_activation_and_preserve_or_recover_focus(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    cx.update(|window, cx| {
        window.render_frame(cx);
        let state = view.read(cx).toolbar.clone();
        window.within("primary").click("first", cx);
        state.update(cx, |s, cx| {
            s.set_items(
                vec![
                    ToolbarItem::button("first", "Changed")
                        .disabled(true)
                        .focusable_when_disabled(false),
                    ToolbarItem::link("docs", "New destination", "/new"),
                ],
                window,
                cx,
            )
        });
        window.within("primary").click("first", cx);
        window.render_frame(cx);
        assert!(state.read(cx).items[1].focus.is_focused(window));
        window.press("enter", cx);
        window.render_frame(cx);
        state.update(cx, |s, cx| {
            s.set_items(
                vec![
                    ToolbarItem::link("docs", "New destination", "/new"),
                    ToolbarItem::button("first", "Changed"),
                ],
                window,
                cx,
            )
        });
        window.render_frame(cx);
        assert!(state.read(cx).items[0].focus.is_focused(window));
        state.update(cx, |s, cx| s.set_items(vec![], window, cx));
        window.render_frame(cx);
        assert_eq!(
            window.find("after").focused(),
            Some(true),
            "focused {:?}, own {:?}, other {:?}",
            window.focused(cx),
            view.read(cx)
                .toolbar
                .read(cx)
                .items
                .iter()
                .map(|i| i.focus.is_focused(window))
                .collect::<Vec<_>>(),
            view.read(cx)
                .other
                .read(cx)
                .items
                .iter()
                .map(|i| i.focus.is_focused(window))
                .collect::<Vec<_>>()
        );
        assert!(view.read(cx).other.read(cx).active.is_none());
    });
    cx.update(|_, cx| {
        let events = &view.read(cx).events;
        assert_eq!(events.len(), 2);
        assert!(
            matches!(&events[1],ToolbarEvent::Navigate{request,..} if request.href.as_ref()=="/new")
        );
    });
}
#[gpui_kit::test]
fn orientation_looping_group_availability_and_narrow_reveal(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    cx.update(|window, cx| {
        view.update(cx, |s, cx| {
            s.orientation = Orientation::Vertical;
            s.looping = false;
            s.width = 220.;
            cx.notify();
        });
        window.render_frame(cx);
        window.within("primary").click("first", cx);
        let state = view.read(cx).toolbar.clone();
        window.press("right", cx);
        assert!(state.read(cx).items[0].focus.is_focused(window));
        for _ in 0..4 {
            window.press("down", cx);
            window.render_frame(cx);
        }
        assert!(
            state.read(cx).items[5].focus.is_focused(window),
            "orientation {:?}, focus {:?}, offset {:?}, own {:?}",
            state.read(cx).orientation,
            window.focused(cx),
            state.read(cx).scroll.offset(),
            state
                .read(cx)
                .items
                .iter()
                .map(|i| i.focus.is_focused(window))
                .collect::<Vec<_>>()
        );
        window.press("down", cx);
        window.render_frame(cx);
        assert!(
            state.read(cx).items[5].focus.is_focused(window),
            "orientation {:?}, focus {:?}, offset {:?}, own {:?}",
            state.read(cx).orientation,
            window.focused(cx),
            state.read(cx).scroll.offset(),
            state
                .read(cx)
                .items
                .iter()
                .map(|i| i.focus.is_focused(window))
                .collect::<Vec<_>>()
        );
        assert!(state.read(cx).scroll.offset().x < px(0.));
        let viewport = state.read(cx).scroll.bounds();
        let last = window.within("primary").find("last").bounds();
        assert!(last.left() >= viewport.left() && last.right() <= viewport.right());
        state.update(cx, |s, cx| s.set_disabled(true, window, cx));
        window.press("enter", cx);
        window.render_frame(cx);
        window.press("up", cx);
        window.render_frame(cx);
        assert!(state.read(cx).items[4].focus.is_focused(window));
        window.press("enter", cx);
        window.render_frame(cx);
    });
    cx.update(|_, cx| {
        assert_eq!(view.read(cx).events.len(), 2);
        assert!(matches!(
            &view.read(cx).events[1],
            ToolbarEvent::Navigate { .. }
        ));
    });
}

#[gpui_kit::test]
fn link_uses_current_target_and_retained_focus_with_actual_pointer_modifiers(
    cx: &mut TestAppContext,
) {
    let (view, cx) = harness(cx);
    let point = cx.update(|window, cx| {
        window.render_frame(cx);
        let state = view.read(cx).toolbar.read(cx);
        state.items[4]
            .bounds
            .get()
            .expect("rendered link geometry")
            .center()
            + state.scroll.offset()
    });
    let modifiers = gpui_kit::Modifiers {
        control: true,
        ..Default::default()
    };
    cx.simulate_mouse_down(point, gpui_kit::MouseButton::Left, modifiers);
    cx.update(|window, cx| {
        let state = view.read(cx).toolbar.clone();
        assert!(
            state.read(cx).items[4].focus.is_focused(window),
            "Base's actual pointer focus matches the retained handle"
        );
        state.update(cx, |s, cx| {
            let mut updated = items();
            updated[4] = ToolbarItem::link("docs", "Documentation", "/changed");
            s.set_items(updated, window, cx)
        });
    });
    cx.simulate_mouse_up(point, gpui_kit::MouseButton::Left, modifiers);
    cx.update(|window,cx|{
        let state=view.read(cx).toolbar.clone();
        assert!(state.read(cx).items[4].focus.is_focused(window));
        let events=&view.read(cx).events;
        assert_eq!(events.len(),1);
        assert!(matches!(&events[0],ToolbarEvent::Navigate{request,..} if request.href.as_ref()=="/changed" && request.activation.modifiers().control));
    });
}

#[gpui_kit::test]
fn group_availability_preserves_source_ghost_hover_and_original_disabled_opacity(
    cx: &mut TestAppContext,
) {
    let (view, cx) = harness(cx);
    cx.update(|window, cx| {
        window.render_frame(cx);
        view.read(cx)
            .toolbar
            .clone()
            .update(cx, |s, cx| s.set_disabled(true, window, cx));
        window.render_frame(cx);
        window.within("primary").hover("first", cx);
        window.render_frame(cx);
        let bounds = window
            .within("primary")
            .find("first")
            .bounds()
            .scale(window.scale_factor());
        let tint = crate::theme(cx).colors.tint;
        assert!(
            window
                .painted_quads()
                .iter()
                .any(|q| q.bounds == bounds && q.background.as_solid() == Some(tint)),
            "source group disabled gates activation while retaining original ghost hover paint"
        );
        window.within("primary").hover("paused", cx);
        window.render_frame(cx);
        let bounds = window
            .within("primary")
            .find("paused")
            .bounds()
            .scale(window.scale_factor());
        assert!(
            window
                .painted_quads()
                .iter()
                .any(|q| q.bounds == bounds && q.background.as_solid() == Some(tint.opacity(0.5))),
            "only original disabled props dim the hovered control"
        );
    });
}

struct Editors {
    toolbar: Entity<ToolbarState>,
    inputs: Vec<Entity<InputState>>,
    vertical: bool,
    standalone: bool,
    events: Vec<InputEvent>,
    _subscriptions: Vec<gpui_kit::Subscription>,
}
fn editor_items(inputs: &[Entity<InputState>]) -> Vec<ToolbarItem> {
    vec![
        ToolbarItem::button("first", "Before"),
        ToolbarItem::input("query", &inputs[0], px(180.)),
        ToolbarItem::input("paused", &inputs[1], px(130.)).disabled(true),
        ToolbarItem::input("skipped", &inputs[2], px(130.))
            .disabled(true)
            .focusable_when_disabled(false),
        ToolbarItem::input_group("filter", &inputs[3], px(220.))
            .start(InputGroupAddon::text("Filter"))
            .end(InputGroupAddon::text("units"))
            .suffix("ms"),
        ToolbarItem::button("last", "After"),
    ]
}
impl Render for Editors {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .tab_group()
            .flex()
            .flex_col()
            .w(px(300.))
            .child(Button::new("outside-before", "Outside before"))
            .child(
                Toolbar::new("editors", "Editors", &self.toolbar).orientation(if self.vertical {
                    Orientation::Vertical
                } else {
                    Orientation::Horizontal
                }),
            )
            .when(self.standalone, |v| {
                v.child(Input::new("standalone", &self.inputs[0]))
            })
            .child(Button::new("outside-after", "Outside after"))
    }
}
fn editors(cx: &mut TestAppContext) -> (Entity<Editors>, &mut VisualTestContext) {
    cx.update(crate::init);
    cx.add_window_view(|window, cx| {
        let inputs: Vec<_> = [
            ("Query", "café 🦀"),
            ("Paused query", "locked"),
            ("Skipped query", "skip"),
            ("Filter query", "status"),
        ]
        .into_iter()
        .map(|(name, value)| {
            cx.new(|cx| {
                let mut s = InputState::new(name, window, cx);
                s.set_value(value, window, cx);
                s
            })
        })
        .collect();
        let toolbar = cx.new(|cx| ToolbarState::new(editor_items(&inputs), cx));
        let subscription =
            cx.subscribe(&inputs[0], |s: &mut Editors, _, event: &InputEvent, cx| {
                s.events.push(event.clone());
                cx.notify();
            });
        Editors {
            toolbar,
            inputs,
            vertical: false,
            standalone: false,
            events: vec![],
            _subscriptions: vec![subscription],
        }
    })
}
fn focused_editor(view: &Entity<Editors>, index: usize, window: &Window, cx: &App) -> bool {
    view.read(cx).inputs[index]
        .read(cx)
        .focus_handle(cx)
        .is_focused(window)
}
#[gpui_kit::test]
fn editor_arrows_preserve_unicode_selection_and_exit_only_at_text_edges(cx: &mut TestAppContext) {
    let (view, cx) = editors(cx);
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.click("outside-before", cx);
        window.focus_next(cx);
        window.render_frame(cx);
        window.press("right", cx);
        window.render_frame(cx);
        assert!(focused_editor(&view, 0, window, cx));
        assert_eq!(
            window.within("query").find("control").role(),
            Some(gpui_kit::Role::TextInput)
        );
        window.press("home", cx);
        window.press("right", cx);
        window.render_frame(cx);
        assert!(
            focused_editor(&view, 0, window, cx),
            "inside text, Right edits the caret"
        );
        window.press("end", cx);
        window.press("shift-left", cx);
        window.render_frame(cx);
        assert_eq!(
            view.read(cx).inputs[0].read(cx).selected_value(cx).as_ref(),
            "🦀"
        );
        window.press("right", cx);
        window.render_frame(cx);
        assert!(
            focused_editor(&view, 0, window, cx),
            "a selected range first collapses in the editor"
        );
        window.press("right", cx);
        window.render_frame(cx);
        assert!(focused_editor(&view, 1, window, cx));
        window.press("backspace", cx);
        window.press("enter", cx);
        window.render_frame(cx);
        assert_eq!(
            view.read(cx).inputs[1].read(cx).value(cx).as_ref(),
            "locked"
        );
        window.press("right", cx);
        window.render_frame(cx);
        assert!(
            focused_editor(&view, 3, window, cx),
            "skip unavailable editor with focusableWhenDisabled=false"
        );
        window.press("home", cx);
        window.press("left", cx);
        window.render_frame(cx);
        assert!(focused_editor(&view, 1, window, cx));
        window.press("left", cx);
        window.render_frame(cx);
        assert!(
            focused_editor(&view, 0, window, cx),
            "focus {:?}, active {:?}",
            view.read(cx)
                .toolbar
                .read(cx)
                .items
                .iter()
                .map(|i| (&i.control.id, i.focus.is_focused(window)))
                .collect::<Vec<_>>(),
            view.read(cx).toolbar.read(cx).active
        );
        window.press("home", cx);
        window.press("left", cx);
        window.render_frame(cx);
        assert!(
            view.read(cx).toolbar.read(cx).items[0]
                .focus
                .is_focused(window)
        );
        window.press("right", cx);
        window.render_frame(cx);
        window.focus_next(cx);
        window.render_frame(cx);
        assert_eq!(window.find("outside-after").focused(), Some(true));
        window.focus_prev(cx);
        window.render_frame(cx);
        assert!(
            focused_editor(&view, 0, window, cx),
            "last editor is the only reentry tab stop"
        );
    });
    cx.simulate_input("!");
    cx.update(|window, cx| {
        assert!(view.read(cx).inputs[0].read(cx).value(cx).contains('!'));
        window.press("enter", cx);
    });
    cx.update(|_, cx| {
        assert_eq!(
            view.read(cx)
                .events
                .iter()
                .filter(|e| matches!(e, InputEvent::Change))
                .count(),
            1
        );
        assert_eq!(
            view.read(cx)
                .events
                .iter()
                .filter(|e| matches!(e, InputEvent::Submit { .. }))
                .count(),
            1
        );
    });
}
#[gpui_kit::test]
fn editor_owner_availability_recovery_and_group_disabled_edit_guard_are_current(
    cx: &mut TestAppContext,
) {
    let (view, cx) = editors(cx);
    cx.update(|window, cx| {
        window.render_frame(cx);
        let state = view.read(cx).toolbar.clone();
        state.update(cx, |s, cx| {
            let mut items = editor_items(&view.read(cx).inputs);
            items[1] = items[1].clone().focusable_when_disabled(false);
            s.set_items(items, window, cx);
            s.focus_item(&"query".into(), window, cx);
        });
        window.render_frame(cx);
        view.read(cx).inputs[0]
            .clone()
            .update(cx, |s, cx| s.set_disabled(true, cx));
        window.render_frame(cx);
        assert!(
            state.read(cx).items[0].focus.is_focused(window),
            "owner availability is observed and recovers skipped focus"
        );
        view.read(cx).inputs[0]
            .clone()
            .update(cx, |s, cx| s.set_disabled(false, cx));
        window.render_frame(cx);
        state.update(cx, |s, cx| {
            s.focus_item(&"filter".into(), window, cx);
            s.set_disabled(true, window, cx);
        });
        window.render_frame(cx);
        assert!(
            focused_editor(&view, 3, window, cx),
            "disabled default-focusable editor keeps focus"
        );
        assert!(
            !view.read(cx).inputs[3].read(cx).is_disabled(),
            "group availability does not mutate the editor owner's disabled prop"
        );
        assert_eq!(
            window.within("filter").find("surface").bounds().size.height,
            px(36.)
        );
        window.press("shift-left", cx);
        #[cfg(target_os = "macos")]
        let select_all = "cmd-a";
        #[cfg(not(target_os = "macos"))]
        let select_all = "ctrl-a";
        window.press(select_all, cx);
        assert_eq!(
            view.read(cx).inputs[3].read(cx).selected_value(cx).as_ref(),
            ""
        );
        window.press("backspace", cx);
        window.press("enter", cx);
    });
    cx.simulate_input("blocked");
    cx.update(|window, cx| {
        assert_eq!(
            view.read(cx).inputs[3].read(cx).value(cx).as_ref(),
            "status"
        );
        let state = view.read(cx).toolbar.clone();
        state.update(cx, |s, cx| s.set_disabled(false, window, cx));
        window.render_frame(cx);
    });
    cx.simulate_input("!");
    cx.update(|_, cx| {
        assert_eq!(
            view.read(cx).inputs[3].read(cx).value(cx).as_ref(),
            "status!"
        )
    });
}
#[gpui_kit::test]
fn grouped_editor_reorder_removal_vertical_boundary_and_standalone_remount_preserve_state(
    cx: &mut TestAppContext,
) {
    let (view, cx) = editors(cx);
    cx.update(|window, cx| {
        window.render_frame(cx);
        let state = view.read(cx).toolbar.clone();
        state.update(cx, |s, cx| s.focus_item(&"filter".into(), window, cx));
        window.render_frame(cx);
        let bounds = window.within("filter").find("surface").bounds();
        let viewport = state.read(cx).scroll.bounds();
        assert!(
            bounds.left() >= viewport.left() && bounds.right() <= viewport.right(),
            "actual grouped editor is revealed in narrow viewport"
        );
        window.press("home", cx);
        window.press("shift-right", cx);
        window.render_frame(cx);
        let selected = view.read(cx).inputs[3].read(cx).selected_value(cx);
        state.update(cx, |s, cx| {
            let mut items = editor_items(&view.read(cx).inputs);
            items.rotate_right(2);
            s.set_items(items, window, cx);
        });
        window.render_frame(cx);
        assert!(focused_editor(&view, 3, window, cx));
        assert_eq!(
            view.read(cx).inputs[3].read(cx).selected_value(cx),
            selected
        );
        view.update(cx, |s, cx| {
            s.vertical = true;
            cx.notify();
        });
        window.render_frame(cx);
        window.press("down", cx);
        window.render_frame(cx);
        assert!(
            focused_editor(&view, 3, window, cx),
            "selection stays in editor even on navigation axis"
        );
        window.press("end", cx);
        window.press("down", cx);
        window.render_frame(cx);
        assert!(
            !focused_editor(&view, 3, window, cx),
            "vertical navigation exits at the text end"
        );
        state.update(cx, |s, cx| {
            s.set_disabled(true, window, cx);
            s.focus_item(&"query".into(), window, cx);
            s.set_items(vec![], window, cx);
        });
        view.update(cx, |s, cx| {
            s.standalone = true;
            cx.notify();
        });
        window.render_frame(cx);
        let input = view.read(cx).inputs[0].clone();
        input.read(cx).focus_handle(cx).focus(window, cx);
        assert!(!input.read(cx).is_disabled());
    });
    cx.simulate_input("restored");
    cx.update(|_, cx| {
        assert!(
            view.read(cx).inputs[0]
                .read(cx)
                .value(cx)
                .contains("restored")
        )
    });
}

#[gpui_kit::test]
fn toolbar_disabled_clipboard_guard_reads_current_owner_before_rerender(cx: &mut TestAppContext) {
    let (view, cx) = editors(cx);
    cx.update(|window, cx| {
        window.render_frame(cx);
        let toolbar = view.read(cx).toolbar.clone();
        let input = view.read(cx).inputs[3].clone();
        toolbar.update(cx, |s, cx| s.focus_item(&"filter".into(), window, cx));
        input.update(cx, |s, cx| s.set_read_only(true, cx));
        window.render_frame(cx);
        #[cfg(target_os = "macos")]
        let (select, copy) = ("cmd-a", "cmd-c");
        #[cfg(not(target_os = "macos"))]
        let (select, copy) = ("ctrl-a", "ctrl-c");
        window.press(select, cx);
        assert_eq!(input.read(cx).selected_value(cx).as_ref(), "status");
        window.press(copy, cx);
        assert_eq!(
            cx.read_from_clipboard().unwrap().text().unwrap(),
            "status",
            "available read-only editors can copy"
        );
        cx.write_to_clipboard(gpui_kit::ClipboardItem::new_string("sentinel".into()));
        toolbar.update(cx, |s, cx| s.set_disabled(true, window, cx));
        // Deliberately keep the preceding rendered action callback alive.
        window.press(copy, cx);
        assert_eq!(
            cx.read_from_clipboard().unwrap().text().unwrap(),
            "sentinel",
            "current unavailable policy rejects the old copy callback"
        );
        assert_eq!(input.read(cx).selected_value(cx).as_ref(), "status");
        assert!(input.read(cx).is_read_only());
        assert!(!input.read(cx).is_disabled());
        toolbar.update(cx, |s, cx| s.set_disabled(false, window, cx));
        window.render_frame(cx);
        window.press(copy, cx);
        assert_eq!(cx.read_from_clipboard().unwrap().text().unwrap(), "status");
    });
}
