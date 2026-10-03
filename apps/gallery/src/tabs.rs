use gpui_kit::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, Subscription, Window,
};
use gpui_kumo::tabs::{Size, Variant};
use gpui_kumo::{TabItem, Tabs, TabsEvent, TabsState, Text, theme};
pub struct TabExamples {
    states: Vec<Entity<TabsState<u32>>>,
    proposals: usize,
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
        Self {
            states,
            proposals: 0,
            _subscription: subscription,
        }
    }
}
impl Render for TabExamples {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        crate::panel(theme(cx), "Tabs · retained selection and manual keyboard activation")
            .child(Tabs::new("segmented-base", "Project sections", &self.states[0]))
            .child(Tabs::new("segmented-small", "Compact project sections", &self.states[1]).size(Size::Sm))
            .child(Tabs::new("underline-base", "Underlined project sections", &self.states[2]).variant(Variant::Underline))
            .child(Tabs::new("underline-small", "Compact underlined sections", &self.states[3]).variant(Variant::Underline).size(Size::Sm))
            .child(Text::new("tabs-status", format!("Overview selection: {:?} · {} user changes. Arrow keys focus; Enter/Space selects.", self.states[0].read(cx).selected(), self.proposals)))
    }
}
