use gpui_kit::{Context, IntoElement, ParentElement, Render, Styled, Subscription, Window, div};
use gpui_kumo::{
    Button, ButtonGroup, Icon, Theme,
    button::{Size, Variant},
    theme,
};
pub(super) struct ButtonGroups {
    saves: usize,
    options: usize,
    _theme: Subscription,
}
impl ButtonGroups {
    pub(super) fn new(cx: &mut Context<Self>) -> Self {
        Self {
            saves: 0,
            options: 0,
            _theme: cx.observe_global::<Theme>(|_, cx| cx.notify()),
        }
    }
}
impl Render for ButtonGroups {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let save = cx.entity().downgrade();
        let options = save.clone();
        super::panel(
            theme(cx),
            "ButtonGroup · joined corners and independent actions",
        )
        .child(
            ButtonGroup::new("save-actions", "Save actions")
                .item(Button::new("save", "Save").on_click(move |_, _, cx| {
                    let _ = save.update(cx, |this, cx| {
                        this.saves += 1;
                        cx.notify();
                    });
                }))
                .item(
                    Button::icon("options", "More save options", Icon::new("caret-down.svg"))
                        .on_click(move |_, _, cx| {
                            let _ = options.update(cx, |this, cx| {
                                this.options += 1;
                                cx.notify();
                            });
                        }),
                ),
        )
        .child(format!(
            "Saves: {} · Options callbacks: {}",
            self.saves, self.options
        ))
        .child(
            div().flex().flex_wrap().items_center().gap_4().children(
                [Size::Sm, Size::Base, Size::Lg]
                    .into_iter()
                    .enumerate()
                    .map(|(i, size)| {
                        ButtonGroup::new(("sizes", i), "Deploy actions")
                            .item(
                                Button::new("deploy", "Deploy")
                                    .size(size)
                                    .variant(Variant::Primary),
                            )
                            .item(
                                Button::icon(
                                    "more",
                                    "More deploy options",
                                    Icon::new("caret-down.svg"),
                                )
                                .size(size)
                                .variant(Variant::Primary),
                            )
                    }),
            ),
        )
        .child(
            ButtonGroup::new("availability", "Availability example")
                .item(Button::new("enabled", "Enabled"))
                .item(Button::new("disabled", "Disabled").disabled(true))
                .item(Button::new("loading", "Loading").loading(true)),
        )
    }
}
