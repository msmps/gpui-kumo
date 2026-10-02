use super::*;
use gpui_kit::{Context, Render, Subscription, TestAppContext, px, test::TestWindowExt};
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

struct Harness {
    line_height: gpui_kit::Pixels,
    _theme: Subscription,
}

#[gpui_kit::test]
fn rich_content_receives_inherited_fonts_weights_and_current_theme_tones(cx: &mut TestAppContext) {
    struct Probe {
        styles: Rc<RefCell<BTreeMap<&'static str, gpui_kit::TextStyle>>>,
        _theme: Subscription,
    }
    impl Render for Probe {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let styles = self.styles.clone();
            div()
                .flex()
                .flex_col()
                .font_family("Consumer Font")
                .font_weight(FontWeight::BOLD)
                .line_height(px(31.))
                .children(
                    [
                        ("body", Style::default()),
                        (
                            "bold",
                            Style::Copy {
                                tone: Tone::Default,
                                size: Size::Base,
                                bold: true,
                            },
                        ),
                        (
                            "secondary",
                            Style::Copy {
                                tone: Tone::Secondary,
                                size: Size::Sm,
                                bold: false,
                            },
                        ),
                        (
                            "success",
                            Style::Copy {
                                tone: Tone::Success,
                                size: Size::Lg,
                                bold: false,
                            },
                        ),
                        (
                            "error",
                            Style::Copy {
                                tone: Tone::Error,
                                size: Size::Xs,
                                bold: false,
                            },
                        ),
                        ("heading", Style::Heading(HeadingSize::Large)),
                        (
                            "mono",
                            Style::Mono {
                                tone: MonoTone::Secondary,
                                size: MonoSize::Large,
                            },
                        ),
                    ]
                    .map(move |(id, style)| {
                        let styles = styles.clone();
                        Text::new(id, id).style(style).rich_content(
                            gpui_kit::canvas(
                                move |_, window, _| {
                                    styles.borrow_mut().insert(id, window.text_style());
                                },
                                |_, _, _, _| {},
                            )
                            .w(px(10.))
                            .h(px(10.)),
                        )
                    }),
                )
        }
    }
    cx.update(|cx| {
        crate::init(cx);
        let mut theme = crate::theme(cx).clone();
        theme.typography.mono_font_family = "Consumer Mono".into();
        crate::set_theme(theme, cx);
    });
    let styles = Rc::new(RefCell::new(BTreeMap::new()));
    let (_, cx) = cx.add_window_view(|_, cx| Probe {
        styles: styles.clone(),
        _theme: cx.observe_global::<crate::Theme>(|_, cx| cx.notify()),
    });
    for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
        cx.update(|window, cx| {
            crate::set_appearance(appearance, cx);
            window.render_frame(cx);
            let captured = styles.borrow();
            for id in ["body", "bold", "secondary", "success", "error", "heading"] {
                assert_eq!(captured[id].font_family.as_ref(), "Consumer Font");
            }
            assert_eq!(captured["body"].font_weight, FontWeight::BOLD);
            assert_eq!(captured["bold"].font_weight, FontWeight::MEDIUM);
            assert_eq!(captured["heading"].font_weight, FontWeight::SEMIBOLD);
            assert_eq!(captured["mono"].font_weight, FontWeight::BOLD);
            assert_eq!(captured["mono"].font_family.as_ref(), "Consumer Mono");
            assert_eq!(captured["body"].font_size, px(14.).into());
            assert_eq!(captured["secondary"].font_size, px(13.).into());
            assert_eq!(captured["success"].font_size, px(16.).into());
            assert_eq!(captured["error"].font_size, px(12.).into());
            assert_eq!(captured["heading"].font_size, px(20.).into());
            assert_eq!(captured["mono"].font_size, px(14.).into());
            let theme = crate::theme(cx);
            assert_eq!(captured["body"].color, theme.text.default);
            assert_eq!(captured["secondary"].color, theme.text.subtle);
            assert_eq!(captured["success"].color, theme.text.link);
            assert_eq!(captured["error"].color, theme.text.danger);
            assert_eq!(captured["mono"].color, theme.text.subtle);
            assert_eq!(window.find("body").label(), Some("body"));
        });
    }
}

impl Render for Harness {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .w(px(120.))
            .flex()
            .flex_col()
            .line_height(self.line_height)
            .child(Text::new("empty", "").truncate(true))
            .child(Text::new("body", "Same text"))
            .child(Text::new("repeated", "Same text"))
            .child(Text::new("heading", "Heading").style(Style::Heading(HeadingSize::Standard)))
            .child(
                Text::new("large", "Title")
                    .style(Style::Heading(HeadingSize::Large))
                    .heading_level(HeadingLevel::Two),
            )
            .child(Text::new("mono", "println!").style(Style::Mono {
                tone: MonoTone::Secondary,
                size: MonoSize::Standard,
            }))
            .child(
                Text::new(
                    "truncated",
                    "café 🦀 — a long line with complete accessible content",
                )
                .truncate(true),
            )
            .child(Text::new(
                "wrapped",
                "café 🦀 — a long line with complete accessible content",
            ))
    }
}

#[gpui_kit::test]
fn typography_inherits_copy_line_height_but_headings_use_their_recipe(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|_, cx| Harness {
        line_height: px(21.),
        _theme: cx.observe_global::<crate::Theme>(|_, cx| cx.notify()),
    });
    cx.update(|window, cx| {
        window.render_frame(cx);
        let empty = window.find("empty");
        assert!(empty.bounds().size.width <= px(120.));
        assert_eq!(empty.bounds().size.height, px(21.));
        assert_eq!(window.find("body").bounds().size.height, px(21.));
        assert_eq!(window.find("mono").bounds().size.height, px(21.));
        assert_eq!(window.find("heading").bounds().size.height, px(24.));
        assert_eq!(window.find("large").bounds().size.height, px(28.));
        view.update(cx, |view, cx| {
            view.line_height = px(30.);
            cx.notify();
        });
        window.render_frame(cx);
        assert_eq!(window.find("body").bounds().size.height, px(30.));
        assert_eq!(window.find("mono").bounds().size.height, px(30.));
        assert_eq!(window.find("heading").bounds().size.height, px(24.));
        assert_eq!(window.find("large").bounds().size.height, px(28.));
    });
}

#[gpui_kit::test]
fn truncation_preserves_full_accessible_content_and_repeated_labels_keep_identity(
    cx: &mut TestAppContext,
) {
    cx.update(crate::init);
    let (_, cx) = cx.add_window_view(|_, cx| Harness {
        line_height: px(21.),
        _theme: cx.observe_global::<crate::Theme>(|_, cx| cx.notify()),
    });
    cx.update(|window, cx| {
        window.render_frame(cx);
        let body = window.find("body");
        let repeated = window.find("repeated");
        assert_eq!(body.label(), Some("Same text"));
        assert_eq!(repeated.label(), Some("Same text"));
        assert_ne!(body.path(), repeated.path());
        assert_eq!(window.find("heading").role(), Some(Role::Label));
        assert_eq!(window.find("large").role(), Some(Role::Heading));
        let truncated = window.find("truncated");
        assert_eq!(truncated.bounds().size.height, px(21.));
        assert!(truncated.bounds().size.width <= px(120.));
        assert_eq!(
            truncated.label(),
            Some("café 🦀 — a long line with complete accessible content")
        );
        assert!(window.find("wrapped").bounds().size.height > truncated.bounds().size.height);
        let bounds = truncated.bounds();
        crate::set_appearance(crate::Appearance::Dark, cx);
        window.render_frame(cx);
        assert_eq!(window.find("truncated").bounds(), bounds);
        assert_eq!(window.find("large").role(), Some(Role::Heading));
    });
}
