use gpui_kit::{
    Context, IntoElement, ParentElement, Render, SharedString, Styled, Subscription, Window, div,
    px,
};
use gpui_kumo::{Badge, Link, Text, badge, link::Variant, theme};

pub(super) struct Links {
    navigations: usize,
    target: SharedString,
    _theme: Subscription,
}

impl Links {
    pub(super) fn new(cx: &mut Context<Self>) -> Self {
        Self {
            navigations: 0,
            target: "No navigation yet".into(),
            _theme: cx.observe_global::<gpui_kumo::Theme>(|_, cx| cx.notify()),
        }
    }
}

impl Render for Links {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        super::panel(theme(cx), "Link · application-owned navigation")
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .items_center()
                    .gap(px(16.))
                    .children(
                        [
                            ("inline", "Documentation", Variant::Inline),
                            ("current", "Inherited color", Variant::Current),
                            ("plain", "Plain link", Variant::Plain),
                        ]
                        .map(|(id, label, variant)| {
                            Link::new(id, label, format!("app://{id}"))
                                .variant(variant)
                                .external_icon(id == "inline")
                                .on_navigate(cx.listener(
                                    |this, request: &gpui_kumo::link::NavigationRequest, _, cx| {
                                        this.navigations += 1;
                                        this.target = request.href.clone();
                                        cx.notify();
                                    },
                                ))
                        }),
                    ),
            )
            .child(
                div()
                    .flex()
                    .gap(px(12.))
                    .child(
                        Link::new("badge-first", "Release notes", "app://release")
                            .badge(
                                Badge::new("badge", "Release notes")
                                    .variant(badge::Variant::Outline),
                            )
                            .on_navigate(cx.listener(
                                |this, request: &gpui_kumo::link::NavigationRequest, _, cx| {
                                    this.navigations += 1;
                                    this.target = request.href.clone();
                                    cx.notify();
                                },
                            )),
                    )
                    .child(
                        Link::new("badge-second", "Beta documentation", "app://beta").badge(
                            Badge::new("badge", "Beta documentation").variant(badge::Variant::Beta),
                        ),
                    )
                    .child(
                        Link::new("unavailable", "Unavailable", "app://disabled").disabled(true),
                    ),
            )
            .child(
                Link::new(
                    "unicode",
                    "café 🦀 — a long documentation link in a narrow column",
                    "app://unicode",
                )
                .w(px(220.))
                .external_icon(true),
            )
            .child(Text::new(
                "navigation-count",
                format!("{} navigations · {}", self.navigations, self.target),
            ))
    }
}
