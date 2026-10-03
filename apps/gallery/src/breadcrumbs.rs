use gpui_kit::{Context, IntoElement, ParentElement, Render, Styled, Window, div, px, svg};
use gpui_kumo::{
    BreadcrumbClipboard, BreadcrumbCurrent, Breadcrumbs, Button, Link, breadcrumbs::Size, theme,
};
pub struct Trails {
    loading: bool,
    route: String,
    payload: String,
}
impl Default for Trails {
    fn default() -> Self {
        Self {
            loading: false,
            route: "No navigation yet".into(),
            payload: "https://example.test/projects/café".into(),
        }
    }
}
impl Render for Trails {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let home_icon = |color| {
            svg()
                .data(include_bytes!("../assets/house.svg").as_slice())
                .size(px(16.))
                .text_color(color)
        };
        let owner = cx.entity().downgrade();
        let parent = owner.clone();
        let root = Breadcrumbs::new("breadcrumbs-main")
            .link(
                Link::new("crumb-home", "Home", "/")
                    .rich_content(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(4.))
                            .child(home_icon(theme(cx).text.subtle))
                            .child("Home"),
                    )
                    .on_navigate(move |r, _, cx| {
                        let _ = owner.update(cx, |v, cx| {
                            v.route = r.href.to_string();
                            cx.notify();
                        });
                    }),
            )
            .separator()
            .link(
                Link::new("crumb-docs", "Projects", "/projects").on_navigate(move |r, _, cx| {
                    let _ = parent.update(cx, |v, cx| {
                        v.route = r.href.to_string();
                        cx.notify();
                    });
                }),
            )
            .separator()
            .current(
                BreadcrumbCurrent::new(
                    "crumb-current",
                    "Long current project café 🦀 with a full accessible name",
                )
                .loading(self.loading),
            )
            .clipboard(BreadcrumbClipboard::new("crumb-copy", self.payload.clone()));
        crate::panel(
            theme(cx),
            "Breadcrumbs · responsive navigation and retained copy",
        )
        .child(root)
        .child(
            Breadcrumbs::new("breadcrumbs-small")
                .size(Size::Sm)
                .link(Link::new("small-home", "Home", "/"))
                .separator()
                .current(BreadcrumbCurrent::new("small-current", "Compact page"))
                .clipboard(BreadcrumbClipboard::new("small-copy", "compact café 🦀")),
        )
        .child(
            Breadcrumbs::new("breadcrumbs-root").current(
                BreadcrumbCurrent::new("root-current", "Worker Analytics")
                    .icon(home_icon(theme(cx).text.default)),
            ),
        )
        .child(
            Breadcrumbs::new("breadcrumbs-disabled")
                .link(Link::new("disabled-link", "Unavailable project", "/disabled").disabled(true))
                .separator()
                .current(BreadcrumbCurrent::new("disabled-current", "Current page"))
                .clipboard(BreadcrumbClipboard::new("disabled-copy", "disabled").disabled(true)),
        )
        .child(div().child(format!("Route: {}", self.route)))
        .child(
            div()
                .flex()
                .flex_wrap()
                .gap(px(8.))
                .child(
                    Button::new("breadcrumbs-loading", "Toggle loading").on_click(cx.listener(
                        |v, _, _, cx| {
                            v.loading = !v.loading;
                            cx.notify();
                        },
                    )),
                )
                .child(
                    Button::new("breadcrumbs-payload", "Replace copy payload").on_click(
                        cx.listener(|v, _, _, cx| {
                            v.payload = "https://example.test/updated".into();
                            cx.notify();
                        }),
                    ),
                ),
        )
    }
}
