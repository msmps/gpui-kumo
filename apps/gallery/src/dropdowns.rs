use gpui_kit::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, Subscription, Window,
};
use gpui_kumo::{
    Dropdown, DropdownEvent, DropdownItem, DropdownPart, DropdownState, DropdownVariant, Text,
    theme,
};

pub struct Dropdowns {
    menu: Entity<DropdownState>,
    count: usize,
    last: String,
    _events: Subscription,
}
impl Dropdowns {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let menu = cx.new(|cx| {
            DropdownState::new(
                "Save actions",
                vec![
                    DropdownPart::Label("Save document".into()),
                    DropdownItem::new("save", "Save").shortcut("⌘S").into(),
                    DropdownItem::new("draft", "Save as draft").into(),
                    DropdownItem::new("paused", "Publish (unavailable)")
                        .disabled(true)
                        .into(),
                    DropdownPart::Separator,
                    DropdownItem::new("duplicate", "Duplicate").into(),
                    DropdownItem::new("delete", "Delete")
                        .variant(DropdownVariant::Danger)
                        .into(),
                ],
                cx,
            )
        });
        let events = cx.subscribe(&menu, |state: &mut Self, _, event: &DropdownEvent, cx| {
            if let DropdownEvent::Activated(id) = event {
                state.count += 1;
                state.last = id.to_string();
            }
            cx.notify();
        });
        Self {
            menu,
            count: 0,
            last: "none".into(),
            _events: events,
        }
    }
}
impl Render for Dropdowns {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        crate::panel(theme(cx), "Dropdown · document actions")
            .child(Dropdown::new("save-menu", &self.menu, "Save options"))
            .child(Text::new(
                "action-result",
                format!("{} actions · last: {}", self.count, self.last),
            ))
    }
}
