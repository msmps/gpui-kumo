use gpui_kit::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Subscription, Window,
    div, px,
};
use gpui_kumo::tabs::{Size, Variant};
use gpui_kumo::{Button, TabItem, Tabs, TabsEvent, TabsState, Text, theme};
pub struct TabExamples {
    states: Vec<Entity<TabsState<u32>>>,
    proposals: usize,
    overflow: Entity<TabsState<u32>>,
    extra: bool,
    overflow_proposals: usize,
    _overflow_subscription: Subscription,
    _subscription: Subscription,
}
impl TabExamples {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let states: Vec<_> = (0..4)
            .map(|_| {
                cx.new(|cx| {
                    TabsState::new(
                        vec![
                            TabItem::new("overview", 0, "Overview"),
                            TabItem::new("metrics", 1, "Metrics"),
                            TabItem::new("unavailable", 2, "Unavailable").disabled(true),
                            TabItem::new("settings", 3, "Settings café 🦀"),
                        ],
                        Some(0),
                        cx,
                    )
                })
            })
            .collect();
        let subscription =
            cx.subscribe(&states[0], |this: &mut Self, _, _: &TabsEvent<u32>, cx| {
                this.proposals += 1;
                cx.notify();
            });
        let overflow = cx.new(|cx| TabsState::new(overflow_items(true), Some(0), cx));
        let overflow_subscription =
            cx.subscribe(&overflow, |this: &mut Self, _, _: &TabsEvent<u32>, cx| {
                this.overflow_proposals += 1;
                cx.notify();
            });
        Self {
            overflow,
            extra: true,
            overflow_proposals: 0,
            _overflow_subscription: overflow_subscription,
            states,
            proposals: 0,
            _subscription: subscription,
        }
    }
    pub fn toggle_extra(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.extra = !self.extra;
        self.overflow.update(cx, |state, cx| {
            state.set_items(overflow_items(self.extra), window, cx)
        });
        cx.notify();
    }
}
fn overflow_items(extra: bool) -> Vec<TabItem<u32>> {
    [
        "Overview",
        "Metrics",
        "Unavailable",
        "Settings café 🦀",
        "Domains",
        "Access",
        "Analytics",
        "Logs",
        "Security",
        "Deployments",
    ]
    .into_iter()
    .take(if extra { 10 } else { 2 })
    .enumerate()
    .map(|(i, label)| TabItem::new(label.to_lowercase(), i as u32, label))
    .collect()
}
impl Render for TabExamples {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let owner = cx.entity().downgrade();
        crate::panel(theme(cx), "Tabs · retained selection and manual keyboard activation")
            .child(Tabs::new("segmented-base", "Project sections", &self.states[0]))
            .child(Tabs::new("segmented-small", "Compact project sections", &self.states[1]).size(Size::Sm))
            .child(Tabs::new("underline-base", "Underlined project sections", &self.states[2]).variant(Variant::Underline))
            .child(Tabs::new("underline-small", "Compact underlined sections", &self.states[3]).variant(Variant::Underline).size(Size::Sm))
            .child(div().w(px(250.)).max_w_full().child(Tabs::new("dynamic-tabs", "Overflow project sections", &self.overflow)))
            .child(Button::new("toggle-extra-tabs", "Toggle extra tabs").on_click(move |_, window, cx| { let _ = owner.update(cx, |this, cx| this.toggle_extra(window, cx)); }))
            .child(Text::new("overflow-status", format!("Overflow selection: {:?} · {} user changes · {} tabs", self.overflow.read(cx).selected(), self.overflow_proposals, if self.extra {10} else {2})))
            .child(Text::new("tabs-status", format!("Overview selection: {:?} · {} user changes. Arrow keys focus; Enter/Space selects.", self.states[0].read(cx).selected(), self.proposals)))
    }
}
