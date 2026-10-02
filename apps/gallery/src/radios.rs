use gpui_kit::{
    Context, IntoElement, ParentElement, Render, SharedString, Styled, Subscription, Window, div,
};
use gpui_kumo::{
    Badge, RadioGroup, RadioItem, Theme, badge,
    radio::{Appearance, Orientation, Variant},
    theme,
};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Plan {
    Free,
    Pro,
    Business,
    Contract,
}
pub(super) struct Radios {
    size: u32,
    changes: usize,
    plan: Plan,
    _theme: Subscription,
}
impl Radios {
    pub(super) fn new(cx: &mut Context<Self>) -> Self {
        Self {
            size: 10,
            changes: 0,
            plan: Plan::Pro,
            _theme: cx.observe_global::<Theme>(|_, cx| cx.notify()),
        }
    }
}
impl Render for Radios {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let owner = cx.entity().downgrade();
        let plan_owner = owner.clone();
        super::panel(
            theme(cx),
            "Radio · typed values, keyboard navigation and choice cards",
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap_6()
                .child(
                    RadioGroup::new("page-size", "Items per page", Some(self.size))
                        .orientation(Orientation::Horizontal)
                        .item(RadioItem::new("ten", 10, "10"))
                        .item(RadioItem::new("twenty-five", 25, "25"))
                        .item(RadioItem::new("fifty", 50, "50 (unavailable)").disabled(true))
                        .on_change(move |value, _, _, cx| {
                            let _ = owner.update(cx, |this, cx| {
                                this.size = value;
                                this.changes += 1;
                                cx.notify();
                            });
                        }),
                )
                .child(SharedString::from(format!(
                    "Page size: {} · changes: {}",
                    self.size, self.changes
                )))
                .child(
                    RadioGroup::new("plans", "Choose a plan", Some(self.plan))
                        .appearance(Appearance::Card)
                        .orientation(Orientation::Horizontal)
                        .item(
                            RadioItem::new("free", Plan::Free, "Free")
                                .description("For personal and hobby projects."),
                        )
                        .item(
                            RadioItem::new("pro", Plan::Pro, "Pro, popular")
                                .description("For professional websites.")
                                .content(
                                    div().flex().items_center().gap_2().child("Pro").child(
                                        Badge::new("popular", "Popular")
                                            .variant(badge::Variant::Primary),
                                    ),
                                ),
                        )
                        .item(
                            RadioItem::new("business", Plan::Business, "Business")
                                .description("Validation example")
                                .variant(Variant::Error),
                        )
                        .item(
                            RadioItem::new("contract", Plan::Contract, "Contract")
                                .description("Unavailable plan")
                                .disabled(true),
                        )
                        .on_change(move |value, _, _, cx| {
                            let _ = plan_owner.update(cx, |this, cx| {
                                this.plan = value;
                                cx.notify();
                            });
                        }),
                ),
        )
    }
}
