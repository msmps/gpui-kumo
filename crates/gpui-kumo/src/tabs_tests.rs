use super::*;
use gpui_kit::{AppContext, TestAppContext, test::TestWindowExt};
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
            .w(px(280.))
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
        assert_eq!(window.find("after").focused(), Some(true));
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
        assert_eq!(window.find("after").focused(), Some(true));
    });
}
