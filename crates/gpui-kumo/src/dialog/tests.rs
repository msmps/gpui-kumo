use super::*;
use crate::{Input, InputState};
use gpui_kit::{AppContext, Focusable, TestAppContext, VisualTestContext, test::TestWindowExt};

struct Harness {
    dialog: Entity<DialogState>,
    input: Entity<InputState>,
    trigger: FocusHandle,
    mounted: bool,
    opener: bool,
    explicit: bool,
    size: DialogSize,
    events: Vec<DialogEvent>,
    activations: usize,
    _events: Subscription,
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let input = self.input.clone();
        let initial = input.read(cx).focus_handle(cx);
        div()
            .tab_group()
            .flex()
            .flex_col()
            .gap_4()
            .child(Button::new("before", "Before"))
            .when(self.opener, |root| {
                root.child(DialogTrigger::new(
                    "trigger",
                    &self.dialog,
                    Button::new("open", "Open")
                        .track_focus(&self.trigger)
                        .on_click(cx.listener(|s, _, _, cx| {
                            s.activations += 1;
                            cx.notify();
                        })),
                ))
            })
            .child(Button::new("after", "After"))
            .when(self.mounted, |root| {
                root.child(
                    Dialog::new("dialog", &self.dialog, move |close, _, _| {
                        let save = close.clone();
                        div()
                            .p_8()
                            .flex()
                            .flex_col()
                            .gap_4()
                            .child(Input::new("draft", &input))
                            .child(Button::new("disabled", "Disabled").disabled(true))
                            .child(close.button(Button::new("cancel", "Cancel")))
                            .child(Button::new("save", "Save").on_click(move |_, window, cx| {
                                save.request(DialogCloseReason::Action, window, cx);
                            }))
                            .into_any_element()
                    })
                    .size(self.size)
                    .description("Edit the retained draft")
                    .when(self.explicit, |dialog| dialog.initial_focus(&initial)),
                )
            })
    }
}
fn harness(cx: &mut TestAppContext) -> (Entity<Harness>, &mut VisualTestContext) {
    cx.update(crate::init);
    cx.add_window_view(|window, cx| {
        let dialog = cx.new(|cx| DialogState::new("Edit document", cx));
        let input = cx.new(|cx| InputState::new("Document name", window, cx));
        let events = cx.subscribe(&dialog, |s: &mut Harness, _, event: &DialogEvent, cx| {
            s.events.push(*event);
            cx.notify();
        });
        Harness {
            dialog,
            input,
            trigger: cx.focus_handle(),
            mounted: true,
            opener: true,
            explicit: true,
            size: DialogSize::Base,
            events: vec![],
            activations: 0,
            _events: events,
        }
    })
}
fn frame(window: &mut Window, cx: &mut App) {
    window.render_frame(cx);
    window.render_frame(cx);
}
fn open(window: &mut Window, cx: &mut App) {
    frame(window, cx);
    window.click("open", cx);
    frame(window, cx);
}

#[gpui_kit::test]
fn dialog_keyboard_boundary_skips_disabled_and_restores_opener(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    cx.update(|window, cx| {
        open(window, cx);
        let input = view.read(cx).input.clone();
        assert!(input.read(cx).focus_handle(cx).is_focused(window));
        assert_eq!(view.read(cx).activations, 1);
        window.press("shift-tab", cx);
        frame(window, cx);
        assert_eq!(window.find("save").focused(), Some(true));
        window.press("tab", cx);
        frame(window, cx);
        assert!(input.read(cx).focus_handle(cx).is_focused(window));
        window.press("enter", cx);
        frame(window, cx);
        assert!(view.read(cx).dialog.read(cx).is_open());
        window.press("tab", cx);
        frame(window, cx);
        assert_eq!(window.find("cancel").focused(), Some(true));
        window.press("enter", cx);
        frame(window, cx);
        assert!(!view.read(cx).dialog.read(cx).is_open());
        assert!(view.read(cx).trigger.is_focused(window));
    });
    cx.read(|cx| {
        assert_eq!(
            view.read(cx)
                .events
                .iter()
                .filter(|e| matches!(e, DialogEvent::Closed(_)))
                .count(),
            1
        )
    });
}

#[gpui_kit::test]
fn dialog_rejected_close_preserves_draft_then_accepts_once(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    cx.update(|window, cx| {
        let dialog = view.read(cx).dialog.clone();
        let input = view.read(cx).input.clone();
        let draft = input.downgrade();
        dialog.update(cx, |state, cx| {
            state.set_close_guard(
                move |reason, _, cx| {
                    reason != DialogCloseReason::Action
                        || draft
                            .upgrade()
                            .is_some_and(|input| !input.read(cx).value(cx).is_empty())
                },
                cx,
            )
        });
        open(window, cx);
        window.click("save", cx);
        frame(window, cx);
        assert!(dialog.read(cx).is_open());
        assert_eq!(window.find("save").focused(), Some(true));
        input.update(cx, |state, cx| state.set_value("café 🦀", window, cx));
        window.press("enter", cx);
        frame(window, cx);
        assert!(!dialog.read(cx).is_open());
        assert_eq!(input.read(cx).value(cx).as_ref(), "café 🦀");
        assert!(view.read(cx).trigger.is_focused(window));
    });
    cx.read(|cx| {
        assert!(
            view.read(cx)
                .events
                .contains(&DialogEvent::ClosePrevented(DialogCloseReason::Action))
        )
    });
}

#[gpui_kit::test]
fn dialog_default_initial_focus_runs_after_content_is_painted(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    cx.update(|window, cx| {
        view.update(cx, |state, cx| {
            state.explicit = false;
            cx.notify();
        });
        open(window, cx);
    });
    cx.run_until_parked();
    cx.update(|window, cx| {
        frame(window, cx);
        assert!(
            view.read(cx)
                .input
                .read(cx)
                .focus_handle(cx)
                .is_focused(window)
        );
    });
}

#[gpui_kit::test]
fn dialog_escape_and_backdrop_policy_prevent_underlying_activation(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    cx.update(|window, cx| {
        open(window, cx);
        let dialog = view.read(cx).dialog.clone();
        dialog.update(cx, |state, cx| {
            state.set_role(DialogRole::AlertDialog, cx);
            state.set_escape_dismissal(false, cx);
        });
        frame(window, cx);
        window.click("before", cx);
        frame(window, cx);
        assert!(dialog.read(cx).is_open());
        window.press("escape", cx);
        frame(window, cx);
        assert!(dialog.read(cx).is_open());
        dialog.update(cx, |state, cx| state.set_escape_dismissal(true, cx));
        window.press("escape", cx);
        frame(window, cx);
        assert!(!dialog.read(cx).is_open());
        open(window, cx);
        dialog.update(cx, |state, cx| state.set_role(DialogRole::Dialog, cx));
        frame(window, cx);
        window.click("before", cx);
        frame(window, cx);
        assert!(!dialog.read(cx).is_open());
    });
}

#[gpui_kit::test]
fn dialog_removed_opener_is_not_restored_even_with_retained_handle(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    cx.update(|window, cx| {
        open(window, cx);
        view.update(cx, |state, cx| {
            state.opener = false;
            cx.notify();
        });
        frame(window, cx);
        window.press("escape", cx);
        frame(window, cx);
        assert!(!view.read(cx).dialog.read(cx).is_open());
        assert!(!view.read(cx).trigger.is_focused(window));
    });
}

#[gpui_kit::test]
fn dialog_unmount_closes_retained_state_and_releases_content(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    cx.update(|window, cx| {
        open(window, cx);
        view.update(cx, |state, cx| {
            state.mounted = false;
            cx.notify();
        });
        frame(window, cx);
    });
    cx.run_until_parked();
    cx.update(|window, cx| {
        let dialog = view.read(cx).dialog.clone();
        assert!(!dialog.read(cx).is_open());
        assert!(dialog.read(cx).content.is_none());
        assert!(view.read(cx).trigger.is_focused(window));
        dialog.update(cx, |state, cx| state.set_open(true, window, cx));
        assert!(!dialog.read(cx).is_open());
        view.update(cx, |state, cx| {
            state.mounted = true;
            cx.notify();
        });
        frame(window, cx);
        open(window, cx);
        assert!(dialog.read(cx).is_open());
    });
}

#[gpui_kit::test]
fn dialog_source_sizes_and_top_offset_survive_resize(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    cx.update(open);
    for width in [1040., 520.] {
        cx.simulate_resize(gpui_kit::size(px(width), px(800.)));
        cx.update(|window, cx| {
            for (size, expected) in [
                (DialogSize::Small, 288.),
                (DialogSize::Base, 384.),
                (DialogSize::Large, 512.),
                (DialogSize::ExtraLarge, 768.),
            ] {
                view.update(cx, |state, cx| {
                    state.size = size;
                    cx.notify();
                });
                frame(window, cx);
                let panel = window.find("modal").bounds();
                assert_eq!(
                    panel.size.width,
                    px(if width < 640. { width - 32. } else { expected })
                );
                assert_eq!(panel.top(), px(if width < 640. { 32. } else { 64. }));
                assert_eq!(panel.left(), px((width - f32::from(panel.size.width)) / 2.));
            }
        });
    }
}

#[gpui_kit::test]
fn dialog_empty_panel_tab_does_not_escape_or_loop(cx: &mut TestAppContext) {
    struct Empty(Entity<DialogState>);
    impl Render for Empty {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .child(Button::new("outside", "Outside"))
                .child(Dialog::new("empty", &self.0, |_, _, _| {
                    div().p_8().child("Message").into_any_element()
                }))
        }
    }
    cx.update(crate::init);
    let (view, cx) =
        cx.add_window_view(|_, cx| Empty(cx.new(|cx| DialogState::new("Message", cx))));
    cx.update(|window, cx| {
        frame(window, cx);
        let dialog = view.read(cx).0.clone();
        dialog.update(cx, |state, cx| state.set_open(true, window, cx));
        frame(window, cx);
        for key in ["tab", "shift-tab"] {
            window.press(key, cx);
            frame(window, cx);
            assert!(dialog.read(cx).content_focus.is_focused(window));
        }
    });
}

#[gpui_kit::test]
fn dialog_long_content_scrolls_within_the_viewport(cx: &mut TestAppContext) {
    struct Long(Entity<DialogState>);
    impl Render for Long {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            Dialog::new("long", &self.0, |close, _, _| {
                div()
                    .p_8()
                    .flex()
                    .flex_col()
                    .flex_none()
                    .child(div().h(px(900.)).flex_none().child("Long content"))
                    .child(close.button(Button::new("last", "Close")))
                    .into_any_element()
            })
        }
    }
    cx.update(crate::init);
    let (view, cx) =
        cx.add_window_view(|_, cx| Long(cx.new(|cx| DialogState::new("Long content", cx))));
    cx.simulate_resize(gpui_kit::size(px(520.), px(400.)));
    cx.update(|window, cx| {
        frame(window, cx);
        let dialog = view.read(cx).0.clone();
        dialog.update(cx, |state, cx| state.set_open(true, window, cx));
        frame(window, cx);
        let panel = window.find("modal").bounds();
        assert_eq!(panel.top(), px(32.));
        assert_eq!(panel.size.height, px(336.));
        window.scroll(
            "content-scroll",
            gpui_kit::ScrollDelta::Pixels(gpui_kit::point(px(0.), px(-1000.))),
            cx,
        );
        frame(window, cx);
        let last = window.find("last").bounds();
        assert!(last.top() >= panel.top() && last.bottom() <= panel.bottom());
        window.click("last", cx);
        frame(window, cx);
        assert!(!dialog.read(cx).is_open());
    });
}

#[gpui_kit::test]
fn dialog_authoritative_close_keeps_newer_outside_focus(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    cx.update(|window, cx| {
        open(window, cx);
        let other = cx.focus_handle();
        other.focus(window, cx);
        view.read(cx)
            .dialog
            .clone()
            .update(cx, |state, cx| state.set_open(false, window, cx));
        frame(window, cx);
        assert!(other.is_focused(window));
    });
}

#[gpui_kit::test]
fn dialog_nested_topmost_owns_escape_tab_and_returns_to_parent(cx: &mut TestAppContext) {
    struct Nested {
        parent: Entity<DialogState>,
        child: Entity<DialogState>,
        child_trigger: FocusHandle,
    }
    impl Render for Nested {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let child = self.child.clone();
            let trigger = self.child_trigger.clone();
            div()
                .child(DialogTrigger::new(
                    "parent-trigger",
                    &self.parent,
                    Button::new("open-parent", "Open parent"),
                ))
                .child(
                    Dialog::new("parent", &self.parent, move |close, _, _| {
                        div()
                            .p_8()
                            .flex()
                            .flex_col()
                            .child(DialogTrigger::new(
                                "child-trigger",
                                &child,
                                Button::new("open-child", "Open child").track_focus(&trigger),
                            ))
                            .child(close.button(Button::new("close-parent", "Close parent")))
                            .into_any_element()
                    })
                    .initial_focus(&self.child_trigger),
                )
                .child(Dialog::new("child", &self.child, |close, _, _| {
                    div()
                        .p_8()
                        .child(close.button(Button::new("close-child", "Close child")))
                        .into_any_element()
                }))
        }
    }
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|_, cx| Nested {
        parent: cx.new(|cx| DialogState::new("Parent", cx)),
        child: cx.new(|cx| DialogState::new("Child", cx)),
        child_trigger: cx.focus_handle(),
    });
    cx.update(|window, cx| {
        frame(window, cx);
        window.click("open-parent", cx);
        frame(window, cx);
        window.press("enter", cx);
        frame(window, cx);
    });
    cx.run_until_parked();
    cx.update(|window, cx| {
        frame(window, cx);
        assert!(view.read(cx).parent.read(cx).is_open());
        assert!(view.read(cx).child.read(cx).is_open());
        window.press("tab", cx);
        frame(window, cx);
        assert_eq!(window.find("close-child").focused(), Some(true));
        window.press("escape", cx);
        frame(window, cx);
        assert!(view.read(cx).parent.read(cx).is_open());
        assert!(!view.read(cx).child.read(cx).is_open());
        assert!(view.read(cx).child_trigger.is_focused(window));
        window.press("escape", cx);
        frame(window, cx);
        assert!(!view.read(cx).parent.read(cx).is_open());
    });
}

#[gpui_kit::test]
fn rename_open_dialog_preserves_draft_focus_and_lifecycle(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    cx.update(|window, cx| {
        window.click("open", cx);
        window.render_frame(cx);
        window.render_frame(cx);
        let input = view.read(cx).input.clone();
        input.read(cx).focus_handle(cx).focus(window, cx);
    });
    cx.simulate_input("retained draft");
    cx.update(|window, cx| {
        let input = view.read(cx).input.clone();
        let dialog = view.read(cx).dialog.clone();
        dialog.update(cx, |dialog, cx| dialog.set_name("Modifier le dossier", cx));
        window.render_frame(cx);
        assert_eq!(window.find("modal").label(), Some("Modifier le dossier"));
        assert!(dialog.read(cx).is_open());
        assert_eq!(input.read(cx).value(cx).as_ref(), "retained draft");
        assert!(input.read(cx).focus_handle(cx).is_focused(window));
        window.click("cancel", cx);
        window.render_frame(cx);
        assert!(!dialog.read(cx).is_open());
    });
}
