use gpui_kit::{Context, Div, ParentElement, Styled, div, px};
use gpui_kumo::{
    Button, Icon, Theme,
    button::{Shape, Size, Variant},
};

use super::{Gallery, panel};

pub(super) fn interaction_panel(gallery: &Gallery, theme: &Theme, cx: &Context<Gallery>) -> Div {
    panel(theme, "Activation and availability")
        .child(
            div()
                .flex()
                .items_center()
                .gap(theme.spacing.sixteen)
                .child(
                    Button::new("activate", "Save changes")
                        .variant(Variant::Primary)
                        .disabled(gallery.disabled)
                        .loading(gallery.loading)
                        .on_click(cx.listener(|gallery, _, _, cx| {
                            gallery.activations += 1;
                            cx.notify();
                        })),
                )
                .child(
                    Button::new(
                        "toggle-disabled",
                        if gallery.disabled {
                            "Enable action"
                        } else {
                            "Disable action"
                        },
                    )
                    .on_click(cx.listener(|gallery, _, _, cx| {
                        gallery.disabled = !gallery.disabled;
                        cx.notify();
                    })),
                )
                .child(
                    Button::new(
                        "toggle-loading",
                        if gallery.loading {
                            "Stop loading"
                        } else {
                            "Start loading"
                        },
                    )
                    .on_click(cx.listener(|gallery, _, _, cx| {
                        gallery.loading = !gallery.loading;
                        cx.notify();
                    })),
                )
                .child(
                    div()
                        .text_color(theme.text.subtle)
                        .child(format!("Activations: {}", gallery.activations)),
                ),
        )
        .child(div().text_color(theme.text.subtle).child(
            "Click, or focus with Tab and press Enter or Space. Each activation counts once.",
        ))
}

pub(super) fn variant_panel(theme: &Theme) -> Div {
    panel(theme, "Variants")
        .child(
            div()
                .flex()
                .gap(theme.spacing.sixteen)
                .text_color(theme.text.subtle)
                .child(div().w(px(160.)).child("Variant"))
                .children(
                    ["Enabled", "Disabled", "Loading"].map(|label| div().w(px(200.)).child(label)),
                ),
        )
        .children(
            [
                ("Primary", Variant::Primary),
                ("Secondary", Variant::Secondary),
                ("Ghost", Variant::Ghost),
                ("Destructive", Variant::Destructive),
                ("Secondary destructive", Variant::SecondaryDestructive),
                ("Outline", Variant::Outline),
            ]
            .into_iter()
            .enumerate()
            .map(|(index, (label, variant))| {
                div()
                    .flex()
                    .items_center()
                    .gap(theme.spacing.sixteen)
                    .child(div().w(px(160.)).child(label))
                    .children(
                        [(false, false), (true, false), (false, true)]
                            .into_iter()
                            .enumerate()
                            .map(|(state, (disabled, loading))| {
                                div().w(px(200.)).child(
                                    Button::new(("variant", index * 3 + state), label)
                                        .variant(variant)
                                        .disabled(disabled)
                                        .loading(loading),
                                )
                            }),
                    )
            }),
        )
}

pub(super) fn size_panel(theme: &Theme) -> Div {
    panel(theme, "Sizes, shapes and icon slots")
        .children(
            [
                (Size::Xs, "Extra small"),
                (Size::Sm, "Small"),
                (Size::Base, "Base"),
                (Size::Lg, "Large"),
            ]
            .into_iter()
            .enumerate()
            .map(|(index, (size, label))| {
                let icon = || {
                    Icon::new("workspace.svg").size(px(if size == Size::Xs { 10. } else { 14. }))
                };
                div()
                    .flex()
                    .items_center()
                    .gap(theme.spacing.sixteen)
                    .child(div().w(px(160.)).child(label))
                    .child(
                        div().w(px(200.)).child(
                            Button::new(("size", index), "Create project")
                                .size(size)
                                .leading_icon(icon())
                                .variant(Variant::Primary),
                        ),
                    )
                    .child(Button::icon(("square", index), "Create project", icon()).size(size))
                    .child(
                        Button::icon(("circle", index), "Create project", icon())
                            .size(size)
                            .shape(Shape::Circle),
                    )
                    .child(
                        Button::new(("trailing", index), "Next")
                            .size(size)
                            .trailing_icon(icon()),
                    )
            }),
        )
        .child(
            div()
                .flex()
                .gap(theme.spacing.sixteen)
                .child(Button::new("open-trigger", "Open trigger").open(true))
                .child(
                    Button::new(
                        "long-label",
                        "Create a project with a longer descriptive name",
                    )
                    .variant(Variant::Outline),
                ),
        )
}
