use gpui_kit::AppContext;
use gpui_kit::{Context, Entity, IntoElement, ParentElement, Render, Styled, Subscription, Window};
use gpui_kumo::{InputGroup, InputGroupAddon, InputState, Theme, input::Size, theme};
pub(super) struct InputGroups {
    inputs: Vec<Entity<InputState>>,
    _theme: Subscription,
}
impl InputGroups {
    pub(super) fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            _theme: cx.observe_global::<Theme>(|_, cx| cx.notify()),
            inputs: (0..7)
                 .map(|i| cx.new(|cx| {
                    let mut state = InputState::new("API endpoint", window, cx);
                    if i == 6 { state.set_value("a-long-domain-name-for-testing-horizontal-scrolling-and-suffix-clipping", window, cx); }
                    state
                }))
                .collect(),
        }
    }
}
impl Render for InputGroups {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        super::panel(
            theme(cx),
            "InputGroup · shared container and retained editing",
        )
        .children(
            [Size::Xs, Size::Sm, Size::Base, Size::Lg]
                .into_iter()
                .enumerate()
                .map(|(i, size)| {
                    InputGroup::new(("endpoint", i), &self.inputs[i])
                        .size(size)
                        .start(InputGroupAddon::text("/api/"))
                        .end(InputGroupAddon::text(".json"))
                }),
        )
        .child(
            InputGroup::new("domain", &self.inputs[5])
                .label(true)
                .suffix(".workers.dev"),
        )
        .child(InputGroup::new("long-domain", &self.inputs[6]).suffix(".workers.dev"))
        .child(gpui_kit::div().w(gpui_kit::px(128.)).child(
            InputGroup::new("long-addon", &self.inputs[4]).start(InputGroupAddon::text(
                "A long addon must stay inside its surface",
            )),
        ))
    }
}
