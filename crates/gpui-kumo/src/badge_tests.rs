use super::*;
use gpui_kit::{
    BorderStyle, Context, Render, Subscription, TestAppContext, canvas, test::TestWindowExt,
};
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

const VARIANTS: [(Variant, &str); 16] = [
    (Variant::Primary, "primary"),
    (Variant::Secondary, "secondary"),
    (Variant::Error, "error"),
    (Variant::Warning, "warning"),
    (Variant::Success, "success"),
    (Variant::Info, "info"),
    (Variant::Beta, "beta"),
    (Variant::Outline, "outline"),
    (Variant::Red, "red"),
    (Variant::Green, "green"),
    (Variant::Neutral, "neutral"),
    (Variant::Orange, "orange"),
    (Variant::Purple, "purple"),
    (Variant::Teal, "teal"),
    (Variant::TealSubtle, "teal-subtle"),
    (Variant::Blue, "blue"),
];

#[gpui_kit::test]
fn filled_variants_preserve_content_width_typography_and_theme_roles(cx: &mut TestAppContext) {
    struct Matrix {
        styles: Rc<RefCell<BTreeMap<&'static str, gpui_kit::TextStyle>>>,
        _theme: Subscription,
    }
    impl Render for Matrix {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .flex()
                .flex_col()
                .w(px(300.))
                .font_family("Consumer Font")
                .line_height(px(31.))
                .children(VARIANTS.map(|(variant, id)| {
                    let styles = Rc::clone(&self.styles);
                    Badge::new(id, id).variant(variant).rich_content(
                        canvas(
                            move |_, window, _| {
                                styles.borrow_mut().insert(id, window.text_style());
                            },
                            |_, _, _, _| {},
                        )
                        .w(px(30.))
                        .h(px(16.)),
                    )
                }))
        }
    }
    cx.update(crate::init);
    let styles = Rc::new(RefCell::new(BTreeMap::new()));
    let (_, cx) = cx.add_window_view(|_, cx| Matrix {
        styles: Rc::clone(&styles),
        _theme: cx.observe_global::<crate::Theme>(|_, cx| cx.notify()),
    });
    for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
        cx.update(|window, cx| {
            crate::set_appearance(appearance, cx);
            window.render_frame(cx);
            let theme = crate::theme(cx);
            let quads = window.painted_quads();
            for (variant, id) in VARIANTS {
                let border = matches!(variant, Variant::Beta | Variant::Outline);
                let bounds = window.find(id).bounds();
                assert_eq!(
                    bounds.size,
                    gpui_kit::size(
                        px(if border { 48. } else { 46. }),
                        px(if border { 22. } else { 20. })
                    )
                );
                assert_eq!(window.find(id).role(), Some(Role::Label));
                assert_eq!(window.find(id).label(), Some(id));
                let style = &styles.borrow()[id];
                assert_eq!(style.font_family.as_ref(), "Consumer Font");
                assert_eq!(style.font_size, px(12.).into());
                assert_eq!(style.font_weight, FontWeight::MEDIUM);
                assert_eq!(style.line_height, px(16.).into());
                let painted = quads
                    .iter()
                    .find(|quad| quad.bounds == bounds.scale(window.scale_factor()));
                match variant {
                    Variant::Primary => assert_eq!(
                        painted.unwrap().background.as_solid(),
                        Some(theme.badge.inverted)
                    ),
                    Variant::Error => assert_eq!(style.color, theme.text.danger),
                    Variant::Success => assert_eq!(style.color, theme.text.success),
                    Variant::Warning => assert_eq!(style.color, theme.text.warning),
                    Variant::Info => assert_eq!(style.color, theme.text.info),
                    Variant::Orange => assert_eq!(style.color, theme.badge.orange_foreground),
                    Variant::Beta => assert_eq!(painted.unwrap().border_style, BorderStyle::Dashed),
                    Variant::Outline => {
                        assert!(
                            quads
                                .iter()
                                .any(|quad| quad.bounds == bounds.scale(window.scale_factor())
                                    && quad.border_color == theme.colors.fill)
                        )
                    }
                    Variant::TealSubtle => {
                        assert!(painted.is_none());
                        assert_eq!(style.color, theme.badge.teal_subtle_foreground);
                    }
                    _ => {}
                }
            }
            assert_ne!(
                theme.text.success, theme.text.link,
                "Badge success is a distinct semantic role from Text's success/link recipe"
            );
        });
    }
}

#[gpui_kit::test]
fn dot_icon_and_caller_overrides_keep_source_spacing_and_full_names(cx: &mut TestAppContext) {
    struct Slots;
    impl Render for Slots {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div().flex().flex_col().w(px(100.)).children([
                Badge::dot("healthy", "café 🦀", Variant::Success).into_any_element(),
                Badge::dot("no-dot", "café 🦀", Variant::Primary).into_any_element(),
                Badge::new("icon-label", "Verified")
                    .icon(div().size(px(12.)))
                    .into_any_element(),
                Badge::new("override", "A long café 🦀 label never wraps")
                    .py(px(5.))
                    .bg(gpui_kit::red())
                    .into_any_element(),
                Badge::new("empty", "").into_any_element(),
            ])
        }
    }
    cx.update(crate::init);
    let (_, cx) = cx.add_window_view(|_, _| Slots);
    cx.update(|window, cx| {
        window.render_frame(cx);
        let root = window.find("healthy").bounds();
        let dot = window.within("healthy").find("dot");
        assert_eq!(dot.bounds().size, gpui_kit::size(px(7.), px(7.)));
        assert_eq!(dot.bounds().origin.x - root.origin.x, px(8.));
        assert_eq!(dot.role(), None);
        assert_eq!(root.size.height, px(20.));
        assert!(window.within("no-dot").try_find("dot").is_none());
        assert_eq!(
            root.size.width - window.find("no-dot").bounds().size.width,
            px(13.)
        );
        let icon = window.within("icon-label").find("icon").bounds();
        assert_eq!(icon.size, gpui_kit::size(px(12.), px(16.)));
        assert_eq!(
            icon.origin.x - window.find("icon-label").bounds().origin.x,
            px(6.)
        );
        let bounds = window.find("override").bounds();
        assert!(
            bounds.size.width > px(100.),
            "No-wrap/no-shrink survives a narrow parent"
        );
        assert_eq!(bounds.size.height, px(26.));
        assert_eq!(
            window.find("override").label(),
            Some("A long café 🦀 label never wraps")
        );
        assert!(
            window
                .painted_quads()
                .iter()
                .any(|quad| quad.bounds == bounds.scale(window.scale_factor())
                    && quad.background.as_solid() == Some(gpui_kit::red()))
        );
        assert_eq!(window.find("empty").label(), Some(""));
        assert_eq!(window.find("empty").bounds().size.height, px(20.));
    });
}

#[gpui_kit::test]
fn transparent_dot_and_link_hover_use_outside_border_only_rings(cx: &mut TestAppContext) {
    struct Rings;
    impl Render for Rings {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .flex()
                .flex_col()
                .p(px(12.))
                .gap(px(12.))
                .child(Badge::dot("dot-badge", "Healthy", Variant::Success))
                .child(
                    div()
                        .id("link")
                        .test_support()
                        .group("link-group")
                        .w(px(150.))
                        .h(px(40.))
                        .child(
                            Badge::new("linked-badge", "Linked")
                                .variant(Variant::Outline)
                                .text_color(gpui_kit::green())
                                .link_hover_group("link-group"),
                        ),
                )
                .child(div().id("outside").test_support().size(px(20.)))
        }
    }
    cx.update(crate::init);
    let (_, cx) = cx.add_window_view(|_, _| Rings);
    cx.update(|window, cx| {
        window.render_frame(cx);
        let scale = window.scale_factor();
        let dot = window.find("dot-badge").bounds();
        let ring = window
            .painted_quads()
            .into_iter()
            .find(|quad| quad.bounds == dot.dilate(px(1.)).scale(scale))
            .expect("Dot needs a real outside outline, not a filled shadow");
        assert_eq!(ring.background.as_solid().unwrap().a, 0.);
        assert_eq!(ring.border_color, crate::theme(cx).colors.hairline);
        assert_eq!(
            ring.corner_radii,
            gpui_kit::Corners::all(px(11.).scale(scale)),
            "Custom paint must clamp pill radii to the quad"
        );
        assert_eq!(
            ring.border_widths,
            gpui_kit::Edges::all(px(1.).scale(scale))
        );
        let linked = window.find("linked-badge").bounds();
        let hover_bounds = linked.dilate(px(1.)).scale(scale);
        assert!(
            !window
                .painted_quads()
                .iter()
                .any(|quad| quad.bounds == hover_bounds)
        );
        window.hover("link", cx);
        window.render_frame(cx);
        let ring = window
            .painted_quads()
            .into_iter()
            .find(|quad| quad.bounds == hover_bounds)
            .expect("Ancestor hover adds a ring outside the outlined badge's border box");
        assert_eq!(ring.background.as_solid().unwrap().a, 0.);
        assert_eq!(
            ring.border_color,
            gpui_kit::green(),
            "Ring follows caller current color"
        );
        assert_eq!(
            window.find("linked-badge").bounds(),
            linked,
            "Hover ring consumes no space"
        );
        window.hover("outside", cx);
        window.render_frame(cx);
        assert!(
            !window
                .painted_quads()
                .iter()
                .any(|quad| quad.bounds == hover_bounds)
        );
    });
}
