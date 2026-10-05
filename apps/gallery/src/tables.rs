use gpui_kit::{
    Context, IntoElement, ParentElement, Render, ScrollHandle, Styled, Window, div, px,
};
use gpui_kumo::{
    Button, Table, TableBody, TableCaption, TableCell, TableCheckCell, TableCheckHead, TableFooter,
    TableHead, TableHeader, TableResizeHandle, TableRow, Text,
    checkbox::State,
    table::{ColumnWidth, HeaderVariant, Layout, RowVariant, Sticky},
    theme,
};

pub struct Tables {
    selected: [bool; 3],
    reversed: bool,
    disabled: bool,
    compact: bool,
    path_width: f32,
    scroll: ScrollHandle,
}
impl Default for Tables {
    fn default() -> Self {
        Self {
            selected: [false, true, false],
            reversed: false,
            disabled: false,
            compact: true,
            path_width: 260.,
            scroll: ScrollHandle::new(),
        }
    }
}
impl Render for Tables {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = theme(cx);
        let compact = if self.compact {
            HeaderVariant::Compact
        } else {
            HeaderVariant::Default
        };
        let selected = self.selected.iter().filter(|v| **v).count();
        let all_state = match selected {
            0 => State::Unchecked,
            3 => State::Checked,
            _ => State::Indeterminate,
        };
        let owner = cx.entity().downgrade();
        let all_owner = owner.clone();
        let resize_owner = owner.clone();
        let mut body = TableBody::new("requests");
        let order = if self.reversed { [2, 1, 0] } else { [0, 1, 2] };
        for i in order {
            let path = ["/", "/wp-login.php", "/.env"][i];
            let owner = owner.clone();
            body = body.row(
                TableRow::new(format!("request-{i}"))
                    .variant(if self.selected[i] {
                        RowVariant::Selected
                    } else {
                        RowVariant::Default
                    })
                    .cell(
                        TableCheckCell::new("select", format!("Select {path}"))
                            .state(if self.selected[i] {
                                State::Checked
                            } else {
                                State::Unchecked
                            })
                            .disabled(self.disabled)
                            .on_change(move |state, _, _, cx| {
                                let _ = owner.update(cx, |this, cx| {
                                    this.selected[i] = state == State::Checked;
                                    cx.notify();
                                });
                            }),
                    )
                    .cell(TableCell::new("path").child(Text::new("text", path).style(
                        gpui_kumo::text::Style::Mono {
                            tone: Default::default(),
                            size: Default::default(),
                        },
                    )))
                    .cell(TableCell::new("status").text(if i == 0 { "200" } else { "404" }))
                    .cell(TableCell::new("count").text(["91", "41", "73"][i]))
                    .cell(
                        TableCell::new("action").child(
                            Button::new("inspect", "Inspect")
                                .variant(gpui_kumo::button::Variant::Ghost)
                                .size(gpui_kumo::button::Size::Sm)
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.selected[i] = !this.selected[i];
                                    cx.notify();
                                })),
                        ),
                    ),
            );
        }
        let selection = Table::new("selection-table")
            .accessibility_label("Request selection")
            .layout(Layout::Fixed)
            .min_table_width(px(540.))
            .columns([
                ColumnWidth::Pixels(px(40.)),
                ColumnWidth::Auto,
                ColumnWidth::Pixels(px(80.)),
                ColumnWidth::Pixels(px(100.)),
                ColumnWidth::Pixels(px(90.)),
            ])
            .header(
                TableHeader::new("header").variant(compact).row(
                    TableRow::new("columns")
                        .cell(
                            TableCheckHead::new("select", "Select all requests")
                                .state(all_state)
                                .disabled(self.disabled)
                                .on_change(move |state, _, _, cx| {
                                    let _ = all_owner.update(cx, |this, cx| {
                                        this.selected.fill(state == State::Checked);
                                        cx.notify();
                                    });
                                }),
                        )
                        .cell(TableHead::new("path").text("Path"))
                        .cell(TableHead::new("status").text("Status"))
                        .cell(TableHead::new("count").text("Requests"))
                        .cell(TableHead::new("action").text("Action")),
                ),
            )
            .body(body)
            .footer(
                TableFooter::new("footer").row(
                    TableRow::new("summary").cell(
                        TableCell::new("summary")
                            .column_span(5)
                            .text(format!("{selected} of 3 rows selected · owner-controlled")),
                    ),
                ),
            );
        let sticky =
            Table::new("sticky-table")
                .accessibility_label("Sticky columns and resizable header")
                .layout(Layout::Fixed)
                .min_table_width(px(760.))
                .max_h(px(220.))
                .scroll_handle(&self.scroll)
                .columns([
                    ColumnWidth::Pixels(px(40.)),
                    ColumnWidth::Pixels(px(self.path_width)),
                    ColumnWidth::Pixels(px(180.)),
                    ColumnWidth::Auto,
                    ColumnWidth::Pixels(px(90.)),
                ])
                .header(
                    TableHeader::new("sticky-header")
                        .variant(compact)
                        .sticky(true)
                        .row(
                            TableRow::new("columns")
                                .cell(TableHead::new("number").sticky(Sticky::Left).text("#"))
                                .cell(
                                    TableHead::new("path")
                                        .text("Path — drag edge or use ←/→")
                                        .resize_handle(
                                            TableResizeHandle::new(
                                                "resize",
                                                "Resize path column",
                                                px(self.path_width),
                                            )
                                            .range(px(180.)..=px(420.))
                                            .disabled(self.disabled)
                                            .on_resize(move |width, _, cx| {
                                                let _ = resize_owner.update(cx, |this, cx| {
                                                    this.path_width = width.into();
                                                    cx.notify();
                                                });
                                            }),
                                        ),
                                )
                                .cell(TableHead::new("origin").text("Origin"))
                                .cell(TableHead::new("note").text("Result"))
                                .cell(
                                    TableHead::new("actions")
                                        .sticky(Sticky::Right)
                                        .text("Action"),
                                ),
                        ),
                )
                .body(TableBody::new("body").rows((0..12).map(|i| {
                    TableRow::new(format!("item-{i}"))
                    .variant(if i == 1 {
                        RowVariant::Selected
                    } else {
                        RowVariant::Default
                    })
                    .cell(
                        TableCell::new("number")
                            .sticky(Sticky::Left)
                            .text(format!("{}", i + 1)),
                    )
                    .cell(TableCell::new("path").child(
                        Text::new("text", format!("/regional/archive/café-{i}/requests")).style(
                            gpui_kumo::text::Style::Mono {
                                tone: Default::default(),
                                size: Default::default(),
                            },
                        ),
                    ))
                    .cell(TableCell::new("origin").text("Worker / static assets"))
                    .cell(TableCell::new("note").text(
                        "A long result wraps under fixed sizing without losing column alignment.",
                    ))
                    .cell(
                        TableCell::new("actions").sticky(Sticky::Right).child(
                            Button::new("view", "View")
                                .variant(gpui_kumo::button::Variant::Ghost)
                                .size(gpui_kumo::button::Size::Sm),
                        ),
                    )
                })));
        crate::panel(
            t,
            "Table · semantic parts, selection, sizing and sticky edges",
        )
        .child(
            div()
                .flex()
                .flex_wrap()
                .gap(px(8.))
                .child(Button::new("reverse", "Reverse rows").on_click(cx.listener(
                    |this, _, _, cx| {
                        this.reversed = !this.reversed;
                        cx.notify();
                    },
                )))
                .child(
                    Button::new("disabled", "Toggle disabled").on_click(cx.listener(
                        |this, _, _, cx| {
                            this.disabled = !this.disabled;
                            cx.notify();
                        },
                    )),
                )
                .child(
                    Button::new("compact", "Toggle compact header").on_click(cx.listener(
                        |this, _, _, cx| {
                            this.compact = !this.compact;
                            cx.notify();
                        },
                    )),
                ),
        )
        .child(selection)
        .child(Text::new(
            "scroll-help",
            "Scroll horizontally and vertically below. Header and edge actions stay visible.",
        ))
        .child(sticky)
        .child(
            Table::new("empty-table")
                .caption(TableCaption::new(
                    "caption",
                    "Empty body with a readable caption",
                ))
                .header(
                    TableHeader::new("head")
                        .row(TableRow::new("row").cell(TableHead::new("result").text("Result"))),
                )
                .body(TableBody::new("empty")),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui_kit::{
        AppContext, Entity, InteractiveElement, ScrollDelta, StatefulInteractiveElement,
        TestAppContext, point, test::TestWindowExt,
    };
    struct Narrow {
        tables: Entity<Tables>,
        page_scroll: ScrollHandle,
    }
    impl Render for Narrow {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .id("page")
                .w(px(356.))
                .h(px(800.))
                .overflow_y_scroll()
                .track_scroll(&self.page_scroll)
                .child(self.tables.clone())
                .child(div().h(px(400.)))
        }
    }
    #[gpui_kit::test]
    fn gallery_minimum_content_width_exposes_horizontal_scroll_in_narrow_viewport(
        cx: &mut TestAppContext,
    ) {
        cx.update(gpui_kumo::init);
        let (view, cx) = cx.add_window_view(|_, cx| Narrow {
            tables: cx.new(|_| Tables::default()),
            page_scroll: ScrollHandle::new(),
        });
        cx.update(|window, cx| {
            for _ in 0..3 {
                window.render_frame(cx);
            }
            let scroll = view.read(cx).tables.read(cx).scroll.clone();
            assert!(
                scroll.max_offset().x >= px(400.),
                "range: {:?}, bounds: {:?}",
                scroll.max_offset(),
                scroll.bounds()
            );
            let viewport = window.find("sticky-table").bounds();
            window.scroll(
                "sticky-table",
                ScrollDelta::Pixels(point(px(-240.), px(-15.))),
                cx,
            );
            for _ in 0..3 {
                window.render_frame(cx);
            }
            assert!(scroll.offset().x <= px(-240.));
            assert_eq!(scroll.offset().y, px(0.));
            assert_eq!(view.read(cx).page_scroll.offset().y, px(0.));
            let header = window.within("sticky-header");
            assert_eq!(header.find("number").bounds().left(), viewport.left());
            assert_eq!(header.find("actions").bounds().right(), viewport.right());
            window.scroll("sticky-table", ScrollDelta::Lines(point(0., -2.)), cx);
            for _ in 0..3 {
                window.render_frame(cx);
            }
            assert!(scroll.offset().y < px(0.));
            assert_eq!(view.read(cx).page_scroll.offset().y, px(0.));
            let bottom = -scroll.max_offset().y;
            scroll.set_offset(point(scroll.offset().x, bottom));
            let tables = view.read(cx).tables.clone();
            tables.update(cx, |_, cx| cx.notify());
            for _ in 0..3 {
                window.render_frame(cx);
            }
            window.scroll("sticky-table", ScrollDelta::Lines(point(0., -2.)), cx);
            for _ in 0..3 {
                window.render_frame(cx);
            }
            assert_eq!(scroll.offset().y, bottom);
            assert!(view.read(cx).page_scroll.offset().y < px(0.));
        });
    }
}
