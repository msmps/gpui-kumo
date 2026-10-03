use gpui_kit::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, Subscription, Window, div,
};
use gpui_kumo::{
    Button, Text, Toolbar, ToolbarEvent, ToolbarItem, ToolbarState, theme, toolbar::Orientation,
};
pub struct Toolbars {
    states: Vec<Entity<ToolbarState>>,
    changes: usize,
    routes: usize,
    short: bool,
    _subscription: Subscription,
}
fn items(short: bool) -> Vec<ToolbarItem> {
    let mut items = vec![
        ToolbarItem::button("refresh", "Refresh"),
        ToolbarItem::button("paused", "Paused").disabled(true),
        ToolbarItem::button("skipped", "Skipped")
            .disabled(true)
            .focusable_when_disabled(false),
        ToolbarItem::button("saving", "Saving").loading(true),
        ToolbarItem::link("docs", "Documentation", "/docs"),
        ToolbarItem::button("settings", "Settings").icon("toolbar-settings.svg", true),
        ToolbarItem::button("deploy", "Deploy"),
    ];
    if short {
        items = vec![
            ToolbarItem::button("refresh", "Refresh"),
            ToolbarItem::link("docs", "Documentation", "/docs"),
        ];
    }
    items
}
impl Toolbars {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let states: Vec<_> = (0..4)
            .map(|i| cx.new(|cx| ToolbarState::new(items(false), cx).disabled(i == 3)))
            .collect();
        let subscription = cx.subscribe(&states[0], |s: &mut Self, _, event: &ToolbarEvent, cx| {
            match event {
                ToolbarEvent::Activate { .. } => s.changes += 1,
                ToolbarEvent::Navigate { .. } => s.routes += 1,
            };
            cx.notify();
        });
        Self {
            states,
            changes: 0,
            routes: 0,
            short: false,
            _subscription: subscription,
        }
    }
    pub fn toggle(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.short = !self.short;
        self.states[0].update(cx, |s, cx| s.set_items(items(self.short), window, cx));
        cx.notify();
    }
}
impl Render for Toolbars {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let owner = cx.entity().downgrade();
        crate::panel(theme(cx),"Toolbar · joined actions and retained focus")
            .child(Toolbar::new("actions","Actions toolbar",&self.states[0]))
            .child(Toolbar::new("vertical","Vertical navigation toolbar",&self.states[1]).orientation(Orientation::Vertical))
            .child(Toolbar::new("no-loop","Nonlooping toolbar",&self.states[2]).loop_focus(false))
            .child(Toolbar::new("disabled","Disabled actions toolbar",&self.states[3]))
            .child(Button::new("toggle-toolbar","Toggle toolbar items").on_click(move|_,w,cx|{let _=owner.update(cx,|s,cx|s.toggle(w,cx));}))
            .child(Text::new("toolbar-status",format!("{} activations · {} navigation requests",self.changes,self.routes)))
            .child(div().child("Arrows move focus; disabled actions remain focusable. Input/popup composition follows this slice."))
    }
}
