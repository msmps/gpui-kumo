use gpui_kit::*;
use gpui_kumo::{
    breadcrumbs::{BreadcrumbClipboard, BreadcrumbCurrent},
    select::{SelectOption, SelectState, SelectValue},
    *,
};
struct Review {
    select: Entity<SelectState<u32>>,
    _theme: Subscription,
}
impl Render for Review {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = theme(cx).clone();
        div()
            .size_full()
            .bg(t.colors.base)
            .text_color(t.text.default)
            .font_family(t.typography.font_family.clone())
            .p(px(24.))
            .flex()
            .flex_col()
            .gap(px(20.))
            .child(
                div()
                    .flex()
                    .gap(px(12.))
                    .child(Button::new("theme", "Toggle theme").on_click(|_, _, cx| {
                        let a = if theme(cx).appearance == Appearance::Light {
                            Appearance::Dark
                        } else {
                            Appearance::Light
                        };
                        set_appearance(a, cx);
                    }))
                    .child(Button::new("narrow", "Narrow / wide").on_click(|_, w, _| {
                        w.resize(size(
                            px(if w.viewport_size().width > px(600.) {
                                520.
                            } else {
                                1040.
                            }),
                            px(640.),
                        ));
                    }))
                    .child(
                        Button::new("reduce", "Reduced motion")
                            .on_click(|_, _, cx| cx.set_reduce_motion(!cx.reduce_motion())),
                    ),
            )
            .child(
                Breadcrumbs::new("trail")
                    .h(px(56.))
                    .mr(px(0.))
                    .px(px(8.))
                    .rounded(px(6.))
                    .bg(t.colors.tint)
                    .link(Link::new("home", "Home", "/"))
                    .separator()
                    .link(Link::new("projects", "Projects", "/projects"))
                    .separator()
                    .current(BreadcrumbCurrent::new(
                        "current",
                        "Current café with a long readable name",
                    ))
                    .clipboard(BreadcrumbClipboard::new(
                        "crumb-copy",
                        "https://example.test/café",
                    )),
            )
            .child(Meter::new("meter-default", "Storage café", 65.))
            .child(
                Meter::new(
                    "meter-custom",
                    "Styled quota with a long Unicode label café",
                    25.,
                )
                .custom_value("25 / 100 GB")
                .track_style(
                    StyleRefinement::default()
                        .h(px(12.))
                        .rounded(px(4.))
                        .bg(t.colors.tint),
                )
                .indicator_style(
                    StyleRefinement::default()
                        .rounded(px(4.))
                        .bg(t.colors.success),
                ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(16.))
                    .child(
                        InlineCopyText::new("copy", "café identifier 0c239dd2")
                            .labels("Copy identifier", "Identifier copied"),
                    )
                    .child(Button::new("save", "Save")),
            )
            .child(div().w(px(54.)).child(Select::new("page", &self.select)))
    }
}
fn main() {
    gpui_kit::application().run(|cx| {
        gpui_kumo::init(cx);
        let handle = gpui_kit::open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(1040.), px(640.)),
                    cx,
                ))),
                titlebar: Some(TitlebarOptions {
                    title: Some("Quick win review".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            cx,
            |_, cx| {
                cx.new(|cx| Review {
                    select: cx.new(|cx| {
                        SelectState::new(
                            "Page",
                            SelectValue::Single(Some(3)),
                            (1..=10)
                                .map(|n| SelectOption::new(format!("number-{n}"), n, n.to_string()))
                                .collect(),
                            cx,
                        )
                    }),
                    _theme: cx.observe_global::<Theme>(|_, cx| cx.notify()),
                })
            },
        )
        .unwrap();
        cx.activate(true);
        if std::env::args().any(|a| a == "--capture") {
            cx.spawn(async move |cx| {
                use gpui_kit::test::TestWindowExt;
                for (name, appearance) in [("light", Appearance::Light), ("dark", Appearance::Dark)] {
                    for width in [1040., 520.] {
                        for selected in [3, 10] {
                            cx.background_executor().timer(std::time::Duration::from_millis(250)).await;
                            cx.update(|cx| handle.0.update(cx, |_, window, cx| {
                                set_appearance(appearance, cx);
                                cx.set_reduce_motion(true);
                                window.resize(size(px(width), px(640.)));
                                window.bounds_changed(cx);
                                handle.1.read(cx).select.clone().update(cx, |s, cx| {
                                    s.set_open(false, window, cx);
                                    s.set_value(SelectValue::Single(Some(selected)), cx);
                                });
                            })).unwrap();
                            cx.background_executor().timer(std::time::Duration::from_millis(250)).await;
                            cx.update(|cx| handle.0.update(cx, |_, window, cx| {
                                window.render_frame(cx);
                                window.render_frame(cx);
                                window.hover("trail", cx);
                                window.render_frame(cx);
                                window.click("crumb-copy", cx);
                                window.press("space", cx);
                                window.render_frame(cx);
                                assert_eq!(window.find("crumb-copy").focused(), Some(true));
                                assert_eq!(window.find("crumb-copy").label(), Some("Copied"));
                                window.render_to_image().unwrap().save(format!("tools/validation/quick-wins/{name}-{width}-breadcrumb.png")).unwrap();
                                window.click("copy", cx);
                                window.press("space", cx);
                                window.render_frame(cx);
                                assert_eq!(window.find("copy").label(), Some("Identifier copied"));
                                window.render_to_image().unwrap().save(format!("tools/validation/quick-wins/{name}-{width}-feedback-{selected}.png")).unwrap();
                                handle.1.read(cx).select.clone().update(cx, |s, cx| s.set_open(true, window, cx));
                                window.render_frame(cx);
                                window.render_frame(cx);
                                eprintln!("{name} {width} selected={selected} popup_width={:?} row_height={:?}", window.find("surface").bounds().size.width, window.find(format!("number-{selected}")).bounds().size.height);
                                window.render_to_image().unwrap().save(format!("tools/validation/quick-wins/{name}-{width}-selected-{selected}.png")).unwrap();
                            })).unwrap();
                        }
                    }
                }
                cx.update(|cx| cx.quit());
            }).detach();
        }
    });
}
