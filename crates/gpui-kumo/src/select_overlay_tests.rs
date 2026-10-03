use super::*;
use crate::{Popover, PopoverState};
use gpui_kit::{
    AppContext, Modifiers, TestAppContext, VisualTestContext, size, test::TestWindowExt,
};

struct Host {
    parent: Option<Entity<PopoverState>>,
    select: Entity<SelectState<u32>>,
    outside: FocusHandle,
    associate: bool,
    show_select: bool,
}
impl Render for Host {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let select = self.select.clone();
        let associate = self.associate;
        let show_select = self.show_select;
        div()
            .p(px(24.))
            .flex()
            .flex_col()
            .gap(px(24.))
            .children(self.parent.as_ref().map(|parent| {
                Popover::new("host", parent, "Region settings")
                    .width(px(260.))
                    .content(move |parent, _, _| {
                        div().children(show_select.then(|| {
                            Select::new("nested", &select).when(associate, |s| s.parent(&parent))
                        }))
                    })
            }))
            .child(
                div()
                    .ml(px(420.))
                    .child(crate::Button::new("outside", "Outside").track_focus(&self.outside)),
            )
    }
}
fn host(cx: &mut TestAppContext) -> (Entity<Host>, &mut VisualTestContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|_, cx| Host {
        parent: Some(cx.new(|cx| PopoverState::new("Region settings dialog", cx))),
        select: cx.new(|cx| {
            SelectState::new(
                "Nested region",
                SelectValue::Single(Some(0)),
                (0..9)
                    .map(|i| SelectOption::new(format!("nested-{i}"), i, format!("Region {i}")))
                    .collect(),
                cx,
            )
        }),
        outside: cx.focus_handle(),
        associate: true,
        show_select: true,
    });
    cx.update(|window, _| window.activate_window());
    cx.run_until_parked();
    cx.simulate_resize(size(px(600.), px(480.)));
    cx.update(|window, cx| {
        let parent = view.read(cx).parent.as_ref().unwrap().clone();
        parent.read(cx).trigger_focus().focus(window, cx);
        parent.update(cx, |p, cx| p.set_open(true, window, cx));
        for _ in 0..3 {
            window.render_frame(cx);
        }
        let trigger = view.read(cx).select.read(cx).trigger.clone();
        trigger.focus(window, cx);
        window.press("enter", cx);
        for _ in 0..3 {
            window.render_frame(cx);
        }
    });
    cx.run_until_parked();
    (view, cx)
}
#[gpui_kit::test]
fn select_option_outside_parent_keeps_parent_open(cx: &mut TestAppContext) {
    let (view, cx) = host(cx);
    let (parent, select) = view.read_with(cx, |v, _| {
        (v.parent.as_ref().unwrap().clone(), v.select.clone())
    });
    let point = cx.update(|window, _| {
        let parent_surface = base::test_support::snapshots(window)
            .into_iter()
            .find(|node| node.role() == Some(Role::Dialog))
            .unwrap()
            .bounds();
        let row = window.find("nested-8").bounds();
        assert!(row.top() > parent_surface.bottom());
        row.center()
    });
    cx.simulate_click(point, Modifiers::default());
    cx.run_until_parked();
    assert!(
        parent.read_with(cx, |p, _| p.is_open()),
        "Selecting beyond the parent surface must not dismiss the parent"
    );
    assert_eq!(
        select.read_with(cx, |s, _| s.value().clone()),
        SelectValue::Single(Some(8))
    );
    assert!(!select.read_with(cx, |s, _| s.is_open()));
    cx.update(|window, cx| assert!(select.read(cx).trigger.is_focused(window)));
}
#[gpui_kit::test]
fn select_nested_escape_parent_close_and_outside_focus(cx: &mut TestAppContext) {
    let (view, cx) = host(cx);
    let (parent, select, outside) = view.read_with(cx, |v, _| {
        (
            v.parent.as_ref().unwrap().clone(),
            v.select.clone(),
            v.outside.clone(),
        )
    });
    cx.update(|window, cx| {
        window.press("escape", cx);
        for _ in 0..2 {
            window.render_frame(cx);
        }
        assert!(!select.read(cx).is_open());
        assert!(parent.read(cx).is_open());
        assert!(select.read(cx).trigger.is_focused(window));
        window.press("escape", cx);
        for _ in 0..2 {
            window.render_frame(cx);
        }
        assert!(!parent.read(cx).is_open());
        assert!(parent.read(cx).trigger_focus().is_focused(window));
        assert!(!base::GlobalState::is_in_deferred_context(cx));
        parent.update(cx, |p, cx| p.set_open(true, window, cx));
        for _ in 0..2 {
            window.render_frame(cx);
        }
        select.update(cx, |s, cx| s.set_open(true, window, cx));
        for _ in 0..2 {
            window.render_frame(cx);
        }
        parent.update(cx, |p, cx| p.dismiss(window, cx));
        for _ in 0..2 {
            window.render_frame(cx);
        }
        assert!(!select.read(cx).is_open());
        assert!(parent.read(cx).trigger_focus().is_focused(window));
        assert!(!base::GlobalState::is_in_deferred_context(cx));
        select.update(cx, |s, cx| s.set_open(true, window, cx));
        assert!(
            !select.read(cx).is_open(),
            "A child of a closed parent cannot open"
        );
        parent.update(cx, |p, cx| p.set_open(true, window, cx));
        for _ in 0..2 {
            window.render_frame(cx);
        }
        select.update(cx, |s, cx| s.set_open(true, window, cx));
        for _ in 0..2 {
            window.render_frame(cx);
        }
    });
    cx.run_until_parked();
    let point = cx.update(|window, _| window.find("outside").bounds().center());
    cx.simulate_click(point, Modifiers::default());
    cx.run_until_parked();
    assert!(!parent.read_with(cx, |p, _| p.is_open()));
    assert!(!select.read_with(cx, |s, _| s.is_open()));
    cx.update(|window, cx| {
        assert!(outside.is_focused(window));
        assert!(!base::GlobalState::is_in_deferred_context(cx));
    });
}
#[gpui_kit::test]
fn select_parent_release_cleans_retained_child_without_outside_focus_theft(
    cx: &mut TestAppContext,
) {
    let (view, cx) = host(cx);
    let (parent, select, outside) = view.read_with(cx, |v, _| {
        (
            v.parent.as_ref().unwrap().downgrade(),
            v.select.clone(),
            v.outside.clone(),
        )
    });
    cx.update(|window, cx| {
        outside.focus(window, cx);
        // Owner removal must clean registration even before the deferred focus-out callback.
        view.update(cx, |v, cx| {
            v.parent = None;
            cx.notify();
        });
        for _ in 0..3 {
            window.render_frame(cx);
        }
    });
    cx.run_until_parked();
    assert!(parent.upgrade().is_none());
    assert!(!select.read_with(cx, |s, _| s.is_open()));
    cx.update(|window, cx| {
        assert!(outside.is_focused(window));
        assert!(!base::GlobalState::is_in_deferred_context(cx));
        select.update(cx, |s, cx| s.set_open(true, window, cx));
        assert!(!select.read(cx).is_open());
    });
}
#[gpui_kit::test]
fn select_focused_parent_release_cleans_retained_child(cx: &mut TestAppContext) {
    let (view, cx) = host(cx);
    let (parent, select) = view.read_with(cx, |v, _| {
        (v.parent.as_ref().unwrap().downgrade(), v.select.clone())
    });
    cx.update(|window, cx| assert!(select.read(cx).content.is_focused(window)));
    view.update(cx, |v, cx| {
        v.parent = None;
        cx.notify();
    });
    cx.update(|window, cx| {
        for _ in 0..3 {
            window.render_frame(cx);
        }
    });
    cx.run_until_parked();
    assert!(parent.upgrade().is_none());
    assert!(!select.read_with(cx, |s, _| s.is_open()));
    cx.update(|window, cx| {
        assert!(!base::GlobalState::is_in_deferred_context(cx));
        assert!(!select.read(cx).content.is_focused(window));
        assert!(!select.read(cx).trigger.is_focused(window));
    });
}
#[gpui_kit::test]
fn select_reparent_and_omit_detach_old_dismissal_registration(cx: &mut TestAppContext) {
    let (view, cx) = host(cx);
    let (old_parent, select) = view.read_with(cx, |v, _| {
        (v.parent.as_ref().unwrap().clone(), v.select.clone())
    });
    let new_parent = cx.new(|cx| PopoverState::new("New settings", cx));
    view.update(cx, |v, cx| {
        v.parent = Some(new_parent.clone());
        cx.notify();
    });
    cx.update(|window, cx| {
        for _ in 0..2 {
            window.render_frame(cx);
        }
        new_parent.update(cx, |p, cx| p.set_open(true, window, cx));
        for _ in 0..3 {
            window.render_frame(cx);
        }
        select.update(cx, |s, cx| s.set_open(true, window, cx));
        for _ in 0..3 {
            window.render_frame(cx);
        }
        old_parent.update(cx, |p, cx| p.dismiss(window, cx));
        window.render_frame(cx);
        assert!(
            select.read(cx).is_open(),
            "Old parent must no longer dismiss reparented child"
        );
        view.update(cx, |v, cx| {
            v.associate = false;
            cx.notify();
        });
        for _ in 0..2 {
            window.render_frame(cx);
        }
        new_parent.update(cx, |p, cx| p.dismiss(window, cx));
        // Omission removes explicit lifecycle ownership; close the retained standalone state ourselves.
        assert!(select.read(cx).is_open());
        select.update(cx, |s, cx| s.set_open(false, window, cx));
    });
}
#[gpui_kit::test]
fn select_unmounted_retained_child_releases_stale_parent_boundary(cx: &mut TestAppContext) {
    let (view, cx) = host(cx);
    let (parent, select) = view.read_with(cx, |v, _| {
        (v.parent.as_ref().unwrap().clone(), v.select.clone())
    });
    let former_row = cx.update(|window, _| window.find("nested-8").bounds().center());
    view.update(cx, |v, cx| {
        v.show_select = false;
        cx.notify();
    });
    cx.update(|window, cx| {
        for _ in 0..3 {
            window.render_frame(cx);
        }
    });
    cx.run_until_parked();
    assert!(parent.read_with(cx, |p, _| p.is_open()));
    assert!(
        !select.read_with(cx, |s, _| s.is_open()),
        "Retaining the state must not retain an unmounted popup"
    );
    assert!(select.read_with(cx, |s, _| s.deferred.is_none()));
    cx.simulate_click(former_row, Modifiers::default());
    cx.run_until_parked();
    assert!(
        !parent.read_with(cx, |p, _| p.is_open()),
        "Former popup bounds must not protect the parent from outside dismissal"
    );
    cx.update(|_, cx| assert!(!base::GlobalState::is_in_deferred_context(cx)));
    select.update(cx, |s, cx| s.set_value(SelectValue::Single(Some(5)), cx));
    view.update(cx, |v, cx| {
        v.show_select = true;
        cx.notify();
    });
    cx.update(|window, cx| {
        parent.update(cx, |p, cx| p.set_open(true, window, cx));
        for _ in 0..3 {
            window.render_frame(cx);
        }
        select.update(cx, |s, cx| s.set_open(true, window, cx));
        for _ in 0..3 {
            window.render_frame(cx);
        }
        assert!(select.read(cx).is_open());
        assert_eq!(select.read(cx).value(), &SelectValue::Single(Some(5)));
        parent.update(cx, |p, cx| p.dismiss(window, cx));
        assert!(
            !select.read(cx).is_open(),
            "Remount must restore parent lifecycle registration"
        );
    });
}
