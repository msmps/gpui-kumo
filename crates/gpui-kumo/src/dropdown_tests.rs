use super::*;
use gpui_kit::{AppContext, TestAppContext, VisualTestContext, test::TestWindowExt};
struct Harness {
    menu: Entity<DropdownState>,
    events: Vec<DropdownEvent>,
    _subscription: Subscription,
    mounted: bool,
    outside: usize,
}
fn parts() -> Vec<DropdownPart> {
    vec![
        DropdownPart::Label("Actions".into()),
        DropdownItem::new("save", "Save").into(),
        DropdownItem::new("paused", "Paused").disabled(true).into(),
        DropdownItem::new("draft", "Save draft").into(),
        DropdownPart::Separator,
        DropdownItem::new("delete", "Delete")
            .variant(DropdownVariant::Danger)
            .into(),
    ]
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .tab_group()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                Button::new("before", "Before").on_click(cx.listener(|s, _, _, cx| {
                    s.outside += 1;
                    cx.notify();
                })),
            )
            .when(self.mounted, |root| {
                root.child(Dropdown::new("actions", &self.menu, "Save options"))
            })
            .child(Button::new("after", "After"))
    }
}
fn harness(cx: &mut TestAppContext) -> (Entity<Harness>, &mut VisualTestContext) {
    cx.update(crate::init);
    cx.add_window_view(|_, cx| {
        let menu = cx.new(|cx| DropdownState::new("Save actions", parts(), cx));
        let subscription = cx.subscribe(&menu, |s: &mut Harness, _, e: &DropdownEvent, cx| {
            s.events.push(e.clone());
            cx.notify();
        });
        Harness {
            menu,
            events: vec![],
            _subscription: subscription,
            mounted: true,
            outside: 0,
        }
    })
}

#[gpui_kit::test]
fn menu_unmount_releases_popup_with_retained_state_and_remounts_closed(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.render_frame(cx);
        let menu = view.read(cx).menu.clone();
        menu.read(cx).trigger.clone().focus(window, cx);
        window.press("enter", cx);
        window.render_frame(cx);
        assert!(base::GlobalState::is_in_deferred_context(cx));
        view.update(cx, |s, cx| {
            s.mounted = false;
            cx.notify();
        });
        window.render_frame(cx);
        window.render_frame(cx);
    });
    cx.run_until_parked();
    cx.update(|window, cx| {
        let menu = view.read(cx).menu.clone();
        assert!(!menu.read(cx).is_open());
        assert!(!base::GlobalState::is_in_deferred_context(cx));
        view.update(cx, |s, cx| {
            s.mounted = true;
            cx.notify();
        });
        window.render_frame(cx);
        window.render_frame(cx);
        window.click("trigger", cx);
        window.render_frame(cx);
        assert!(menu.read(cx).is_open());
    });
}
#[gpui_kit::test]
fn menu_keyboard_entry_disabled_focus_wrap_activation_and_escape(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.render_frame(cx);
        let menu = view.read(cx).menu.clone();
        menu.read(cx).trigger.clone().focus(window, cx);
        window.press("down", cx);
        window.render_frame(cx);
        assert!(menu.read(cx).is_open());
        assert!(menu.read(cx).handles["save"].is_focused(window));
        window.press("down", cx);
        window.render_frame(cx);
        assert!(menu.read(cx).handles["paused"].is_focused(window));
        window.press("enter", cx);
        window.render_frame(cx);
        assert!(menu.read(cx).is_open());
        window.press("down", cx);
        window.render_frame(cx);
        assert!(menu.read(cx).handles["draft"].is_focused(window));
        window.press("end", cx);
        window.render_frame(cx);
        assert!(menu.read(cx).handles["delete"].is_focused(window));
        window.press("down", cx);
        window.render_frame(cx);
        assert!(menu.read(cx).handles["save"].is_focused(window));
        window.press("space", cx);
        window.render_frame(cx);
        assert!(!menu.read(cx).is_open());
        assert!(menu.read(cx).trigger.is_focused(window));
        window.press("up", cx);
        window.render_frame(cx);
        assert!(menu.read(cx).handles["delete"].is_focused(window));
        window.press("escape", cx);
        window.render_frame(cx);
        assert!(!menu.read(cx).is_open());
        assert!(menu.read(cx).trigger.is_focused(window));
    });
    cx.read(|cx| {
        assert_eq!(
            view.read(cx)
                .events
                .iter()
                .filter(|e| matches!(e, DropdownEvent::Activated(_)))
                .count(),
            1
        )
    });
}
#[gpui_kit::test]
fn menu_pointer_disabled_current_parts_and_focus_restoration(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.render_frame(cx);
        window.click("trigger", cx);
        window.render_frame(cx);
        window.render_frame(cx);
        let menu = view.read(cx).menu.clone();
        assert!(menu.read(cx).is_open());
        window.click("paused", cx);
        window.render_frame(cx);
        assert!(menu.read(cx).is_open());
        assert!(
            !view
                .read(cx)
                .events
                .iter()
                .any(|e| matches!(e, DropdownEvent::Activated(_)))
        );
        let draft = menu.read(cx).handles["draft"].clone();
        draft.focus(window, cx);
        menu.update(cx, |s, cx| {
            s.set_parts(
                vec![
                    DropdownItem::new("draft", "Renamed draft").into(),
                    DropdownItem::new("save", "Save").into(),
                ],
                window,
                cx,
            )
        });
        window.render_frame(cx);
        assert!(draft.is_focused(window));
        menu.update(cx, |s, cx| {
            s.set_parts(vec![DropdownItem::new("save", "Save").into()], window, cx)
        });
        window.render_frame(cx);
        assert!(menu.read(cx).handles["save"].is_focused(window));
        window.click("save", cx);
        window.render_frame(cx);
        assert!(menu.read(cx).trigger.is_focused(window));
        window.click("trigger", cx);
        window.render_frame(cx);
        window.render_frame(cx);
        window.click("before", cx);
        window.render_frame(cx);
        assert!(!menu.read(cx).is_open());
        assert_eq!(view.read(cx).outside, 0);
    });
    cx.read(|cx| {
        assert_eq!(
            view.read(cx)
                .events
                .iter()
                .filter(|e| matches!(e, DropdownEvent::Activated(_)))
                .count(),
            1
        )
    });
}
#[gpui_kit::test]
fn menu_typeahead_and_tab_exit_do_not_activate(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.render_frame(cx);
        let menu = view.read(cx).menu.clone();
        menu.read(cx).trigger.clone().focus(window, cx);
        window.press("enter", cx);
        window.render_frame(cx);
        window.press("d", cx);
        window.render_frame(cx);
        assert!(menu.read(cx).handles["delete"].is_focused(window));
        window.press("tab", cx);
        window.render_frame(cx);
        assert!(!menu.read(cx).is_open());
        assert_eq!(window.find("after").focused(), Some(true));
        assert!(
            !view
                .read(cx)
                .events
                .iter()
                .any(|e| matches!(e, DropdownEvent::Activated(_)))
        );
        menu.read(cx).trigger.clone().focus(window, cx);
        window.press("enter", cx);
        window.render_frame(cx);
        window.press("shift-tab", cx);
        window.render_frame(cx);
        assert!(menu.read(cx).trigger.is_focused(window));
        menu.update(cx, |s, cx| s.set_disabled(true, window, cx));
        menu.read(cx).trigger.clone().focus(window, cx);
        window.press("enter", cx);
        window.render_frame(cx);
        assert!(!menu.read(cx).is_open());
    });
}

#[gpui_kit::test]
fn menu_word_typeahead_preserves_space_and_activates_after_timeout(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.render_frame(cx);
        let menu = view.read(cx).menu.clone();
        menu.read(cx).trigger.clone().focus(window, cx);
        window.press("enter", cx);
        window.render_frame(cx);
        window.press("s", cx);
        window.press("a", cx);
        window.press("v", cx);
        window.press("e", cx);
        window.press("space", cx);
        window.press("d", cx);
        window.render_frame(cx);
        assert!(menu.read(cx).is_open());
        assert!(menu.read(cx).handles["draft"].is_focused(window));
    });
    cx.executor().advance_clock(Duration::from_millis(600));
    cx.update(|window, cx| {
        window.press("space", cx);
        window.render_frame(cx);
        assert!(!view.read(cx).menu.read(cx).is_open());
    });
    cx.read(|cx| {
        assert_eq!(
            view.read(cx)
                .events
                .iter()
                .filter(|e| matches!(e, DropdownEvent::Activated(_)))
                .count(),
            1
        )
    });
}

#[gpui_kit::test]
fn menu_long_list_reveals_focused_rows_and_keeps_source_row_height(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.render_frame(cx);
        let menu = view.read(cx).menu.clone();
        menu.update(cx, |s, cx| {
            s.set_parts(
                (0..50)
                    .map(|i| DropdownItem::new(format!("item-{i}"), format!("Item {i}")).into())
                    .collect(),
                window,
                cx,
            )
        });
        menu.read(cx).trigger.clone().focus(window, cx);
        window.press("down", cx);
        window.render_frame(cx);
        window.render_frame(cx);
        window.press("end", cx);
        window.render_frame(cx);
        window.render_frame(cx);
        assert!(menu.read(cx).handles["item-49"].is_focused(window));
        let surface = window.find("menu").bounds();
        let last = window.find("item-49").bounds();
        assert_eq!(last.size.height, px(33.));
        assert!(last.bottom() <= surface.bottom());
        assert!(last.top() >= surface.top());
        window.press("home", cx);
        window.render_frame(cx);
        window.render_frame(cx);
        let first = window.find("item-0").bounds();
        assert!(first.top() >= surface.top());
    });
}
