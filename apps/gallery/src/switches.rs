use gpui_kit::{Context, IntoElement, ParentElement, Render, Styled, Subscription, Window, div};
use gpui_kumo::{
    Switch, SwitchGroup, Theme,
    switch::{Size, Variant},
    theme,
};
pub(super) struct Switches {
    checked: bool,
    changes: usize,
    email: bool,
    _theme: Subscription,
}
impl Switches {
    pub(super) fn new(cx: &mut Context<Self>) -> Self {
        Self {
            checked: false,
            changes: 0,
            email: true,
            _theme: cx.observe_global::<Theme>(|_, cx| cx.notify()),
        }
    }
}
impl Render for Switches {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let owner = cx.entity().downgrade();
        let email_owner = owner.clone();
        super::panel(
            theme(cx),
            "Switch · source tracks, neutral tones and controlled groups",
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap_4()
                .child(
                    Switch::new("live", "Enable notifications")
                        .checked(self.checked)
                        .on_change(move |value, _, _, cx| {
                            let _ = owner.update(cx, |this, cx| {
                                this.checked = value;
                                this.changes += 1;
                                cx.notify();
                            });
                        }),
                )
                .child(format!(
                    "Changes: {} · {}",
                    self.changes,
                    if self.checked { "On" } else { "Off" }
                ))
                .child(
                    div().flex().flex_wrap().gap_6().children(
                        [Size::Sm, Size::Base, Size::Lg]
                            .into_iter()
                            .enumerate()
                            .map(|(i, size)| {
                                Switch::new(("size", i), format!("{size:?}"))
                                    .size(size)
                                    .checked(true)
                            }),
                    ),
                )
                .child(Switch::new("neutral-off", "Neutral off").variant(Variant::Neutral))
                .child(
                    Switch::new("neutral-on", "Neutral on")
                        .checked(true)
                        .variant(Variant::Neutral)
                        .control_first(false)
                        .required(false),
                )
                .child(
                    Switch::new("disabled", "Disabled on")
                        .checked(true)
                        .disabled(true),
                )
                .child(
                    SwitchGroup::new("delivery", "Delivery settings")
                        .item(
                            Switch::new("email", "Email notifications")
                                .checked(self.email)
                                .on_change(move |value, _, _, cx| {
                                    let _ = email_owner.update(cx, |this, cx| {
                                        this.email = value;
                                        cx.notify();
                                    });
                                }),
                        )
                        .item(Switch::new("sms", "SMS notifications").disabled(true))
                        .description("Each item keeps its own value and callback."),
                ),
        )
    }
}
