use super::*;
use gpui_kit::AppContext;
use gpui_kit::{Context, Focusable, Render, TestAppContext, test::TestWindowExt};
struct Harness {
    state: Entity<InputState>,
    size: Size,
    narrow: bool,
}

struct SuffixHarness {
    state: Entity<InputState>,
}
impl Render for SuffixHarness {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .w(px(180.))
            .child(InputGroup::new("domain", &self.state).suffix(".workers.dev"))
    }
}
#[gpui_kit::test]
fn suffix_follows_value_width_without_owning_selection_or_overflow(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|window, cx| SuffixHarness {
        state: cx.new(|cx| InputState::new("Subdomain", window, cx)),
    });
    let state = cx.read(|cx| view.read(cx).state.clone());
    cx.update(|window, cx| {
        window.activate_window();
        window.render_frame(cx);
        state.read(cx).focus_handle(cx).focus(window, cx);
    });
    cx.simulate_input("my-worker");
    cx.update(|window, cx| {
        assert_eq!(state.read(cx).value(cx).as_ref(), "my-worker");
        state.update(cx, |state, cx| state.set_value("", window, cx));
        window.render_frame(cx);
        let empty = window.find("editor-zone").bounds();
        assert_eq!(
            window.find("suffix").bounds().left(),
            empty.right() - SUFFIX_OVERLAP
        );
        state.update(cx, |state, cx| {
            state.set_placeholder("placeholder", window, cx)
        });
        window.render_frame(cx);
        let placeholder = window.find("editor-zone").bounds().size.width;
        assert!(placeholder > empty.size.width);
        state.update(cx, |state, cx| state.set_value("é🦀", window, cx));
        window.render_frame(cx);
        let short = window.find("editor-zone").bounds();
        assert_eq!(
            window.find("suffix").bounds().left(),
            short.right() - SUFFIX_OVERLAP
        );
        assert!(short.size.width > empty.size.width);
        state.read(cx).focus_handle(cx).focus(window, cx);
        window.press("end", cx);
        window.render_frame(cx);
        assert_visible_caret(window, cx);
        #[cfg(target_os = "macos")]
        window.press("cmd-a", cx);
        #[cfg(not(target_os = "macos"))]
        window.press("ctrl-a", cx);
        assert_eq!(state.read(cx).selected_value(cx).as_ref(), "é🦀");
        state.update(cx, |state, cx| {
            state.set_value("long-domain".repeat(30), window, cx)
        });
        for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
            crate::set_appearance(appearance, cx);
            window.render_frame(cx);
            let control = window.find("surface").bounds();
            let editor = window.find("editor-zone").bounds();
            let suffix = window.find("suffix").bounds();
            assert!(editor.right() <= control.right());
            assert!(suffix.right() <= control.right());
            assert!(
                suffix.size.width > px(12.),
                "natural suffix basis must retain content room"
            );
            assert_eq!(suffix.left(), editor.right() - SUFFIX_OVERLAP);
            assert_eq!(window.find("editor-clip").bounds().right(), suffix.left());
            window.press("end", cx);
            window.render_frame(cx);
            assert_visible_caret(window, cx);
            window.press("home", cx);
            window.render_frame(cx);
            assert_eq!(
                window.find("editor-clip").bounds().right(),
                window.find("suffix").bounds().left()
            );
            #[cfg(target_os = "macos")]
            window.press("cmd-a", cx);
            #[cfg(not(target_os = "macos"))]
            window.press("ctrl-a", cx);
            window.render_frame(cx);
            assert_eq!(
                state.read(cx).selected_value(cx).as_ref(),
                "long-domain".repeat(30)
            );
            #[cfg(target_os = "macos")]
            window.press("cmd-c", cx);
            #[cfg(not(target_os = "macos"))]
            window.press("ctrl-c", cx);
            assert_eq!(
                cx.read_from_clipboard().unwrap().text().unwrap(),
                "long-domain".repeat(30)
            );
            assert!(state.read(cx).focus_handle(cx).is_focused(window));
        }
    });
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().w(px(if self.narrow { 52. } else { 220. })).child(
            InputGroup::new("group", &self.state)
                .size(self.size)
                .start(InputGroupAddon::text(if self.narrow {
                    "A very long addon that must be clipped"
                } else {
                    "/api/"
                }))
                .end(InputGroupAddon::text(".json")),
        )
    }
}
#[gpui_kit::test]
fn container_addons_preserve_editor_selection_and_source_geometry(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|window, cx| Harness {
        state: cx.new(|cx| InputState::new("Endpoint", window, cx)),
        size: Size::Base,
        narrow: false,
    });
    let state = cx.read(|cx| view.read(cx).state.clone());
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.click("addon-start", cx);
        assert!(state.read(cx).focus_handle(cx).is_focused(window));
    });
    cx.simulate_input("café 🦀");
    cx.update(|window, cx| {
        for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
            crate::set_appearance(appearance, cx);
            for (size, height) in [
                (Size::Xs, 24.),
                (Size::Sm, 28.),
                (Size::Base, 36.),
                (Size::Lg, 44.),
            ] {
                view.update(cx, |view, cx| {
                    view.size = size;
                    cx.notify();
                });
                window.render_frame(cx);
                let control = window.find("surface").bounds();
                assert_eq!(control.size.height, px(height));
                assert_eq!(control.size.width, px(220.));
                assert!(window.find("editor-zone").bounds().size.width > px(0.));
                assert!(window.find("addon-end").bounds().right() <= control.right());
                assert_eq!(state.read(cx).value(cx).as_ref(), "café 🦀");
            }
        }
        view.update(cx, |view, cx| {
            view.narrow = true;
            cx.notify();
        });
        window.render_frame(cx);
        assert_eq!(window.find("surface").bounds().size.width, px(52.));
        assert_eq!(window.find("content-row").bounds().size.width, px(52.));
        assert!(window.find("editor-zone").bounds().size.width >= px(0.));
        window.press("backspace", cx);
        assert_eq!(state.read(cx).value(cx).as_ref(), "café ");
        state.update(cx, |state, cx| state.set_disabled(true, cx));
        window.render_frame(cx);
        window.press("backspace", cx);
        assert_eq!(state.read(cx).value(cx).as_ref(), "café ");
    });
}

fn assert_visible_caret(window: &Window, cx: &gpui_kit::App) {
    let width = (EDITOR_CARET_MARGIN - SUFFIX_OVERLAP).scale(window.scale_factor());
    let caret = window
        .painted_quads()
        .into_iter()
        .find(|q| {
            q.background.as_solid() == Some(crate::theme(cx).text.default)
                && q.bounds.size.width == width
        })
        .expect("focused End caret must paint");
    assert!(caret.content_mask.bounds.contains(&caret.bounds.origin));
    assert!(
        caret.bounds.right() <= caret.content_mask.bounds.right(),
        "whole caret stays visible before suffix"
    );
}

struct Actions {
    state: Entity<InputState>,
    focus: gpui_kit::FocusHandle,
    calls: usize,
    size: Size,
    editor_focus_events: usize,
    _events: gpui_kit::Subscription,
}
impl Render for Actions {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let owner = cx.entity().downgrade();
        let focus = self.focus.clone();
        div()
            .tab_group()
            .flex()
            .flex_col()
            .w(px(240.))
            .child(crate::Button::new("before", "Before"))
            .child(InputGroup::new("actions", &self.state).size(self.size).end(
                InputGroupAddon::parts([
                    InputGroupAddon::text("Reset"),
                    InputGroupAddon::parts([InputGroupAddon::button(
                        "clear",
                        "Clear",
                        move |button, _, _| {
                            let owner = owner.clone();
                            button.track_focus(&focus).disabled(false).on_click(
                                move |_, window, cx| {
                                    let _ = owner.update(cx, |this, cx| {
                                        this.calls += 1;
                                        this.state.update(cx, |state, cx| {
                                            state.set_value("", window, cx)
                                        });
                                        cx.notify();
                                    });
                                },
                            )
                        },
                    )]),
                ]),
            ))
            .child(crate::Button::new("after", "After"))
    }
}
#[gpui_kit::test]
fn addon_actions_keep_base_focus_callbacks_and_root_disabled_gating(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|window, cx| {
        let state = cx.new(|cx| InputState::new("Search", window, cx));
        let events = cx.subscribe(&state, |this: &mut Actions, _, event, cx| {
            if matches!(event, crate::InputEvent::Focus) {
                this.editor_focus_events += 1;
                cx.notify();
            }
        });
        Actions {
            state,
            focus: cx.focus_handle(),
            calls: 0,
            size: Size::Base,
            editor_focus_events: 0,
            _events: events,
        }
    });
    cx.update(|window, cx| {
        window.activate_window();
        window.render_frame(cx);
        window.render_frame(cx);
        let state = view.read(cx).state.clone();
        state.update(cx, |state, cx| state.set_value("café 🦀", window, cx));
        window.render_frame(cx);
        assert_eq!(
            window.find("group-content").role(),
            Some(gpui_kit::Role::Group)
        );
        assert_eq!(
            window.find("control").role(),
            Some(gpui_kit::Role::TextInput)
        );
        assert_eq!(window.find("clear").role(), Some(gpui_kit::Role::Button));
        let text = window.find(("addon-item", 0_usize)).bounds();
        let action = window.find("clear").bounds();
        assert_eq!(action.left() - text.right(), px(6.));
        assert_eq!(action.center().y, text.center().y);
        assert_eq!(
            window.find("addon-end").bounds().right() - action.right(),
            px(4.)
        );
        window.click(("addon-item", 0_usize), cx);
        assert!(
            state.read(cx).focus_handle(cx).is_focused(window),
            "passive mixed-addon content still focuses editor"
        );
        view.update(cx, |view, _| view.editor_focus_events = 0);
        window.click("clear", cx);
        window.render_frame(cx);
        assert_eq!(view.read(cx).calls, 1);
        assert_eq!(state.read(cx).value(cx).as_ref(), "");
        assert!(
            view.read(cx).focus.is_focused(window),
            "pointer must retain action focus"
        );
        assert!(!state.read(cx).focus_handle(cx).is_focused(window));
        assert_eq!(
            view.read(cx).editor_focus_events,
            0,
            "action mouse down must not briefly focus editor"
        );
        window.press("space", cx);
        assert_eq!(view.read(cx).calls, 2, "Space activates exactly once");
        window.press("enter", cx);
        assert_eq!(view.read(cx).calls, 3, "Enter activates exactly once");
        for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
            crate::set_appearance(appearance, cx);
            window.render_frame(cx);
            let bounds = window.find("surface").bounds();
            let width = crate::theme(cx).effects.input_focus_ring_width;
            assert!(
                window.painted_quads().iter().any(|q| q.bounds
                    == bounds.dilate(width).scale(window.scale_factor())
                    && q.border_color == crate::theme(cx).colors.focus.opacity(0.5)),
                "action focus must light shared ring"
            );
        }
        for (size, group_height, button_height) in [
            (Size::Xs, 24., 20.),
            (Size::Sm, 28., 20.),
            (Size::Base, 36., 26.),
            (Size::Lg, 44., 36.),
        ] {
            view.update(cx, |view, cx| {
                view.size = size;
                cx.notify();
            });
            window.render_frame(cx);
            assert_eq!(
                window.find("surface").bounds().size.height,
                px(group_height)
            );
            assert_eq!(window.find("clear").bounds().size.height, px(button_height));
            assert!(
                window.find("control").bounds().right() <= window.find("clear").bounds().left()
            );
        }
        window.focus_prev(cx);
        assert!(state.read(cx).focus_handle(cx).is_focused(window));
        window.focus_next(cx);
        assert!(view.read(cx).focus.is_focused(window));
        state.update(cx, |state, cx| state.set_disabled(true, cx));
        window.render_frame(cx);
        window.click("clear", cx);
        window.press("space", cx);
        window.press("enter", cx);
        assert_eq!(
            view.read(cx).calls,
            3,
            "root disabled rejects every path despite explicit child false"
        );
        window.click("before", cx);
        window.focus_next(cx);
        window.render_frame(cx);
        assert_eq!(
            window.find("after").focused(),
            Some(true),
            "disabled editor and action are skipped"
        );
    });
}

struct Mount {
    state: Option<Entity<InputState>>,
}
impl Render for Mount {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let mut root = div().w(px(200.));
        if let Some(state) = &self.state {
            let target = state.downgrade();
            root = root.child(
                InputGroup::new("mounted", state).end(InputGroupAddon::button(
                    "clear",
                    "Clear",
                    move |button, _, _| {
                        let target = target.clone();
                        button.on_click(move |_, window, cx| {
                            let _ = target.update(cx, |state, cx| state.set_value("", window, cx));
                        })
                    },
                )),
            );
        }
        root
    }
}
#[gpui_kit::test]
fn weak_action_factory_releases_input_and_focus_subscriptions_after_unmount(
    cx: &mut TestAppContext,
) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|window, cx| Mount {
        state: Some(cx.new(|cx| InputState::new("Search", window, cx))),
    });
    let target = cx.read(|cx| view.read(cx).state.as_ref().unwrap().downgrade());
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.render_frame(cx);
        window.click("clear", cx);
        view.update(cx, |view, cx| {
            view.state = None;
            cx.notify();
        });
        window.render_frame(cx);
        window.render_frame(cx);
    });
    cx.run_until_parked();
    assert!(
        target.upgrade().is_none(),
        "unmount must release retained factory, editor and focus subscriptions"
    );
}

struct ReorderedParts {
    state: Entity<InputState>,
    before: bool,
}
impl Render for ReorderedParts {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let action = InputGroupAddon::button("stable-action", "Action", |button, _, _| button);
        let text = InputGroupAddon::text("Info");
        let parts = if self.before {
            vec![text, action]
        } else {
            vec![action, text]
        };
        div()
            .w(px(240.))
            .child(InputGroup::new("reordered", &self.state).end(InputGroupAddon::parts(parts)))
    }
}
#[gpui_kit::test]
fn addon_reordering_preserves_default_base_action_focus(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|window, cx| ReorderedParts {
        state: cx.new(|cx| InputState::new("Query", window, cx)),
        before: true,
    });
    cx.update(|window, cx| {
        window.activate_window();
        window.render_frame(cx);
        window.click("stable-action", cx);
        window.render_frame(cx);
        assert!(window.find("stable-action").focused() == Some(true));
        view.update(cx, |view, cx| {
            view.before = false;
            cx.notify();
        });
        window.render_frame(cx);
        assert!(
            window.find("stable-action").focused() == Some(true),
            "passive ordering must not replace the Base keyed focus handle"
        );
    });
}

struct Zones {
    state: Entity<InputState>,
    size: Size,
    hybrid: bool,
    narrow: bool,
    calls: usize,
    ghost: bool,
}
impl Render for Zones {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let owner = cx.entity().downgrade();
        let mut group = InputGroup::new("zones", &self.state).size(self.size);
        if self.hybrid {
            group = group
                .start(InputGroupAddon::text("/api/"))
                .end(InputGroupAddon::button(
                    "zone-clear",
                    "Clear",
                    |button, _, _| button,
                ));
        }
        div()
            .tab_group()
            .w(px(if self.narrow { 180. } else { 360. }))
            .child(
                group
                    .button(
                        "submit",
                        "Submit",
                        if self.ghost {
                            crate::button::Variant::Ghost
                        } else {
                            crate::button::Variant::Secondary
                        },
                        move |button, _, _| {
                            let owner = owner.clone();
                            button.disabled(false).on_click(move |_, _, cx| {
                                let _ = owner.update(cx, |view, cx| {
                                    view.calls += 1;
                                    cx.notify();
                                });
                            })
                        },
                    )
                    .button(
                        "zone-more",
                        "More",
                        if self.ghost {
                            crate::button::Variant::Ghost
                        } else {
                            crate::button::Variant::Secondary
                        },
                        |button, _, _| button,
                    ),
            )
    }
}
#[gpui_kit::test]
fn joined_zones_keep_source_geometry_focus_and_base_activation(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|window, cx| Zones {
        state: cx.new(|cx| InputState::new("Query", window, cx)),
        size: Size::Base,
        hybrid: false,
        narrow: false,
        calls: 0,
        ghost: false,
    });
    let state = cx.read(|cx| view.read(cx).state.clone());
    cx.update(|window, cx| {
        window.activate_window();
        window.render_frame(cx);
        window.render_frame(cx);
        for hybrid in [false, true] {
            view.update(cx, |view, cx| {
                view.hybrid = hybrid;
                cx.notify();
            });
            for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
                crate::set_appearance(appearance, cx);
                for size in [Size::Xs, Size::Sm, Size::Base, Size::Lg] {
                    view.update(cx, |view, cx| {
                        view.size = size;
                        cx.notify();
                    });
                    window.render_frame(cx);
                    let surface = window.find("surface").bounds();
                    let submit = window.find("submit").bounds();
                    let more = window.find("zone-more").bounds();
                    assert_eq!(surface.size.height, height(size));
                    assert_eq!(submit.size.height, height(size));
                    assert_eq!(more.size.height, height(size));
                    assert_eq!(surface.right() - submit.left(), px(1.));
                    assert_eq!(submit.right() - more.left(), px(1.));
                    assert_eq!(surface.center().y, submit.center().y);
                    state.read(cx).focus_handle(cx).focus(window, cx);
                    window.render_frame(cx);
                    assert_zone_focus_border(
                        window,
                        surface,
                        crate::theme(cx).colors.focus.opacity(0.5),
                    );
                    if hybrid {
                        window.focus_next(cx);
                        window.render_frame(cx);
                        assert_eq!(window.find("zone-clear").focused(), Some(true));
                        assert_zone_focus_border(
                            window,
                            surface,
                            crate::theme(cx).colors.focus.opacity(0.5),
                        );
                        window.focus_next(cx);
                        window.render_frame(cx);
                        assert_eq!(window.find("submit").focused(), Some(true));
                    }
                    let calls = view.read(cx).calls;
                    window.click("submit", cx);
                    window.render_frame(cx);
                    assert_eq!(view.read(cx).calls, calls + 1, "pointer activates once");
                    assert_eq!(window.find("submit").focused(), Some(true));
                    assert!(!state.read(cx).focus_handle(cx).is_focused(window));
                    assert_zone_focus_border(window, surface, crate::theme(cx).colors.line);
                    window.press("space", cx);
                    window.render_frame(cx);
                    assert_eq!(view.read(cx).calls, calls + 2, "Space activates once");
                    assert_zone_focus_border(
                        window,
                        submit,
                        crate::theme(cx).colors.focus.opacity(0.5),
                    );
                }
            }
        }
        for ghost in [true, false] {
            view.update(cx, |view, cx| {
                view.ghost = ghost;
                cx.notify();
            });
            window.render_frame(cx);
            assert_eq!(
                window.find("submit").focused(),
                Some(true),
                "mode changes preserve direct action focus"
            );
        }
        let calls = view.read(cx).calls;
        window.press("enter", cx);
        assert_eq!(view.read(cx).calls, calls + 1);
        view.update(cx, |view, cx| {
            view.narrow = true;
            cx.notify();
        });
        window.render_frame(cx);
        assert!(window.find("zone-more").bounds().right() <= px(180.));
        state.update(cx, |state, cx| state.set_disabled(true, cx));
        window.render_frame(cx);
        window.click("submit", cx);
        window.press("space", cx);
        window.press("enter", cx);
        assert_eq!(
            view.read(cx).calls,
            calls + 1,
            "root availability is atomic"
        );
    });
}
fn assert_zone_focus_border(
    window: &mut gpui_kit::Window,
    bounds: gpui_kit::Bounds<gpui_kit::Pixels>,
    color: gpui_kit::Hsla,
) {
    assert!(
        window
            .painted_quads()
            .iter()
            .any(|quad| quad.bounds == bounds.scale(window.scale_factor())
                && quad.border_color == color),
        "zone must paint its own 1px border with expected focus color"
    );
}

struct GroupField {
    state: Entity<InputState>,
    required: Option<bool>,
    error: Option<bool>,
    label: bool,
}
impl Render for GroupField {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let mut group = InputGroup::new("group-field", &self.state)
            .label(self.label)
            .description("Account recovery only");
        if let Some(required) = self.required {
            group = group.required(required);
        }
        if let Some(show) = self.error {
            group = group.error_visible("Invalid phone", show);
        }
        div()
            .w(px(240.))
            .child(group)
            .child(crate::Button::new("field-outside", "Outside"))
    }
}
#[gpui_kit::test]
fn input_group_field_optional_and_hidden_error_preserve_editor_contract(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|window, cx| GroupField {
        state: cx.new(|cx| InputState::new("Phone", window, cx)),
        required: None,
        error: None,
        label: true,
    });
    let state = cx.read(|cx| view.read(cx).state.clone());
    cx.update(|window, cx| {
        window.activate_window();
        window.render_frame(cx);
        for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
            crate::set_appearance(appearance, cx);
            for required in [None, Some(false), Some(true)] {
                view.update(cx, |view, cx| {
                    view.required = required;
                    cx.notify();
                });
                window.render_frame(cx);
                assert_eq!(
                    window.find("label").label(),
                    Some(if required == Some(false) {
                        "Phone (optional)"
                    } else {
                        "Phone"
                    })
                );
                assert_eq!(
                    window.find("message").label(),
                    Some("Account recovery only")
                );
                assert_eq!(
                    window.find("surface").bounds().top() - window.find("label").bounds().bottom(),
                    px(8.)
                );
                window.click("label", cx);
                assert!(state.read(cx).focus_handle(cx).is_focused(window));
            }
        }
        window.input("café 🦀", cx);
        for show in [true, false] {
            view.update(cx, |view, cx| {
                view.error = Some(show);
                cx.notify();
            });
            window.render_frame(cx);
            assert_eq!(
                window
                    .try_find("message")
                    .map(|message| message.label().map(str::to_owned)),
                if show {
                    Some(Some("Invalid phone".to_owned()))
                } else {
                    None
                }
            );
            assert_eq!(state.read(cx).value(cx).as_ref(), "café 🦀");
        }
        view.update(cx, |view, cx| {
            view.label = false;
            view.error = None;
            cx.notify();
        });
        window.render_frame(cx);
        assert!(window.try_find("label").is_none());
        assert_eq!(
            window.find("message").label(),
            Some("Account recovery only")
        );
        view.update(cx, |view, cx| {
            view.label = true;
            cx.notify();
        });
        state.update(cx, |state, cx| state.set_disabled(true, cx));
        window.render_frame(cx);
        window.click("field-outside", cx);
        window.render_frame(cx);
        assert_eq!(window.find("field-outside").focused(), Some(true));
        window.click("label", cx);
        window.render_frame(cx);
        assert!(!state.read(cx).focus_handle(cx).is_focused(window));
        assert_eq!(
            window.find("field-outside").focused(),
            Some(true),
            "disabled label must preserve outside focus"
        );
    });
}
