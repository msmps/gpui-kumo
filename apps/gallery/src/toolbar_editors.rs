use gpui_kit::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, Subscription, Window,
};
use gpui_kumo::{
    Button, InputEvent, InputGroupAddon, InputState, Text, Toolbar, ToolbarItem, ToolbarState,
    theme, toolbar::Orientation,
};

pub struct ToolbarEditors {
    rows: Vec<Entity<ToolbarState>>,
    inputs: Vec<Vec<Entity<InputState>>>,
    short: bool,
    addon: bool,
    disabled: bool,
    changes: usize,
    clears: usize,
    submits: usize,
    _subscription: Subscription,
}
fn items(
    inputs: &[Entity<InputState>],
    short: bool,
    mode: usize,
    addon: bool,
    owner: gpui_kit::WeakEntity<ToolbarEditors>,
) -> Vec<ToolbarItem> {
    let mut group = ToolbarItem::input_group("filter-query", &inputs[3], gpui_kit::px(220.)).start(
        InputGroupAddon::parts([
            InputGroupAddon::icon("toolbar-settings.svg"),
            InputGroupAddon::text("Filter"),
        ]),
    );
    let target = inputs[3].downgrade();
    if addon {
        group = group.end(InputGroupAddon::button(
            "clear-filter",
            "Clear",
            move |button, _, _| {
                let target = target.clone();
                let owner = owner.clone();
                button.on_click(move |_, window, cx| {
                    let _ = target.update(cx, |state, cx| state.set_value("", window, cx));
                    let _ = owner.update(cx, |s, cx| {
                        s.clears += 1;
                        cx.notify();
                    });
                })
            },
        ));
    }
    if mode == 1 {
        group = group.suffix("units");
    }
    if mode == 2 {
        group = group.suffix("ms");
    }
    let mut items = vec![
        ToolbarItem::button("before-editor", "Before"),
        ToolbarItem::input("query", &inputs[0], gpui_kit::px(180.)),
        ToolbarItem::input("paused-query", &inputs[1], gpui_kit::px(130.)).disabled(true),
        ToolbarItem::input("skipped-query", &inputs[2], gpui_kit::px(130.))
            .disabled(true)
            .focusable_when_disabled(false),
        group,
        ToolbarItem::button("after-editor", "After"),
    ];
    if short {
        items = vec![
            ToolbarItem::button("before-editor", "Before"),
            ToolbarItem::button("after-editor", "After"),
        ];
    }
    items
}
impl ToolbarEditors {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let inputs: Vec<Vec<_>> = (0..3)
            .map(|_| {
                [
                    ("Query", "café 🦀"),
                    ("Paused query", "locked"),
                    ("Skipped query", "skip"),
                    ("Filter query", "status"),
                ]
                .into_iter()
                .map(|(name, value)| {
                    cx.new(|cx| {
                        let mut input = InputState::new(name, window, cx);
                        input.set_value(value, window, cx);
                        input
                    })
                })
                .collect()
            })
            .collect();
        let owner = cx.entity().downgrade();
        let rows = inputs
            .iter()
            .enumerate()
            .map(|(index, inputs)| {
                cx.new(|cx| {
                    ToolbarState::new(items(inputs, false, index, true, owner.clone()), cx)
                        .disabled(index == 2)
                })
            })
            .collect();
        let subscription =
            cx.subscribe(&inputs[0][0], |s: &mut Self, _, event: &InputEvent, cx| {
                match event {
                    InputEvent::Change => s.changes += 1,
                    InputEvent::Submit { .. } => s.submits += 1,
                    _ => {}
                }
                cx.notify();
            });
        Self {
            rows,
            inputs,
            short: false,
            addon: true,
            disabled: false,
            changes: 0,
            clears: 0,
            submits: 0,
            _subscription: subscription,
        }
    }
    pub fn toggle_items(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.short = !self.short;
        let owner = cx.entity().downgrade();
        self.rows[0].update(cx, |s, cx| {
            s.set_items(
                items(&self.inputs[0], self.short, 0, self.addon, owner),
                window,
                cx,
            )
        });
        cx.notify();
    }
    pub fn toggle_addon(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.addon = !self.addon;
        let owner = cx.entity().downgrade();
        self.rows[0].update(cx, |s, cx| {
            s.set_items(
                items(&self.inputs[0], self.short, 0, self.addon, owner),
                window,
                cx,
            )
        });
        cx.notify();
    }
    pub fn toggle_disabled(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.disabled = !self.disabled;
        self.rows[0].update(cx, |s, cx| s.set_disabled(self.disabled, window, cx));
        cx.notify();
    }
    pub fn toggle_group_disabled(&mut self, cx: &mut Context<Self>) {
        self.inputs[0][3].update(cx, |s, cx| s.set_disabled(!s.is_disabled(), cx));
        cx.notify();
    }
    pub fn toggle_readonly(&mut self, cx: &mut Context<Self>) {
        self.inputs[0][0].update(cx, |s, cx| s.set_read_only(!s.is_read_only(), cx));
        cx.notify();
    }
}
impl Render for ToolbarEditors {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let owner = cx.entity().downgrade();
        let disable = owner.clone();
        let readonly = owner.clone();
        let group_disabled = owner.clone();
        let addon = owner.clone();
        crate::panel(theme(cx), "Toolbar · retained editors")
            .child(Toolbar::new("editing", "Editing toolbar", &self.rows[0]))
            .child(
                Toolbar::new(
                    "vertical-editing",
                    "Vertical editing toolbar",
                    &self.rows[1],
                )
                .orientation(Orientation::Vertical),
            )
            .child(Toolbar::new(
                "disabled-editing",
                "Disabled editing toolbar",
                &self.rows[2],
            ))
            .child(
                Button::new("toggle-editor-items", "Toggle editor items").on_click(
                    move |_, w, cx| {
                        let _ = owner.update(cx, |s, cx| s.toggle_items(w, cx));
                    },
                ),
            )
            .child(
                Button::new("toggle-editor-disabled", "Toggle toolbar availability").on_click(
                    move |_, w, cx| {
                        let _ = disable.update(cx, |s, cx| s.toggle_disabled(w, cx));
                    },
                ),
            )
            .child(
                Button::new("toggle-editor-readonly", "Toggle read-only query").on_click(
                    move |_, _, cx| {
                        let _ = readonly.update(cx, |s, cx| s.toggle_readonly(cx));
                    },
                ),
            )
            .child(
                Button::new("toggle-group-disabled", "Toggle InputGroup availability").on_click(
                    move |_, _, cx| {
                        let _ = group_disabled.update(cx, |s, cx| s.toggle_group_disabled(cx));
                    },
                ),
            )
            .child(
                Button::new("toggle-addon", "Toggle filter action").on_click(
                    move |_, window, cx| {
                        let _ = addon.update(cx, |s, cx| s.toggle_addon(window, cx));
                    },
                ),
            )
            .child(Text::new(
                "clear-events",
                format!("{} filter clears", self.clears),
            ))
            .child(Text::new(
                "editor-events",
                format!(
                    "{} query changes · {} query submits",
                    self.changes, self.submits
                ),
            ))
    }
}
