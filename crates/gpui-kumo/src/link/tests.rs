use super::*;
use gpui_kit::{
    Context, KeyDownEvent, KeyUpEvent, Keystroke, Modifiers, Render, Subscription, TestAppContext,
    VisualTestContext, point, px, test::TestWindowExt,
};
use std::{cell::RefCell, rc::Rc};

fn activate(cx: &mut VisualTestContext, key: &str) {
    let keystroke = Keystroke::parse(key).unwrap();
    cx.simulate_event(KeyDownEvent {
        keystroke: keystroke.clone(),
        is_held: false,
        prefer_character_input: false,
    });
    cx.simulate_event(KeyUpEvent { keystroke });
}

#[gpui_kit::test]
fn navigation_uses_updated_target_once_and_preserves_native_focus(cx: &mut TestAppContext) {
    struct Owner {
        href: SharedString,
        disabled: bool,
        events: Rc<RefCell<Vec<String>>>,
    }
    impl Render for Owner {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let navigation = Rc::clone(&self.events);
            let observer = Rc::clone(&self.events);
            div().tab_group().child(
                Link::new("route", "Settings", self.href.clone())
                    .disabled(self.disabled)
                    .size(px(100.))
                    .on_navigate(move |request, _, _| {
                        navigation.borrow_mut().push(format!(
                            "{}:{}",
                            request.href,
                            if matches!(request.activation, ClickEvent::Keyboard(_)) {
                                "keyboard"
                            } else {
                                "pointer"
                            }
                        ));
                    })
                    .on_activate(move |_, _, _| observer.borrow_mut().push("observe".into())),
            )
        }
    }
    cx.update(crate::init);
    let events = Rc::new(RefCell::new(Vec::new()));
    let (view, cx) = cx.add_window_view(|_, _| Owner {
        href: "app://first".into(),
        disabled: false,
        events: Rc::clone(&events),
    });
    cx.simulate_click(point(px(10.), px(10.)), Modifiers::default());
    let focus = cx.update(|window, cx| window.focused(cx).unwrap());
    view.update(cx, |owner, cx| {
        owner.href = "app://updated".into();
        cx.notify();
    });
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert!(focus.is_focused(window));
    });
    activate(cx, "enter");
    activate(cx, "space");
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert!(
            window
                .painted_quads()
                .iter()
                .any(|quad| quad.border_color == crate::theme(cx).colors.focus),
            "The observed keyed handle must paint the actual native keyboard focus"
        );
    });
    assert_eq!(
        &*events.borrow(),
        &[
            "app://first:pointer",
            "observe",
            "app://updated:keyboard",
            "observe",
            "app://updated:keyboard",
            "observe",
        ]
    );
    let pending_key = Keystroke::parse("space").unwrap();
    cx.simulate_event(KeyDownEvent {
        keystroke: pending_key.clone(),
        is_held: false,
        prefer_character_input: false,
    });
    view.update(cx, |owner, cx| {
        owner.disabled = true;
        cx.notify();
    });
    cx.update(|window, cx| window.render_frame(cx));
    cx.simulate_event(KeyUpEvent {
        keystroke: pending_key,
    });
    cx.simulate_click(point(px(10.), px(10.)), Modifiers::default());
    activate(cx, "enter");
    activate(cx, "space");
    assert_eq!(events.borrow().len(), 6);
    assert_eq!(cx.opened_url(), None);
    view.update(cx, |owner, cx| {
        owner.disabled = false;
        cx.notify();
    });
    cx.update(|window, cx| window.render_frame(cx));
    cx.simulate_click(point(px(10.), px(10.)), Modifiers::default());
    assert_eq!(events.borrow().len(), 8);
}

#[gpui_kit::test]
fn variants_inherit_typography_and_paint_theme_underlines(cx: &mut TestAppContext) {
    struct Matrix {
        styles: Rc<RefCell<Vec<gpui_kit::TextStyle>>>,
        _theme: Subscription,
    }
    impl Render for Matrix {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .flex()
                .flex_col()
                .gap(px(8.))
                .text_size(px(20.))
                .line_height(px(30.))
                .text_color(gpui_kit::green())
                .children(
                    [
                        ("inline", Variant::Inline),
                        ("current", Variant::Current),
                        ("plain", Variant::Plain),
                    ]
                    .map(|(id, variant)| {
                        let styles = Rc::clone(&self.styles);
                        Link::new(id, "café 🦀", "/docs")
                            .variant(variant)
                            .external_icon(true)
                            .rich_content(
                                div()
                                    .id("content")
                                    .test_support()
                                    .child("café 🦀")
                                    .child(canvas(
                                        |_, _, _| {},
                                        move |_, _, window, _| {
                                            styles.borrow_mut().push(window.text_style())
                                        },
                                    )),
                            )
                    }),
                )
        }
    }
    cx.update(crate::init);
    let styles = Rc::new(RefCell::new(Vec::new()));
    let (_, cx) = cx.add_window_view(|_, cx| Matrix {
        styles: Rc::clone(&styles),
        _theme: cx.observe_global::<crate::Theme>(|_, cx| cx.notify()),
    });
    for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
        cx.update(|window, cx| {
            window.dispatch_event(
                gpui_kit::PlatformInput::MouseMove(gpui_kit::MouseMoveEvent {
                    position: point(px(500.), px(500.)),
                    pressed_button: None,
                    modifiers: Modifiers::default(),
                }),
                cx,
            );
            crate::set_appearance(appearance, cx);
            styles.borrow_mut().clear();
            window.render_frame(cx);
            let captured = styles.borrow();
            assert_eq!(captured.len(), 3);
            for style in captured.iter() {
                assert_eq!(style.font_size, px(20.).into());
                assert_eq!(style.line_height, px(30.).into());
            }
            assert_eq!(captured[0].color, crate::theme(cx).text.link);
            assert_eq!(captured[1].color, gpui_kit::green());
            assert!(captured[2].underline.is_none());
            let decoration = captured[0].underline.unwrap();
            assert_eq!(decoration.thickness, px(1.25));
            assert_eq!(
                decoration.color,
                Some(crate::theme(cx).text.link.opacity(
                    if appearance == crate::Appearance::Dark {
                        0.65
                    } else {
                        0.35
                    }
                ))
            );
            assert!(!window.painted_underlines().is_empty());
            let content = window.within("inline").find("content").bounds();
            let icon = window.within("inline").find("external-icon").bounds();
            assert_eq!(icon.size, gpui_kit::size(px(20.), px(20.)));
            assert!(
                (f32::from(icon.origin.x - content.right()) - 3.75).abs() <= 0.5,
                "Em spacing is snapped to the window's physical pixel grid"
            );
            drop(captured);
            for (index, id) in [(0, "inline"), (1, "current"), (2, "plain")] {
                window.within(id).hover("content", cx);
                styles.borrow_mut().clear();
                window.render_frame(cx);
                let hovered = styles.borrow()[index].clone();
                if index == 2 {
                    assert_eq!(hovered.color, crate::theme(cx).text.link.opacity(0.7));
                    assert!(hovered.underline.is_none());
                } else {
                    assert_eq!(hovered.underline.unwrap().color, Some(hovered.color));
                }
            }
            window.dispatch_event(
                gpui_kit::PlatformInput::MouseMove(gpui_kit::MouseMoveEvent {
                    position: point(px(500.), px(500.)),
                    pressed_button: None,
                    modifiers: Modifiers::default(),
                }),
                cx,
            );
            window.render_frame(cx);
        });
    }
}

#[gpui_kit::test]
fn repeated_link_groups_only_outline_the_hovered_badge(cx: &mut TestAppContext) {
    struct Links;
    impl Render for Links {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div().flex().flex_col().p(px(12.)).gap(px(12.)).children(
                ["first", "second"]
                    .map(|id| Link::new(id, id, "/docs").badge(crate::Badge::new("badge", id))),
            )
        }
    }
    cx.update(crate::init);
    let (_, cx) = cx.add_window_view(|_, _| Links);
    cx.update(|window, cx| {
        window.render_frame(cx);
        let first = window.within("first").find("badge").bounds();
        let second = window.within("second").find("badge").bounds();
        for (hovered, other) in [(first, second), (second, first)] {
            window.dispatch_event(
                gpui_kit::PlatformInput::MouseMove(gpui_kit::MouseMoveEvent {
                    position: hovered.center(),
                    pressed_button: None,
                    modifiers: Modifiers::default(),
                }),
                cx,
            );
            window.render_frame(cx);
            let scale = window.scale_factor();
            let quads = window.painted_quads();
            assert!(
                quads
                    .iter()
                    .any(|quad| quad.bounds == hovered.dilate(px(1.)).scale(scale))
            );
            assert!(
                !quads
                    .iter()
                    .any(|quad| quad.bounds == other.dilate(px(1.)).scale(scale))
            );
        }
    });
}

#[gpui_kit::test]
fn narrow_unicode_label_wraps_without_shrinking_external_icon(cx: &mut TestAppContext) {
    struct Narrow;
    impl Render for Narrow {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div().text_size(px(14.)).line_height(px(21.)).child(
                Link::new(
                    "narrow",
                    "café 🦀 — a long documentation link in a narrow column",
                    "/docs",
                )
                .w(px(120.))
                .external_icon(true),
            )
        }
    }
    cx.update(crate::init);
    let (_, cx) = cx.add_window_view(|_, _| Narrow);
    cx.update(|window, cx| {
        window.render_frame(cx);
        let label = window.within("narrow").find("label").bounds();
        let icon = window.within("narrow").find("external-icon").bounds();
        assert!(label.size.height > px(21.));
        assert!(label.size.width < px(120.));
        assert_eq!(icon.size, gpui_kit::size(px(14.), px(14.)));
        assert!(icon.right() <= px(120.5));
    });
}

#[gpui_kit::test]
fn disabled_links_skip_focus_traversal_and_reenter_after_enable(cx: &mut TestAppContext) {
    struct Traversal {
        before: gpui_kit::FocusHandle,
        after: gpui_kit::FocusHandle,
        disabled: bool,
    }
    impl Render for Traversal {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .tab_group()
                .flex()
                .flex_col()
                .child(
                    div()
                        .track_focus(&self.before.clone().tab_stop(true))
                        .size(px(20.)),
                )
                .child(Link::new("route", "Route", "/route").disabled(self.disabled))
                .child(
                    div()
                        .track_focus(&self.after.clone().tab_stop(true))
                        .size(px(20.)),
                )
        }
    }
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|_, cx| Traversal {
        before: cx.focus_handle(),
        after: cx.focus_handle(),
        disabled: true,
    });
    cx.update(|window, cx| {
        window.render_frame(cx);
        view.read(cx).before.clone().focus(window, cx);
        window.focus_next(cx);
        assert!(view.read(cx).after.is_focused(window));
        view.update(cx, |owner, cx| {
            owner.disabled = false;
            cx.notify();
        });
        window.render_frame(cx);
        view.read(cx).before.clone().focus(window, cx);
        window.focus_next(cx);
        assert!(!view.read(cx).after.is_focused(window));
        assert!(!view.read(cx).before.is_focused(window));
        assert!(window.focused(cx).is_some());
        window.focus_next(cx);
        assert!(view.read(cx).after.is_focused(window));
    });
}
