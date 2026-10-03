use gpui_kit::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Subscription, Window,
    div, px,
};
use gpui_kumo::{
    Button, Popover, PopoverState, Select, SelectEvent, SelectGroup, SelectOption, SelectPart,
    SelectState, SelectValue, SelectValueContent,
    select::{Align, Placement, Size},
    theme,
};
pub struct Selects {
    single: Entity<SelectState<u32>>,
    multiple: Entity<SelectState<u32>>,
    controlled: Entity<SelectState<u32>>,
    disabled: Entity<SelectState<u32>>,
    proposals: usize,
    loading: bool,
    compact: Entity<SelectState<u32>>,
    popover: Entity<PopoverState>,
    nested_select: Entity<SelectState<u32>>,
    _subscription: Subscription,
}
fn options() -> Vec<SelectOption<u32>> {
    (0..30)
        .map(|i| {
            SelectOption::new(
                format!("option-{i}"),
                i,
                if i == 0 {
                    "Café 🦀 with a long readable selection that truncates in a narrow control"
                        .into()
                } else {
                    format!("Region {i:02}")
                },
            )
            .disabled(i == 2)
        })
        .collect()
}
fn grouped_options() -> Vec<SelectPart<u32>> {
    let mut first = options();
    let rest = first.split_off(5);
    vec![
        SelectGroup::new("primary-regions", first)
            .label("Available regions")
            .into(),
        SelectPart::separator("region-divider"),
        SelectGroup::new("more-regions", rest)
            .label("More regions")
            .into(),
    ]
}
impl Selects {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let single = cx.new(|cx| {
            let mut state = SelectState::new(
                "Deployment region",
                SelectValue::Single(Some(0)),
                vec![],
                cx,
            );
            state.set_parts(grouped_options(), cx);
            state
        });
        let multiple = cx.new(|cx| {
            let mut state = SelectState::new(
                "Allowed regions",
                SelectValue::Multiple(vec![1, 3]),
                vec![],
                cx,
            );
            state.set_parts(grouped_options(), cx);
            state.set_value_content(
                |value, _, _| {
                    let SelectValue::Multiple(values) = value else {
                        return None;
                    };
                    Some(SelectValueContent::new(
                        format!("{} allowed regions selected", values.len()),
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.))
                            .child(gpui_kumo::Badge::new(
                                "selection-count",
                                format!("{}", values.len()),
                            ))
                            .child("regions selected"),
                    ))
                },
                cx,
            );
            state
        });
        let controlled = cx.new(|cx| {
            let mut s = SelectState::new(
                "Rejected proposal",
                SelectValue::Single(Some(1)),
                options(),
                cx,
            );
            s.set_controlled(true, cx);
            s
        });
        let disabled = cx.new(|cx| {
            let mut s = SelectState::new(
                "Unavailable region",
                SelectValue::Single(Some(3)),
                options(),
                cx,
            );
            s.set_disabled(true, window, cx);
            s
        });
        let subscription =
            cx.subscribe(&controlled, |v: &mut Self, _, _: &SelectEvent<u32>, cx| {
                v.proposals += 1;
                cx.notify();
            });
        Self {
            single,
            multiple,
            controlled,
            disabled,
            proposals: 0,
            loading: false,
            popover: cx.new(|cx| PopoverState::new("Nested region settings", cx)),
            nested_select: cx.new(|cx| {
                SelectState::new(
                    "Nested deployment region",
                    SelectValue::Single(Some(1)),
                    options().into_iter().take(9).collect(),
                    cx,
                )
            }),
            compact: cx.new(|cx| {
                let mut s = SelectState::new(
                    "Compact read-only region",
                    SelectValue::Single(Some(1)),
                    options().into_iter().take(4).collect(),
                    cx,
                );
                s.set_read_only(true, cx);
                s
            }),
            _subscription: subscription,
        }
    }
}
impl Render for Selects {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let nested_select = self.nested_select.clone();
        crate::panel(
            theme(cx),
            "Select · retained values, keyboard navigation and owner proposals",
        )
        .child(
            Select::new("region", &self.single)
                .label(true)
                .loading(self.loading)
                .required(false)
                .description("Disabled options, long labels and scrolling"),
        )
        .child("Allowed regions · grouped multiple selection stays open")
        .child(Select::new("multi-region", &self.multiple))
        .child("Controlled · owner rejects proposals")
        .child(
            Select::new("controlled-region", &self.controlled)
                .size(Size::Sm)
                .invalid(true),
        )
        .child(div().child(format!("Rejected proposals: {}", self.proposals)))
        .child(Select::new("disabled-region", &self.disabled).size(Size::Lg))
        .child(
            Button::new("select-clear", "Clear selection").on_click(cx.listener(|v, _, _, cx| {
                v.single
                    .update(cx, |s, cx| s.set_value(SelectValue::Single(None), cx));
            })),
        )
        .child(
            div().w(px(200.)).max_w_full().child(
                Select::new("compact-region", &self.compact)
                    .size(Size::Xs)
                    .placement(Placement::Right)
                    .align(Align::End)
                    .offset(px(8.)),
            ),
        )
        .child(
            Button::new("select-loading", "Toggle loading").on_click(cx.listener(|v, _, _, cx| {
                v.loading = !v.loading;
                cx.notify();
            })),
        )
        .child(
            Popover::new(
                "nested-region-settings",
                &self.popover,
                "Nested region settings",
            )
            .width(px(260.))
            .content(move |parent, _, _| {
                div()
                    .flex()
                    .flex_col()
                    .gap(px(12.))
                    .child(
                        Select::new("nested-deployment-region", &nested_select)
                            .parent(&parent)
                            .label(true),
                    )
                    .child(
                        Button::new("done-region-settings", "Done")
                            .on_click(move |_, window, cx| parent.dismiss(window, cx)),
                    )
            }),
        )
        .child(div().h(px(8.)))
    }
}
