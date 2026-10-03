use super::*;
use gpui_kit::{MouseButton, Pixels};

pub(super) struct Drag {
    pub start: Pixels,
    pub offset: Pixels,
    pub moved: bool,
}
#[derive(Clone, Copy)]
pub(super) struct ScrollMotion {
    pub from: Pixels,
    pub target: Pixels,
    pub generation: usize,
}
impl<T: Clone + Eq + 'static> TabsState<T> {
    pub(super) fn scroll_edges(&self) -> [bool; 2] {
        if !self.layout_valid.get()
            || self.disabled
            || self.variant != Variant::Segmented
            || self.items.is_empty()
        {
            return [false; 2];
        }
        let max = self.scroll.max_offset().x;
        let position = -self.scroll.offset().x;
        [
            max > px(1.) && position > px(1.),
            max > px(1.) && max - position > px(1.),
        ]
    }
    pub(super) fn restore_hidden_control(&self, window: &mut Window, cx: &mut Context<Self>) {
        let visible = self.scroll_edges();
        let hidden = self
            .controls
            .iter()
            .enumerate()
            .find(|(i, f)| f.is_focused(window) && !visible[*i])
            .map(|(i, _)| i);
        if let Some(side) = hidden {
            let target = if !self.disabled && self.scroll.max_offset().x > px(1.) {
                if side == 0 {
                    self.items.iter().position(|i| !i.tab.disabled)
                } else {
                    self.items.iter().rposition(|i| !i.tab.disabled)
                }
            } else {
                self.entry(window)
            };
            if let Some(entry) = target {
                self.items[entry].focus.focus(window, cx);
                self.scroll.scroll_to_item(entry + 1);
            } else {
                self.leave_compound(window, cx);
            }
        }
    }
    pub(super) fn leave_compound(&self, window: &mut Window, cx: &mut Context<Self>) {
        for _ in 0..self.items.len() + 3 {
            window.focus_next(cx);
            if !self.controls.iter().any(|f| f.is_focused(window))
                && !self.items.iter().any(|i| i.focus.is_focused(window))
            {
                return;
            }
        }
        window.blur(cx);
    }
    pub(super) fn scroll_page(&mut self, side: usize, window: &mut Window, cx: &mut Context<Self>) {
        if !self.scroll_edges()[side] {
            return;
        }
        self.controls[side].focus(window, cx);
        self.drag.borrow_mut().take();
        let viewport = self.scroll.bounds().size.width;
        let mut distance = px(0.);
        for index in 0..self.items.len() {
            let Some(bounds) = self.scroll.bounds_for_item(index + 1) else {
                break;
            };
            if distance + bounds.size.width > viewport {
                if distance == px(0.) {
                    distance = viewport;
                }
                break;
            }
            distance += bounds.size.width;
            if index == self.items.len() - 1 {
                distance = (viewport * 0.8).floor().max(px(80.));
            }
        }
        let from = self.scroll.offset().x;
        let target = (from + if side == 0 { distance } else { -distance })
            .clamp(-self.scroll.max_offset().x, px(0.));
        if cx.reduce_motion() {
            self.scroll_motion.borrow_mut().take();
            self.scroll.set_offset(gpui_kit::point(target, px(0.)));
        } else {
            self.scroll_generation += 1;
            *self.scroll_motion.borrow_mut() = Some(ScrollMotion {
                from,
                target,
                generation: self.scroll_generation,
            });
        }
        cx.notify();
    }
    pub(super) fn begin_drag(&mut self, x: Pixels, button: MouseButton) {
        if button == MouseButton::Left
            && self.layout_valid.get()
            && !self.disabled
            && self.scroll.max_offset().x > px(1.)
        {
            self.scroll_motion.borrow_mut().take();
            *self.drag.borrow_mut() = Some(Drag {
                start: x,
                offset: self.scroll.offset().x,
                moved: false,
            });
        }
    }
    pub(super) fn move_drag(
        &mut self,
        x: Pixels,
        button: Option<MouseButton>,
        cx: &mut Context<Self>,
    ) -> bool {
        if button != Some(MouseButton::Left) {
            self.drag.borrow_mut().take();
            return false;
        }
        let mut active = self.drag.borrow_mut();
        let Some(drag) = active.as_mut() else {
            return false;
        };
        let delta = x - drag.start;
        if !drag.moved && delta.abs() <= px(3.) {
            return false;
        }
        drag.moved = true;
        let offset = (drag.offset + delta).clamp(-self.scroll.max_offset().x, px(0.));
        self.scroll.set_offset(gpui_kit::point(offset, px(0.)));
        cx.notify();
        true
    }
}
