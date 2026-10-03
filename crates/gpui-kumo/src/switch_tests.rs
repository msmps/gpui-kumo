use super::*;
use gpui_kit::{Context, Render, TestAppContext, test::TestWindowExt};
struct Harness {
    checked: bool,
    disabled: bool,
    apply: bool,
    changes: Vec<bool>,
    focus: FocusHandle,
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let owner = cx.entity().downgrade();
        div().tab_group().w(px(130.)).child(
            Switch::new("target", "A long wrapping label")
                .checked(self.checked)
                .disabled(self.disabled)
                .track_focus(&self.focus)
                .on_change(move |value, _, _, cx| {
                    let _ = owner.update(cx, |this, cx| {
                        this.changes.push(value);
                        if this.apply {
                            this.checked = value;
                        }
                        cx.notify();
                    });
                }),
        )
    }
}
#[gpui_kit::test]
fn pointer_keyboard_and_disabled_paths_keep_one_owner(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|_, cx| Harness {
        checked: false,
        disabled: false,
        apply: true,
        changes: vec![],
        focus: cx.focus_handle(),
    });
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert_eq!(window.find("target").role(), Some(Role::Switch));
        assert_eq!(window.find("label").role(), None);
        window.click("label", cx);
        window.render_frame(cx);
        window.press("space", cx);
        window.render_frame(cx);
        window.press("enter", cx);
        assert_eq!(view.read(cx).changes, &[true, false, true]);
        view.update(cx, |this, cx| {
            this.apply = false;
            cx.notify();
        });
        window.render_frame(cx);
        window.press("space", cx);
        window.render_frame(cx);
        window.press("space", cx);
        assert!(view.read(cx).checked);
        assert_eq!(&view.read(cx).changes[3..], &[false, false]);
        crate::set_appearance(crate::Appearance::Dark, cx);
        window.render_frame(cx);
        view.update(cx, |this, cx| {
            this.disabled = true;
            cx.notify();
        });
        window.render_frame(cx);
        window.click("label", cx);
        view.read(cx).focus.clone().focus(window, cx);
        window.press("space", cx);
        window.press("enter", cx);
        assert_eq!(view.read(cx).changes.len(), 5);
    });
}
struct Geometry;
impl Render for Geometry {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(px(12.))
            .children(
                [Size::Sm, Size::Base, Size::Lg]
                    .into_iter()
                    .enumerate()
                    .map(|(i, size)| {
                        div()
                            .id(("row", i))
                            .test_support()
                            .flex()
                            .gap(px(16.))
                            .child(Switch::new(("off", i), "Off").size(size).bare())
                            .child(Switch::new(("on", i), "On").size(size).checked(true).bare())
                    }),
            )
            .child(
                div()
                    .w(px(130.))
                    .child(Switch::new("long", "A long label that wraps").control_first(false)),
            )
            .child(
                SwitchGroup::new("group", "Settings")
                    .error("Error")
                    .description("Help")
                    .item(Switch::new("item", "Item")),
            )
    }
}
#[gpui_kit::test]
fn source_track_dimensions_alignment_and_group_messages(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (_, cx) = cx.add_window_view(|_, _| Geometry);
    cx.update(|window, cx| {
        for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
            crate::set_appearance(appearance, cx);
            window.render_frame(cx);
            for (i, side) in [16., 18., 20.].into_iter().enumerate() {
                let off = window.find(("off", i)).bounds();
                let on = window.find(("on", i)).bounds();
                assert_eq!(off.size, gpui_kit::size(px(side * 2.), px(side)));
                assert_eq!(off.size, on.size);
                assert_eq!(off.origin.y, on.origin.y);
            }
            assert!(window.find("long").bounds().size.width <= px(130.));
            assert!(window.find("long").bounds().size.height > px(21.));
            assert!(window.find("error").bounds().size.height > px(0.));
            assert!(window.find("description").bounds().size.height > px(0.));
        }
    });
}
#[test]
#[should_panic(expected = "Switch requires a readable name")]
fn blank_names_rejected() {
    let _ = Switch::new("empty", " ");
}

#[gpui_kit::test]
fn thumb_travel_reversal_and_reduced_motion_are_observable(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|_, cx| Harness {
        checked: false,
        disabled: false,
        apply: true,
        changes: vec![],
        focus: cx.focus_handle(),
    });
    cx.update(|window, cx| {
        let thumb = |window: &mut Window| {
            let track = window
                .within("target")
                .find("visual")
                .bounds()
                .scale(window.scale_factor());
            let side = px(18.).scale(window.scale_factor());
            let quad = window
                .painted_quads()
                .into_iter()
                .find(|q| {
                    q.bounds.size == gpui_kit::size(side, side)
                        && track.contains(&q.bounds.center())
                })
                .unwrap();
            assert_eq!(
                quad.corner_radii,
                gpui_kit::Corners::all(px(5.).scale(window.scale_factor()))
            );
            (
                quad.bounds.left(),
                track.left(),
                track.right() - side,
                window
                    .painted_quads()
                    .into_iter()
                    .find(|q| q.bounds == track && q.background.as_solid().is_some())
                    .unwrap()
                    .background
                    .as_solid()
                    .unwrap(),
            )
        };
        window.render_frame(cx);
        let (x, left, right, off_color) = thumb(window);
        assert_eq!(off_color, theme(cx).switch.off_track);
        assert_eq!(x, left);
        view.update(cx, |this, cx| {
            this.checked = true;
            cx.notify();
        });
        window.render_frame(cx);
        // A delayed host thread must not advance the test animation clock.
        std::thread::sleep(std::time::Duration::from_millis(180));
        window.render_frame(cx);
        assert_eq!(thumb(window).0, left);
        assert_eq!(thumb(window).3, off_color);
        cx.background_executor()
            .advance_clock(std::time::Duration::from_millis(40));
        window.render_frame(cx);
        let (mid, _, _, mid_color) = thumb(window);
        assert_ne!(mid_color, off_color);
        assert_ne!(mid_color, theme(cx).switch.on_track);
        assert!(
            mid > left && mid < right,
            "thumb must paint between endpoints"
        );
        view.update(cx, |this, cx| {
            this.checked = false;
            cx.notify();
        });
        window.render_frame(cx);
        let (reversed, _, _, _) = thumb(window);
        assert!(
            (reversed - mid).0.abs() < px(2.).scale(window.scale_factor()).0,
            "reversal must preserve presented position"
        );
        cx.background_executor()
            .advance_clock(std::time::Duration::from_millis(180));
        window.render_frame(cx);
        assert_eq!(thumb(window).0, left);
        cx.set_reduce_motion(true);
        view.update(cx, |this, cx| {
            this.checked = true;
            cx.notify();
        });
        window.render_frame(cx);
        assert_eq!(thumb(window).0, right);
        assert_eq!(thumb(window).3, theme(cx).switch.on_track);
    });
}

struct Groups {
    first: usize,
    second: usize,
    disabled: bool,
}
impl Render for Groups {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let first = cx.entity().downgrade();
        let second = first.clone();
        div()
            .tab_group()
            .flex()
            .flex_col()
            .child(
                SwitchGroup::new("first", "First")
                    .disabled(self.disabled)
                    .item(Switch::new("same-item", "Shared label").on_change(
                        move |_, _, _, cx| {
                            let _ = first.update(cx, |this, cx| {
                                this.first += 1;
                                cx.notify();
                            });
                        },
                    )),
            )
            .child(SwitchGroup::new("second", "Second").hide_legend().item(
                Switch::new("same-item", "Shared label").on_change(move |_, _, _, cx| {
                    let _ = second.update(cx, |this, cx| {
                        this.second += 1;
                        cx.notify();
                    });
                }),
            ))
    }
}
#[gpui_kit::test]
fn repeated_group_identity_callbacks_and_group_disabled_gate(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|_, _| Groups {
        first: 0,
        second: 0,
        disabled: false,
    });
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.within("first").click("label", cx);
        window.render_frame(cx);
        window.press("space", cx);
        assert_eq!(view.read(cx).first, 2);
        assert_eq!(view.read(cx).second, 0);
        window.render_frame(cx);
        window.within("second").click("label", cx);
        window.render_frame(cx);
        window.press("enter", cx);
        assert_eq!(view.read(cx).second, 2);
        assert_eq!(view.read(cx).first, 2);
        assert!(window.within("second").try_find("legend").is_none());
        view.update(cx, |this, cx| {
            this.disabled = true;
            cx.notify();
        });
        window.render_frame(cx);
        window.within("first").click("label", cx);
        window.press("space", cx);
        assert_eq!(view.read(cx).first, 2);
    });
}
