use gpui_kit::{Div, FontWeight, ParentElement, Styled, div, px};
use gpui_kumo::Theme;

/// Compose a consistently styled gallery section.
pub fn panel(theme: &Theme, title: &'static str) -> Div {
    div()
        .flex()
        .flex_col()
        .flex_shrink_0()
        .gap(theme.spacing.sixteen)
        .p(px(24.))
        .bg(theme.colors.base)
        .rounded(theme.radii.lg)
        .border_1()
        .border_color(theme.colors.hairline)
        .child(div().font_weight(FontWeight::MEDIUM).child(title))
}
