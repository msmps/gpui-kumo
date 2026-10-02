use gpui_kit::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Subscription, Window,
    div,
};
use gpui_kumo::{
    Button, Theme, Tooltip, TooltipProvider, TooltipState, theme,
    tooltip::{Align, Side},
};

pub(super) struct Tooltips {
    states: Vec<Entity<TooltipState>>,
    calls: usize,
    _theme: Subscription,
}
impl Tooltips {
    pub(super) fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            states: (0..6)
                .map(|_| cx.new(|cx| TooltipState::new(window, cx)))
                .collect(),
            calls: 0,
            _theme: cx.observe_global::<Theme>(|_, cx| cx.notify()),
        }
    }
}
impl Render for Tooltips {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = theme(cx).clone();
        let owner = cx.entity().downgrade();
        TooltipProvider::new("tooltip-provider", self.states.iter().map(Entity::downgrade),
            super::panel(&t, "Tooltip · hover, focus and placement")
                .child(div().text_color(t.text.subtle).child("Hover for 600ms or focus with the keyboard. Escape and activation dismiss without taking focus."))
                .child(div().flex().flex_wrap().gap(t.spacing.twelve).children(
                    [Side::Top, Side::Bottom, Side::Left, Side::Right].into_iter().enumerate().map(|(i, side)| {
                        let owner = owner.clone();
                        let label = format!("{side:?}");
                        Tooltip::new(("side-tip", i), &self.states[i], format!("{side:?} explanation"), move |_, _| {
                            let owner = owner.clone();
                            Button::new(("side-trigger", i), label.clone()).on_click(move |_, _, cx| {
                                let _ = owner.update(cx, |view, cx| { view.calls += 1; cx.notify(); });
                            })
                        }).side(side)
                    })
                ))
                .child(div().flex().flex_wrap().gap(t.spacing.twelve)
                    .child(Tooltip::new("long-tip", &self.states[4], "A longer explanation with café, 日本語 and 🦀. Resize the window to check wrapping, edge fitting and the rounded popup outline.", |_, _| Button::new("long-trigger", "Long explanation")).align(Align::Start))
                    .child(Tooltip::new("disabled-tip", &self.states[5], "Unavailable action", |_, _| Button::new("disabled-trigger", "Disabled trigger").disabled(true))))
                .child(div().text_color(t.text.subtle).child(format!("Trigger activations: {}", self.calls)))
        )
    }
}
