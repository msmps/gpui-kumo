use gpui_kit::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Subscription, Window,
    div, px,
};
use gpui_kumo::{Button, Input, InputState, LayerCard, Text, layer_card::Section, theme};

pub(super) struct Cards {
    input: Entity<InputState>,
    activations: usize,
    _subscriptions: Vec<Subscription>,
}

impl Cards {
    pub(super) fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| InputState::new("Card project name", window, cx));
        input.update(cx, |state, cx| state.set_placeholder("café 🦀", window, cx));
        let subscriptions = vec![
            cx.observe_global::<gpui_kumo::Theme>(|_, cx| cx.notify()),
            cx.subscribe(&input, |_, _, _, cx| cx.notify()),
        ];
        Self {
            input,
            activations: 0,
            _subscriptions: subscriptions,
        }
    }
}

impl Render for Cards {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = theme(cx);
        let panel = super::panel(theme, "LayerCard · surfaces and nested controls");
        panel.child(
            div()
                .flex()
                .flex_wrap()
                .gap(px(24.))
                .child(
                    LayerCard::new("simple-card")
                        .w(px(240.))
                        .p(px(16.))
                        .child(Text::new(
                            "simple-copy",
                            "A simple surface inherits its surrounding typography.",
                        )),
                )
                .child(
                    LayerCard::new("interactive-card")
                        .w(px(280.))
                        .section(
                            Section::secondary("header")
                                .justify_between()
                                .child(Text::new("title", "Project"))
                                .child(Button::new("card-action", "Save").on_click(cx.listener(
                                    |this, _, _, cx| {
                                        this.activations += 1;
                                        cx.notify();
                                    },
                                ))),
                        )
                        .section(
                            Section::primary("body")
                                .child(Input::new("card-input", &self.input).label(true))
                                .child(Text::new(
                                    "card-count",
                                    format!("Saved {} times", self.activations),
                                )),
                        ),
                )
                .child(
                    LayerCard::new("narrow-card")
                        .w(px(160.))
                        .section(
                            Section::secondary("header").child(Text::new("title", "Narrow card")),
                        )
                        .section(Section::primary("body").child(Text::new(
                            "copy",
                            "Unicode content — café 🦀 — wraps within the primary layer.",
                        ))),
                ),
        )
    }
}
