//! Public API verification fixture: --dark, --width=520; F6 renames, F8 hides,
//! F9 changes theme, F10 toggles availability, F11 removes the focused action.
use gpui_kit::{
    App, AppContext, Bounds, Context, Entity, FocusHandle, InteractiveElement, IntoElement,
    KeyBinding, ParentElement, Render, Styled, Window, WindowBounds, WindowOptions, div, px, size,
};
use gpui_kumo::{
    Appearance, Button, Dialog, DialogState, DialogTrigger, Dropdown, DropdownItem, DropdownPart,
    DropdownState, Field, Input, InputArea, InputAreaState, InputGroup, InputGroupAddon,
    InputState, Popover, PopoverState, RadioGroup, RadioItem, Select, SelectOption, SelectState,
    SelectValue, SensitiveInput, SensitiveInputState, Text, Toolbar, ToolbarItem, ToolbarState,
};
gpui_kit::actions!(
    public_api_review,
    [Next, Previous, Rename, Hide, Theme, Availability, Remove]
);
struct Review {
    focus: FocusHandle,
    input: Entity<InputState>,
    field: Entity<InputState>,
    area: Entity<InputAreaState>,
    secret: Entity<SensitiveInputState>,
    select: Entity<SelectState<u32>>,
    dialog: Entity<DialogState>,
    popup: Entity<PopoverState>,
    toolbar: Entity<ToolbarState>,
    menu: Entity<DropdownState>,
    renamed: bool,
    hidden: bool,
    unavailable: bool,
    activations: usize,
    checks: usize,
    removed: bool,
}
impl Review {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let toolbar_input = cx.new(|cx| InputState::new("Toolbar query", window, cx));
        let toolbar = cx.new(|cx| {
            ToolbarState::new(
                vec![
                    ToolbarItem::button("refresh", "Refresh").build(),
                    ToolbarItem::link("docs", "Documentation", "/docs").build(),
                    ToolbarItem::input("toolbar-editor", &toolbar_input, px(180.)).build(),
                ],
                cx,
            )
        });
        Self {
            focus: cx.focus_handle(),
            input: cx.new(|cx| InputState::new("Project name", window, cx)),
            field: cx.new(|cx| InputState::new("Endpoint", window, cx)),
            area: cx.new(|cx| InputAreaState::new("Notes", window, cx)),
            secret: cx.new(|cx| SensitiveInputState::new("API key", "", window, cx)),
            select: cx.new(|cx| {
                SelectState::new(
                    "Region",
                    SelectValue::Single(Some(1)),
                    vec![
                        SelectOption::new("one", 1, "Europe"),
                        SelectOption::new("two", 2, "Asia"),
                    ],
                    cx,
                )
            }),
            dialog: cx.new(|cx| DialogState::new("Edit project", cx)),
            popup: cx.new(|cx| PopoverState::new("Project settings", cx)),
            toolbar,
            menu: cx.new(|cx| {
                DropdownState::new(
                    "Project actions",
                    vec![DropdownPart::Item(
                        DropdownItem::new("archive", "Old")
                            .accessibility_label("Archive current project")
                            .label("Archive"),
                    )],
                    cx,
                )
            }),
            renamed: false,
            hidden: false,
            unavailable: false,
            activations: 0,
            checks: 0,
            removed: false,
        }
    }
    fn rename(&mut self, cx: &mut Context<Self>) {
        self.renamed = !self.renamed;
        let names = if self.renamed {
            [
                "Nom du projet",
                "Point de terminaison",
                "Remarques",
                "Clé API",
                "Région",
                "Modifier le projet",
                "Paramètres du projet",
                "Actions du projet",
            ]
        } else {
            [
                "Project name",
                "Endpoint",
                "Notes",
                "API key",
                "Region",
                "Edit project",
                "Project settings",
                "Project actions",
            ]
        };
        self.input.update(cx, |s, cx| s.set_name(names[0], cx));
        self.field.update(cx, |s, cx| s.set_name(names[1], cx));
        self.area.update(cx, |s, cx| s.set_name(names[2], cx));
        self.secret.update(cx, |s, cx| s.set_name(names[3], cx));
        self.select.update(cx, |s, cx| s.set_name(names[4], cx));
        self.dialog.update(cx, |s, cx| s.set_name(names[5], cx));
        self.popup.update(cx, |s, cx| s.set_name(names[6], cx));
        self.menu.update(cx, |s, cx| s.set_name(names[7], cx));
        cx.notify();
    }
}
impl Render for Review {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let title = self.dialog.read(cx).name().clone();
        let dialog_input = self.input.clone();
        let owner = cx.entity().downgrade();
        let t = gpui_kumo::theme(cx).clone();
        let field_control =
            InputGroup::new("endpoint", &self.field).start(InputGroupAddon::text("/api/"));
        let field_control = if self.removed {
            field_control
        } else {
            field_control.button(
                "check",
                "Check endpoint",
                gpui_kumo::button::Variant::Ghost,
                {
                    let owner = owner.clone();
                    move |button, _, _| {
                        let owner = owner.clone();
                        button.on_click(move |_, _, cx| {
                            let _ = owner.update(cx, |s, cx| {
                                s.checks += 1;
                                cx.notify();
                            });
                        })
                    }
                },
            )
        };
        div()
            .id("api-review")
            .track_focus(&self.focus)
            .tab_group()
            .on_action(|_: &Next, window, cx| window.focus_next(cx))
            .on_action(|_: &Previous, window, cx| window.focus_prev(cx))
            .on_action(cx.listener(|s, _: &Rename, _, cx| s.rename(cx)))
            .on_action(cx.listener(|s, _: &Remove, _, cx| {
                s.removed = !s.removed;
                cx.notify();
            }))
            .on_action(cx.listener(|s, _: &Hide, _, cx| {
                s.hidden = !s.hidden;
                cx.notify();
            }))
            .on_action(cx.listener(|_, _: &Theme, _, cx| {
                let next = gpui_kumo::theme(cx).appearance.opposite();
                gpui_kumo::set_appearance(next, cx);
                cx.notify();
            }))
            .on_action(cx.listener(|s, _: &Availability, window, cx| {
                s.unavailable = !s.unavailable;
                let disabled = s.unavailable;
                s.input.update(cx, |s, cx| s.set_disabled(disabled, cx));
                s.field.update(cx, |s, cx| s.set_disabled(disabled, cx));
                s.area.update(cx, |s, cx| s.set_disabled(disabled, cx));
                s.secret.update(cx, |s, cx| s.set_disabled(disabled, cx));
                s.select
                    .update(cx, |s, cx| s.set_disabled(disabled, window, cx));
                s.toolbar
                    .update(cx, |s, cx| s.set_disabled(disabled, window, cx));
                cx.notify();
            }))
            .size_full()
            .flex()
            .flex_col()
            .gap(px(12.))
            .p(px(24.))
            .bg(t.colors.canvas)
            .text_color(t.text.default)
            .font_family(t.typography.font_family)
            .text_size(t.typography.base.size)
            .line_height(t.typography.base.line_height)
            .child(Text::new(
                "instructions",
                "F6 rename · F8 labels · F9 theme · F10 availability",
            ))
            .child(Text::new(
                "count",
                format!("Activations: {}", self.activations),
            ))
            .child(Text::new("checks", format!("Checks: {}", self.checks)))
            .child(
                div()
                    .flex()
                    .gap(px(8.))
                    .flex_wrap()
                    .child(
                        Button::new("save", "Save")
                            .disabled(self.unavailable)
                            .on_click(cx.listener(|s, _, _, cx| {
                                s.activations += 1;
                                cx.notify();
                            })),
                    )
                    .child(
                        Button::new("delete", "Delete")
                            .accessibility_label("Delete project Foo")
                            .label("Delete")
                            .loading(self.unavailable)
                            .on_click(cx.listener(|s, _, _, cx| {
                                s.activations += 1;
                                cx.notify();
                            })),
                    )
                    .child(Button::icon(
                        "settings",
                        "Open settings",
                        gpui_kit::svg()
                            .data(include_bytes!("../assets/toolbar-settings.svg"))
                            .size(px(16.))
                            .text_color(t.text.default),
                    ))
                    .child(DialogTrigger::new(
                        "dialog-trigger",
                        &self.dialog,
                        Button::new("edit", "Open dialog"),
                    ))
                    .child(Dropdown::new("menu", &self.menu, "Actions"))
                    .child(Popover::new("popup", &self.popup, "Open popover").content(
                        |close, _, _| {
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(8.))
                                .child(Text::new("popup-text", "Retained popup content"))
                                .child(
                                    Button::new("popup-close", "Close popover")
                                        .on_click(move |_, window, cx| close.dismiss(window, cx)),
                                )
                        },
                    )),
            )
            .child(
                Input::new("project", &self.input)
                    .show_label(!self.hidden)
                    .required(false),
            )
            .child(
                Field::control("endpoint-field", field_control, cx)
                    .show_label(!self.hidden)
                    .disabled(self.unavailable),
            )
            .child(
                InputArea::new("notes", &self.area)
                    .show_label(!self.hidden)
                    .rows(2),
            )
            .child(SensitiveInput::new("key", &self.secret).show_label(!self.hidden))
            .child(Select::new("region", &self.select).show_label(!self.hidden))
            .child(Toolbar::new("toolbar", "Project tools", &self.toolbar))
            .child(
                RadioGroup::new("radios", "Old", Some(1usize))
                    .accessibility_label("Layout preference")
                    .label("Layout")
                    .show_label(false)
                    .item(
                        RadioItem::new("compact", 1, "Old")
                            .accessibility_label("Compact layout")
                            .label("Compact"),
                    ),
            )
            .child(Dialog::new("dialog", &self.dialog, move |close, _, cx| {
                div()
                    .p(px(24.))
                    .flex()
                    .flex_col()
                    .gap(px(12.))
                    .child(
                        Text::new("title", title.clone())
                            .heading_level(gpui_kumo::text::HeadingLevel::Two),
                    )
                    // A distinct field entity avoids mounting the same editor twice while the dialog is open.
                    .child(Text::new("draft", dialog_input.read(cx).name().clone()))
                    .child(close.button(Button::new("cancel", "Cancel")))
                    .child(Button::new("dialog-rename", "Rename record").on_click({
                        let owner = owner.clone();
                        move |_, _, cx| {
                            let _ = owner.update(cx, |s, cx| s.rename(cx));
                        }
                    }))
                    .into_any_element()
            }))
    }
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let width = args
        .iter()
        .find_map(|arg| {
            arg.strip_prefix("--width=")
                .and_then(|v| v.parse::<f32>().ok())
        })
        .unwrap_or(1040.);
    let dark = args.iter().any(|arg| arg == "--dark");
    gpui_kit::application().run(move |cx: &mut App| {
        gpui_kumo::init(cx);
        gpui_kumo::set_appearance(
            if dark {
                Appearance::Dark
            } else {
                Appearance::Light
            },
            cx,
        );
        cx.bind_keys([
            KeyBinding::new("tab", Next, None),
            KeyBinding::new("shift-tab", Previous, None),
            KeyBinding::new("f6", Rename, None),
            KeyBinding::new("f8", Hide, None),
            KeyBinding::new("f9", Theme, None),
            KeyBinding::new("f10", Availability, None),
            KeyBinding::new("f11", Remove, None),
        ]);
        cx.on_window_closed(|cx, _| cx.quit()).detach();
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(width), px(850.)),
                    cx,
                ))),
                ..Default::default()
            },
            |window, cx| cx.new(|cx| Review::new(window, cx)),
        )
        .expect("open public API fixture");
        cx.activate(true);
    });
}
