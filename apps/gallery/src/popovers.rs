use gpui_kit::{
    AppContext, Context, Entity, Focusable, FontWeight, IntoElement, ParentElement, Render, Styled,
    Subscription, Window, div, px,
};
use gpui_kumo::{Button, Input, InputState, Popover, PopoverState, popover::Placement, theme};

pub(super) struct Popovers {
    state: Entity<PopoverState>,
    nested: Entity<PopoverState>,
    name: Entity<InputState>,
    _subscription: Subscription,
}
impl Popovers {
    pub(super) fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let state = cx.new(|cx| PopoverState::new("Project settings", cx));
        let nested = cx.new(|cx| PopoverState::new("More settings", cx));
        let name = cx.new(|cx| InputState::new("Project name", window, cx));
        name.update(cx, |input, cx| input.set_value("My project", window, cx));
        let subscription = cx.observe(&state, |_, _, cx| cx.notify());
        Self {
            state,
            nested,
            name,
            _subscription: subscription,
        }
    }
}
impl Render for Popovers {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = theme(cx).clone();
        let name = self.name.clone();
        let nested = self.nested.clone();
        let disabled = self.state.read(cx).is_disabled();
        super::panel(&theme, "Popover · composition and focus")
            .child(div().text_color(theme.text.subtle).child("Open settings, edit the name, or open the nested panel. Escape closes the focused panel; click outside to dismiss."))
            .child(Popover::new("settings", &self.state, "Project settings")
                .arrow(true)
                .initial_focus(&name.read(cx).focus_handle(cx))
                .content(move |close, _, cx| {
                    let theme = gpui_kumo::theme(cx).clone();
                    div().flex().flex_col().gap(px(12.))
                        .child(div().font_weight(FontWeight::MEDIUM).text_size(theme.typography.popover_title.size)
                            .line_height(theme.typography.popover_title.line_height).child("Project settings"))
                        .child(div().text_size(theme.typography.popover_description.size)
                            .line_height(theme.typography.popover_description.line_height).text_color(theme.text.subtle)
                            .child("Editing state survives dismissal."))
                        .child(Input::new("project-name", &name).show_label(true))
                        .child(Popover::new("nested-settings", &nested, "More settings").parent(&close).placement(Placement::Right).arrow(true)
                            .content(|close, _, _| div().flex().flex_col().gap(px(12.))
                                .child("A nested nonmodal panel.")
                                .child(Button::new("close-nested", "Done").on_click(move |_, window, cx| close.dismiss(window, cx)))))
                        .child(Button::new("close-settings", "Done").on_click(move |_, window, cx| close.dismiss(window, cx)))
                }))
            .child(Button::new("disable-popover", if disabled { "Enable settings" } else { "Disable settings" })
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.state.update(cx, |state, cx| state.set_disabled(!disabled, window, cx));
                })))
    }
}
