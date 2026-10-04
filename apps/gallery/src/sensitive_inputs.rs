use gpui_kit::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Subscription, Window,
    div, px,
};
use gpui_kumo::{
    Button, SensitiveInput, SensitiveInputEvent, SensitiveInputState, TooltipState, input::Size,
    theme,
};

pub struct SensitiveInputs {
    entries: Vec<Entity<SensitiveInputState>>,
    help: Entity<TooltipState>,
    changes: usize,
    copies: usize,
    _events: Subscription,
}
impl SensitiveInputs {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let names = [
            "Extra small",
            "Small",
            "API key",
            "Large",
            "Read only",
            "Disabled",
            "Empty secret",
            "Invalid key",
        ];
        let entries: Vec<_> = names
            .into_iter()
            .enumerate()
            .map(|(i, name)| {
                cx.new(|cx| {
                    let mut state = SensitiveInputState::new(
                        name,
                        if i == 6 {
                            ""
                        } else {
                            "sk_live_café_🦀_a_long_secret_value_for_native_narrow_overflow_and_clipboard"
                        },
                        window,
                        cx,
                    );
                    if i == 4 {
                        state.set_read_only(true, cx);
                    }
                    if i == 5 {
                        state.set_disabled(true, cx);
                    }
                    state.set_placeholder("Enter a secret", window, cx);
                    state
                })
            })
            .collect();
        let events = cx.subscribe(&entries[2], |this: &mut Self, _, event, cx| {
            match event {
                SensitiveInputEvent::Copy => this.copies += 1,
                SensitiveInputEvent::Change => this.changes += 1,
                _ => {}
            }
            cx.notify();
        });
        Self {
            entries,
            help: cx.new(|cx| TooltipState::new(window, cx)),
            changes: 0,
            copies: 0,
            _events: events,
        }
    }
}
impl Render for SensitiveInputs {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = theme(cx);
        let sizes = [Size::Xs, Size::Sm, Size::Base, Size::Lg];
        let primary = self.entries[2].downgrade();
        let clear = primary.clone();
        crate::panel(theme, "SensitiveInput · retained editing, reveal and copy")
            .child(gpui_kumo::TooltipProvider::new("secret-help", [self.help.downgrade()],
                div().flex().flex_col().gap(px(24.)).children(self.entries.iter().enumerate().map(|(i, state)| {
                    let input = SensitiveInput::new(("secret", i), state).show_label(true).size(sizes.get(i).copied().unwrap_or(Size::Base));
                    if i == 2 { input.label_tooltip(&self.help, "Keep this API key secure.").description("Click or press Enter to reveal. Escape or leaving the field hides it.") }
                    else if i == 7 { input.error_visible("This API key is invalid", true) }
                    else { input }
                }))))
            .child(format!("API key: {:?} · changes {} · copies {}", self.entries[2].read(cx).mode(), self.changes, self.copies))
            .child(div().flex().gap(theme.spacing.eight)
                .child(Button::new("replace-secret", "Replace key").on_click(move |_, window, cx| { let _ = primary.update(cx, |state, cx| state.set_value("replacement_café_🦀", window, cx)); }))
                .child(Button::new("clear-secret", "Clear key").on_click(move |_, window, cx| { let _ = clear.update(cx, |state, cx| state.set_value("", window, cx)); })))
    }
}
