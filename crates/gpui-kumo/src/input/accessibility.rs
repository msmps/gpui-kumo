//! The missing AccessKit integration over Base's retained single-line editor.

use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use gpui_kit::{A11ySubtreeBuilder, Context, EntityInputHandler, SharedString, Window, accesskit};

use super::InputState;

#[derive(Clone, Default)]
pub(super) struct Bridge {
    snapshot: Rc<RefCell<Option<Snapshot>>>,
    pub run_id: Rc<Cell<Option<accesskit::NodeId>>>,
}

impl Bridge {
    pub fn capture(&self, editor: &gpui_kit::base::input::InputState) -> bool {
        let next = Snapshot::new(editor);
        let mut snapshot = self.snapshot.borrow_mut();
        if next.is_some() && *snapshot != next {
            *snapshot = next;
            true
        } else {
            false
        }
    }

    pub fn build(&self, builder: &mut A11ySubtreeBuilder) {
        let snapshot = self.snapshot.borrow().clone().unwrap_or_else(|| Snapshot {
            text: SharedString::default(),
            bounds: builder.parent_node().bounds().unwrap_or_default(),
            positions: Vec::new(),
            widths: Vec::new(),
            anchor: 0,
            focus: 0,
        });
        let id = builder.synthetic_node_id("input-text");
        self.run_id.set(Some(id));
        let (run, selection) = snapshot.nodes(id);
        builder.push_child(id, run);
        builder.parent_node().set_text_selection(selection);
    }
}

#[derive(Clone, PartialEq)]
pub(super) struct Snapshot {
    text: SharedString,
    bounds: accesskit::Rect,
    positions: Vec<f32>,
    widths: Vec<f32>,
    anchor: usize,
    focus: usize,
}

impl Snapshot {
    pub fn new(editor: &gpui_kit::base::input::InputState) -> Option<Self> {
        let text = editor.value();
        let bounds = editor.range_to_bounds(&(0..text.len()))?;
        let mut positions = Vec::new();
        let mut widths = Vec::new();
        for (start, ch) in text.char_indices() {
            let character = editor.range_to_bounds(&(start..start + ch.len_utf8()))?;
            positions.push((character.origin.x - bounds.origin.x).into());
            widths.push(f32::from(character.size.width).max(0.));
        }
        let selected = editor.selected_range();
        let focus = editor.cursor();
        let anchor = if focus == selected.start {
            selected.end
        } else {
            selected.start
        };
        Some(Self {
            anchor: text[..anchor].chars().count(),
            focus: text[..focus].chars().count(),
            text,
            bounds: accesskit::Rect::new(
                f32::from(bounds.origin.x) as f64,
                f32::from(bounds.origin.y) as f64,
                f32::from(bounds.right()) as f64,
                f32::from(bounds.bottom()) as f64,
            ),
            positions,
            widths,
        })
    }

    pub fn nodes(self, id: accesskit::NodeId) -> (accesskit::Node, accesskit::TextSelection) {
        let mut run = accesskit::Node::new(accesskit::Role::TextRun);
        run.set_value(self.text.to_string());
        run.set_character_lengths(
            self.text
                .chars()
                .map(|ch| ch.len_utf8() as u8)
                .collect::<Vec<_>>(),
        );
        run.set_bounds(self.bounds);
        run.set_text_direction(accesskit::TextDirection::LeftToRight);
        run.set_character_positions(self.positions);
        run.set_character_widths(self.widths);
        (
            run,
            accesskit::TextSelection {
                anchor: accesskit::TextPosition {
                    node: id,
                    character_index: self.anchor,
                },
                focus: accesskit::TextPosition {
                    node: id,
                    character_index: self.focus,
                },
            },
        )
    }
}

impl InputState {
    pub(super) fn accessibility_action(
        &mut self,
        run_id: Option<accesskit::NodeId>,
        data: Option<&accesskit::ActionData>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.disabled {
            return;
        }
        match data {
            Some(accesskit::ActionData::SetTextSelection(selection)) => {
                if Some(selection.anchor.node) != run_id || Some(selection.focus.node) != run_id {
                    return;
                }
                let value = self.value(cx);
                let offset = |index| {
                    value
                        .char_indices()
                        .map(|(byte, _)| byte)
                        .chain(std::iter::once(value.len()))
                        .nth(index)
                };
                let (Some(anchor), Some(focus)) = (
                    offset(selection.anchor.character_index),
                    offset(selection.focus.character_index),
                ) else {
                    return;
                };
                self.editor.update(cx, |editor, cx| {
                    // Base's public setter accepts an ordered byte range and
                    // owns boundary clipping. It cannot retain a reversed anchor.
                    editor.set_selected_range(anchor.min(focus)..anchor.max(focus), cx);
                });
            }
            Some(accesskit::ActionData::Value(value)) if run_id.is_none() && !self.read_only => {
                let length = self.value(cx).encode_utf16().count();
                self.editor.update(cx, |editor, cx| {
                    editor.replace_text_in_range(Some(0..length), value, window, cx);
                });
            }
            _ => {}
        }
    }

    pub(super) fn accessibility_replace_selection(
        &mut self,
        data: Option<&accesskit::ActionData>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.disabled || self.read_only {
            return;
        }
        if let Some(accesskit::ActionData::Value(value)) = data {
            self.editor.update(cx, |editor, cx| {
                editor.replace_text_in_range(None, value, window, cx);
            });
        }
    }
}
