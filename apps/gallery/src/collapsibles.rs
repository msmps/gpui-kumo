use gpui_kit::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div,
};
use gpui_kumo::{Button, Collapsible, CollapsiblePanel, Input, InputState, Text, theme};

pub struct Collapsibles {
    open: bool,
    proposals: usize,
    input: Entity<InputState>,
}
impl Collapsibles {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            open: true,
            proposals: 0,
            input: cx.new(|cx| {
                let mut input = InputState::new("Name", window, cx);
                input.set_value("café 🦀 retained details", window, cx);
                input
            }),
        }
    }
}
impl Render for Collapsibles {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = theme(cx);
        let owner = cx.entity().downgrade();
        let closer = owner.clone();
        crate::panel(theme, "Collapsible · disclosure, retained forms and custom composition")
            .child(Collapsible::new("details", "Contact settings").open(self.open)
                .on_open_change(move |open, _, cx| { let _ = owner.update(cx, |v, cx| {
                    v.open = open; v.proposals += 1; cx.notify();
                }); })
                .panel(CollapsiblePanel::new().keep_mounted(true)
                    .child(Text::new("details-description", "Type below, collapse and reopen: native editing state survives."))
                    .child(Input::new("details-name", &self.input).show_label(true))
                    .child(Button::new("close-details", "Save and close").on_click(move |_, _, cx| {
                        let _ = closer.update(cx, |v, cx| { v.open=false; cx.notify(); });
                    }))))
            .child(Collapsible::new("basic-details", "What is Kumo? Uncontrolled disclosure")
                .panel(CollapsiblePanel::new().child("Kumo is Cloudflare's design system.")))
            .child(Collapsible::new("custom-details", "Details")
                .trigger(Button::new("custom-details-trigger", "Show custom details").size(gpui_kumo::button::Size::Sm))
                .panel(CollapsiblePanel::unstyled().mt(theme.spacing.twelve).rounded(theme.radii.lg)
                    .bg(theme.colors.tint).p(theme.spacing.sixteen)
                    .child("Custom Kumo Button and panel presentation.")))
            .child(Collapsible::new("long-details", "A long disclosure title with café 🦀 and enough content to wrap in a narrow native layout")
                .panel(CollapsiblePanel::new().child(Collapsible::new("nested-details", "Nested disclosure")
                    .panel(CollapsiblePanel::new().child("Independent nested content.")))))
            .child(Collapsible::new("disabled-details", "Disabled disclosure").disabled(true)
                .panel(CollapsiblePanel::new().child("Unavailable")))
            .child(div().child(format!("Contact open {} · {} proposals", self.open, self.proposals)))
    }
}
