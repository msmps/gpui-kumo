use gpui_kit::{
    Context, IntoElement, ParentElement, Render, SharedString, Styled, Subscription, Window, div,
};
use gpui_kumo::{
    Checkbox, CheckboxGroup, CheckboxItem, Theme,
    checkbox::{State, Variant},
    theme,
};
pub(super) struct Checkboxes {
    state: State,
    changes: usize,
    preferences: Vec<SharedString>,
    _theme: Subscription,
}
impl Checkboxes {
    pub(super) fn new(cx: &mut Context<Self>) -> Self {
        Self {
            state: State::Indeterminate,
            changes: 0,
            preferences: vec!["email".into()],
            _theme: cx.observe_global::<Theme>(|_, cx| cx.notify()),
        }
    }
}
impl Render for Checkboxes {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let owner = cx.entity().downgrade();
        let group_owner = owner.clone();
        super::panel(theme(cx), "Checkbox · controlled state and Base activation").child(
            div()
                .flex()
                .flex_col()
                .gap_4()
                .child(
                    Checkbox::new("terms", "Accept terms and conditions")
                        .state(self.state)
                        .on_change(move |state, _, _, cx| {
                            let _ = owner.update(cx, |this, cx| {
                                this.state = state;
                                this.changes += 1;
                                cx.notify();
                            });
                        }),
                )
                .child(format!("Changes: {} · {:?}", self.changes, self.state))
                .child(
                    CheckboxGroup::new(
                        "preferences",
                        "Notification preferences",
                        &self.preferences,
                    )
                    .select_all("Select all", &["email".into(), "sms".into()])
                    .item(CheckboxItem::new("email", "Email notifications"))
                    .item(CheckboxItem::new("sms", "SMS notifications"))
                    .description("Choose delivery methods")
                    .on_change(move |values, _, cx| {
                        let _ = group_owner.update(cx, |this, cx| {
                            this.preferences = values;
                            cx.notify();
                        });
                    }),
                )
                .child(
                    Checkbox::new("optional", "Email updates")
                        .required(false)
                        .control_first(false),
                )
                .child(Checkbox::new("error", "Invalid option").variant(Variant::Error))
                .child(
                    Checkbox::new("disabled", "Disabled option")
                        .state(State::Checked)
                        .disabled(true),
                ),
        )
    }
}
