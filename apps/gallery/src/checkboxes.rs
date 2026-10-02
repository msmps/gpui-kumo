use gpui_kit::{
    Context, IntoElement, ParentElement, Render, SharedString, Styled, Subscription, Window, div,
};
use gpui_kumo::{
    Checkbox, CheckboxGroup, CheckboxItem, Theme,
    checkbox::{State, Variant},
    theme,
};
use std::ops::ControlFlow;
pub(super) struct Checkboxes {
    state: State,
    changes: usize,
    preferences: Vec<SharedString>,
    item_changes: usize,
    rejected: usize,
    _theme: Subscription,
}
impl Checkboxes {
    pub(super) fn new(cx: &mut Context<Self>) -> Self {
        Self {
            state: State::Indeterminate,
            changes: 0,
            preferences: vec!["email".into()],
            item_changes: 0,
            rejected: 0,
            _theme: cx.observe_global::<Theme>(|_, cx| cx.notify()),
        }
    }
}
impl Render for Checkboxes {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let owner = cx.entity().downgrade();
        let group_owner = owner.clone();
        let item_owner = owner.clone();
        let cancel_owner = owner.clone();
        super::panel(theme(cx), "Checkbox · controlled state and Base activation").child(
            div()
                .flex()
                .flex_col()
                .gap_4()
                .child(
                    Checkbox::new("terms", "Accept terms and conditions for café 🦀 and international notifications")
                        .content(gpui_kit::StyledText::new("Accept terms and conditions for café 🦀 and international notifications")
                            .with_highlights([(7..12, gpui_kit::HighlightStyle { font_weight: Some(gpui_kit::FontWeight::BOLD), ..Default::default() })]))
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
                    .item(CheckboxItem::new("email", "Email notifications").on_change(move |_, _, _, cx| {
                        let _ = item_owner.update(cx, |this, cx| { this.item_changes += 1; cx.notify(); });
                        ControlFlow::Continue(())
                    }))
                    .item(CheckboxItem::new("sms", "SMS notifications (individual changes canceled)").on_change(move |_, _, _, cx| {
                        let _ = cancel_owner.update(cx, |this, cx| { this.rejected += 1; cx.notify(); });
                        ControlFlow::Break(())
                    }))
                    .description("Observe Email or cancel an individual SMS change. Select all proposes a group change.")
                    .on_change(move |values, _, cx| {
                        let _ = group_owner.update(cx, |this, cx| {
                            this.preferences = values;
                            cx.notify();
                        });
                    }),
                )
                .child(format!("Email events: {} · SMS canceled: {}", self.item_changes, self.rejected))
                .child(
                    Checkbox::new("optional", "Email updates")
                        .content(gpui_kit::StyledText::new("Email updates").with_highlights([(6..13, gpui_kit::HighlightStyle { font_style: Some(gpui_kit::FontStyle::Italic), ..Default::default() })]))
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
