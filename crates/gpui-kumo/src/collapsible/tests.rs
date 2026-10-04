use super::*;
use gpui_kit::{AppContext, Focusable};
use gpui_kit::{Entity, Render, TestAppContext, VisualTestContext, test::TestWindowExt};
struct Harness {
    controlled: Option<bool>,
    disabled: bool,
    keep: bool,
    custom: bool,
    cancel: bool,
    proposals: Vec<bool>,
    consumer: usize,
    child_clicks: usize,
    child_focus: FocusHandle,
    custom_focus: FocusHandle,
    input: Entity<crate::InputState>,
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let owner = cx.entity().downgrade();
        let consumer = owner.clone();
        let child = owner.clone();
        let mut disclosure = Collapsible::new("disclosure", "Contact settings café 🦀")
            .disabled(self.disabled)
            .on_open_change(move |open, window, cx| {
                let _ = owner.update(cx, |v, _| {
                    v.proposals.push(open);
                    if v.cancel {
                        window.prevent_default();
                    }
                });
            })
            .panel(
                CollapsiblePanel::new()
                    .keep_mounted(self.keep)
                    .child(crate::Input::new("input", &self.input).show_label(true))
                    .child(crate::InlineCopyText::new("copy", "retained copy feedback"))
                    .child(
                        Button::new("inside", "Save")
                            .track_focus(&self.child_focus)
                            .on_click(move |_, _, cx| {
                                let _ = child.update(cx, |v, _| v.child_clicks += 1);
                            }),
                    ),
            );
        if let Some(open) = self.controlled {
            disclosure = disclosure.open(open);
        }
        if self.custom {
            disclosure = disclosure.trigger(
                Button::new("custom-trigger", "Custom settings")
                    .track_focus(&self.custom_focus)
                    .on_click(move |_, window, cx| {
                        let _ = consumer.update(cx, |v, _| {
                            v.consumer += 1;
                            if v.cancel {
                                window.prevent_default();
                            }
                        });
                    }),
            );
        }
        div()
            .flex()
            .flex_col()
            .w(px(230.))
            .gap(px(24.))
            .child(disclosure)
            .child(
                Collapsible::new("independent", "Independent")
                    .default_open(true)
                    .panel(CollapsiblePanel::unstyled().child("Another region")),
            )
            .child(Button::new("outside", "Outside"))
    }
}
fn harness(cx: &mut TestAppContext) -> (Entity<Harness>, &mut VisualTestContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|window, cx| Harness {
        controlled: None,
        disabled: false,
        keep: false,
        custom: false,
        cancel: false,
        proposals: vec![],
        consumer: 0,
        child_clicks: 0,
        child_focus: cx.focus_handle(),
        custom_focus: cx.focus_handle(),
        input: cx.new(|cx| crate::InputState::new("Name", window, cx)),
    });
    cx.update(|window, _| window.activate_window());
    cx.run_until_parked();
    (view, cx)
}
#[gpui_kit::test]
fn uncontrolled_once_only_activation_disabled_and_independent_state(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
        cx.update(|window, cx| {
            crate::set_appearance(appearance, cx);
            window.render_frame(cx);
            assert_eq!(
                window.within("disclosure").find("trigger").expanded(),
                Some(false)
            );
            window.within("disclosure").click("trigger", cx);
            window.render_frame(cx);
            assert_eq!(
                window.within("disclosure").find("trigger").expanded(),
                Some(true)
            );
            assert!(window.within("disclosure").try_find("inside").is_some());
            window.press("space", cx);
            window.render_frame(cx);
            assert!(window.within("disclosure").try_find("inside").is_none());
            window.press("enter", cx);
            window.render_frame(cx);
            assert_eq!(
                window.within("independent").find("trigger").expanded(),
                Some(true)
            );
            view.update(cx, |v, _| v.disabled = true);
            window.render_frame(cx);
            window.within("disclosure").click("trigger", cx);
            window.press("space", cx);
            window.press("enter", cx);
            view.update(cx, |v, _| v.disabled = false);
            window.render_frame(cx);
            window.press("enter", cx);
            window.render_frame(cx);
        });
    }
    cx.read(|cx| {
        assert_eq!(
            view.read(cx).proposals,
            [true, false, true, false, true, false, true, false]
        )
    });
}
#[gpui_kit::test]
fn controlled_proposals_custom_consumer_cancellation_and_owner_close_focus(
    cx: &mut TestAppContext,
) {
    let (view, cx) = harness(cx);
    cx.update(|window, cx| {
        view.update(cx, |v, _| {
            v.controlled = Some(false);
            v.custom = true;
        });
        window.render_frame(cx);
        window.click("custom-trigger", cx);
        window.press("space", cx);
        window.press("enter", cx);
        window.render_frame(cx);
        assert_eq!(window.find("custom-trigger").expanded(), Some(false));
        assert_eq!(view.read(cx).proposals, [true, true, true]);
        assert_eq!(view.read(cx).consumer, 3);
        assert!(view.read(cx).custom_focus.is_focused(window));
        view.update(cx, |v, _| {
            v.cancel = true;
        });
        window.render_frame(cx);
        window.click("custom-trigger", cx);
        window.press("space", cx);
        window.press("enter", cx);
        assert_eq!(view.read(cx).proposals.len(), 3);
        view.update(cx, |v, _| {
            v.cancel = false;
            v.controlled = Some(true);
        });
        window.render_frame(cx);
        view.read(cx)
            .input
            .read(cx)
            .focus_handle(cx)
            .focus(window, cx);
        window.render_frame(cx);
        assert_eq!(window.within("input").find("control").focused(), Some(true));
        view.update(cx, |v, _| v.controlled = Some(false));
        window.render_frame(cx);
        assert_eq!(window.find("custom-trigger").focused(), Some(true));
        assert!(view.read(cx).custom_focus.is_focused(window));
        assert_eq!(view.read(cx).proposals.len(), 3);
    });
}
#[gpui_kit::test]
fn kept_hidden_panel_excludes_focus_actions_and_retains_editor_and_keyed_feedback(
    cx: &mut TestAppContext,
) {
    let (view, cx) = harness(cx);
    cx.update(|window, cx| {
        view.update(cx, |v, _| {
            v.controlled = Some(true);
            v.keep = true;
        });
        window.render_frame(cx);
        window.click("inside", cx);
        assert_eq!(view.read(cx).child_clicks, 1);
        window.click("copy", cx);
        window.render_frame(cx);
        assert_eq!(window.find("copy").label(), Some("Copied"));
        view.read(cx)
            .input
            .read(cx)
            .focus_handle(cx)
            .focus(window, cx);
    });
    cx.simulate_input("café 🦀");
    for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
        cx.update(|window, cx| {
            crate::set_appearance(appearance, cx);
            view.update(cx, |v, _| v.controlled = Some(false));
            window.render_frame(cx);
            assert!(window.try_find("inside").is_none());
            assert!(window.try_find("copy").is_none());
            assert!(window.within("disclosure").try_find("control").is_none());
            assert_eq!(
                window.within("disclosure").find("trigger").focused(),
                Some(true)
            );
            window.focus_next(cx);
            window.render_frame(cx);
            assert_eq!(
                window.within("independent").find("trigger").focused(),
                Some(true)
            );
            window.focus_prev(cx);
            window.render_frame(cx);
            assert_eq!(
                window.within("disclosure").find("trigger").focused(),
                Some(true)
            );
            // A manually retained stale child focus cannot dispatch its former activation.
            view.read(cx).child_focus.clone().focus(window, cx);
            window.press("space", cx);
            window.press("enter", cx);
            assert_eq!(view.read(cx).child_clicks, 1);
            view.update(cx, |v, _| v.controlled = Some(true));
            window.render_frame(cx);
            assert_eq!(window.find("copy").label(), Some("Copied"));
            assert_eq!(view.read(cx).input.read(cx).value(cx).as_ref(), "café 🦀");
            view.read(cx)
                .input
                .read(cx)
                .focus_handle(cx)
                .focus(window, cx);
            window.render_frame(cx);
        });
    }
}
