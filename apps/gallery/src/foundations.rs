use crate::panel::panel;

use gpui_kit::base::TestSupportExt;
use gpui_kit::{
    InteractiveElement, IntoElement, ParentElement, Pixels, Styled, div, prelude::FluentBuilder, px,
};
use gpui_kumo::Theme;

// A conservative650px combined column budget,24px gap and64px gallery padding.
// Stack before intrinsic labels and paired effects have to squeeze.
pub fn panels(theme: &Theme, viewport_width: Pixels) -> impl IntoElement {
    div()
        .id("foundations")
        .test_support()
        .flex()
        .when(viewport_width < px(738.), |this| this.flex_col())
        .gap(px(24.))
        .child(color_panel(theme))
        .child(
            div()
                .id("foundation-details")
                .test_support()
                .min_w_0()
                .flex()
                .flex_col()
                .gap(px(24.))
                .flex_1()
                .child(typography_panel(theme))
                .child(effects_panel(theme)),
        )
}

fn color_panel(theme: &Theme) -> impl IntoElement {
    panel(theme, "Semantic colors")
        .id("foundation-colors")
        .test_support()
        .min_w_0()
        .flex_1()
        .children(
            [
                ("Canvas", theme.colors.canvas),
                ("Base", theme.colors.base),
                ("Control", theme.colors.control),
                ("Tint", theme.colors.tint),
                ("Brand", theme.colors.brand),
                ("Danger", theme.colors.danger),
                ("Line", theme.colors.line),
                ("Focus", theme.colors.focus),
                ("Text / default", theme.text.default),
                ("Text / subtle", theme.text.subtle),
                ("Text / brand", theme.text.brand),
            ]
            .map(|(label, color)| {
                div()
                    .flex()
                    .items_center()
                    .gap(theme.spacing.twelve)
                    .child(
                        div()
                            .w(px(48.))
                            .h(px(24.))
                            .flex_shrink_0()
                            .rounded(theme.radii.sm)
                            .bg(color)
                            .border_1()
                            .border_color(theme.colors.hairline),
                    )
                    .child(label)
            }),
        )
}

fn typography_panel(theme: &Theme) -> impl IntoElement {
    panel(theme, "Typography")
        .id("foundation-typography")
        .test_support()
        .min_w_0()
        .children(
            [
                ("Extra small · 12 / 16", theme.typography.xs),
                ("Small · 13 / 15.29", theme.typography.sm),
                ("Base · 14 / 21", theme.typography.base),
                ("Large · 16 / 24", theme.typography.lg),
            ]
            .map(|(label, style)| {
                div()
                    .text_size(style.size)
                    .line_height(style.line_height)
                    .font_weight(style.weight)
                    .child(label)
            }),
        )
}

fn effects_panel(theme: &Theme) -> impl IntoElement {
    panel(theme, "Gradients and elevation")
        .id("foundation-effects")
        .test_support()
        .min_w_0()
        .child(
            div().flex().gap(theme.spacing.sixteen).children(
                [
                    ("Primary", &theme.primary),
                    ("Destructive", &theme.destructive),
                ]
                .map(|(label, emphasis)| {
                    div()
                        .id(label)
                        .test_support()
                        .min_w_0()
                        .flex()
                        .flex_col()
                        .gap(theme.spacing.eight)
                        .flex_1()
                        .child(
                            div()
                                .text_color(theme.text.subtle)
                                .text_size(theme.typography.xs.size)
                                .child(label),
                        )
                        .child(
                            div()
                                .h(px(40.))
                                .rounded(theme.radii.lg)
                                .bg(emphasis.gradient(false))
                                .shadow(vec![emphasis.inset_highlight()])
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_color(gpui_kit::rgb(0xffffff))
                                .child("Rest"),
                        )
                        .child(
                            div()
                                .h(px(40.))
                                .rounded(theme.radii.lg)
                                .bg(emphasis.gradient(true))
                                .shadow(vec![emphasis.inset_highlight()])
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_color(gpui_kit::rgb(0xffffff))
                                .child("Hover"),
                        )
                }),
            ),
        )
        .child(
            div()
                .flex()
                .gap(theme.spacing.sixteen)
                .pt(theme.spacing.eight)
                .child(
                    div()
                        .flex_1()
                        .p(theme.spacing.twelve)
                        .rounded(theme.radii.lg)
                        .bg(theme.colors.control)
                        .shadow(theme.effects.shadow_xs.clone())
                        .child("Shadow / xs"),
                )
                .child(
                    div()
                        .flex_1()
                        .p(theme.spacing.twelve)
                        .rounded(theme.radii.lg)
                        .bg(theme.colors.control)
                        .shadow(theme.effects.shadow_md.clone())
                        .child("Shadow / md"),
                ),
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui_kit::{Context, IntoElement, Render, TestAppContext, Window, test::TestWindowExt};

    struct Harness {
        width: f32,
    }
    impl Render for Harness {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            let theme = gpui_kumo::theme(cx);
            div()
                .w(px(self.width))
                .p(px(32.))
                .font_family(theme.typography.font_family.clone())
                .text_size(theme.typography.base.size)
                .line_height(theme.typography.base.line_height)
                .child(panels(theme, px(self.width)))
        }
    }

    #[gpui_kit::test]
    fn foundation_columns_and_effects_fit_resized_layouts(cx: &mut TestAppContext) {
        cx.update(gpui_kumo::init);
        let (view, cx) = cx.add_window_view(|_, _| Harness { width: 1040. });
        cx.update(|window, cx| {
            for appearance in [gpui_kumo::Appearance::Light, gpui_kumo::Appearance::Dark] {
                gpui_kumo::set_appearance(appearance, cx);
                for width in [1040., 520., 738., 737., 1040.] {
                    view.update(cx, |v, _| v.width = width);
                    window.render_frame(cx);
                    let row = window.find("foundations").bounds();
                    for id in [
                        "foundation-colors",
                        "foundation-details",
                        "foundation-typography",
                        "foundation-effects",
                        "Primary",
                        "Destructive",
                    ] {
                        let bounds = window.find(id).bounds();
                        assert!(
                            bounds.left() >= row.left() && bounds.right() <= row.right(),
                            "{id} overflows at {width}: {bounds:?} outside {row:?}"
                        );
                    }
                    let colors = window.find("foundation-colors").bounds();
                    let details = window.find("foundation-details").bounds();
                    if width < 738. {
                        assert_eq!(colors.left(), details.left());
                        assert_eq!(details.top() - colors.bottom(), px(24.));
                    } else {
                        assert_eq!(colors.top(), details.top());
                        assert_eq!(details.left() - colors.right(), px(24.));
                    }
                }
            }
        });
    }
}
