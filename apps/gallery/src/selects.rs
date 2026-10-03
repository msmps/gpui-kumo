use gpui_kit::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Subscription, Window,
    div, px,
};
use gpui_kumo::{
    Button, Select, SelectEvent, SelectGroup, SelectOption, SelectPart, SelectState, SelectValue,
    select::Size, theme,
};
pub struct Selects {
    single: Entity<SelectState<u32>>,
    multiple: Entity<SelectState<u32>>,
    controlled: Entity<SelectState<u32>>,
    disabled: Entity<SelectState<u32>>,
    proposals: usize,
    loading: bool,
    compact: Entity<SelectState<u32>>,
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
            compact: cx.new(|cx| {
                let mut s = SelectState::new(
                    "Compact read-only region",
                    SelectValue::Single(Some(1)),
                    options(),
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
        .child(Select::new("compact-region", &self.compact).size(Size::Xs))
        .child(
            Button::new("select-loading", "Toggle loading").on_click(cx.listener(|v, _, _, cx| {
                v.loading = !v.loading;
                cx.notify();
            })),
        )
        .child(div().h(px(8.)))
    }
}
