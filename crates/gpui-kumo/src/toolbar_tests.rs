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
