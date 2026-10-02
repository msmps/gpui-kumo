use gpui_kit::AppContext;
use gpui_kit::{Context, Entity, IntoElement, ParentElement, Render, Styled, Subscription, Window};
use gpui_kumo::{InputGroup, InputGroupAddon, InputState, Theme, input::Size, theme};
pub(super) struct InputGroups {
    inputs: Vec<Entity<InputState>>,
    actions: usize,
    _theme: Subscription,
}
impl InputGroups {
    pub(super) fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            actions: 0,
            _theme: cx.observe_global::<Theme>(|_, cx| cx.notify()),
            inputs: (0..10)
                 .map(|i| cx.new(|cx| {
                    let mut state = InputState::new(if i == 7 { "Search query" } else { "API endpoint" }, window, cx);
                    if i == 6 { state.set_value("a-long-domain-name-for-testing-horizontal-scrolling-and-suffix-clipping", window, cx); }
                    if i == 7 { state.set_value("café 🦀", window, cx); }
                    if i == 8 { state.set_value("/api/packages", window, cx); }
                    if i == 9 { state.set_disabled(true, cx); }
                    state
                }))
                .collect(),
        }
    }
}
impl Render for InputGroups {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let clear_owner = cx.entity().downgrade();
        let copy_owner = clear_owner.clone();
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
        .child(
            InputGroup::new("search-actions", &self.inputs[7]).end(InputGroupAddon::button(
                "clear",
                "Clear",
                move |button, _, _| {
                    let owner = clear_owner.clone();
                    button.on_click(move |_, window, cx| {
                        let _ = owner.update(cx, |this, cx| {
                            this.actions += 1;
                            this.inputs[7].update(cx, |state, cx| state.set_value("", window, cx));
                            cx.notify();
                        });
                    })
                },
            )),
        )
        .child(
            InputGroup::new("copy-actions", &self.inputs[8]).end(InputGroupAddon::icon_button(
                "copy",
                "Copy endpoint",
                "empty-copy.svg",
                move |button, _, _| {
                    let owner = copy_owner.clone();
                    button.on_click(move |_, _, cx| {
                        let _ = owner.update(cx, |this, cx| {
                            this.actions += 1;
                            let text = this.inputs[8].read(cx).value(cx);
                            cx.write_to_clipboard(gpui_kit::ClipboardItem::new_string(
                                text.to_string(),
                            ));
                            cx.notify();
                        });
                    })
                },
            )),
        )
        .child(
            InputGroup::new("disabled-actions", &self.inputs[9]).end(InputGroupAddon::button(
                "disabled-clear",
                "Clear",
                |button, _, _| button.disabled(false),
            )),
        )
        .child(format!("Addon actions: {}", self.actions))
        .child(gpui_kit::div().w(gpui_kit::px(128.)).child(
            InputGroup::new("long-addon", &self.inputs[4]).start(InputGroupAddon::text(
                "A long addon must stay inside its surface",
            )),
        ))
    }
}
