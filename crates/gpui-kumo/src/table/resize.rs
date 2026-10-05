use gpui_kit::{
    App, ClickEvent, DispatchPhase, ElementId, FocusHandle, InteractiveElement, IntoElement,
    KeyBinding, MouseButton, MouseMoveEvent, MouseUpEvent, ParentElement, Pixels, RenderOnce,
    SharedString, Styled, Window, base, canvas, div, prelude::FluentBuilder, px,
};
use std::{cell::RefCell, ops::RangeInclusive, rc::Rc};

gpui_kit::actions!(kumo_table_resize, [Narrower, Wider]);
pub(crate) fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("left", Narrower, Some("KumoTableResize")),
        KeyBinding::new("right", Wider, Some("KumoTableResize")),
    ]);
}
type Handler = Rc<dyn Fn(Pixels, &mut Window, &mut App)>;
#[derive(Default)]
struct Drag {
    start: Option<(Pixels, Pixels)>,
    last: Option<Pixels>,
}

/// An owner-controlled column resize affordance, placed inside `TableHead`.
/// Dragging proposes pixel widths; Left/Right propose eight-pixel adjustments.
/// The owner updates `Table::columns` and redraws. The handle never owns a second
/// column-width model. It appears on header hover or keyboard focus.
///
/// ```no_run
/// use gpui_kit::{ParentElement, px};
/// use gpui_kumo::{TableHead, TableResizeHandle};
/// let column = TableHead::new("path")
///     .child("Path")
///     .resize_handle(TableResizeHandle::new("resize", "Resize path column", px(240.))
///         .on_resize(|width, window, cx| { /* update the owner's width, then notify */ }));
/// ```
#[derive(IntoElement)]
#[must_use]
pub struct TableResizeHandle {
    id: ElementId,
    name: SharedString,
    width: Pixels,
    range: RangeInclusive<Pixels>,
    disabled: bool,
    focus: Option<FocusHandle>,
    handler: Option<Handler>,
}
impl TableResizeHandle {
    /// Create a named handle for the owner's current column width.
    ///
    /// # Panics
    /// Panics for blank names or non-positive/non-finite widths.
    pub fn new(id: impl Into<ElementId>, name: impl Into<SharedString>, width: Pixels) -> Self {
        super::positive(width);
        Self {
            id: id.into(),
            name: crate::name::nonblank(name),
            width,
            range: px(24.)..=px(4096.),
            disabled: false,
            focus: None,
            handler: None,
        }
    }
    /// Override the authored accessible name.
    ///
    /// # Panics
    /// Panics if `name` is blank.
    pub fn accessibility_label(mut self, name: impl Into<SharedString>) -> Self {
        self.name = crate::name::nonblank(name);
        self
    }
    /// Restrict proposed widths to a finite positive inclusive range.
    ///
    /// # Panics
    /// Panics for invalid endpoints or a reversed range.
    pub fn range(mut self, range: RangeInclusive<Pixels>) -> Self {
        super::positive(*range.start());
        super::positive(*range.end());
        assert!(
            range.start() <= range.end(),
            "Table resize range must be ordered"
        );
        self.range = range;
        self
    }
    /// Gate pointer and keyboard resizing; unavailable handles leave traversal.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    /// Use a caller-retained focus target.
    pub fn track_focus(mut self, focus: &FocusHandle) -> Self {
        self.focus = Some(focus.clone());
        self
    }
    /// Propose a new column width. The owner must commit it and notify its view.
    pub fn on_resize(mut self, handler: impl Fn(Pixels, &mut Window, &mut App) + 'static) -> Self {
        self.handler = Some(Rc::new(handler));
        self
    }
}
impl std::fmt::Debug for TableResizeHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TableResizeHandle")
            .field("id", &self.id)
            .field("width", &self.width)
            .field("range", &self.range)
            .field("disabled", &self.disabled)
            .finish_non_exhaustive()
    }
}
impl RenderOnce for TableResizeHandle {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = crate::theme(cx).clone();
        let focus = self.focus.unwrap_or_else(|| {
            window
                .use_keyed_state(self.id.clone(), cx, |_, cx| cx.focus_handle())
                .read(cx)
                .clone()
        });
        let drag = window
            .use_keyed_state((self.id.clone(), "drag"), cx, |_, _| {
                Rc::new(RefCell::new(Drag::default()))
            })
            .read(cx)
            .clone();
        if self.disabled {
            *drag.borrow_mut() = Drag::default();
        }
        let disabled = self.disabled;
        let width = self.width;
        let range = self.range;
        let move_drag = drag.clone();
        let release_drag = drag.clone();
        let down_focus = focus.clone();
        let handler = self.handler;
        let move_handler = handler.clone();
        let narrow_handler = handler.clone();
        let narrow_range = range.clone();
        let move_range = range.clone();
        let events = canvas(
            |_, _, _| (),
            move |_, _, window, _| {
                window.on_mouse_event(move |event: &MouseMoveEvent, phase, window, cx| {
                    if phase != DispatchPhase::Bubble || disabled {
                        return;
                    }
                    let next = {
                        let mut drag = move_drag.borrow_mut();
                        let Some((start, initial)) = drag.start else {
                            return;
                        };
                        let value = (initial + event.position.x - start)
                            .clamp(*move_range.start(), *move_range.end());
                        if drag.last == Some(value) {
                            return;
                        }
                        drag.last = Some(value);
                        value
                    };
                    if let Some(handler) = &move_handler {
                        handler(next, window, cx);
                    }
                    cx.stop_propagation();
                });
                window.on_mouse_event(move |event: &MouseUpEvent, phase, _, _| {
                    if phase == DispatchPhase::Bubble && event.button == MouseButton::Left {
                        release_drag.borrow_mut().start = None;
                    }
                });
            },
        )
        .absolute()
        .inset_0();
        base::Button::new(self.id)
            .accessibility_label(self.name)
            .disabled(disabled)
            .track_focus(&focus)
            .key_context("KumoTableResize")
            .absolute()
            .right_0()
            .top_0()
            .w(px(10.))
            .h_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(t.colors.base)
            .opacity(0.)
            .group_hover("kumo-table-head", |this| this.opacity(1.))
            .focus_visible(|style| style.opacity(1.).border_2().border_color(t.colors.brand))
            .when(!disabled, |this| this.cursor_col_resize())
            .on_mouse_down(MouseButton::Left, move |event, window, cx| {
                if disabled {
                    return;
                }
                down_focus.focus(window, cx);
                let mut drag = drag.borrow_mut();
                drag.start = Some((event.position.x, width));
                drag.last = Some(width);
                cx.stop_propagation();
            })
            .on_action(move |_: &Narrower, window, cx| {
                if !disabled && let Some(handler) = &narrow_handler {
                    handler(
                        (width - px(8.)).clamp(*narrow_range.start(), *narrow_range.end()),
                        window,
                        cx,
                    );
                }
                cx.stop_propagation();
            })
            .on_action(move |_: &Wider, window, cx| {
                if !disabled && let Some(handler) = &handler {
                    handler(
                        (width + px(8.)).clamp(*range.start(), *range.end()),
                        window,
                        cx,
                    );
                }
                cx.stop_propagation();
            })
            .on_click(|_: &ClickEvent, _, cx| cx.stop_propagation())
            .child(
                div()
                    .w(px(2.))
                    .h(px(20.))
                    .rounded(px(1.))
                    .bg(t.colors.hairline),
            )
            .child(events)
    }
}
