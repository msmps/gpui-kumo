use gpui_kit::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Subscription, Window,
    div, prelude::FluentBuilder, px,
};
use gpui_kumo::{Button, Input, InputEvent, InputState, input::Size, theme};

pub(super) struct Inputs {
    email: Entity<InputState>,
    sizes: [Entity<InputState>; 4],
    submitted: usize,
    _subscription: Subscription,
}

impl Inputs {
    pub(super) fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let email = cx.new(|cx| InputState::new("Email address", window, cx));
        email.update(cx, |state, cx| {
            state.set_placeholder("you@example.com", window, cx)
        });
        let sizes = ["Extra small", "Small", "Base", "Large"].map(|name| {
            let input = cx.new(|cx| InputState::new(name, window, cx));
            input.update(cx, |state, cx| {
                state.set_placeholder("Type here…", window, cx)
            });
            input
        });
        let subscription = cx.subscribe(&email, |this, _, event, cx| {
            if matches!(event, InputEvent::Submit { .. }) {
                this.submitted += 1;
            }
            cx.notify();
        });
        Self {
            email,
            sizes,
            submitted: 0,
            _subscription: subscription,
        }
    }
}

impl Render for Inputs {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = theme(cx).clone();
        let email = self.email.read(cx);
        let disabled = email.is_disabled();
        let read_only = email.is_read_only();
        let value = email.value(cx);
        let invalid = !value.is_empty() && !value.contains('@');
        super::panel(&theme, "Input · editing and validation")
            .child(Input::new("email", &self.email).label(true)
                .description("Type an email address and press Enter. Selection, clipboard and undo use native shortcuts.")
                .when(invalid, |input| input.error("Include @ in the email address.")))
            .child(div().flex().gap(theme.spacing.eight)
                .child(Button::new("disable-input", if disabled { "Enable input" } else { "Disable input" })
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.email.update(cx, |input, cx| input.set_disabled(!disabled, cx));
                        cx.notify();
                    })))
                .child(Button::new("readonly-input", if read_only { "Allow editing" } else { "Read only" })
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.email.update(cx, |input, cx| input.set_read_only(!read_only, cx));
                        cx.notify();
                    })))
                .child(Button::new("reset-input", "Reset")
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.email.update(cx, |input, cx| input.set_value("", window, cx));
                        cx.notify();
                    }))))
            .child(div().text_color(theme.text.subtle).child(format!("Submissions: {} · Value: {}", self.submitted, value)))
            .child(div().flex().flex_col().gap(px(16.)).children(
                self.sizes.iter().zip([Size::Xs, Size::Sm, Size::Base, Size::Lg]).enumerate().map(|(index, (state, size))| {
                    Input::new(("input-size", index), state).size(size).label(true)
                }),
            ))
    }
}
