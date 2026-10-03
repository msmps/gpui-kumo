use super::*;
use gpui_kit::{Modifiers, Render, TestAppContext, size, test::TestWindowExt};
struct Harness {
    href: SharedString,
    route: Vec<String>,
    disabled: bool,
    copy: SharedString,
    loading: bool,
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let owner = cx.entity().downgrade();
        let home_owner = owner.clone();
        div().flex().flex_col().w_full().child(
            Breadcrumbs::new("trail")
                .link(
                    Link::new("home", "Home", "/").on_navigate(move |request, _, cx| {
                        let _ =
                            home_owner.update(cx, |v, _| v.route.push(request.href.to_string()));
                    }),
                )
                .separator()
                .link(
                    Link::new("parent", "Projects", self.href.clone())
                        .disabled(self.disabled)
                        .on_navigate(move |request, _, cx| {
                            let _ = owner.update(cx, |v, _| v.route.push(request.href.to_string()));
                        }),
                )
                .separator()
                .current(
                    BreadcrumbCurrent::new(
                        "current",
                        "Long current page café 🦀 with complete accessible label",
                    )
                    .loading(self.loading),
                )
                .clipboard(
                    BreadcrumbClipboard::new("copy", self.copy.clone()).disabled(self.disabled),
                ),
        )
    }
}
#[gpui_kit::test]
fn responsive_parts_current_truncation_and_source_dimensions(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|_, _| Harness {
        href: "/projects".into(),
        route: vec![],
        disabled: false,
        copy: "café 🦀".into(),
        loading: false,
    });
    for (width, home) in [
        (800., true),
        (640., true),
        (639., false),
        (320., false),
        (800., true),
    ] {
        cx.simulate_resize(size(px(width), px(240.)));
        cx.update(|window, cx| {
            cx.set_reduce_motion(true);
            for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
                crate::set_appearance(appearance, cx);
                window.render_frame(cx);
                assert_eq!(window.find("trail").role(), Some(Role::Navigation));
                assert_eq!(window.find("trail").bounds().size.height, px(48.));
                assert_eq!(
                    gpui_kit::base::test_support::snapshots(window)
                        .iter()
                        .any(|s| s.path().contains(&ElementId::from("home"))),
                    home
                );
                assert_eq!(
                    window.find("current").label(),
                    Some("Long current page café 🦀 with complete accessible label")
                );
                let current = window.find("current").bounds();
                let root = window.find("trail").bounds();
                assert!(current.right() <= root.right());
                assert!(window.find("copy").bounds().right() <= root.right());
            }
        });
    }
    view.update(cx, |v, _| v.loading = true);
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert_eq!(window.find("current").bounds().size.width, px(125.));
    });
}
#[gpui_kit::test]
fn navigation_updates_keyboard_and_disabled_paths(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|_, _| Harness {
        href: "/first".into(),
        route: vec![],
        disabled: false,
        copy: "café 🦀".into(),
        loading: false,
    });
    cx.simulate_resize(size(px(800.), px(240.)));
    let target = cx.update(|window, cx| {
        window.render_frame(cx);
        window.within("parent").find("label").bounds().center()
    });
    cx.simulate_click(target, Modifiers::default());
    view.update(cx, |v, cx| {
        v.href = "/updated".into();
        cx.notify();
    });
    cx.update(|window, cx| window.render_frame(cx));
    cx.update(|window, cx| window.press("enter", cx));
    cx.update(|window, cx| window.press("space", cx));
    cx.update(|window, cx| {
        window.render_frame(cx);
        let root = window.find("trail").bounds().scale(window.scale_factor());
        let ring = window
            .painted_quads()
            .into_iter()
            .find(|quad| {
                quad.border_color == theme(cx).colors.focus && quad.border_widths.left.as_f32() > 0.
            })
            .expect("Actual keyboard ring must paint");
        assert!(ring.bounds.left() >= root.left() && ring.bounds.right() <= root.right());
        assert!(ring.bounds.top() >= root.top() && ring.bounds.bottom() <= root.bottom());
    });
    assert_eq!(
        view.read_with(cx, |v, _| v.route.clone()),
        ["/first", "/updated", "/updated"]
    );
    view.update(cx, |v, cx| {
        v.disabled = true;
        cx.notify();
    });
    cx.update(|window, cx| window.render_frame(cx));
    cx.simulate_click(target, Modifiers::default());
    cx.update(|window, cx| window.press("enter", cx));
    cx.update(|window, cx| window.press("space", cx));
    assert_eq!(view.read_with(cx, |v, _| v.route.len()), 3);
}
#[gpui_kit::test]
fn copy_unicode_reset_replacement_and_empty_payload(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|_, _| Harness {
        href: "/projects".into(),
        route: vec![],
        disabled: false,
        copy: "café 🦀".into(),
        loading: false,
    });
    let target = cx.update(|window, cx| {
        window.render_frame(cx);
        window.find("copy").bounds().center()
    });
    cx.simulate_click(target, Modifiers::default());
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert_eq!(window.find("copy").label(), Some("Copied"));
        assert_eq!(
            window.find("copy").focused(),
            Some(true),
            "pointer copy must own focus before keyboard activation"
        );
        assert_eq!(
            cx.read_from_clipboard().unwrap().text(),
            Some("café 🦀".into())
        );
    });
    // A repeat activation restarts the source two-second interval.
    cx.background_executor
        .advance_clock(Duration::from_millis(1000));
    cx.simulate_click(target, Modifiers::default());
    cx.background_executor
        .advance_clock(Duration::from_millis(1999));
    cx.run_until_parked();
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert_eq!(window.find("copy").label(), Some("Copied"));
    });
    cx.background_executor
        .advance_clock(Duration::from_millis(1));
    cx.run_until_parked();
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert_eq!(window.find("copy").label(), Some("Copy"));
    });
    cx.simulate_click(target, Modifiers::default());
    view.update(cx, |v, cx| {
        v.copy = "".into();
        cx.notify();
    });
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert_eq!(window.find("copy").label(), Some("Copy"));
    });
    cx.simulate_click(target, Modifiers::default());
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert_eq!(window.find("copy").label(), Some("Copy"));
        assert_eq!(
            cx.read_from_clipboard().unwrap().text(),
            Some("café 🦀".into())
        );
    });
    view.update(cx, |v, cx| {
        v.copy = "must not copy".into();
        v.disabled = true;
        cx.notify();
    });
    cx.update(|window, cx| {
        window.render_frame(cx);
        cx.write_to_clipboard(ClipboardItem::new_string("untouched".into()));
    });
    cx.simulate_click(target, Modifiers::default());
    cx.update(|window, cx| {
        window.press("enter", cx);
        window.press("space", cx);
        window.render_frame(cx);
        assert_eq!(window.find("copy").label(), Some("Copy"));
        assert_eq!(
            cx.read_from_clipboard().unwrap().text(),
            Some("untouched".into())
        );
    });
}

#[gpui_kit::test]
fn collapsed_focused_ancestor_cannot_activate_and_parent_focus_survives_resize(
    cx: &mut TestAppContext,
) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|_, _| Harness {
        href: "/projects".into(),
        route: vec![],
        disabled: false,
        copy: "café 🦀".into(),
        loading: false,
    });
    cx.simulate_resize(size(px(800.), px(240.)));
    let home = cx.update(|window, cx| {
        window.render_frame(cx);
        window.within("home").find("label").bounds().center()
    });
    cx.simulate_click(home, Modifiers::default());
    assert_eq!(view.read_with(cx, |v, _| v.route.clone()), ["/"]);
    cx.simulate_resize(size(px(320.), px(240.)));
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.press("enter", cx);
        window.press("space", cx);
    });
    assert_eq!(
        view.read_with(cx, |v, _| v.route.clone()),
        ["/"],
        "Collapsed ancestor must not retain registered actions"
    );
    let parent = cx.update(|window, _| window.within("parent").find("label").bounds().center());
    cx.simulate_click(parent, Modifiers::default());
    let focus = cx.update(|window, cx| window.focused(cx).unwrap());
    cx.simulate_resize(size(px(800.), px(240.)));
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert!(focus.is_focused(window));
        window.press("enter", cx);
    });
    assert_eq!(
        view.read_with(cx, |v, _| v.route.clone()),
        ["/", "/projects", "/projects"]
    );
}

#[gpui_kit::test]
fn compact_size_empty_trail_and_extra_reordering_preserve_action_identity(cx: &mut TestAppContext) {
    struct Parts {
        extra: usize,
    }
    impl Render for Parts {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            div()
                .flex()
                .flex_col()
                .w_full()
                .child(Breadcrumbs::new("empty").size(Size::Sm))
                .child(
                    Breadcrumbs::new("parts")
                        .size(Size::Sm)
                        .extra(Button::new("extra", "Extra").on_click(cx.listener(
                            |v, _, _, cx| {
                                v.extra += 1;
                                cx.notify();
                            },
                        )))
                        .link(Link::new("first", "First", "/first"))
                        .separator()
                        .link(Link::new("last", "Last", "/last"))
                        .separator()
                        .current(BreadcrumbCurrent::new("page", "Current")),
                )
        }
    }
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|_, _| Parts { extra: 0 });
    cx.simulate_resize(size(px(800.), px(240.)));
    let extra = cx.update(|window, cx| {
        window.render_frame(cx);
        assert_eq!(window.find("empty").bounds().size.height, px(40.));
        assert_eq!(window.find("parts").bounds().size.height, px(40.));
        let extra = window.find("extra").bounds();
        assert!(extra.right() <= window.within("first").find("label").bounds().left());
        extra.center()
    });
    cx.simulate_click(extra, Modifiers::default());
    cx.simulate_resize(size(px(400.), px(240.)));
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert!(window.find("extra").bounds().left() >= window.find("page").bounds().right());
        assert_eq!(window.find("extra").focused(), Some(true));
        window.press("space", cx);
    });
    assert_eq!(view.read_with(cx, |v, _| v.extra), 2);
}

#[gpui_kit::test]
fn root_style_overrides_preserve_navigation_and_current_semantics(cx: &mut TestAppContext) {
    struct StyledTrail;
    impl Render for StyledTrail {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            Breadcrumbs::new("styled-trail")
                .h(px(56.))
                .mr(px(0.))
                .px(px(8.))
                .rounded(px(6.))
                .bg(theme(cx).colors.tint)
                .current(BreadcrumbCurrent::new("page", "Current café"))
        }
    }
    cx.update(crate::init);
    let (_, cx) = cx.add_window_view(|_, _| StyledTrail);
    for width in [1040., 320.] {
        cx.simulate_resize(size(px(width), px(200.)));
        cx.update(|window, cx| {
            for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
                crate::set_appearance(appearance, cx);
                window.render_frame(cx);
                let trail = window.find("styled-trail");
                assert_eq!(trail.role(), Some(Role::Navigation));
                assert_eq!(trail.bounds().size.height, px(56.));
                let page = window.find("page");
                assert_eq!(page.label(), Some("Current café"));
                assert_eq!(page.bounds().left(), trail.bounds().left() + px(8.));
                assert!((page.bounds().center().y - trail.bounds().center().y).abs() < px(0.1));
            }
        });
    }
}

#[gpui_kit::test]
fn group_hover_copy_fade_reverses_and_reduced_motion_stops_frames(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (_, cx) = cx.add_window_view(|_, _| Harness {
        href: "/projects".into(),
        route: vec![],
        disabled: false,
        copy: "café".into(),
        loading: false,
    });
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.hover("trail", cx);
        window.render_frame(cx);
        assert!(window.simulate_next_frame(cx) > 0);
    });
    cx.background_executor
        .advance_clock(Duration::from_millis(75));
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.dispatch_event(
            gpui_kit::PlatformInput::MouseMove(gpui_kit::MouseMoveEvent {
                position: gpui_kit::point(px(900.), px(500.)),
                pressed_button: None,
                modifiers: Modifiers::default(),
            }),
            cx,
        );
        window.render_frame(cx);
        assert!(window.simulate_next_frame(cx) > 0);
        cx.set_reduce_motion(true);
        window.render_frame(cx);
        window.simulate_next_frame(cx);
        window.render_frame(cx);
        assert_eq!(window.simulate_next_frame(cx), 0);
        window.click("copy", cx);
        window.press("space", cx);
        window.render_frame(cx);
        assert_eq!(window.find("copy").label(), Some("Copied"));
    });
}
