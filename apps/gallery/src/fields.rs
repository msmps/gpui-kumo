use gpui_kit::{
    AppContext, Context, Entity, Focusable, IntoElement, ParentElement, Render, Subscription,
    Window,
};
use gpui_kumo::{Field, Input, InputState, Theme, TooltipProvider, TooltipState, theme};
pub(super) struct Fields {
    input: Entity<InputState>,
    tooltip: Entity<TooltipState>,
    _theme: Subscription,
}
impl Fields {
    pub(super) fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            input: cx.new(|cx| InputState::new("Phone number", window, cx)),
            tooltip: cx.new(|cx| TooltipState::new(window, cx)),
            _theme: cx.observe_global::<Theme>(|_, cx| cx.notify()),
        }
    }
}
impl Render for Fields {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focus = self.input.read(cx).focus_handle(cx);
        TooltipProvider::new(
            "field-tooltips",
            [self.tooltip.downgrade()],
            super::panel(theme(cx), "Field · shared label and helper composition").child(
                Field::new(
                    "phone",
                    "Phone number",
                    Input::new("phone-control", &self.input),
                )
                .focus_target(&focus)
                .required(false)
                .label_tooltip(
                    &self.tooltip,
                    "Used only for account recovery. We never publish your phone number.",
                )
                .description(
                    "Used only for account recovery. Click the label to focus the retained input.",
                ),
            ),
        )
    }
}
