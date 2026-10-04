use gpui_kit::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Subscription, Window,
    div, px,
};
use gpui_kumo::{
    Button, InputArea, InputAreaEvent, InputAreaState, TooltipState, input::Size, theme,
};

pub struct InputAreas {
    entries: Vec<Entity<InputAreaState>>,
    help: Entity<TooltipState>,
    changes: usize,
    auto: bool,
    _events: Subscription,
}
impl InputAreas {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let entries: Vec<_> = ["Extra small", "Small", "Notes", "Large", "Read only", "Disabled", "Invalid notes"]
            .into_iter().enumerate().map(|(i, name)| cx.new(|cx| {
                let mut state = InputAreaState::new(name, window, cx);
                state.set_placeholder("Write notes…", window, cx);
                if i != 1 { state.set_value("café 🦀 — retained multiline text.\nA second line with long content that wraps when the window becomes narrow.", window, cx); }
                if i == 4 { state.set_read_only(true, cx); }
                if i == 5 { state.set_disabled(true, cx); }
                state
            })).collect();
        let events = cx.subscribe(&entries[2], |this: &mut Self, _, event, cx| {
            if matches!(event, InputAreaEvent::Change) {
                this.changes += 1;
                cx.notify();
            }
        });
        Self {
            entries,
            help: cx.new(|cx| TooltipState::new(window, cx)),
            changes: 0,
            auto: true,
            _events: events,
        }
    }
}
impl Render for InputAreas {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = theme(cx);
        let sizes = [Size::Xs, Size::Sm, Size::Base, Size::Lg];
        let state = self.entries[2].downgrade();
        let clear = state.clone();
        let owner = cx.entity().downgrade();
        crate::panel(theme, "InputArea · multiline editing and wrapped row sizing")
            .child(gpui_kumo::TooltipProvider::new("area-help", [self.help.downgrade()],
                div().flex().flex_col().gap(px(24.)).children(self.entries.iter().enumerate().map(|(i, state)| {
                    let area = InputArea::new(("area", i), state).show_label(true).size(sizes.get(i).copied().unwrap_or(Size::Base));
                    if i == 2 {
                        let area = area.rows(4).optional_indicator(true).label_tooltip(&self.help, "Plain Enter adds a line; Tab leaves the field.").description("Grows from 2 to 4 wrapped rows. Extra content scrolls; switch to fixed 4 rows below.");
                        if self.auto { area.auto_resize(2, Some(4)) } else { area }
                    } else if i == 6 { area.error_visible("Add more detail", true) } else { area }
                }))))
            .child(format!("Notes changes {} · {} rows", self.changes, if self.auto { "automatic 2–4" } else { "fixed 4" }))
            .child(div().flex().flex_wrap().gap(theme.spacing.eight)
                .child(Button::new("area-grow", "Replace with long notes").on_click(move |_, window, cx| { let _ = state.update(cx, |s, cx| s.set_value("Owner-provided café 🦀 notes.\nSecond line\nThird line\nFourth line\nFifth line\nSixth line", window, cx)); }))
                .child(Button::new("area-clear", "Clear notes").on_click(move |_, window, cx| { let _ = clear.update(cx, |s, cx| s.set_value("", window, cx)); }))
                .child(Button::new("area-policy", "Toggle row policy").on_click(move |_, _, cx| { let _ = owner.update(cx, |v, cx| { v.auto = !v.auto; cx.notify(); }); })))
    }
}
