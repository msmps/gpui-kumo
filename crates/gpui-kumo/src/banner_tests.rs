use super::*;
use gpui_kit::{Context, Render, Subscription, TestAppContext, canvas, test::TestWindowExt};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

const VARIANTS: [(Variant, &str); 4] = [
    (Variant::Info, "info"),
    (Variant::Alert, "alert"),
    (Variant::Error, "error"),
    (Variant::Secondary, "secondary"),
];

#[gpui_kit::test]
fn full_width_banner_in_a_scrolling_gallery_keeps_a_usable_text_column(cx: &mut TestAppContext) {
    struct GalleryComposition;
    impl Render for GalleryComposition {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .id("gallery")
                .size_full()
                .overflow_y_scroll()
                .flex()
                .flex_col()
                .p(px(32.))
                .child(
                    div().flex().flex_col().flex_shrink_0().p(px(24.)).child(
                        Banner::new("banner")
                            .title("Update available")
                            .description(
                                "Contextual description — café 🦀 — with three action treatments.",
                            )
                            .icon(div().size(px(16.)))
                            .action(Action::new("continue", "Continue"))
                            .action(
                                Action::new("details", "Details").variant(ActionVariant::Secondary),
                            )
                            .action(Action::new("later", "Later").variant(ActionVariant::Ghost)),
                    ),
                )
        }
    }
    cx.update(crate::init);
    let (_, cx) = cx.add_window_view(|_, _| GalleryComposition);
    cx.update(|window, cx| {
        for width in [1040., 640., 1040.] {
            window.resize(gpui_kit::size(px(width), px(832.)));
            window.render_frame(cx);
            let root = window.find("banner").bounds();
            let title = window.within("banner").find("title").bounds();
            assert!(
                title.size.width > px(100.),
                "A flexible full-width banner must retain a usable text column: {title:?}"
            );
            assert!(
                root.size.height < px(100.),
                "A short banner must not wrap one character per line: {root:?}"
            );
        }
    });
}

#[gpui_kit::test]
fn rich_description_and_action_refinements_keep_typography_and_ring_geometry(
    cx: &mut TestAppContext,
) {
    struct StyledMessage {
        description: Rc<RefCell<Option<gpui_kit::TextStyle>>>,
        action: Rc<RefCell<Option<gpui_kit::TextStyle>>>,
    }
    impl Render for StyledMessage {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let description = Rc::clone(&self.description);
            let action = Rc::clone(&self.action);
            Banner::new("styled")
                .w(px(500.))
                .font_family("Message Font")
                .text_color(gpui_kit::green())
                .title("Message")
                .description_content(
                    canvas(
                        move |_, window, _| {
                            *description.borrow_mut() = Some(window.text_style());
                        },
                        |_, _, _, _| {},
                    )
                    .size(px(16.)),
                )
                .action(
                    Action::new("action", "Styled")
                        .variant(ActionVariant::Secondary)
                        .w(px(96.))
                        .h(px(40.))
                        .border_2()
                        .rounded_full()
                        .text_size(px(18.))
                        .line_height(px(24.))
                        .font_family("Action Font")
                        .leading_icon(
                            canvas(
                                move |_, window, _| {
                                    *action.borrow_mut() = Some(window.text_style());
                                },
                                |_, _, _, _| {},
                            )
                            .size(px(8.)),
                        ),
                )
        }
    }
    cx.update(crate::init);
    let description = Rc::new(RefCell::new(None));
    let action = Rc::new(RefCell::new(None));
    let (_, cx) = cx.add_window_view(|_, _| StyledMessage {
        description: Rc::clone(&description),
        action: Rc::clone(&action),
    });
    cx.update(|window, cx| {
        window.render_frame(cx);
        let text = description.borrow();
        let text = text.as_ref().unwrap();
        assert_eq!(text.font_family.as_ref(), "Message Font");
        assert_eq!(text.font_size, px(13.).into());
        assert_eq!(text.line_height, relative(1.375));
        assert_eq!(text.color, gpui_kit::green());
        let style = action.borrow();
        let style = style.as_ref().unwrap();
        assert_eq!(style.font_family.as_ref(), "Action Font");
        assert_eq!(style.font_size, px(18.).into());
        assert_eq!(style.color, crate::theme(cx).colors.info);
        let bounds = window.within("styled").find("action").bounds();
        assert_eq!(bounds.size, gpui_kit::size(px(96.), px(40.)));
        let scale = window.scale_factor();
        let ring = window
            .painted_quads()
            .into_iter()
            .find(|quad| {
                quad.bounds == bounds.dilate(px(1.)).scale(scale)
                    && quad.border_color == crate::theme(cx).colors.info.opacity(0.5)
            })
            .unwrap();
        assert_eq!(
            ring.corner_radii,
            gpui_kit::Corners::all(px(21.).scale(scale))
        );
        assert_eq!(ring.background.as_solid().unwrap().a, 0.);
    });
}

#[gpui_kit::test]
fn structured_variants_and_sizes_keep_source_geometry_and_distinct_icon_roles(
    cx: &mut TestAppContext,
) {
    struct Matrix {
        styles: Rc<RefCell<Vec<gpui_kit::TextStyle>>>,
        _theme: Subscription,
    }
    impl Render for Matrix {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .flex()
                .flex_col()
                .w(px(400.))
                .gap(px(8.))
                .font_family("Consumer Font")
                .children(VARIANTS.map(|(variant, id)| {
                    let styles = Rc::clone(&self.styles);
                    Banner::new(id)
                        .variant(variant)
                        .title("Message")
                        .description("café 🦀")
                        .icon(
                            canvas(
                                move |_, window, _| styles.borrow_mut().push(window.text_style()),
                                |_, _, _, _| {},
                            )
                            .size(px(16.)),
                        )
                }))
                .child(
                    Banner::new("compact")
                        .size(Size::Sm)
                        .title("Short")
                        .description("Description")
                        .icon(div().size(px(16.))),
                )
                .child(Banner::new("description-only").description("No title"))
                .child(Banner::new("empty"))
                .child(
                    Banner::new("override")
                        .p(px(20.))
                        .rounded(px(12.))
                        .bg(gpui_kit::green())
                        .title("Override"),
                )
        }
    }
    cx.update(crate::init);
    let styles = Rc::new(RefCell::new(Vec::new()));
    let (_, cx) = cx.add_window_view(|_, cx| Matrix {
        styles: Rc::clone(&styles),
        _theme: cx.observe_global::<Theme>(|_, cx| cx.notify()),
    });
    for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
        cx.update(|window, cx| {
            crate::set_appearance(appearance, cx);
            styles.borrow_mut().clear();
            window.render_frame(cx);
            let theme = crate::theme(cx);
            for (index, (variant, id)) in VARIANTS.into_iter().enumerate() {
                let root = window.find(id).bounds();
                let icon = window.within(id).find("icon").bounds();
                let title = window.within(id).find("title");
                let description = window.within(id).find("description").bounds();
                assert_eq!(root.size.width, px(400.));
                assert_eq!(icon.origin.x - root.origin.x, px(16.));
                assert_eq!(icon.origin.y - root.origin.y, px(12.));
                assert!((f32::from(icon.size.height) - 19.25).abs() <= 0.5);
                assert_eq!(title.label(), Some("Message"));
                assert_eq!(title.role(), Some(Role::Label));
                assert!(
                    (f32::from(description.origin.y - title.bounds().bottom()) - 2.).abs() <= 0.5
                );
                assert_eq!(styles.borrow()[index].color, variant.icon_color(theme));
                assert_eq!(styles.borrow()[index].font_family.as_ref(), "Consumer Font");
                let painted = window
                    .painted_quads()
                    .into_iter()
                    .find(|quad| quad.bounds == root.scale(window.scale_factor()))
                    .unwrap();
                assert_eq!(
                    painted.background.as_solid(),
                    Some(variant.background(theme))
                );
                assert_eq!(
                    painted.corner_radii,
                    gpui_kit::Corners::all(px(8.).scale(window.scale_factor()))
                );
                assert_eq!(window.find(id).role(), None);
            }
            let compact = window.find("compact").bounds();
            let icon = window.within("compact").find("icon").bounds();
            assert!((f32::from(icon.size.height) - 16.25).abs() <= 0.5);
            assert_eq!(icon.origin.x - compact.origin.x, px(12.));
            let title = window.within("compact").find("title").bounds();
            let description = window.within("compact").find("description").bounds();
            assert!((f32::from(description.origin.x - title.right()) - 6.).abs() <= 0.5);
            let row = window.within("description-only").find("row").bounds();
            let text = window
                .within("description-only")
                .find("description")
                .bounds();
            assert_eq!(text.origin.y - row.origin.y, px(1.));
            assert_eq!(window.find("empty").bounds().size.height, px(24.));
            assert!(window.within("empty").try_find("row").is_none());
            let root = window.find("override").bounds();
            assert_eq!(
                window.within("override").find("title").bounds().origin.x - root.origin.x,
                px(20.)
            );
        });
    }
}

#[gpui_kit::test]
fn accent_actions_inherit_size_palette_and_single_activation(cx: &mut TestAppContext) {
    struct Actions {
        count: Rc<Cell<usize>>,
        disabled: bool,
        loading: bool,
        _theme: Subscription,
    }
    impl Render for Actions {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .flex()
                .flex_col()
                .gap(px(8.))
                .w(px(500.))
                .children(VARIANTS.map(|(variant, id)| {
                    let mut banner = Banner::new(id).variant(variant).title("Message");
                    for (treatment, action_id) in [
                        (ActionVariant::Primary, "primary"),
                        (ActionVariant::Secondary, "outline"),
                        (ActionVariant::Ghost, "ghost"),
                    ] {
                        let count = Rc::clone(&self.count);
                        banner = banner.action(
                            Action::new(action_id, "Do it")
                                .variant(treatment)
                                .disabled(self.disabled)
                                .loading(self.loading)
                                .on_click(move |_, _, _| count.set(count.get() + 1)),
                        );
                    }
                    banner
                }))
                .child(
                    Banner::new("compact")
                        .size(Size::Sm)
                        .title("Compact")
                        .action(Action::new("small", "Small"))
                        .action(
                            Action::icon("dismiss", "Dismiss", div().size(px(10.)))
                                .variant(ActionVariant::Ghost),
                        ),
                )
        }
    }
    cx.update(crate::init);
    let count = Rc::new(Cell::new(0));
    let (view, cx) = cx.add_window_view(|_, cx| Actions {
        count: Rc::clone(&count),
        disabled: false,
        loading: false,
        _theme: cx.observe_global::<Theme>(|_, cx| cx.notify()),
    });
    cx.update(|window, cx| {
        for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
            crate::set_appearance(appearance, cx);
            window.render_frame(cx);
            for (variant, id) in VARIANTS {
                let scale = window.scale_factor();
                let primary = window.within(id).find("primary").bounds();
                let theme = crate::theme(cx);
                let emphasis = match variant {
                    Variant::Info => &theme.banner.info,
                    Variant::Alert => &theme.banner.warning,
                    Variant::Error => &theme.destructive,
                    Variant::Secondary => &theme.banner.secondary,
                };
                assert!(
                    window
                        .painted_quads()
                        .iter()
                        .any(|quad| quad.bounds == primary.scale(scale)
                            && quad.background == emphasis.gradient(false))
                );
                let outline = window.within(id).find("outline").bounds();
                let ring = if variant == Variant::Secondary {
                    theme.colors.focus.opacity(0.2)
                } else {
                    variant.icon_color(theme).opacity(0.5)
                };
                assert!(
                    window
                        .painted_quads()
                        .iter()
                        .any(|quad| quad.bounds == outline.dilate(px(1.)).scale(scale)
                            && quad.border_color == ring
                            && quad.background.as_solid().unwrap().a == 0.)
                );
                let hover = if variant == Variant::Secondary {
                    theme.colors.contrast
                } else {
                    variant.icon_color(theme)
                }
                .opacity(0.1);
                window.within(id).hover("outline", cx);
                window.render_frame(cx);
                assert!(
                    window
                        .painted_quads()
                        .iter()
                        .any(|quad| quad.bounds == outline.scale(scale)
                            && quad.background.as_solid() == Some(hover))
                );
            }
        }
        window.render_frame(cx);
        for (_, id) in VARIANTS {
            for action in ["primary", "outline", "ghost"] {
                let node = window.within(id).find(action);
                assert_eq!(node.bounds().size.height, px(26.));
                assert_eq!(node.role(), Some(Role::Button));
                assert_eq!(node.label(), Some("Do it"));
            }
        }
        assert_eq!(
            window.within("compact").find("small").bounds().size.height,
            px(20.)
        );
        assert_eq!(
            window.within("compact").find("dismiss").bounds().size,
            gpui_kit::size(px(14.), px(14.))
        );
        window.within("error").click("primary", cx);
        window.press("enter", cx);
        window.press("space", cx);
        assert_eq!(count.get(), 3);
        for (disabled, loading) in [(true, false), (false, true)] {
            view.update(cx, |view, cx| {
                view.disabled = disabled;
                view.loading = loading;
                cx.notify();
            });
            crate::set_appearance(crate::Appearance::Dark, cx);
            window.render_frame(cx);
            window.within("error").click("primary", cx);
            window.press("enter", cx);
            window.press("space", cx);
            assert_eq!(count.get(), 3);
        }
    });
}

#[gpui_kit::test]
fn compact_link_action_uses_content_flow_and_preserves_navigation(cx: &mut TestAppContext) {
    struct Compact(Rc<Cell<usize>>);
    impl Render for Compact {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let count = Rc::clone(&self.0);
            div()
                .w(px(160.))
                .child(
                    Banner::new("compact")
                        .size(Size::Sm)
                        .title("café 🦀 — long title")
                        .description("A narrow description wraps.")
                        .link_action(
                            Link::new("link", "Learn more", "/learn")
                                .on_navigate(move |_, _, _| count.set(count.get() + 1)),
                        ),
                )
                .child(
                    Banner::new("base")
                        .title("Base")
                        .link_action(Link::new("link", "Details", "/details")),
                )
        }
    }
    cx.update(crate::init);
    let count = Rc::new(Cell::new(0));
    let (_, cx) = cx.add_window_view(|_, _| Compact(Rc::clone(&count)));
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert!(window.within("compact").try_find("actions").is_none());
        assert!(window.within("base").try_find("actions").is_some());
        let link = window
            .within("compact")
            .within("link")
            .find("label")
            .bounds();
        assert!(link.bottom() <= window.find("compact").bounds().bottom());
        window.within("compact").within("link").click("label", cx);
        window.press("enter", cx);
        window.press("space", cx);
        assert_eq!(count.get(), 3);
    });
}
