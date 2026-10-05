use super::*;
use gpui_kit::base::StyledExt;
use gpui_kit::{
    AvailableSpace, Bounds, ContentMask, Element, FontWeight, GlobalElementId, InspectorElementId,
    InteractiveElement, LayoutId, Point, Refineable, Size, StatefulInteractiveElement, Style, base,
    canvas, div, linear_color_stop, linear_gradient, point, prelude::FluentBuilder, px, size,
};
use std::{
    cell::{Cell, RefCell},
    collections::HashSet,
    rc::Rc,
};

struct Columns {
    intrinsic: Vec<Pixels>,
    check: Vec<bool>,
    widths: Option<Vec<Pixels>>,
    header_height: Pixels,
}
type Cache = Rc<RefCell<Option<Vec<Pixels>>>>;
type SharedColumns = Rc<RefCell<Columns>>;
type Viewport = Rc<Cell<Bounds<Pixels>>>;
struct FocusScope {
    focus: gpui_kit::FocusHandle,
    _subscription: gpui_kit::Subscription,
}

pub(super) fn render(table: Table, window: &mut Window, cx: &mut App) -> impl IntoElement {
    let t = crate::theme(cx).clone();
    let scope = window
        .use_keyed_state((table.id.clone(), "focus"), cx, |window, cx| {
            let focus = cx.focus_handle().tab_stop(false);
            let subscription = cx.on_focus_lost(window, |state: &mut FocusScope, window, cx| {
                if window.focus_lost_restore_target(cx).as_ref() == Some(&state.focus) {
                    state.focus.focus(window, cx);
                }
            });
            FocusScope {
                focus,
                _subscription: subscription,
            }
        })
        .read(cx)
        .focus
        .clone();
    let count = table
        .header
        .iter()
        .flat_map(|g| &g.rows)
        .map(|r| r.cells.iter().map(|c| c.content.span).sum())
        .chain(
            table
                .bodies
                .iter()
                .flat_map(|g| &g.rows)
                .map(|r| r.cells.iter().map(|c| c.content.span).sum()),
        )
        .chain(
            table
                .footer
                .iter()
                .flat_map(|g| &g.rows)
                .map(|r| r.cells.iter().map(|c| c.content.span).sum()),
        )
        .max()
        .unwrap_or(table.columns.len());
    assert!(count <= 65535, "Table supports at most 65535 columns");
    assert!(
        table.columns.len() <= count,
        "Table width specifications exceed its column count"
    );
    let row_count = table.header.as_ref().map_or(0, |g| g.rows.len())
        + table.bodies.iter().map(|g| g.rows.len()).sum::<usize>()
        + table.footer.as_ref().map_or(0, |g| g.rows.len());
    let cache = window
        .use_keyed_state((table.id.clone(), "columns"), cx, |_, _| {
            Rc::new(RefCell::new(None::<Vec<Pixels>>))
        })
        .read(cx)
        .clone();
    let previous = cache.borrow().clone().filter(|w| w.len() == count);
    let columns = Rc::new(RefCell::new(Columns {
        intrinsic: vec![px(0.); count],
        check: vec![false; count],
        widths: previous,
        header_height: px(0.),
    }));
    let viewport = Rc::new(Cell::new(Bounds::default()));
    let scroll = table.scroll.unwrap_or_else(|| {
        window
            .use_keyed_state((table.id.clone(), "scroll"), cx, |_, _| ScrollHandle::new())
            .read(cx)
            .clone()
    });
    let mut sections = Vec::new();
    let mut group_ids = HashSet::new();
    if let Some(caption) = &table.caption {
        group_ids.insert(caption.id.clone());
    }
    let mut index = 0;
    if let Some(header) = table.header {
        assert!(
            group_ids.insert(header.id.clone()),
            "Table section IDs must be unique"
        );
        let mut ids = HashSet::new();
        let rows = header
            .rows
            .into_iter()
            .enumerate()
            .map(|(stripe, row)| {
                assert!(
                    ids.insert(row.id.clone()),
                    "Table row IDs must be unique within a section"
                );
                index += 1;
                render_row(
                    row.id,
                    row.cells.into_iter().map(|c| c.content).collect(),
                    row.variant,
                    stripe,
                    index,
                    Some(header.variant),
                    header.sticky,
                    &columns,
                    &viewport,
                    &t,
                )
            })
            .collect::<Vec<_>>();
        sections.push(
            base::TableHeader::new(header.id)
                .flex()
                .flex_col()
                .w_full()
                .children(rows)
                .into_any_element(),
        );
    }
    for body in table.bodies {
        assert!(
            group_ids.insert(body.id.clone()),
            "Table section IDs must be unique"
        );
        let mut ids = HashSet::new();
        let rows = body
            .rows
            .into_iter()
            .enumerate()
            .map(|(stripe, row)| {
                assert!(
                    ids.insert(row.id.clone()),
                    "Table row IDs must be unique within a section"
                );
                index += 1;
                render_row(
                    row.id,
                    row.cells.into_iter().map(|c| c.content).collect(),
                    row.variant,
                    stripe,
                    index,
                    None,
                    false,
                    &columns,
                    &viewport,
                    &t,
                )
            })
            .collect::<Vec<_>>();
        sections.push(
            base::TableBody::new(body.id)
                .flex()
                .flex_col()
                .w_full()
                .children(rows)
                .into_any_element(),
        );
    }
    if let Some(footer) = table.footer {
        assert!(
            group_ids.insert(footer.id.clone()),
            "Table section IDs must be unique"
        );
        let mut ids = HashSet::new();
        let rows = footer
            .rows
            .into_iter()
            .enumerate()
            .map(|(stripe, row)| {
                assert!(
                    ids.insert(row.id.clone()),
                    "Table row IDs must be unique within a section"
                );
                index += 1;
                render_row(
                    row.id,
                    row.cells.into_iter().map(|c| c.content).collect(),
                    row.variant,
                    stripe,
                    index,
                    None,
                    false,
                    &columns,
                    &viewport,
                    &t,
                )
            })
            .collect::<Vec<_>>();
        sections.push(
            base::TableBody::new(footer.id)
                .flex()
                .flex_col()
                .w_full()
                .children(rows)
                .into_any_element(),
        );
    }
    let content = div()
        .flex()
        .flex_col()
        .w_full()
        .children(sections)
        .into_any_element();
    let before_gesture = Rc::new(Cell::new(scroll.offset()));
    let grid = TableLayout {
        content: Rc::new(RefCell::new(content)),
        columns,
        strategy: table.layout,
        specified: table.columns,
        minimum: table.minimum,
        cache,
        viewport: viewport.clone(),
        scroll: scroll.clone(),
        before_gesture: before_gesture.clone(),
    };
    let rect = viewport.clone();
    let viewport_scroll = scroll.clone();
    let gesture_scroll = scroll.clone();
    let mut root = base::Table::new(table.id)
        .track_focus(&scope)
        .row_count(row_count)
        .column_count(count)
        .relative()
        .w_full()
        .min_w_0()
        .flex()
        .flex_col()
        .overflow_x_scroll()
        .overflow_y_scroll()
        .restrict_scroll_to_axis()
        .track_scroll(&scroll)
        .on_scroll_wheel(move |event, window, cx| {
            // GPUI's own scroll listener runs first in the bubble phase. Keep a
            // consumed gesture out of enclosing page scrollers; at a vertical
            // boundary, allow the page to continue scrolling normally.
            let offset = gesture_scroll.offset();
            // GPUI clamps raw offsets during the next layout, so an attempted
            // scroll past an edge can look like movement here. Compare the
            // effective positions to preserve boundary handoff on this event.
            let maximum = gesture_scroll.max_offset();
            let clamp = |p: Point<Pixels>| {
                point(p.x.clamp(-maximum.x, px(0.)), p.y.clamp(-maximum.y, px(0.)))
            };
            let changed = clamp(before_gesture.get()) != clamp(offset);
            let delta = event.delta.pixel_delta(window.line_height());
            if changed || delta.x.abs() > delta.y.abs() {
                cx.stop_propagation();
            }
        })
        .text_size(t.typography.base.size)
        .line_height(t.typography.base.line_height)
        .font_weight(FontWeight::NORMAL)
        .text_color(t.text.default)
        .when_some(table.name, |this, name| this.accessibility_label(name))
        .child(
            canvas(
                move |bounds, _, _| {
                    rect.set(Bounds::new(
                        bounds.origin - viewport_scroll.offset(),
                        bounds.size,
                    ))
                },
                |_, _, _, _| {},
            )
            .absolute()
            .inset_0()
            .size_full(),
        );
    if let Some(caption) = table.caption {
        root = root.child(
            base::TableCaption::new(caption.id)
                .w_full()
                .p(px(12.))
                .child(crate::Text::new("text", caption.text)),
        );
    }
    root.child(grid)
        .refine_style(&table.style)
        .render(window, cx)
}

#[expect(
    clippy::too_many_arguments,
    reason = "Private rendering context keeps public composition independent of Base"
)]
fn render_row(
    id: ElementId,
    cells: Vec<CellContent>,
    variant: RowVariant,
    stripe: usize,
    index: usize,
    header: Option<HeaderVariant>,
    sticky_header: bool,
    columns: &SharedColumns,
    viewport: &Viewport,
    t: &crate::Theme,
) -> impl IntoElement {
    let row_fill = if variant == RowVariant::Selected {
        t.colors.tint
    } else if stripe % 2 == 1 {
        t.colors.elevated
    } else {
        t.colors.base
    };
    let fill = match header {
        Some(HeaderVariant::Compact) => t.colors.elevated,
        Some(HeaderVariant::Default) => t.colors.base,
        None => row_fill,
    };
    let mut column = 0;
    let mut ids = HashSet::new();
    let entries = cells
        .into_iter()
        .map(|cell| {
            assert!(
                ids.insert(cell.id.clone()),
                "Table cell IDs must be unique within a row"
            );
            let start = column;
            column += cell.span;
            let check = cell.check.is_some();
            if check {
                columns.borrow_mut().check[start] = true;
            }
            let mut content = cell.children;
            if let Some(checkbox) = cell.check {
                content.push(
                    checkbox
                        .table_cell(px(if header == Some(HeaderVariant::Compact) {
                            8.
                        } else {
                            12.
                        }))
                        .into_any_element(),
                );
            }
            // A plain text element in a horizontal flex row keeps its min-content
            // width. A shrinkable block gives rich/plain content the actual column
            // constraint while the outer cell vertically centres it.
            if !check {
                content = vec![
                    div()
                        .min_w_0()
                        .flex_1()
                        .children(content)
                        .into_any_element(),
                ];
            }
            if let Some(handle) = cell.resize {
                content.push(handle.into_any_element());
            }
            let mut style = StyleRefinement::default()
                .relative()
                .w_full()
                .h_full()
                .flex()
                .items_center()
                .p(px(if check { 0. } else { 12. }))
                .bg(fill);
            if header.is_some() {
                style = style
                    .border_b_1()
                    .border_color(t.colors.fill)
                    .font_weight(FontWeight::SEMIBOLD);
                if header == Some(HeaderVariant::Compact) {
                    style = style
                        .py(px(if check { 0. } else { 8. }))
                        .text_color(t.text.strong);
                }
            }
            style.refine(&cell.style);
            // Spans are table semantics, not duplicate labels on every cell.
            let span = cell.span;
            let element = if header.is_some() {
                base::TableHead::new(cell.id, start + 1)
                    .group("kumo-table-head")
                    .a11y_synthetic_children(move |b| b.parent_node().set_column_span(span))
                    .children(content)
                    .refine_style(&style)
                    .into_any_element()
            } else {
                base::TableCell::new(cell.id, start + 1)
                    .a11y_synthetic_children(move |b| b.parent_node().set_column_span(span))
                    .children(content)
                    .refine_style(&style)
                    .into_any_element()
            };
            Entry {
                element: Some(element),
                start,
                span,
                sticky: cell.sticky,
                size: Size::default(),
                origin: Point::default(),
                mask: Bounds::default(),
                fill,
            }
        })
        .collect();
    base::TableRow::new(id, index)
        .w_full()
        .bg(row_fill)
        .child(RowLayout {
            entries: Rc::new(RefCell::new(entries)),
            columns: columns.clone(),
            viewport: viewport.clone(),
            sticky_header,
            header: header.is_some(),
        })
}

struct TableLayout {
    content: Rc<RefCell<AnyElement>>,
    columns: SharedColumns,
    strategy: Layout,
    specified: Vec<ColumnWidth>,
    minimum: Pixels,
    cache: Cache,
    viewport: Viewport,
    scroll: ScrollHandle,
    before_gesture: Rc<Cell<Point<Pixels>>>,
}
impl IntoElement for TableLayout {
    type Element = Self;
    fn into_element(self) -> Self {
        self
    }
}
impl Element for TableLayout {
    type RequestLayoutState = ();
    type PrepaintState = ();
    fn id(&self) -> Option<ElementId> {
        None
    }
    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }
    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        _cx: &mut App,
    ) -> (LayoutId, ()) {
        let width = self
            .columns
            .borrow()
            .widths
            .as_ref()
            .map(|w| w.iter().copied().sum::<Pixels>());
        let measured = self.content.borrow_mut().layout_as_root(
            size(
                width.map_or(AvailableSpace::MaxContent, AvailableSpace::Definite),
                AvailableSpace::MaxContent,
            ),
            window,
            _cx,
        );
        // An auto-width flex-column child is stretched to its viewport despite
        // reporting a wider intrinsic measurement. Author the grid width so the
        // scroll container sees the same content extent that rows actually paint.
        let mut style = Style::default();
        style.refine(
            &StyleRefinement::default()
                .w(measured.width.max(self.minimum))
                .flex_shrink_0(),
        );
        let id = window.request_measured_layout(style, move |_, _, _, _| measured);
        (id, ())
    }
    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        let columns = self.columns.borrow();
        let budget = self.viewport.get().size.width.max(self.minimum);
        let desired = resolve_widths(&columns, &self.specified, self.strategy, budget);
        if self.cache.borrow().as_ref() != Some(&desired) {
            *self.cache.borrow_mut() = Some(desired);
            let owner = window.current_view();
            cx.notify(owner);
            window.request_animation_frame();
        }
        drop(columns);
        self.content
            .borrow_mut()
            .prepaint_at(bounds.origin, window, cx);
    }
    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut (),
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        // Register from the full grid, not the viewport canvas: the absolute
        // canvas can be outside the paint mask once the content is scrolled.
        let scroll = self.scroll.clone();
        let before = self.before_gesture.clone();
        window.on_mouse_event(move |_: &gpui_kit::ScrollWheelEvent, phase, _, _| {
            if phase == gpui_kit::DispatchPhase::Capture {
                before.set(scroll.offset());
            }
        });
        self.content.borrow_mut().paint(window, cx);
    }
}

struct Entry {
    element: Option<AnyElement>,
    start: usize,
    span: usize,
    sticky: Option<Sticky>,
    size: Size<Pixels>,
    origin: Point<Pixels>,
    mask: Bounds<Pixels>,
    fill: gpui_kit::Hsla,
}
struct RowLayout {
    entries: Rc<RefCell<Vec<Entry>>>,
    columns: SharedColumns,
    viewport: Viewport,
    sticky_header: bool,
    header: bool,
}
impl IntoElement for RowLayout {
    type Element = Self;
    fn into_element(self) -> Self {
        self
    }
}
impl Element for RowLayout {
    type RequestLayoutState = ();
    type PrepaintState = ();
    fn id(&self) -> Option<ElementId> {
        None
    }
    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }
    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        _cx: &mut App,
    ) -> (LayoutId, ()) {
        let widths = self.columns.borrow().widths.clone();
        let mut entries = self.entries.borrow_mut();
        let mut height = px(0.);
        let mut total = px(0.);
        for entry in entries.iter_mut() {
            let element = entry
                .element
                .as_mut()
                .expect("cells are measured before prepaint");
            let natural = element.layout_as_root(
                size(AvailableSpace::MaxContent, AvailableSpace::MaxContent),
                window,
                _cx,
            );
            for i in entry.start..entry.start + entry.span {
                let mut columns = self.columns.borrow_mut();
                columns.intrinsic[i] = columns.intrinsic[i].max(natural.width / entry.span as f32);
            }
            let width = widths.as_ref().map(|w| {
                w[entry.start..entry.start + entry.span]
                    .iter()
                    .copied()
                    .sum::<Pixels>()
            });
            let measured = element.layout_as_root(
                size(
                    width.map_or(AvailableSpace::MaxContent, AvailableSpace::Definite),
                    AvailableSpace::MaxContent,
                ),
                window,
                _cx,
            );
            entry.size = size(width.unwrap_or(measured.width), measured.height);
            height = height.max(measured.height);
            total += entry.size.width;
        }
        let measured = size(total, height);
        let id = window.request_measured_layout(Style::default(), move |_, _, _, _| measured);
        (id, ())
    }
    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        let viewport = self.viewport.get();
        let mut entries = self.entries.borrow_mut();
        let mut x = bounds.left();
        let mut left = viewport.left();
        let mut right = viewport.right();
        let header_offset = self.columns.borrow().header_height;
        if self.sticky_header {
            self.columns.borrow_mut().header_height += bounds.size.height;
        }
        for entry in entries.iter_mut() {
            entry.size.height = bounds.size.height;
            entry
                .element
                .as_mut()
                .expect("cells are laid out before prepaint")
                .layout_as_root(
                    size(
                        AvailableSpace::Definite(entry.size.width),
                        AvailableSpace::Definite(bounds.size.height),
                    ),
                    window,
                    cx,
                );
            entry.origin = point(
                x,
                if self.sticky_header {
                    bounds.top().max(viewport.top() + header_offset)
                } else {
                    bounds.top()
                },
            );
            x += entry.size.width;
        }
        let mut mask_left = viewport.left();
        let mut mask_right = viewport.right();
        for entry in entries
            .iter_mut()
            .filter(|e| e.sticky == Some(Sticky::Left))
        {
            entry.origin.x = entry.origin.x.max(left);
            if entry.origin.x == left {
                mask_left = left + entry.size.width;
            }
            left += entry.size.width;
        }
        for entry in entries
            .iter_mut()
            .rev()
            .filter(|e| e.sticky == Some(Sticky::Right))
        {
            right -= entry.size.width;
            entry.origin.x = entry.origin.x.min(right);
            if entry.origin.x == right {
                mask_right = right;
            }
        }
        for entry in entries.iter_mut() {
            let top = if self.header {
                viewport.top()
            } else {
                viewport.top() + self.columns.borrow().header_height
            };
            entry.mask = Bounds::from_corners(
                point(
                    if entry.sticky.is_some() {
                        viewport.left()
                    } else {
                        mask_left
                    },
                    top,
                ),
                point(
                    if entry.sticky.is_some() {
                        viewport.right()
                    } else {
                        mask_right.max(mask_left)
                    },
                    viewport.bottom().max(top),
                ),
            );
            window.with_content_mask(Some(ContentMask { bounds: entry.mask }), |window| {
                entry
                    .element
                    .as_mut()
                    .expect("cell prepainted once")
                    .prepaint_at(entry.origin, window, cx);
            });
            if entry.sticky.is_some() || self.sticky_header {
                let element = entry.element.take().expect("cell painted once");
                let priority = if self.header { 30 } else { 20 };
                // Prepaint in the semantic row scope. Only painting is deferred:
                // GPUI deferred prepaint does not restore accessibility ancestry.
                let mut paint = PaintOnly(element).into_any_element();
                paint.layout_as_root(
                    size(
                        AvailableSpace::Definite(entry.size.width),
                        AvailableSpace::Definite(entry.size.height),
                    ),
                    window,
                    cx,
                );
                window.defer_draw(
                    paint,
                    entry.origin,
                    priority,
                    Some(ContentMask { bounds: entry.mask }),
                );
                if let Some(side) = entry.sticky {
                    let origin = point(
                        if side == Sticky::Left {
                            entry.origin.x + entry.size.width
                        } else {
                            entry.origin.x - px(24.)
                        },
                        entry.origin.y,
                    );
                    let (first, last) = if side == Sticky::Left {
                        (entry.fill, entry.fill.alpha(0.))
                    } else {
                        (entry.fill.alpha(0.), entry.fill)
                    };
                    let mut fade = div()
                        .w(px(24.))
                        .h(entry.size.height)
                        .bg(linear_gradient(
                            90.,
                            linear_color_stop(first, 0.),
                            linear_color_stop(last, 1.),
                        ))
                        .into_any_element();
                    fade.layout_as_root(
                        size(
                            AvailableSpace::Definite(px(24.)),
                            AvailableSpace::Definite(entry.size.height),
                        ),
                        window,
                        cx,
                    );
                    window.defer_draw(
                        fade,
                        origin,
                        priority,
                        Some(ContentMask { bounds: entry.mask }),
                    );
                }
            }
        }
    }
    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut (),
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        for entry in self.entries.borrow_mut().iter_mut() {
            if let Some(element) = &mut entry.element {
                window.with_content_mask(Some(ContentMask { bounds: entry.mask }), |window| {
                    element.paint(window, cx)
                });
            }
        }
    }
}

fn resolve_widths(
    columns: &Columns,
    specified: &[ColumnWidth],
    strategy: Layout,
    budget: Pixels,
) -> Vec<Pixels> {
    let mut widths = vec![px(0.); columns.intrinsic.len()];
    let mut automatic = Vec::new();
    for (i, width) in widths.iter_mut().enumerate() {
        match specified.get(i).copied().unwrap_or_default() {
            ColumnWidth::Pixels(value) => *width = value,
            ColumnWidth::Auto if columns.check[i] => *width = px(40.),
            ColumnWidth::Auto => {
                automatic.push(i);
                if strategy == Layout::Auto {
                    *width = columns.intrinsic[i];
                }
            }
        }
    }
    let occupied: Pixels = widths.iter().copied().sum();
    if !automatic.is_empty() {
        let spare = (budget - occupied).max(px(0.)) / automatic.len() as f32;
        for i in automatic {
            widths[i] += spare.max(if strategy == Layout::Fixed {
                px(24.)
            } else {
                px(0.)
            });
        }
    }
    widths
}

// Preserve native semantics/hitboxes in RowLayout while deferring only visuals.
struct PaintOnly(AnyElement);
impl IntoElement for PaintOnly {
    type Element = Self;
    fn into_element(self) -> Self {
        self
    }
}
impl Element for PaintOnly {
    type RequestLayoutState = ();
    type PrepaintState = ();
    fn id(&self) -> Option<ElementId> {
        None
    }
    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }
    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, ()) {
        (window.request_layout(Style::default(), [], cx), ())
    }
    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut (),
        _: &mut Window,
        _: &mut App,
    ) {
    }
    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut (),
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        self.0.paint(window, cx);
    }
}
