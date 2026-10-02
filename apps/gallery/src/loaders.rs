use gpui_kit::{Div, ParentElement, Styled, div, px};
use gpui_kumo::{Loader, Text, Theme, loader::Size};

pub(super) fn panel(theme: &Theme) -> Div {
    super::panel(theme, "Loader")
        .child(div().flex().items_center().gap(px(24.)).children([
            Loader::new("small").size(Size::Small),
            Loader::new("base"),
            Loader::new("large").size(Size::Large),
            Loader::new("custom").size(Size::Custom(px(40.))).accessible_name("Chargement"),
        ]))
        .child(Text::new("loader-help", "16 / 24 / 32 / custom 40px; inherits foreground. Reduced motion retains a static indicator."))
}
