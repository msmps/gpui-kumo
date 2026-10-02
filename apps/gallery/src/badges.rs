use gpui_kit::{Div, InteractiveElement, ParentElement, Styled, div, px, svg};
use gpui_kumo::{Badge, Theme, badge::Variant};

pub(super) fn panel(theme: &Theme) -> Div {
    super::panel(theme, "Badge")
        .child(
            div()
                .flex()
                .flex_wrap()
                .items_center()
                .gap(theme.spacing.eight)
                .children(
                    [
                        (Variant::Primary, "Primary"),
                        (Variant::Secondary, "Secondary"),
                        (Variant::Error, "Error"),
                        (Variant::Warning, "Warning"),
                        (Variant::Success, "Success"),
                        (Variant::Info, "Info"),
                        (Variant::Beta, "Beta"),
                        (Variant::Outline, "Outline"),
                        (Variant::Red, "Red"),
                        (Variant::Green, "Green"),
                        (Variant::Neutral, "Neutral"),
                        (Variant::Orange, "Orange"),
                        (Variant::Purple, "Purple"),
                        (Variant::Teal, "Teal"),
                        (Variant::TealSubtle, "Teal subtle"),
                        (Variant::Blue, "Blue"),
                    ]
                    .map(|(variant, label)| {
                        Badge::new(label, label).variant(variant).self_center()
                    }),
                ),
        )
        .child(
            div()
                .flex()
                .flex_wrap()
                .items_center()
                .gap(theme.spacing.eight)
                .child(Badge::dot("healthy", "Healthy", Variant::Success))
                .child(Badge::dot("warning", "Warning", Variant::Warning))
                .child(Badge::dot("error", "Error", Variant::Error))
                .child(Badge::dot("neutral", "Neutral", Variant::Neutral))
                .child(Badge::dot("none", "No dot", Variant::Primary))
                .child(
                    Badge::new("icon", "With icon")
                        .variant(Variant::Success)
                        .icon(
                            svg()
                                .path("workspace.svg")
                                .size(px(12.))
                                .text_color(theme.text.success),
                        ),
                )
                .child(
                    div().group("badge-hover-example").child(
                        Badge::new("hover-ring", "Hover ring")
                            .variant(Variant::Outline)
                            .link_hover_group("badge-hover-example"),
                    ),
                ),
        )
}
