use gpui_kit::{
    AppContext, Context, Entity, Focusable, FontWeight, Subscription, Window, div, prelude::*, px,
};
use gpui_kumo::button::Variant;
use gpui_kumo::{
    Button, Dialog, DialogCloseReason, DialogEvent, DialogRole, DialogSize, DialogState,
    DialogTrigger, Dropdown, DropdownEvent, DropdownItem, DropdownState, DropdownVariant, Input,
    InputState, Text, theme,
};

pub struct Dialogs {
    menu: Entity<DropdownState>,
    edit: Entity<DialogState>,
    delete: Entity<DialogState>,
    draft: Entity<InputState>,
    result: String,
    count: usize,
    _events: Vec<Subscription>,
}
impl Dialogs {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let edit = cx.new(|cx| DialogState::new("Edit document", cx));
        let delete = cx.new(|cx| DialogState::new("Delete document?", cx));
        delete.update(cx, |state, cx| state.set_role(DialogRole::AlertDialog, cx));
        let draft = cx.new(|cx| {
            let mut state = InputState::new("Document name", window, cx);
            state.set_value("Launch announcement", window, cx);
            state
        });
        let input = draft.downgrade();
        edit.update(cx, |state, cx| {
            state.set_close_guard(
                move |reason, _, cx| {
                    reason != DialogCloseReason::Action
                        || input
                            .upgrade()
                            .is_some_and(|draft| !draft.read(cx).value(cx).trim().is_empty())
                },
                cx,
            )
        });
        let menu = cx.new(|cx| {
            DropdownState::new(
                "Document actions",
                vec![
                    DropdownItem::new("edit", "Edit document").into(),
                    DropdownItem::new("delete", "Delete document")
                        .variant(DropdownVariant::Danger)
                        .into(),
                ],
                cx,
            )
        });
        let edit_target = edit.downgrade();
        let delete_target = delete.downgrade();
        let handle = window.window_handle();
        let menu_events = cx.subscribe(&menu, move |_: &mut Self, _, event: &DropdownEvent, cx| {
            if let DropdownEvent::Activated(id) = event {
                let target = if id.as_ref() == "edit" {
                    edit_target.clone()
                } else {
                    delete_target.clone()
                };
                // Dropdown has closed and restored its surviving trigger before this event.
                let _ = handle.update(cx, |_, window, cx| {
                    let _ = target.update(cx, |state, cx| state.set_open(true, window, cx));
                });
            }
        });
        let edit_events = cx.subscribe(&edit, |state: &mut Self, _, event: &DialogEvent, cx| {
            match event {
                DialogEvent::Closed(DialogCloseReason::Action) => {
                    state.count += 1;
                    state.result = format!("Saved {}", state.draft.read(cx).value(cx));
                }
                DialogEvent::ClosePrevented(DialogCloseReason::Action) => {
                    state.result = "Enter a document name before saving".into()
                }
                _ => {}
            }
            cx.notify();
        });
        let delete_events =
            cx.subscribe(&delete, |state: &mut Self, _, event: &DialogEvent, cx| {
                if *event == DialogEvent::Closed(DialogCloseReason::Action) {
                    state.count += 1;
                    state.result = "Document deleted".into();
                }
                cx.notify();
            });
        Self {
            menu,
            edit,
            delete,
            draft,
            result: "No changes yet".into(),
            count: 0,
            _events: vec![menu_events, edit_events, delete_events],
        }
    }
}
impl Render for Dialogs {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let draft = self.draft.clone();
        let focus = draft.read(cx).focus_handle(cx);
        crate::panel(theme(cx), "Dialog · document workflow")
            .child(Dropdown::new("document-menu", &self.menu, "Document options"))
            .child(DialogTrigger::new("edit-trigger", &self.edit, Button::new("open-editor", "Edit document")))
            .child(DialogTrigger::new("delete-trigger", &self.delete, Button::new("open-delete", "Delete document").variant(Variant::Destructive)))
            .child(Text::new("result", format!("{} operations · {}", self.count, self.result)))
            .child(Dialog::new("editor", &self.edit, move |close, _, cx| {
                let save = close.clone(); let _ = cx;
                div().p(px(32.)).flex().flex_col().gap(px(16.))
                    .child(div().text_size(px(24.)).line_height(px(32.)).font_weight(FontWeight::SEMIBOLD).child("Edit document"))
                    .child(Text::new("description", "Update the name, then save your changes.").style(gpui_kumo::text::Style::Copy { tone: gpui_kumo::text::Tone::Secondary, size: gpui_kumo::text::Size::Base, bold: false }))
                    .child(Input::new("document-name", &draft))
                    .child(div().flex().justify_end().gap(px(8.))
                        .child(close.button(Button::new("cancel-edit", "Cancel")))
                        .child(Button::new("save-document", "Save changes").variant(Variant::Primary).on_click(move |_, window, cx| { save.request(DialogCloseReason::Action, window, cx); })))
                    .into_any_element()
            }).size(DialogSize::Large).description("Update the name, then save your changes.").initial_focus(&focus))
            .child(Dialog::new("delete-dialog", &self.delete, |close, _, cx| {
                let confirm = close.clone(); let _ = cx;
                div().p(px(32.)).flex().flex_col().gap(px(16.))
                    .child(div().text_size(px(24.)).line_height(px(32.)).font_weight(FontWeight::SEMIBOLD).child("Delete document?"))
                    .child(Text::new("delete-description", "This demo records the operation. Outside clicks keep this confirmation open.").style(gpui_kumo::text::Style::Copy { tone: gpui_kumo::text::Tone::Secondary, size: gpui_kumo::text::Size::Base, bold: false }))
                    .child(div().flex().justify_end().gap(px(8.))
                        .child(close.button(Button::new("cancel-delete", "Cancel")))
                        .child(Button::new("confirm-delete", "Delete").variant(Variant::Destructive).on_click(move |_, window, cx| { confirm.request(DialogCloseReason::Action, window, cx); })))
                    .into_any_element()
            }).size(DialogSize::Large).description("Confirm deletion of this document"))
    }
}
