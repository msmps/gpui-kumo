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
            inputs: (0..14)
                 .map(|i| cx.new(|cx| {
                    let mut state = InputState::new(if i == 7 { "Search query" } else { "API endpoint" }, window, cx);
                    if i == 13 { state.set_value("5", window, cx); }
                    if i == 6 { state.set_value("a-long-domain-name-for-testing-horizontal-scrolling-and-suffix-clipping", window, cx); }
                    if i == 7 { state.set_value("café 🦀", window, cx); }
                    if i == 8 { state.set_value("/api/packages", window, cx); }
                    if i == 9 || i == 12 { state.set_disabled(true, cx); }
                    if i == 10 || i == 11 { state.set_value("packages", window, cx); }
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
        let submit_owner = clear_owner.clone();
        let hybrid_owner = clear_owner.clone();
        super::panel(
            theme(cx),
            "InputGroup · shared container and retained editing",
        )
        .child(
            gpui_kit::div().w(gpui_kit::px(260.)).child(
                InputGroup::new("page-composition", &self.inputs[13])
                    .editor_width(gpui_kit::px(50.))
                    .text_align(gpui_kit::TextAlign::Center)
                    .leading_button(
                        "page-first",
                        "First",
                        gpui_kumo::button::Variant::Secondary,
                        |b, _, _| b.disabled(true),
                    )
                    .leading_button(
                        "page-prev",
                        "Previous",
                        gpui_kumo::button::Variant::Secondary,
                        |b, _, _| b,
                    )
                    .button(
                        "page-next",
                        "Next",
                        gpui_kumo::button::Variant::Secondary,
                        |b, _, _| b,
                    ),
            ),
        )
        .child(
            InputGroup::new("joined-search", &self.inputs[10])
                .show_label(true)
                .required(false)
                .description("Search for packages.")
                .button(
                    "joined-submit",
                    "Search",
                    gpui_kumo::button::Variant::Secondary,
                    move |button, _, _| {
                        let owner = submit_owner.clone();
                        button.on_click(move |_, _, cx| {
                            let _ = owner.update(cx, |this, cx| {
                                this.actions += 1;
                                cx.notify();
                            });
                        })
                    },
                )
                .button(
                    "joined-disabled",
                    "Reset",
                    gpui_kumo::button::Variant::Secondary,
                    |button, _, _| button.disabled(true),
                )
                .button(
                    "joined-more",
                    "More",
                    gpui_kumo::button::Variant::Secondary,
                    |button, _, _| button,
                ),
        )
        .child(
            InputGroup::new("hybrid-search", &self.inputs[11])
                .start(InputGroupAddon::text("/api/"))
                .end(InputGroupAddon::button(
                    "hybrid-clear",
                    "Clear",
                    move |button, _, _| {
                        let owner = hybrid_owner.clone();
                        button.on_click(move |_, window, cx| {
                            let _ = owner.update(cx, |this, cx| {
                                this.inputs[11]
                                    .update(cx, |state, cx| state.set_value("", window, cx));
                                cx.notify();
                            });
                        })
                    },
                ))
                .button(
                    "hybrid-submit",
                    "Search",
                    gpui_kumo::button::Variant::Primary,
                    |button, _, _| button,
                ),
        )
        .child(
            InputGroup::new("disabled-search", &self.inputs[12])
                .start(InputGroupAddon::text("/api/"))
                .button(
                    "disabled-submit",
                    "Search",
                    gpui_kumo::button::Variant::Secondary,
                    |button, _, _| button.disabled(false),
                ),
        )
        .child(format!("Joined actions: {}", self.actions))
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
                .show_label(true)
                .suffix(".workers.dev"),
        )
        .child(InputGroup::new("long-domain", &self.inputs[6]).suffix(".workers.dev"))
        .child(
            InputGroup::new("search-actions", &self.inputs[7]).end(InputGroupAddon::parts([
                InputGroupAddon::text("Reset"),
                InputGroupAddon::button("clear", "Clear", move |button, _, _| {
                    let owner = clear_owner.clone();
                    button.on_click(move |_, window, cx| {
                        let _ = owner.update(cx, |this, cx| {
                            this.actions += 1;
                            this.inputs[7].update(cx, |state, cx| state.set_value("", window, cx));
                            cx.notify();
                        });
                    })
                }),
            ])),
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
