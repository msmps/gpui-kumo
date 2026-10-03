use super::*;
use gpui_kit::{EntityInputHandler, TestAppContext, VisualTestContext, test::TestWindowExt};

#[gpui_kit::test]
fn password_presentation_redacts_value_and_synthetic_text(cx: &mut TestAppContext) {
    let (input, cx) = harness(cx);
    cx.update(|window, cx| {
        input.update(cx, |input, cx| {
            input.set_value("café 🦀 secret", window, cx);
            input.set_masked(true, window, cx);
        });
        for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
            crate::set_appearance(appearance, cx);
            window.render_frame(cx);
            assert_eq!(
                window.find("control").role(),
                Some(gpui_kit::Role::PasswordInput)
            );
            assert_eq!(window.find("control").value(), Some("••••••••"));
            assert!(accessibility::Snapshot::new(input.read(cx).editor.read(cx)).is_none());
            assert_eq!(input.read(cx).value(cx).as_ref(), "café 🦀 secret");
        }
        input.update(cx, |input, cx| input.set_masked(false, window, cx));
        window.render_frame(cx);
        assert_eq!(
            window.find("control").role(),
            Some(gpui_kit::Role::TextInput)
        );
        assert_eq!(window.find("control").value(), Some("café 🦀 secret"));
        assert!(accessibility::Snapshot::new(input.read(cx).editor.read(cx)).is_some());
    });
}

struct Harness {
    input: Entity<InputState>,
    before: FocusHandle,
    after: FocusHandle,
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("harness")
            .tab_group()
            .flex()
            .flex_col()
            .gap(px(12.))
            .w(px(300.))
            .child(crate::Button::new("before", "Before").track_focus(&self.before))
            .child(
                Input::new("input", &self.input)
                    .label(true)
                    .description("A useful hint"),
            )
            .child(crate::Button::new("after", "After").track_focus(&self.after))
    }
}
fn harness(cx: &mut TestAppContext) -> (Entity<InputState>, &mut VisualTestContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|window, cx| Harness {
        input: cx.new(|cx| InputState::new("Project name", window, cx)),
        before: cx.focus_handle(),
        after: cx.focus_handle(),
    });
    let input = cx.read(|cx| view.read(cx).input.clone());
    cx.update(|window, cx| {
        window.render_frame(cx);
        input.read(cx).focus_handle(cx).focus(window, cx);
    });
    (input, cx)
}

#[gpui_kit::test]
fn editing_survives_rerenders_and_theme_changes(cx: &mut TestAppContext) {
    let (input, cx) = harness(cx);
    cx.simulate_input("hello 👋");
    cx.update(|window, cx| {
        assert_eq!(input.read(cx).value(cx).as_ref(), "hello 👋");
        assert_eq!(window.find("control").label(), Some("Project name"));
        assert_eq!(
            window.find("control").role(),
            Some(gpui_kit::Role::TextInput)
        );
        window.render_frame(cx);
        assert_eq!(window.find("control").value(), Some("hello 👋"));
        crate::set_appearance(crate::Appearance::Dark, cx);
        window.render_frame(cx);
        assert_eq!(input.read(cx).value(cx).as_ref(), "hello 👋");
        assert!(input.read(cx).focus_handle(cx).is_focused(window));
        window.press("backspace", cx);
        assert_eq!(input.read(cx).value(cx).as_ref(), "hello ");
    });
}

#[gpui_kit::test]
fn clipboard_selection_and_read_only_availability(cx: &mut TestAppContext) {
    let (input, cx) = harness(cx);
    cx.simulate_input("café 🦀");
    cx.update(|window, cx| {
        let editor = input.read(cx).editor.clone();
        editor.update(cx, |editor, cx| editor.select_all(window, cx));
        assert_eq!(input.read(cx).selected_value(cx).as_ref(), "café 🦀");
        #[cfg(target_os = "macos")]
        let (copy, cut, paste) = ("cmd-c", "cmd-x", "cmd-v");
        #[cfg(not(target_os = "macos"))]
        let (copy, cut, paste) = ("ctrl-c", "ctrl-x", "ctrl-v");
        window.press(copy, cx);
        assert_eq!(cx.read_from_clipboard().unwrap().text().unwrap(), "café 🦀");
        input.update(cx, |input, cx| input.set_read_only(true, cx));
        window.render_frame(cx);
        window.press(copy, cx);
        assert_eq!(cx.read_from_clipboard().unwrap().text().unwrap(), "café 🦀");
        window.press(cut, cx);
        window.press(paste, cx);
        window.press("backspace", cx);
        editor.update(cx, |editor, cx| {
            editor.replace_text_in_range(None, "blocked", window, cx)
        });
        assert_eq!(input.read(cx).value(cx).as_ref(), "café 🦀");
        input.update(cx, |input, cx| input.set_read_only(false, cx));
        window.render_frame(cx);
        window.press(cut, cx);
        assert_eq!(input.read(cx).value(cx).as_ref(), "");
        window.press(paste, cx);
        assert_eq!(input.read(cx).value(cx).as_ref(), "café 🦀");
        #[cfg(target_os = "macos")]
        let (undo, redo) = ("cmd-z", "cmd-shift-z");
        #[cfg(not(target_os = "macos"))]
        let (undo, redo) = ("ctrl-z", "ctrl-y");
        window.press(undo, cx);
        assert_eq!(input.read(cx).value(cx).as_ref(), "");
        window.press(redo, cx);
        assert_eq!(input.read(cx).value(cx).as_ref(), "café 🦀");
    });
}

#[gpui_kit::test]
fn disabled_input_rejects_native_edits_and_skips_traversal(cx: &mut TestAppContext) {
    let (input, cx) = harness(cx);
    cx.update(|window, cx| {
        input.update(cx, |input, cx| {
            input.set_value("retained", window, cx);
            input.set_disabled(true, cx);
        });
        window.render_frame(cx);
        let editor = input.read(cx).editor.clone();
        editor.update(cx, |editor, cx| {
            editor.replace_text_in_range(None, "blocked", window, cx)
        });
        assert_eq!(input.read(cx).value(cx).as_ref(), "retained");
        window.click("before", cx);
        window.focus_next(cx);
        window.press("space", cx);
        assert!(!input.read(cx).focus_handle(cx).is_focused(window));
        input.update(cx, |input, cx| input.set_disabled(false, cx));
        window.render_frame(cx);
        window.click("before", cx);
        window.focus_next(cx);
        assert!(input.read(cx).focus_handle(cx).is_focused(window));
    });
    cx.simulate_input("!");
    cx.read(|cx| assert_eq!(input.read(cx).value(cx).as_ref(), "retained!"));
}

#[gpui_kit::test]
fn composition_uses_utf16_ranges_and_commits_without_duplicate_text(cx: &mut TestAppContext) {
    let (input, cx) = harness(cx);
    cx.update(|window, cx| {
        let editor = input.read(cx).editor.clone();
        editor.update(cx, |editor, cx| {
            editor.replace_text_in_range(None, "🦀", window, cx);
            editor.replace_and_mark_text_in_range(None, "に", Some(1..1), window, cx);
            assert_eq!(editor.marked_text_range(window, cx), Some(2..3));
            editor.replace_and_mark_text_in_range(None, "日本", Some(2..2), window, cx);
            assert_eq!(editor.marked_text_range(window, cx), Some(2..4));
            editor.replace_text_in_range(None, "日本", window, cx);
            editor.unmark_text(window, cx);
            assert_eq!(editor.marked_text_range(window, cx), None);
        });
        assert_eq!(input.read(cx).value(cx).as_ref(), "🦀日本");
    });
}

#[gpui_kit::test]
fn change_and_submit_events_follow_user_edits_but_programmatic_reset_is_silent(
    cx: &mut TestAppContext,
) {
    use std::{cell::Cell, rc::Rc};
    let (input, cx) = harness(cx);
    let changes = Rc::new(Cell::new(0));
    let submits = Rc::new(Cell::new(0));
    let _subscription = cx.update(|_, cx| {
        let changes = changes.clone();
        let submits = submits.clone();
        cx.subscribe(&input, move |_, event, _| match event {
            InputEvent::Change => changes.set(changes.get() + 1),
            InputEvent::Submit { .. } => submits.set(submits.get() + 1),
            _ => {}
        })
    });
    cx.simulate_input("draft");
    assert!(changes.get() > 0);
    let change_count = changes.get();
    cx.update(|window, cx| {
        window.press("enter", cx);
        input.update(cx, |input, cx| input.set_value("reset", window, cx));
        window.render_frame(cx);
    });
    assert_eq!(submits.get(), 1);
    assert_eq!(changes.get(), change_count);
    cx.update(|window, cx| {
        input.update(cx, |input, cx| input.set_disabled(true, cx));
        window.render_frame(cx);
        window.press("enter", cx);
    });
    assert_eq!(submits.get(), 1);
}

#[gpui_kit::test]
fn sizes_fill_the_parent_without_changing_recipe_heights(cx: &mut TestAppContext) {
    struct Sizes {
        inputs: [Entity<InputState>; 4],
    }
    impl Render for Sizes {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div().flex().flex_col().w(px(240.)).gap(px(12.)).children(
                self.inputs
                    .iter()
                    .zip([Size::Xs, Size::Sm, Size::Base, Size::Lg])
                    .enumerate()
                    .map(|(index, (state, size))| Input::new(("sized", index), state).size(size)),
            )
        }
    }
    cx.update(crate::init);
    let (_, cx) = cx.add_window_view(|window, cx| Sizes {
        inputs: std::array::from_fn(|_| cx.new(|cx| InputState::new("Name", window, cx))),
    });
    cx.update(|window, cx| {
        window.render_frame(cx);
        for (index, height) in [20., 26., 36., 40.].into_iter().enumerate() {
            let bounds = window.within(("sized", index)).find("surface").bounds();
            assert_eq!(bounds.size.height, px(height));
            assert_eq!(bounds.size.width, px(240.));
            let editor = window.within(("sized", index)).find("control");
            assert_eq!(editor.role(), Some(gpui_kit::Role::TextInput));
            assert!(editor.bounds().left() >= bounds.left());
            assert!(editor.bounds().right() <= bounds.right());
        }
    });
}

#[gpui_kit::test]
fn accessible_text_uses_base_geometry_selection_and_composition(cx: &mut TestAppContext) {
    use gpui_kit::accesskit::{NodeId, Role};
    let (input, cx) = harness(cx);
    let id = NodeId(42);
    cx.update(|window, cx| {
        input.update(cx, |state, cx| state.set_value("café 🦀", window, cx));
        window.render_frame(cx);
        let editor = input.read(cx).editor.clone();
        editor.update(cx, |editor, cx| editor.set_selected_range(3..10, cx));
        window.render_frame(cx);
        let (run, selection) = accessibility::Snapshot::new(editor.read(cx))
            .unwrap()
            .nodes(id);
        assert_eq!(run.role(), Role::TextRun);
        assert_eq!(run.value(), Some("café 🦀"));
        assert_eq!(run.character_lengths(), &[1, 1, 1, 2, 1, 4]);
        assert_eq!(selection.anchor.character_index, 3);
        assert_eq!(selection.focus.character_index, 6);
        assert_eq!(run.character_positions().unwrap().len(), 6);
        assert_eq!(run.character_widths().unwrap().len(), 6);
        let bounds = run.bounds().unwrap();
        assert!(bounds.width() > 0. && bounds.height() > 0.);
        assert_eq!(input.read(cx).selected_value(cx).as_ref(), "é 🦀");
        // Keyboard direction remains owned by Base and is reflected in the tree.
        window.press("left", cx);
        window.press("shift-left", cx);
        window.render_frame(cx);
        let (_, selection) = accessibility::Snapshot::new(editor.read(cx))
            .unwrap()
            .nodes(id);
        assert_eq!(selection.anchor.character_index, 3);
        assert_eq!(selection.focus.character_index, 2);
        editor.update(cx, |editor, cx| {
            editor.replace_and_mark_text_in_range(None, "日本", Some(2..2), window, cx);
        });
        window.render_frame(cx);
        let (run, selection) = accessibility::Snapshot::new(editor.read(cx))
            .unwrap()
            .nodes(id);
        assert_eq!(run.value(), Some("ca日本é 🦀"));
        assert_eq!(selection.focus.character_index, 4);
        editor.update(cx, |editor, cx| {
            editor.replace_text_in_range(None, "日本", window, cx);
            editor.unmark_text(window, cx);
        });
        window.render_frame(cx);
        let (run, _) = accessibility::Snapshot::new(editor.read(cx))
            .unwrap()
            .nodes(id);
        assert_eq!(run.value(), Some("ca日本é 🦀"));
        input.update(cx, |state, cx| state.set_value("", window, cx));
        window.render_frame(cx);
        let (run, selection) = accessibility::Snapshot::new(editor.read(cx))
            .unwrap()
            .nodes(id);
        assert_eq!(run.value(), Some(""));
        assert!(run.character_lengths().is_empty());
        assert_eq!(selection.focus.character_index, 0);
        assert!(run.bounds().unwrap().height() > 0.);
    });
}

#[gpui_kit::test]
fn accessible_actions_reuse_base_edits_and_emit_once(cx: &mut TestAppContext) {
    use gpui_kit::accesskit::{ActionData, NodeId, TextPosition, TextSelection};
    use std::{cell::Cell, rc::Rc};
    let (input, cx) = harness(cx);
    let changes = Rc::new(Cell::new(0));
    let _subscription = cx.update(|_, cx| {
        let changes = changes.clone();
        cx.subscribe(&input, move |_, event, _| {
            if matches!(event, InputEvent::Change) {
                changes.set(changes.get() + 1);
            }
        })
    });
    let id = NodeId(42);
    let selection = |anchor, focus| {
        ActionData::SetTextSelection(TextSelection {
            anchor: TextPosition {
                node: id,
                character_index: anchor,
            },
            focus: TextPosition {
                node: id,
                character_index: focus,
            },
        })
    };
    cx.update(|window, cx| {
        input.update(cx, |state, cx| {
            state.accessibility_action(
                None,
                Some(&ActionData::Value("café 🦀".into())),
                window,
                cx,
            );
        });
        window.render_frame(cx);
    });
    assert_eq!(changes.get(), 1);
    cx.update(|window, cx| {
        input.update(cx, |state, cx| {
            state.accessibility_action(Some(id), Some(&selection(3, 6)), window, cx);
        });
        assert_eq!(input.read(cx).selected_value(cx).as_ref(), "é 🦀");
        input.update(cx, |state, cx| {
            state.accessibility_action(Some(id), Some(&selection(0, 99)), window, cx);
            state.accessibility_action(Some(NodeId(999)), Some(&selection(0, 1)), window, cx);
        });
        assert_eq!(input.read(cx).selected_value(cx).as_ref(), "é 🦀");
        input.update(cx, |state, cx| {
            state.accessibility_action(Some(id), Some(&selection(6, 3)), window, cx);
        });
        assert_eq!(input.read(cx).selected_value(cx).as_ref(), "é 🦀");
        assert_eq!(input.read(cx).editor.read(cx).cursor(), 10);
    });
    assert_eq!(changes.get(), 1);
    cx.update(|window, cx| {
        input.update(cx, |state, cx| {
            state.accessibility_replace_selection(
                Some(&ActionData::Value("日本".into())),
                window,
                cx,
            )
        });
        assert_eq!(input.read(cx).value(cx).as_ref(), "caf日本");
        window.render_frame(cx);
    });
    assert_eq!(changes.get(), 2);
    cx.update(|window, cx| {
        #[cfg(target_os = "macos")]
        window.press("cmd-z", cx);
        #[cfg(not(target_os = "macos"))]
        window.press("ctrl-z", cx);
        assert_eq!(input.read(cx).value(cx).as_ref(), "café 🦀");
        input.update(cx, |state, cx| state.set_value("reset", window, cx));
    });
    assert_eq!(changes.get(), 3);
}

#[gpui_kit::test]
fn accessible_actions_enforce_current_availability(cx: &mut TestAppContext) {
    use gpui_kit::accesskit::{ActionData, NodeId, TextPosition, TextSelection};
    let (input, cx) = harness(cx);
    let id = NodeId(42);
    let selection = ActionData::SetTextSelection(TextSelection {
        anchor: TextPosition {
            node: id,
            character_index: 0,
        },
        focus: TextPosition {
            node: id,
            character_index: 1,
        },
    });
    cx.update(|window, cx| {
        input.update(cx, |state, cx| {
            state.set_value("🦀日本", window, cx);
            state.set_read_only(true, cx);
            state.accessibility_action(Some(id), Some(&selection), window, cx);
            state.accessibility_action(
                None,
                Some(&ActionData::Value("blocked".into())),
                window,
                cx,
            );
            state.accessibility_replace_selection(
                Some(&ActionData::Value("blocked".into())),
                window,
                cx,
            );
        });
        assert_eq!(input.read(cx).selected_value(cx).as_ref(), "🦀");
        assert_eq!(input.read(cx).value(cx).as_ref(), "🦀日本");
        input.update(cx, |state, cx| {
            state.set_disabled(true, cx);
            state.accessibility_action(
                Some(id),
                Some(&ActionData::SetTextSelection(TextSelection {
                    anchor: TextPosition {
                        node: id,
                        character_index: 1,
                    },
                    focus: TextPosition {
                        node: id,
                        character_index: 3,
                    },
                })),
                window,
                cx,
            );
            state.accessibility_action(
                None,
                Some(&ActionData::Value("blocked".into())),
                window,
                cx,
            );
            state.accessibility_replace_selection(
                Some(&ActionData::Value("blocked".into())),
                window,
                cx,
            );
        });
        assert_eq!(input.read(cx).selected_value(cx).as_ref(), "🦀");
        assert_eq!(input.read(cx).value(cx).as_ref(), "🦀日本");
    });
}

#[gpui_kit::test]
fn accessible_composition_forwards_each_base_change_once(cx: &mut TestAppContext) {
    use std::{cell::Cell, rc::Rc};
    let (input, cx) = harness(cx);
    let changes = Rc::new(Cell::new(0));
    let _subscription = cx.update(|_, cx| {
        let changes = changes.clone();
        cx.subscribe(&input, move |_, event, _| {
            if matches!(event, InputEvent::Change) {
                changes.set(changes.get() + 1);
            }
        })
    });
    for (index, text) in ["🦀", "に", "日本", "日本"].into_iter().enumerate() {
        cx.update(|window, cx| {
            let editor = input.read(cx).editor.clone();
            editor.update(cx, |editor, cx| {
                if index == 1 || index == 2 {
                    editor.replace_and_mark_text_in_range(None, text, None, window, cx);
                } else {
                    editor.replace_text_in_range(None, text, window, cx);
                }
            });
            window.render_frame(cx);
            window.render_frame(cx);
        });
        // Base emits Change for committed edits; preedit stays silent.
        assert_eq!(changes.get(), if index == 3 { 2 } else { 1 });
    }
    cx.read(|cx| assert_eq!(input.read(cx).value(cx).as_ref(), "🦀日本"));
}

#[gpui_kit::test]
fn unmounting_accessible_input_releases_its_entities(cx: &mut TestAppContext) {
    struct OptionalInput(Option<Entity<InputState>>);
    impl Render for OptionalInput {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div().w(px(300.)).when_some(self.0.clone(), |this, input| {
                this.child(Input::new("input", &input))
            })
        }
    }
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|window, cx| {
        OptionalInput(Some(cx.new(|cx| InputState::new("Temporary", window, cx))))
    });
    let (state, editor) = cx.read(|cx| {
        let state = view.read(cx).0.as_ref().unwrap();
        (state.downgrade(), state.read(cx).editor.downgrade())
    });
    cx.update(|window, cx| {
        window.render_frame(cx);
        view.update(cx, |view, cx| {
            view.0.take();
            cx.notify();
        });
        window.render_frame(cx);
        window.render_frame(cx);
    });
    assert!(state.upgrade().is_none());
    assert!(editor.upgrade().is_none());
}

#[gpui_kit::test]
fn surface_padding_focuses_editor_without_changing_value_and_disabled_rejects(
    cx: &mut TestAppContext,
) {
    let (input, cx) = harness(cx);
    cx.update(|window, cx| {
        window.click("before", cx);
        assert!(!input.read(cx).focus_handle(cx).is_focused(window));
        window.click_at("surface", gpui_kit::point(px(2.), px(18.)), cx);
        assert!(input.read(cx).focus_handle(cx).is_focused(window));
        assert_eq!(input.read(cx).value(cx).as_ref(), "");
        input.update(cx, |input, cx| input.set_disabled(true, cx));
        window.render_frame(cx);
        window.click("before", cx);
        window.click_at("surface", gpui_kit::point(px(2.), px(18.)), cx);
        assert!(!input.read(cx).focus_handle(cx).is_focused(window));
        assert_eq!(window.find("before").focused(), Some(true));
    });
}

struct HelpHarness {
    input: Entity<InputState>,
    help: Entity<crate::TooltipState>,
    replacement: Entity<crate::TooltipState>,
    group: bool,
    label: bool,
    attach: bool,
    replace: bool,
    text: SharedString,
}
impl Render for HelpHarness {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let state = if self.replace {
            &self.replacement
        } else {
            &self.help
        };
        let control = if self.group {
            crate::InputGroup::new("with-help", &self.input)
                .label(self.label)
                .description("Retained editing")
                .when(self.attach, |group| {
                    group.label_tooltip(state, self.text.clone())
                })
                .end(crate::InputGroupAddon::button(
                    "action",
                    "Action",
                    |button, _, _| button,
                ))
                .into_any_element()
        } else {
            Input::new("with-help", &self.input)
                .label(self.label)
                .description("Retained editing")
                .when(self.attach, |input| {
                    input.label_tooltip(state, self.text.clone())
                })
                .into_any_element()
        };
        crate::TooltipProvider::new(
            "help-provider",
            [self.help.downgrade(), self.replacement.downgrade()],
            div()
                .w(px(240.))
                .flex()
                .flex_col()
                .tab_group()
                .child(control)
                .child(crate::Button::new("outside-help", "Outside")),
        )
    }
}
#[gpui_kit::test]
fn input_help_preserves_editing_and_excludes_label_from_group_focus(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|window, cx| HelpHarness {
        input: cx.new(|cx| InputState::new("API key", window, cx)),
        help: cx.new(|cx| crate::TooltipState::new(window, cx)),
        replacement: cx.new(|cx| crate::TooltipState::new(window, cx)),
        group: false,
        label: true,
        attach: true,
        replace: false,
        text: "Find this in Settings.".into(),
    });
    let (input, help, replacement) = cx.read(|cx| {
        let v = view.read(cx);
        (v.input.clone(), v.help.clone(), v.replacement.clone())
    });
    for group in [false, true] {
        for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
            cx.update(|window, cx| {
                crate::set_appearance(appearance, cx);
                view.update(cx, |v, cx| {
                    v.group = group;
                    v.attach = true;
                    v.label = true;
                    v.replace = false;
                    v.text = "Find this in Settings.".into();
                    cx.notify();
                });
                input.update(cx, |v, cx| {
                    v.set_disabled(false, cx);
                    v.set_value("café 🦀", window, cx);
                });
                window.activate_window();
                window.render_frame(cx);
                window.click("label", cx);
                let editor = input.read(cx).editor.clone();
                editor.update(cx, |e, cx| e.select_all(window, cx));
                window.click("label-help", cx);
                window.render_frame(cx);
                assert_eq!(window.find("label-help").focused(), Some(true));
                assert!(!input.read(cx).focus_handle(cx).is_focused(window));
                assert_eq!(input.read(cx).selected_value(cx).as_ref(), "café 🦀");
            });
            cx.run_until_parked();
            cx.update(|window, cx| {
                window.render_frame(cx);
                let bounds = window.find("surface").bounds();
                let theme = crate::theme(cx);
                assert!(
                    window.painted_quads().iter().any(|q| q.bounds
                        == bounds
                            .dilate(theme.effects.control_ring_width)
                            .scale(window.scale_factor())
                        && q.border_color == theme.colors.line),
                    "label help must not light editor/container focus ring: group={group}, appearance={appearance:?}"
                );
                assert_eq!(
                    window.find("label-help").bounds().center().y,
                    window.find("label").bounds().center().y
                );
                window.click("label", cx);
                window.render_frame(cx);
                window.press("shift-tab", cx);
                window.focus_prev(cx);
                window.render_frame(cx);
            });
            cx.run_until_parked();
            cx.update(|window, cx| {
                window.render_frame(cx);
                assert!(help.read(cx).is_open());
                assert_eq!(window.find("label-help").focused(), Some(true));
                window.press("escape", cx);
                assert!(!help.read(cx).is_open());
                window.press("tab", cx);
                window.focus_next(cx);
                window.render_frame(cx);
                assert!(input.read(cx).focus_handle(cx).is_focused(window));
                input.update(cx, |v, cx| v.set_disabled(true, cx));
                window.render_frame(cx);
                window.click("label-help", cx);
                window.render_frame(cx);
                assert_eq!(window.find("label-help").focused(), Some(true));
                assert!(!window.find("label-help").disabled().unwrap_or(false));
                assert_eq!(input.read(cx).value(cx).as_ref(), "café 🦀");
                for mode in 0..4 {
                    help.update(cx, |t, cx| t.set_open(true, cx));
                    view.update(cx, |v, cx| {
                        v.label = mode != 0;
                        v.attach = mode != 2;
                        v.replace = mode == 3;
                        v.text = if mode == 1 {
                            ""
                        } else {
                            "Find this in Settings."
                        }
                        .into();
                        cx.notify();
                    });
                    window.render_frame(cx);
                    assert!(
                        !help.read(cx).is_open(),
                        "hidden, empty, removed or replaced help cancels old disclosure"
                    );
                    assert_eq!(window.try_find("label-help").is_some(), mode == 3);
                    assert_eq!(input.read(cx).value(cx).as_ref(), "café 🦀");
                    view.update(cx, |v, cx| {
                        v.label = true;
                        v.attach = true;
                        v.replace = false;
                        v.text = "Find this in Settings.".into();
                        cx.notify();
                    });
                    window.render_frame(cx);
                }
                replacement.update(cx, |t, cx| t.set_open(false, cx));
            });
        }
    }
}

struct ScrolledInput {
    input: Entity<InputState>,
}
impl Render for ScrolledInput {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().w(px(100.)).child(
            crate::InputGroup::new("scrolled-group", &self.input)
                .editor_width(px(50.))
                .text_align(gpui_kit::TextAlign::Center)
                .button(
                    "next",
                    "Next",
                    crate::button::Variant::Secondary,
                    |b, _, _| b,
                ),
        )
    }
}
#[gpui_kit::test]
fn accessible_text_remains_complete_when_centred_input_scrolls(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|window, cx| ScrolledInput {
        input: cx.new(|cx| InputState::new("Page number", window, cx)),
    });
    let input = cx.read(|cx| view.read(cx).input.clone());
    cx.update(|window, cx| {
        window.activate_window();
        window.render_frame(cx);
        input.read(cx).focus_handle(cx).focus(window, cx);
    });
    cx.run_until_parked();
    for length in 1..=24 {
        cx.simulate_input("9");
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.render_frame(cx);
            let editor = input.read(cx).editor.read(cx);
            let (run, selection) = accessibility::Snapshot::new(editor)
                .expect("a scrolled single-line editor must expose its full text")
                .nodes(gpui_kit::accesskit::NodeId(42));
            assert_eq!(run.value(), Some("9".repeat(length).as_str()));
            assert_eq!(selection.focus.character_index, length);
            assert_eq!(run.character_positions().unwrap().len(), length);
        });
    }
}
