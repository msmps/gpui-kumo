use gpui_kit::{Div, ParentElement, Styled, div, px};
use gpui_kumo::{Button, Empty, Icon, Theme, empty::Size};

pub(super) fn panel(theme: &Theme) -> Div {
    super::panel(theme, "Empty · composition and command copy")
        .child(
            Empty::new("packages", "No packages found")
                .size(Size::Sm)
                .icon(Icon::new("workspace.svg").size(px(48.)))
                .description("Get started by installing your first package.")
                .command_line("npm install @cloudflare/kumo")
                .contents(Button::new("examples", "See examples")),
        )
        .child(
            div().flex().gap(px(16.)).children(
                [Size::Sm, Size::Base, Size::Lg]
                    .into_iter()
                    .enumerate()
                    .map(|(i, size)| {
                        div()
                            .flex_1()
                            .min_w_0()
                            .child(Empty::new(("minimal", i), "Nothing here").size(size))
                    }),
            ),
        )
        .child(
            div().w(px(280.)).child(
                Empty::new("narrow", "No results found")
                    .size(Size::Sm)
                    .description(
                        "Try adjusting your search or filter to find what you are looking for.",
                    )
                    .command_line("echo café 🦀 — a long command stays horizontally scrollable"),
            ),
        )
}
