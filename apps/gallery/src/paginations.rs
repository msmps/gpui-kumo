use gpui_kit::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Subscription, Window,
    div, px,
};
use gpui_kumo::{
    Button, Pagination, PaginationEvent, PaginationLabels, PaginationState, PaginationTotal, Theme,
    pagination::Controls, theme,
};
pub struct Paginations {
    states: Vec<Entity<PaginationState>>,
    proposals: usize,
    size_proposals: usize,
    _subscriptions: Vec<Subscription>,
}
impl Paginations {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let configs = [
            (5, 10, PaginationTotal::Known(95)),
            (1, 25, PaginationTotal::Known(1250)),
            (
                1,
                10,
                PaginationTotal::Unknown {
                    has_next_page: true,
                },
            ),
            (1, 10, PaginationTotal::Known(100)),
            (1, 10, PaginationTotal::Known(0)),
            (3, 25, PaginationTotal::Known(500)),
            (1, 10, PaginationTotal::Known(200)),
            (2, 10, PaginationTotal::Known(200)),
        ];
        let states: Vec<_> = configs
            .into_iter()
            .enumerate()
            .map(|(index, (page, size, total))| {
                cx.new(|cx| {
                    let mut s = PaginationState::new(page, size, total, window, cx);
                    s.set_labels(
                        PaginationLabels {
                            navigation: format!("Dataset {index} pages").into(),
                            page_number: format!("Dataset {index} page number").into(),
                            page_size: format!("Dataset {index} page size café 🦀").into(),
                            ..Default::default()
                        },
                        cx,
                    );
                    if index >= 6 {
                        s.set_page_size_options(vec![10, 20, 50], cx);
                    }
                    s
                })
            })
            .collect();
        let mut subscriptions: Vec<_> = states
            .iter()
            .enumerate()
            .map(|(index, state)| {
                cx.subscribe_in(
                    state,
                    window,
                    move |v: &mut Self, state, event, window, cx| {
                        let PaginationEvent::Page(page) = *event else {
                            if let PaginationEvent::PageSize(size) = *event {
                                v.size_proposals += 1;
                                if index != 6 {
                                    state.update(cx, |s, cx| {
                                        s.set_per_page(size, window, cx);
                                        if index == 5 {
                                            s.set_page(1, window, cx);
                                        }
                                    });
                                }
                                cx.notify();
                            }
                            return;
                        };
                        v.proposals += 1;
                        if index != 3 {
                            state.update(cx, |s, cx| {
                                s.set_page(page, window, cx);
                                if index == 2 {
                                    s.set_total(
                                        PaginationTotal::Unknown {
                                            has_next_page: page < 3,
                                        },
                                        window,
                                        cx,
                                    );
                                }
                            });
                        }
                        cx.notify();
                    },
                )
            })
            .collect();
        subscriptions.push(cx.observe_global::<Theme>(|_, cx| cx.notify()));
        Self {
            states,
            proposals: 0,
            size_proposals: 0,
            _subscriptions: subscriptions,
        }
    }
}
impl Render for Paginations {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        crate::panel(
            theme(cx),
            "Pagination · controlled pages, retained native drafts and source joins",
        )
        .child("Full controls · partial last page; type a page and press Enter or Tab")
        .child(Pagination::new("pagination-full", &self.states[0]))
        .child("Simple controls · large dataset")
        .child(Pagination::new("pagination-simple", &self.states[1]).controls(Controls::Simple))
        .child("Unknown total · sequential pages until the server reports no next page")
        .child(
            Pagination::new("pagination-unknown", &self.states[2]).content(|parts, _, _| {
                let page = parts.info.value().page;
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.))
                    .child(parts.info.text(format!("Page {page} · café 🦀")))
                    .child(parts.controls)
                    .into_any_element()
            }),
        )
        .child("Controlled owner rejects proposals")
        .child(Pagination::new("pagination-rejected", &self.states[3]))
        .child("Empty dataset · both directions unavailable")
        .child(Pagination::new("pagination-empty", &self.states[4]))
        .child("Page size · owner accepts and explicitly resets to first page")
        .child(Pagination::new("pagination-size", &self.states[5]).page_size(true))
        .child("Custom label/options · owner rejects size proposals")
        .child(
            Pagination::new("pagination-size-rejected", &self.states[6]).content(|parts, _, _| {
                div()
                    .flex()
                    .flex_wrap()
                    .items_center()
                    .gap(px(8.))
                    .child(parts.info)
                    .child(parts.page_size.label("Résultats par page café 🦀:"))
                    .child(parts.controls)
                    .into_any_element()
            }),
        )
        .child("Hidden visible label · custom options; owner keeps the current page")
        .child(
            Pagination::new("pagination-size-hidden", &self.states[7]).content(|parts, _, _| {
                div()
                    .flex()
                    .flex_wrap()
                    .items_center()
                    .gap(px(8.))
                    .child(parts.info)
                    .child(parts.page_size.without_label())
                    .child(parts.controls)
                    .into_any_element()
            }),
        )
        .child(gpui_kumo::Text::new(
            "pagination-size-proposals",
            format!("Page size proposals: {}", self.size_proposals),
        ))
        .child(
            div()
                .flex()
                .flex_wrap()
                .gap(px(8.))
                .child(
                    Button::new("pagination-size-reset", "Reset size datasets").on_click(
                        cx.listener(|v, _, window, cx| {
                            v.states[5].update(cx, |s, cx| {
                                s.set_disabled(false, window, cx);
                                s.set_per_page(25, window, cx);
                                s.set_page(3, window, cx);
                            });
                            v.states[6].update(cx, |s, cx| s.set_per_page(10, window, cx));
                            v.states[7].update(cx, |s, cx| {
                                s.set_per_page(10, window, cx);
                                s.set_page(2, window, cx);
                            });
                        }),
                    ),
                )
                .child(
                    Button::new("pagination-size-availability", "Toggle size availability")
                        .on_click(cx.listener(|v, _, window, cx| {
                            v.states[5]
                                .update(cx, |s, cx| s.set_disabled(!s.is_disabled(), window, cx));
                        })),
                ),
        )
        .child(format!("Page proposals: {}", self.proposals))
        .child(
            div()
                .flex()
                .flex_wrap()
                .gap(px(8.))
                .child(
                    Button::new("pagination-reset", "Reset full dataset").on_click(cx.listener(
                        |v, _, window, cx| {
                            v.states[0].update(cx, |s, cx| {
                                s.set_total(PaginationTotal::Known(95), window, cx);
                                s.set_page(5, window, cx);
                            });
                        },
                    )),
                )
                .child(
                    Button::new("pagination-availability", "Toggle full availability").on_click(
                        cx.listener(|v, _, window, cx| {
                            v.states[0]
                                .update(cx, |s, cx| s.set_disabled(!s.is_disabled(), window, cx));
                        }),
                    ),
                ),
        )
    }
}
