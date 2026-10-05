use super::*;
use crate::{Appearance, Button, Checkbox, checkbox::State};
use gpui_kit::{Context, Render, Role, TestAppContext, div, px, test::TestWindowExt};
use gpui_kit::{Element, FocusHandle, InteractiveElement, ScrollDelta, point};
gpui_kit::actions!(table_traversal_test, [FocusNext]);

struct Harness;
impl Render for Harness {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().w(px(400.)).child(
            Table::new("table")
                .accessibility_label("Requests")
                .header(
                    TableHeader::new("header").row(
                        TableRow::new("columns")
                            .cell(TableHead::new("head-path").child("Path"))
                            .cell(TableHead::new("head-status").child("Status")),
                    ),
                )
                .body(
                    TableBody::new("body").row(
                        TableRow::new("home")
                            .cell(TableCell::new("path").child("/"))
                            .cell(TableCell::new("status").child("200")),
                    ),
                ),
        )
    }
}

struct Interactive {
    width: f32,
    compact: bool,
    selected: bool,
    disabled: bool,
    apply: bool,
    reversed: bool,
    removed: bool,
    changes: Vec<State>,
    focus: FocusHandle,
    after: FocusHandle,
    resize_focus: FocusHandle,
    column: f32,
    resizes: Vec<f32>,
    scroll: ScrollHandle,
}
impl Render for Interactive {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let owner = cx.entity().downgrade();
        let resize_owner = owner.clone();
        let header = TableHeader::new("header")
            .sticky(true)
            .variant(if self.compact {
                HeaderVariant::Compact
            } else {
                HeaderVariant::Default
            })
            .row(
                TableRow::new("columns")
                    .cell(
                        TableHead::new("head-select")
                            .sticky(Sticky::Left)
                            .child("#"),
                    )
                    .cell(
                        TableHead::new("head-path").text("Path").resize_handle(
                            TableResizeHandle::new("resize", "Resize path", px(self.column))
                                .track_focus(&self.resize_focus)
                                .range(px(80.)..=px(280.))
                                .disabled(self.disabled)
                                .on_resize(move |width, _, cx| {
                                    let _ = resize_owner.update(cx, |v, cx| {
                                        v.column = width.into();
                                        v.resizes.push(width.into());
                                        cx.notify();
                                    });
                                }),
                        ),
                    )
                    .cell(
                        TableHead::new("head-status")
                            .sticky(Sticky::Right)
                            .child("Status"),
                    ),
            );
        let mut body = TableBody::new("body");
        let order = if self.reversed { [1, 0] } else { [0, 1] };
        for i in order {
            if self.removed && i == 0 {
                continue;
            }
            let owner = owner.clone();
            let mut row = TableRow::new(format!("row-{i}")).variant(if i == 0 && self.selected {
                RowVariant::Selected
            } else {
                RowVariant::Default
            });
            if i == 0 {
                row = row.cell(
                    TableCheckCell::new("select", "Select homepage")
                        .state(if self.selected {
                            State::Checked
                        } else {
                            State::Unchecked
                        })
                        .disabled(self.disabled)
                        .track_focus(&self.focus)
                        .sticky(Sticky::Left)
                        .on_change(move |state, _, _, cx| {
                            let _ = owner.update(cx, |v, cx| {
                                v.changes.push(state);
                                if v.apply {
                                    v.selected = state == State::Checked;
                                }
                                cx.notify();
                            });
                        }),
                );
            } else {
                row = row.cell(TableCell::new("select").sticky(Sticky::Left).child("2"));
            }
            body = body.row(
                row.cell(TableCell::new("path").child(if i == 0 {
                    "/regional/café/requests"
                } else {
                    "A longer wrapping text value with several words"
                }))
                .cell(
                    TableCell::new("status")
                        .sticky(Sticky::Right)
                        .child(Button::new("inspect", "Inspect").size(crate::button::Size::Sm)),
                ),
            );
        }
        for i in 2..10 {
            body = body.row(
                TableRow::new(format!("row-{i}"))
                    .cell(
                        TableCell::new("select")
                            .sticky(Sticky::Left)
                            .child(format!("{i}")),
                    )
                    .cell(TableCell::new("path").child("More requests"))
                    .cell(TableCell::new("status").sticky(Sticky::Right).child("200")),
            );
        }
        div()
            .w(px(self.width))
            .flex()
            .flex_col()
            .tab_group()
            .on_action(|_: &FocusNext, window, cx| window.focus_next(cx))
            .child(
                Table::new("table")
                    .layout(Layout::Fixed)
                    .scroll_handle(&self.scroll)
                    .max_h(px(160.))
                    .columns([
                        ColumnWidth::Pixels(px(40.)),
                        ColumnWidth::Pixels(px(self.column)),
                        ColumnWidth::Pixels(px(100.)),
                    ])
                    .header(header)
                    .body(body)
                    .footer(
                        TableFooter::new("footer").row(
                            TableRow::new("total")
                                .cell(TableCell::new("span").column_span(3).child("Total")),
                        ),
                    ),
            )
            .child(Checkbox::new("after", "After table").track_focus(&self.after))
    }
}
fn interactive(cx: &mut gpui_kit::Context<Interactive>) -> Interactive {
    Interactive {
        width: 360.,
        compact: false,
        selected: false,
        disabled: false,
        apply: true,
        reversed: false,
        removed: false,
        changes: Vec::new(),
        focus: cx.focus_handle(),
        after: cx.focus_handle(),
        resize_focus: cx.focus_handle(),
        column: 180.,
        resizes: Vec::new(),
        scroll: ScrollHandle::new(),
    }
}
fn settle(window: &mut Window, cx: &mut App) {
    for _ in 0..3 {
        window.render_frame(cx);
    }
}

#[gpui_kit::test]
fn fixed_columns_compact_header_spans_and_paint_follow_theme_and_width(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|_, cx| interactive(cx));
    cx.update(|window, cx| {
        for appearance in [Appearance::Light, Appearance::Dark] {
            crate::set_appearance(appearance, cx);
            for compact in [false, true] {
                for width in [360., 220.] {
                    view.update(cx, |v, cx| {
                        v.compact = compact;
                        v.width = width;
                        v.selected = true;
                        cx.notify();
                    });
                    settle(window, cx);
                    let header = window.find("head-path").bounds();
                    let data = window.within("row-0").find("path").bounds();
                    assert_eq!(header.left(), data.left());
                    assert_eq!(header.size.width, px(180.));
                    assert_eq!(data.size.width, header.size.width);
                    assert_eq!(header.size.height, px(if compact { 38. } else { 46. }));
                    let span = window.find("span").bounds();
                    assert_eq!(span.size.width, px(320.));
                    let t = crate::theme(cx);
                    let quads = window.painted_quads();
                    assert!(
                        quads
                            .iter()
                            .any(|q| q.background == gpui_kit::Background::from(t.colors.tint))
                    );
                    assert_eq!(
                        window.within("row-0").find("checkbox").label(),
                        Some("Select homepage")
                    );
                }
            }
        }
    });
}

#[gpui_kit::test]
fn whole_check_cell_pointer_keyboard_controlled_rejection_and_reorder_keep_focus(
    cx: &mut TestAppContext,
) {
    cx.update(|cx| {
        crate::init(cx);
        cx.bind_keys([gpui_kit::KeyBinding::new("tab", FocusNext, None)]);
    });
    let (view, cx) = cx.add_window_view(|_, cx| interactive(cx));
    cx.update(settle);
    cx.run_until_parked();
    cx.update(|window, cx| {
        settle(window, cx);
        let cell = window.within("row-0").find("select").bounds();
        let checkbox = window.within("row-0").find("checkbox").bounds();
        assert_eq!(cell.size, checkbox.size);
        window
            .within("row-0")
            .click_at("select", point(px(2.), px(2.)), cx);
        assert_eq!(view.read(cx).changes, &[State::Checked]);
        assert!(view.read(cx).focus.is_focused(window));
        window.press("space", cx);
        window.press("enter", cx);
        assert_eq!(
            view.read(cx).changes,
            &[State::Checked, State::Unchecked, State::Checked]
        );
        view.update(cx, |v, cx| {
            v.reversed = true;
            v.apply = false;
            cx.notify();
        });
        settle(window, cx);
        assert!(view.read(cx).focus.is_focused(window));
        window.press("space", cx);
        window.press("space", cx);
        assert_eq!(
            view.read(cx).changes[3..],
            [State::Unchecked, State::Unchecked]
        );
        view.update(cx, |v, cx| {
            v.disabled = true;
            cx.notify();
        });
        settle(window, cx);
        window.within("row-0").click("select", cx);
        window.press("enter", cx);
        window.press("space", cx);
        assert_eq!(view.read(cx).changes.len(), 5);
        view.update(cx, |v, cx| {
            v.disabled = false;
            v.reversed = false;
            cx.notify();
        });
        settle(window, cx);
        window.within("row-0").click("select", cx);
        assert!(view.read(cx).focus.is_focused(window));
        view.update(cx, |v, cx| {
            v.removed = true;
            cx.notify();
        });
        settle(window, cx);
        assert!(window.try_find("row-0").is_none());
        // Removing the focused action must not trap traversal in stale identity.
        window.press("tab", cx);
        settle(window, cx);
        for _ in 0..5 {
            if view.read(cx).after.is_focused(window) {
                break;
            }
            window.press("tab", cx);
        }
        assert!(view.read(cx).after.is_focused(window));
    });
}

#[gpui_kit::test]
fn sticky_cells_and_header_keep_hit_targets_inside_the_scrolled_viewport(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|_, cx| interactive(cx));
    cx.update(|window, cx| {
        for appearance in [Appearance::Light, Appearance::Dark] {
            crate::set_appearance(appearance, cx);
            view.update(cx, |v, cx| {
                v.width = 220.;
                v.column = 260.;
                cx.notify();
            });
            settle(window, cx);
            let viewport = window.find("table").bounds();
            window.scroll("table", ScrollDelta::Pixels(point(px(-100.), px(0.))), cx);
            settle(window, cx);
            assert!(view.read(cx).scroll.offset().x < px(0.));
            window.scroll("table", ScrollDelta::Pixels(point(px(0.), px(-30.))), cx);
            settle(window, cx);
            assert!(view.read(cx).scroll.offset().y < px(0.));
            let head_left = window.find("head-select").bounds();
            let head_right = window.find("head-status").bounds();
            assert_eq!(head_left.left(), viewport.left());
            assert_eq!(head_right.right(), viewport.right());
            assert_eq!(head_left.top(), viewport.top());
            let cell = window.within("row-1").find("status").bounds();
            assert_eq!(cell.right(), viewport.right());
            // The header is painted above body cells, including their sticky edges.
            assert!(head_left.bottom() > viewport.top());
            view.read(cx).scroll.set_offset(point(px(0.), px(0.)));
            settle(window, cx);
        }
    });
}

#[gpui_kit::test]
fn resize_keyboard_and_drag_propose_owner_widths_and_disabled_is_inert(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|_, cx| interactive(cx));
    cx.update(|window, cx| {
        settle(window, cx);
        view.read(cx).resize_focus.clone().focus(window, cx);
        window.press("right", cx);
        assert_eq!(view.read(cx).column, 188.);
        window.press("left", cx);
        assert_eq!(view.read(cx).column, 180.);
        settle(window, cx);
        let from = window.find("resize").bounds().center();
        window.drag(from, from + point(px(25.), px(0.)), cx);
        assert_eq!(view.read(cx).column, 205.);
        settle(window, cx);
        assert_eq!(window.find("head-path").bounds().size.width, px(205.));
        let head = window.find("head-path").bounds();
        let handle = window.find("resize").bounds();
        assert_eq!(handle.right(), head.right());
        assert_eq!(handle.size.height, head.size.height - px(1.));
        view.update(cx, |v, cx| {
            v.disabled = true;
            cx.notify();
        });
        settle(window, cx);
        let count = view.read(cx).resizes.len();
        window.press("right", cx);
        let from = window.find("resize").bounds().center();
        window.drag(from, from + point(px(50.), px(0.)), cx);
        assert_eq!(view.read(cx).resizes.len(), count);
    });
}

struct Counts {
    empty: bool,
    node: std::rc::Rc<std::cell::RefCell<Option<gpui_kit::accesskit::Node>>>,
}
impl Render for Counts {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let root = if self.empty {
            Table::new("empty").body(TableBody::new("empty-body"))
        } else {
            Table::new("counts")
                .header(
                    TableHeader::new("head").row(
                        TableRow::new("h")
                            .cell(TableHead::new("a"))
                            .cell(TableHead::new("b")),
                    ),
                )
                .body(
                    TableBody::new("body")
                        .row(TableRow::new("r").cell(TableCell::new("all").column_span(2))),
                )
                .footer(
                    TableFooter::new("foot")
                        .row(TableRow::new("f").cell(TableCell::new("all").column_span(2))),
                )
        };
        let element = root.render(window, cx).into_element();
        let mut node = gpui_kit::accesskit::Node::new(Role::Table);
        element.write_a11y_info(&mut node);
        *self.node.borrow_mut() = Some(node);
        element
    }
}
#[gpui_kit::test]
fn root_counts_and_blank_table_empty_sections_are_semantic(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let node = std::rc::Rc::new(std::cell::RefCell::new(None));
    let (view, cx) = cx.add_window_view(|_, _| Counts {
        empty: false,
        node: node.clone(),
    });
    cx.update(|window, cx| {
        settle(window, cx);
        assert_eq!(node.borrow().as_ref().unwrap().row_count(), Some(3));
        assert_eq!(node.borrow().as_ref().unwrap().column_count(), Some(2));
        view.update(cx, |v, cx| {
            v.empty = true;
            cx.notify();
        });
        settle(window, cx);
        assert_eq!(node.borrow().as_ref().unwrap().row_count(), Some(0));
        assert_eq!(node.borrow().as_ref().unwrap().column_count(), Some(0));
    });
}

#[test]
#[should_panic(expected = "finite and positive")]
fn invalid_external_column_width_is_rejected() {
    let _ = Table::new("table").columns([ColumnWidth::Pixels(px(f32::NAN))]);
}
#[gpui_kit::test]
fn headers_and_data_cells_share_columns_and_native_roles(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (_, cx) = cx.add_window_view(|_, _| Harness);
    cx.update(|window, cx| {
        for _ in 0..3 {
            window.render_frame(cx);
        }
        assert_eq!(window.find("table").role(), Some(Role::Table));
        assert_eq!(window.find("table").label(), Some("Requests"));
        assert_eq!(window.find("columns").role(), Some(Role::Row));
        assert_eq!(window.find("head-path").role(), Some(Role::ColumnHeader));
        assert_eq!(window.find("path").role(), Some(Role::Cell));
        let head = window.find("head-path").bounds();
        let cell = window.find("path").bounds();
        assert_eq!(head.left(), cell.left());
        assert_eq!(head.size.width, cell.size.width);
        assert!(cell.size.width > px(0.));
        assert_eq!(window.find("table").bounds().size.width, px(400.));
    });
}

struct ContentChanges {
    fixed: bool,
    long: bool,
    width: f32,
}
impl Render for ContentChanges {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().w(px(self.width)).child(
            Table::new("content-table")
                .layout(if self.fixed { Layout::Fixed } else { Layout::Auto })
                .header(TableHeader::new("header").variant(HeaderVariant::Compact).row(
                    TableRow::new("head")
                        .cell(TableHead::new("path-head").text("Path"))
                        .cell(TableHead::new("result-head").text("Result")),
                ))
                .body(TableBody::new("body").row(
                    TableRow::new("data")
                        .cell(TableCell::new("path").text(if self.long {
                            "/regional/archive/café/international/requests"
                        } else { "/" }))
                        .cell(TableCell::new("result").text(
                            "A long result wraps under fixed sizing without losing shared column alignment.",
                        )),
                )),
        )
    }
}
#[gpui_kit::test]
fn automatic_widths_refresh_with_content_and_fixed_text_wraps_readably(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|_, _| ContentChanges {
        fixed: false,
        long: false,
        width: 700.,
    });
    cx.update(|window, cx| {
        for appearance in [Appearance::Light, Appearance::Dark] {
            crate::set_appearance(appearance, cx);
            view.update(cx, |v, cx| { v.fixed = false; v.long = false; v.width = 700.; cx.notify(); });
            settle(window, cx);
            let short = window.find("path").bounds().size.width;
            view.update(cx, |v, cx| { v.long = true; cx.notify(); });
            settle(window, cx);
            let long = window.find("path").bounds().size.width;
            assert!(long > short);
            assert_eq!(window.find("path-head").bounds().size.width, long);
            let stable = window.find("path").bounds();
            settle(window, cx);
            assert_eq!(window.find("path").bounds(), stable);
            view.update(cx, |v, cx| { v.fixed = true; v.width = 280.; cx.notify(); });
            settle(window, cx);
            let result = window.find("result").bounds();
            assert_eq!(result.size.width, px(140.));
            assert!(result.size.height > px(45.));
            assert_eq!(window.find("path").bounds().size.height, result.size.height);
            assert_eq!(window.within("result").find("text-0").label(), Some(
                "A long result wraps under fixed sizing without losing shared column alignment.",
            ));
        }
    });
}
