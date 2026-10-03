use super::*;
use gpui_kit::{Entity, Render, TestAppContext, VisualTestContext, div, test::TestWindowExt};
struct Harness {
    clicks: usize,
    copies: usize,
    cancel: bool,
    disabled: bool,
    value: SharedString,
    group: bool,
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let click = cx.entity().downgrade();
        let copy = click.clone();
        div()
            .flex()
            .flex_col()
            .w(px(220.))
            .gap(px(24.))
            .child(
                InlineCopyText::new("first", "visible café 🦀 identifier with long text")
                    .value(self.value.clone())
                    .disabled(self.disabled)
                    .group_active(self.group)
                    .labels("Copy identifier", "Identifier copied")
                    .on_click(move |_, window, cx| {
                        let _ = click.update(cx, |v, _| {
                            v.clicks += 1;
                            if v.cancel {
                                window.prevent_default();
                            }
                        });
                    })
                    .on_copy(move |_, cx| {
                        let _ = copy.update(cx, |v, _| v.copies += 1);
                    }),
            )
            .child(InlineCopyText::rich(
                "second",
                "Styled identifier",
                "second payload",
                gpui_kit::StyledText::new("Styled identifier"),
            ))
            .child(crate::Button::new("outside", "Outside"))
    }
}
fn harness(cx: &mut TestAppContext) -> (Entity<Harness>, &mut VisualTestContext) {
    cx.update(crate::init);
    cx.add_window_view(|_, _| Harness {
        clicks: 0,
        copies: 0,
        cancel: false,
        disabled: false,
        value: "full café 🦀\npayload".into(),
        group: false,
    })
}
#[gpui_kit::test]
fn pointer_keyboard_cancellation_and_current_disabled_state(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    cx.update(|window, _| window.activate_window());
    cx.run_until_parked();
    for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
        cx.update(|window, cx| {
            crate::set_appearance(appearance, cx);
            view.update(cx, |v, _| {
                v.disabled = false;
                v.cancel = false;
            });
            window.render_frame(cx);
            window.click("first", cx);
            window.render_frame(cx);
            assert_eq!(window.find("first").label(), Some("Identifier copied"));
            assert_eq!(window.find("first").focused(), Some(true));
            window.press("space", cx);
            window.press("enter", cx);
            assert_eq!(
                cx.read_from_clipboard().unwrap().text().unwrap(),
                "full café 🦀\npayload"
            );
            view.update(cx, |v, _| v.cancel = true);
            window.render_frame(cx);
            window.click("first", cx);
            window.press("space", cx);
            window.press("enter", cx);
            view.update(cx, |v, _| v.disabled = true);
            window.render_frame(cx);
            window.click("first", cx);
            window.press("space", cx);
            window.press("enter", cx);
        });
    }
    cx.read(|cx| {
        assert_eq!(view.read(cx).clicks, 12);
        assert_eq!(view.read(cx).copies, 6);
    });
}
#[gpui_kit::test]
fn latest_activation_reset_instances_and_payload_replacement(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.click("first", cx);
    });
    cx.run_until_parked();
    cx.background_executor
        .advance_clock(Duration::from_millis(1000));
    cx.run_until_parked();
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.click("first", cx);
        window.click("second", cx);
        assert!(
            window.find("second").bounds().size.width < px(220.),
            "compact control must not stretch over its column"
        );
        assert_eq!(
            cx.read_from_clipboard().unwrap().text().unwrap(),
            "second payload"
        );
    });
    cx.run_until_parked();
    cx.background_executor
        .advance_clock(Duration::from_millis(1000));
    cx.run_until_parked();
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert_eq!(window.find("first").label(), Some("Identifier copied"));
        assert_eq!(window.find("second").label(), Some("Copied"));
        view.update(cx, |v, _| v.value = "replacement".into());
        window.render_frame(cx);
        assert_eq!(window.find("first").label(), Some("Copy identifier"));
        assert_eq!(window.find("second").label(), Some("Copied"));
    });
    cx.background_executor
        .advance_clock(Duration::from_millis(500));
    cx.run_until_parked();
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert_eq!(window.find("second").label(), Some("Copy to clipboard"));
        window.click("first", cx);
        assert_eq!(
            cx.read_from_clipboard().unwrap().text().unwrap(),
            "replacement"
        );
    });
}
#[gpui_kit::test]
fn rich_content_and_glyph_geometry_across_text_recipes(cx: &mut TestAppContext) {
    cx.update(crate::init);
    struct Sizes;
    impl Render for Sizes {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let styles = [
                Style::default(),
                Style::Mono {
                    tone: text::MonoTone::Default,
                    size: text::MonoSize::Large,
                },
                Style::Copy {
                    tone: text::Tone::Default,
                    size: text::Size::Xs,
                    bold: false,
                },
                Style::Copy {
                    tone: text::Tone::Secondary,
                    size: text::Size::Sm,
                    bold: true,
                },
                Style::Copy {
                    tone: text::Tone::Success,
                    size: text::Size::Base,
                    bold: false,
                },
                Style::Copy {
                    tone: text::Tone::Error,
                    size: text::Size::Lg,
                    bold: true,
                },
            ];
            div()
                .w(px(130.))
                .flex()
                .flex_col()
                .items_start()
                .gap(px(16.))
                .children(styles.into_iter().enumerate().map(|(i, style)| {
                    InlineCopyText::new(("copy", i), "long café 🦀 value for truncation")
                        .style(style)
                        .group_active(true)
                }))
        }
    }
    let (_, cx) = cx.add_window_view(|_, _| Sizes);
    for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
        cx.update(|window, cx| {
            crate::set_appearance(appearance, cx);
            window.render_frame(cx);
            for i in 0usize..6 {
                let root = window.find(("copy", i));
                let icon = window.within(("copy", i)).find("copy-icon");
                assert_eq!(root.role(), Some(gpui_kit::Role::Button));
                assert_eq!(icon.bounds().size, gpui_kit::size(px(14.), px(14.)));
                assert_eq!(icon.bounds().center().y, root.bounds().center().y);
                assert!(icon.bounds().right() <= root.bounds().right());
                assert!(root.bounds().size.width <= px(130.));
            }
        });
    }
}

#[gpui_kit::test]
fn intrinsic_width_keeps_taller_row_sibling_centred(cx: &mut TestAppContext) {
    cx.update(crate::init);
    struct Row;
    impl Render for Row {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .flex()
                .items_center()
                .gap(px(16.))
                .child(InlineCopyText::new("compact", "ID"))
                .child(crate::Button::new("tall", "Save").size(crate::button::Size::Lg))
        }
    }
    let (_, cx) = cx.add_window_view(|_, _| Row);
    for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
        cx.update(|window, cx| {
            crate::set_appearance(appearance, cx);
            window.render_frame(cx);
            let compact = window.find("compact").bounds();
            let tall = window.find("tall").bounds();
            assert!(compact.size.height < tall.size.height);
            assert_eq!(compact.center().y, tall.center().y);
        });
    }
}
