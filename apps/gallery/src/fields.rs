use gpui_kit::{
    AppContext, Context, Entity, Focusable, IntoElement, ParentElement, Render, Subscription,
    Window,
};
use gpui_kumo::{
    Field, Input, InputGroup, InputGroupAddon, InputState, Label, Theme, TooltipProvider,
    TooltipState, theme,
};
pub(super) struct Fields {
    input: Entity<InputState>,
    tooltip: Entity<TooltipState>,
    direct: Entity<InputState>,
    direct_help: Entity<TooltipState>,
    group: Entity<InputState>,
    group_help: Entity<TooltipState>,
    _theme: Subscription,
}
impl Fields {
    pub(super) fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            input: cx.new(|cx| InputState::new("Phone number", window, cx)),
            tooltip: cx.new(|cx| TooltipState::new(window, cx)),
            direct: cx.new(|cx| InputState::new("API key", window, cx)),
            direct_help: cx.new(|cx| TooltipState::new(window, cx)),
            group: cx.new(|cx| {
                let mut state = InputState::new("API endpoint", window, cx);
                state.set_value("café 🦀", window, cx);
                state
            }),
            group_help: cx.new(|cx| TooltipState::new(window, cx)),
            _theme: cx.observe_global::<Theme>(|_, cx| cx.notify()),
        }
    }
}
impl Render for Fields {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focus = self.input.read(cx).focus_handle(cx);
        let group = self.group.downgrade();
        TooltipProvider::new(
            "field-tooltips",
            [self.tooltip.downgrade(), self.direct_help.downgrade(), self.group_help.downgrade()],
            super::panel(theme(cx), "Field · shared and built-in contextual help")
            .child(Label::new("standalone-label", "Standalone label café 🦀")
                .optional(true).focus_target(&focus))
            .child(
                Field::control("phone", Input::new("phone-control", &self.input), cx)
                .optional_indicator(true)
                .label_tooltip(
                    &self.tooltip,
                    "Used only for account recovery. We never publish your phone number.",
                )
                .description(
                    "Used only for account recovery. Click the label to focus the retained input.",
                ),
            )
            .child(Input::new("api-key", &self.direct).show_label(true)
                .label_tooltip(&self.direct_help, "Find this in your dashboard under Settings > API Keys.")
                .description("Direct Input label help uses the same retained disclosure."))
            .child(InputGroup::new("help-endpoint", &self.group).show_label(true)
                .label_tooltip(&self.group_help, "Choose the API endpoint to call. Clear resets this field.")
                .start(InputGroupAddon::text("/api/"))
                .end(InputGroupAddon::button("help-clear", "Clear", move |button, _, _| {
                    let group = group.clone();
                    button.on_click(move |_, window, cx| {
                        let _ = group.update(cx, |state, cx| state.set_value("", window, cx));
                    })
                }))
                .description("Label help is independent of the input and addon focus ring.")),
        )
    }
}
