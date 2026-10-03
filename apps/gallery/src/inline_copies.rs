use gpui_kit::{
    Context, InteractiveElement, IntoElement, ParentElement, Render, StatefulInteractiveElement,
    Styled, Window, div,
};
use gpui_kumo::{InlineCopyText, inline_copy_text::Style, text, theme};

pub struct InlineCopies {
    group_hovered: bool,
    clicks: usize,
    copies: usize,
    cancelled: usize,
}
impl InlineCopies {
    pub fn new() -> Self {
        Self {
            group_hovered: false,
            clicks: 0,
            copies: 0,
            cancelled: 0,
        }
    }
}
impl Render for InlineCopies {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = theme(cx);
        let click = cx.entity().downgrade();
        let copy = click.clone();
        let cancel = click.clone();
        crate::panel(theme, "InlineCopyText · short values, rich content and row composition")
            .child(InlineCopyText::new("copy-id", "f86b3f10-32e9-4db7-ae95-84a1b2c3d4e5").labels("Copy database ID", "Database ID copied")
                .on_click(move |_, _, cx| { let _ = click.update(cx, |v, cx| { v.clicks += 1; cx.notify(); }); })
                .on_copy(move |_, cx| { let _ = copy.update(cx, |v, cx| { v.copies += 1; cx.notify(); }); }))
            .child(div().id("copy-resource-row").flex().items_center().justify_between().gap(theme.spacing.sixteen).min_w_0().w_full()
                .rounded(theme.radii.lg).border_1().border_color(theme.colors.line).bg(theme.colors.base).px(theme.spacing.sixteen).py(theme.spacing.twelve)
                .on_hover(cx.listener(|v, hovered, _, cx| { v.group_hovered = *hovered; cx.notify(); }))
                .child(gpui_kumo::Text::new("resource-name", "Production database").truncate(true))
                .child(InlineCopyText::new("row-copy", "f86b3f10-32e9-4db7-ae95-84a1b2c3d4e5").group_active(self.group_hovered)))
            .child(InlineCopyText::rich("rich-copy", "Database ID: f86b3f10…", "f86b3f10-32e9-4db7-ae95-84a1b2c3d4e5", gpui_kit::StyledText::new("Database ID: f86b3f10…"))
                .style(Style::Copy { tone: text::Tone::Default, size: text::Size::Base, bold: true }))
            .child(div().flex().items_center().gap(theme.spacing.sixteen)
                .child(InlineCopyText::new("aligned-copy", "ID beside taller control"))
                .child(gpui_kumo::Button::new("aligned-save", "Save").size(gpui_kumo::button::Size::Lg)))
            .child(InlineCopyText::new("long-copy", "Long café 🦀 resource identifier with extra content that must truncate in narrow layouts").value("full café 🦀 payload\nwith native Unicode clipboard"))
            .child(InlineCopyText::new("error-tone-copy", "Error text with inherited copy glyph").style(Style::Copy { tone: text::Tone::Error, size: text::Size::Base, bold: false }).group_active(true))
            .child(InlineCopyText::new("disabled-copy", "Disabled copy control").disabled(true).group_active(true))
            .child(InlineCopyText::new("cancel-copy", "Consumer cancels copy").on_click(move |_, window, cx| { window.prevent_default(); let _ = cancel.update(cx, |v, cx| { v.cancelled += 1; cx.notify(); }); }))
            .child(format!("Database ID: {} activations · {} copies · {} cancellations", self.clicks, self.copies, self.cancelled))
    }
}
