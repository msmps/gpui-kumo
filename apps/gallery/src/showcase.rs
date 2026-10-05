//! A Kumo-inspired native component grid. Run with --light, --width=980 or --reduce-motion.
use gpui_kit::{
    App, AppContext, Bounds, Context, Div, Entity, FocusHandle, Focusable, FontWeight,
    InteractiveElement, IntoElement, KeyBinding, ParentElement, Render, Styled, Subscription,
    TitlebarOptions, Window, WindowBounds, WindowOptions, div, px, relative, size,
};
use gpui_kumo::*;

gpui_kit::actions!(showcase, [FocusNext, FocusPrevious, Quit]);
const PAGES: [&str; 3] = ["Controls", "Surfaces", "Details"];
struct Showcase {
    focus: FocusHandle,
    navigation: Entity<TabsState<usize>>,
    name: Entity<InputState>,
    invalid: Entity<InputState>,
    endpoint: Entity<InputState>,
    notes: Entity<InputAreaState>,
    secret: Entity<SensitiveInputState>,
    region: Entity<SelectState<usize>>,
    tip: Entity<TooltipState>,
    pagination: Entity<PaginationState>,
    menu: Entity<DropdownState>,
    popover: Entity<PopoverState>,
    dialog: Entity<DialogState>,
    toolbar: Entity<ToolbarState>,
    toasts: Entity<ToastState>,
    checked: checkbox::State,
    notifications: bool,
    plan: usize,
    selected: [bool; 3],
    details: bool,
    status: String,
    _subscriptions: Vec<Subscription>,
}
fn stack() -> Div {
    div().flex().flex_col().gap(px(16.))
}
fn row() -> Div {
    div().flex().items_center().gap(px(12.))
}
// A plain display cell, rather than a card around every component.
fn cell(
    id: &'static str,
    title: &'static str,
    content: impl IntoElement,
    cx: &App,
) -> gpui_kit::Stateful<Div> {
    let t = theme(cx);
    div()
        .id(id)
        .flex_1()
        .min_w_0()
        .min_h_0()
        .flex()
        .flex_col()
        .border_r_1()
        .border_b_1()
        .border_color(t.colors.hairline)
        .p(px(16.))
        .child(
            div()
                .text_size(px(12.))
                .line_height(px(16.))
                .text_color(t.text.subtle)
                .child(title),
        )
        .child(
            div()
                .flex_1()
                .min_h_0()
                .flex()
                .items_center()
                .justify_center()
                .px(px(8.))
                .py(px(8.))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .w_full()
                        .items_center()
                        .min_w_0()
                        .max_w_full()
                        .child(content),
                ),
        )
}
fn grid_row() -> Div {
    row().gap(px(0.)).flex_1().min_h_0().items_stretch()
}
fn bounded(content: impl IntoElement) -> Div {
    div().w(px(240.)).max_w_full().child(content)
}
impl Showcase {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let navigation = cx.new(|cx| {
            TabsState::new(
                PAGES
                    .into_iter()
                    .enumerate()
                    .map(|(i, name)| TabItem::new(name, i, name))
                    .collect(),
                Some(0),
                cx,
            )
        });
        let pagination =
            cx.new(|cx| PaginationState::new(1, 2, PaginationTotal::Known(3), window, cx));
        let name = cx.new(|cx| {
            let mut state = InputState::new("Text input", window, cx);
            state.set_placeholder("Type something...", window, cx);
            state
        });
        let invalid = cx.new(|cx| {
            let mut state = InputState::new("Invalid input", window, cx);
            state.set_value("Invalid!", window, cx);
            state
        });
        let endpoint = cx.new(|cx| {
            let mut state = InputState::new("Worker subdomain", window, cx);
            state.set_value("kumo", window, cx);
            state
        });
        let notes = cx.new(|cx| {
            let mut state = InputAreaState::new("Name", window, cx);
            state.set_placeholder("Enter your name", window, cx);
            state
        });
        let region = cx.new(|cx| {
            SelectState::new(
                "Select version",
                SelectValue::Single(None),
                [
                    "All deployed versions",
                    "Active versions",
                    "Specific versions",
                ]
                .into_iter()
                .enumerate()
                .map(|(i, label)| SelectOption::new(label, i, label))
                .collect(),
                cx,
            )
        });
        let mut subscriptions = vec![cx.observe_global::<Theme>(|_, cx| cx.notify())];
        subscriptions.push(cx.subscribe_in(
            &navigation,
            window,
            |this: &mut Self, _, _, window, cx| {
                // Overlays cannot survive navigation away from their trigger.
                this.menu
                    .update(cx, |state, cx| state.set_open(false, window, cx));
                this.popover
                    .update(cx, |state, cx| state.set_open(false, window, cx));
                this.dialog
                    .update(cx, |state, cx| state.set_open(false, window, cx));
                this.region
                    .update(cx, |state, cx| state.set_open(false, window, cx));
                this.tip.update(cx, |state, cx| state.set_open(false, cx));
                cx.notify();
            },
        ));
        subscriptions.push(cx.subscribe_in(
            &pagination,
            window,
            |_: &mut Self, state, event, window, cx| {
                match *event {
                    PaginationEvent::Page(page) => {
                        state.update(cx, |state, cx| state.set_page(page, window, cx))
                    }
                    PaginationEvent::PageSize(per_page) => {
                        state.update(cx, |state, cx| state.set_per_page(per_page, window, cx))
                    }
                    _ => {}
                }
                cx.notify();
            },
        ));
        let menu = cx.new(|cx| {
            DropdownState::new(
                "Add",
                vec![
                    DropdownItem::new("worker", "Worker").into(),
                    DropdownItem::new("pages", "Pages").into(),
                ],
                cx,
            )
        });
        subscriptions.push(cx.subscribe(&menu, |this: &mut Self, _, event, cx| {
            if let DropdownEvent::Activated(id) = event {
                this.status = format!("{} selected", id);
                cx.notify();
            }
        }));
        let search = cx.new(|cx| {
            let mut state = InputState::new("Search DNS records", window, cx);
            state.set_placeholder("Search...", window, cx);
            state
        });
        let toolbar = cx.new(|cx| {
            ToolbarState::new(
                vec![
                    ToolbarItem::input("search", &search, px(190.)).build(),
                    ToolbarItem::button("search-action", "Search")
                        .icon("search.svg")
                        .show_label(false)
                        .build(),
                    ToolbarItem::button("add", "Add")
                        .icon("plus.svg")
                        .show_label(false)
                        .build(),
                ],
                cx,
            )
        });
        subscriptions.push(cx.subscribe(&toolbar, |this: &mut Self, _, event, cx| {
            if let ToolbarEvent::Activate { id, .. } = event {
                this.status = format!("{} requested", id);
                cx.notify();
            }
        }));
        let focus = cx.focus_handle();
        focus.focus(window, cx);
        Self {
            focus,
            navigation,
            pagination,
            name,
            invalid,
            endpoint,
            notes,
            region,
            menu,
            toolbar,
            secret: cx
                .new(|cx| SensitiveInputState::new("Secret", "super-secret-api-key", window, cx)),
            tip: cx.new(|cx| TooltipState::new(window, cx)),
            popover: cx.new(|cx| PopoverState::new("Popover Title", cx)),
            dialog: cx.new(|cx| DialogState::new("Delete Resource?", cx)),
            toasts: cx.new(ToastState::new),
            checked: checkbox::State::Checked,
            notifications: true,
            plan: 0,
            selected: [false; 3],
            details: false,
            status: "Native components for Rust".into(),
            _subscriptions: subscriptions,
        }
    }
    fn page(&self, cx: &App) -> usize {
        self.navigation.read(cx).selected().copied().unwrap_or(0)
    }
    #[cfg(test)]
    fn navigate(&mut self, page: usize, window: &mut Window, cx: &mut Context<Self>) {
        self.menu.update(cx, |s, cx| s.set_open(false, window, cx));
        self.popover
            .update(cx, |s, cx| s.set_open(false, window, cx));
        self.region
            .update(cx, |s, cx| s.set_open(false, window, cx));
        self.tip.update(cx, |s, cx| s.set_open(false, cx));
        self.navigation
            .update(cx, |s, cx| s.set_selected(Some(page), cx));
        self.focus.focus(window, cx);
        cx.notify();
    }
    fn controls(&self, cx: &mut Context<Self>) -> Div {
        let switch = cx.entity().downgrade();
        let checkbox = switch.clone();
        let radio = switch.clone();
        stack()
            .gap(px(0.))
            .size_full()
            .child(
                grid_row()
                    .child(cell(
                        "buttons",
                        "Button",
                        stack()
                            .gap(px(8.))
                            .child(
                                Button::new("create", "Create Worker")
                                    .leading_icon(Icon::new("plus.svg").size(px(14.)))
                                    .size(button::Size::Sm)
                                    .w(px(124.))
                                    .justify_start()
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.status = "Worker created".into();
                                        cx.notify();
                                    })),
                            )
                            .child(
                                Button::new("create-primary", "Create Worker")
                                    .leading_icon(Icon::new("plus.svg").size(px(14.)))
                                    .size(button::Size::Sm)
                                    .w(px(124.))
                                    .justify_start()
                                    .variant(button::Variant::Primary)
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.toasts.update(cx, |state, cx| {
                                            state.add(
                                                Toast::new("created", "Worker created")
                                                    .variant(ToastVariant::Success),
                                                window,
                                                cx,
                                            );
                                        });
                                    })),
                            )
                            .child(
                                Button::new("create-loading", "Create Worker")
                                    .leading_icon(Icon::new("plus.svg").size(px(14.)))
                                    .size(button::Size::Sm)
                                    .w(px(124.))
                                    .justify_start()
                                    .loading(true),
                            ),
                        cx,
                    ))
                    .child(cell(
                        "button-group",
                        "Button Group",
                        ButtonGroup::new("deploy-actions", "Deployment actions")
                            .item(
                                Button::new("deploy", "Deploy")
                                    .size(button::Size::Sm)
                                    .variant(button::Variant::Primary)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.status = "Deployment queued".into();
                                        cx.notify();
                                    })),
                            )
                            .item(
                                Button::icon(
                                    "deploy-options",
                                    "More options",
                                    Icon::new("caret-down.svg"),
                                )
                                .size(button::Size::Sm)
                                .variant(button::Variant::Primary)
                                .on_click(cx.listener(
                                    |this, _, _, cx| {
                                        this.status = "Deployment options opened".into();
                                        cx.notify();
                                    },
                                )),
                            ),
                        cx,
                    ))
                    .child(cell(
                        "input",
                        "Input",
                        bounded(
                            stack()
                                .gap(px(12.))
                                .child(Input::new("name-input", &self.name).show_label(false))
                                .child(
                                    Input::new("invalid-input", &self.invalid)
                                        .show_label(false)
                                        .error_visible("Invalid!", false),
                                ),
                        ),
                        cx,
                    )),
            )
            .child(
                grid_row()
                    .child(cell(
                        "select",
                        "Select",
                        bounded(
                            Select::new("region", &self.region)
                                .show_label(false)
                                .placeholder("Select a version..."),
                        ),
                        cx,
                    ))
                    .child(cell(
                        "toolbar",
                        "Toolbar",
                        Toolbar::new("project-toolbar", "DNS records", &self.toolbar),
                        cx,
                    ))
                    .child(cell(
                        "switch",
                        "Switch",
                        Switch::new("notifications", "Switch")
                            .show_label(false)
                            .checked(self.notifications)
                            .on_change(move |value, _, _, cx| {
                                let _ = switch.update(cx, |this, cx| {
                                    this.notifications = value;
                                    cx.notify();
                                });
                            }),
                        cx,
                    )),
            )
            .child(
                grid_row()
                    .child(cell(
                        "checkbox",
                        "Checkbox",
                        Checkbox::new("updates", "Max bandwidth")
                            .state(self.checked)
                            .on_change(move |value, _, _, cx| {
                                let _ = checkbox.update(cx, |this, cx| {
                                    this.checked = value;
                                    cx.notify();
                                });
                            }),
                        cx,
                    ))
                    .child(cell(
                        "radio",
                        "Radio",
                        div().w(px(100.)).h(px(80.)).child(
                            RadioGroup::new("plan", "Select option", Some(self.plan))
                                .item(RadioItem::new("option1", 0, "Option 1"))
                                .item(RadioItem::new("option2", 1, "Option 2"))
                                .on_change(move |value, _, _, cx| {
                                    let _ = radio.update(cx, |this, cx| {
                                        this.plan = value;
                                        cx.notify();
                                    });
                                }),
                        ),
                        cx,
                    ))
                    .child(cell(
                        "input-group",
                        "Input Group",
                        bounded(
                            InputGroup::new("endpoint", &self.endpoint)
                                .show_label(false)
                                .end(InputGroupAddon::text(".workers.dev")),
                        ),
                        cx,
                    )),
            )
    }
    fn surfaces(&self, cx: &mut Context<Self>) -> Div {
        let tip = self.tip.downgrade();
        let owner = cx.entity().downgrade();
        stack()
            .gap(px(0.))
            .size_full()
            .child(
                grid_row()
                    .child(cell(
                        "dialog",
                        "Dialog",
                        DialogTrigger::new(
                            "edit-trigger",
                            &self.dialog,
                            Button::new("edit-project", "Delete").size(button::Size::Sm),
                        ),
                        cx,
                    ))
                    .child(cell(
                        "dropdown",
                        "Dropdown",
                        Dropdown::new("project-menu", &self.menu, "Add"),
                        cx,
                    ))
                    .child(cell(
                        "popover",
                        "Popover",
                        div().w(px(118.)).h(px(38.)).child(
                            Popover::new("project-details", &self.popover, "Open Popover")
                                .width(px(240.))
                                .content(|_, _, _| {
                                    stack()
                                        .gap(px(8.))
                                        .child(div().font_weight(FontWeight::SEMIBOLD).child("Popover Title"))
                                        .child(Text::new("popover-copy", "This is a popover."))
                                }),
                        ),
                        cx,
                    )),
            )
            .child(
                grid_row()
                    .child(cell(
                        "tooltip",
                        "Tooltip",
                        Tooltip::new("help-tip", &self.tip, "Add", move |_, _| {
                            let tip = tip.clone();
                            Button::icon(
                                "help",
                                "Add",
                                Icon::new("plus.svg").size(px(14.)),
                            )
                            .size(button::Size::Sm)
                            .on_click(move |_, _, cx| {
                                let _ = tip.update(cx, |state, cx| state.set_open(true, cx));
                            })
                        }),
                        cx,
                    ))
                    .child(cell(
                        "collapsible",
                        "Collapsible",
                        div()
                            .w(px(240.))
                            .h(px(if self.details { 84. } else { 28. }))
                            .child(
                                Collapsible::new("advanced", "What is Kumo?")
                                    .trigger(
                                        Button::new("disclosure", "What is Kumo?")
                                            .variant(button::Variant::Ghost)
                                            .size(button::Size::Sm)
                                            .w_full()
                                            .trailing_icon(
                                                Icon::new("caret-down.svg").size(px(14.)),
                                            ),
                                    )
                                    .open(self.details)
                                    .on_open_change(move |open, _, cx| {
                                        let _ = owner.update(cx, |this, cx| {
                                            this.details = open;
                                            cx.notify();
                                        });
                                    })
                                    .panel(CollapsiblePanel::new().child(Text::new(
                                        "advanced-copy",
                                        "Kumo is Cloudflare's component library.",
                                    ))),
                            ),
                        cx,
                    ))
                    .child(cell(
                        "layer-card",
                        "Layer Card · Text",
                        bounded(
                            LayerCard::new("sample-card")
                                .section(
                                    layer_card::Section::secondary("header")
                                        .child(Text::new("card-title", "Next Steps")),
                                )
                                .section(
                                    layer_card::Section::primary("body")
                                        .child(Text::new("card-copy", "Hello")),
                                ),
                        ),
                        cx,
                    )),
            )
            .child(
                grid_row()
                    .child(cell(
                        "badge",
                        "Badge",
                        row().gap(px(6.)).justify_center()
                            .child(Badge::new("blue", "Blue").variant(badge::Variant::Blue))
                            .child(Badge::dot("active", "Active", badge::Variant::Success))
                            .child(Badge::new("outline", "Outline").variant(badge::Variant::Outline))
                            .child(Badge::new("beta", "Beta").variant(badge::Variant::Beta))
                            .child(Badge::new("red", "Red").variant(badge::Variant::Red)),
                        cx,
                    ))
                    .child(cell(
                        "breadcrumbs",
                        "Breadcrumbs · Link",
                        Breadcrumbs::new("trail")
                            .link(Link::new("home", "Home", "app://home").on_navigate(
                                cx.listener(|this, _, _, cx| {
                                    this.status = "Home opened".into();
                                    cx.notify();
                                }),
                            ))
                            .separator()
                            .link(Link::new("docs", "Docs", "app://docs").on_navigate(cx.listener(|this, _, _, cx| { this.status = "Docs opened".into(); cx.notify(); })))
                            .separator()
                            .current(BreadcrumbCurrent::new("current", "Page")),
                        cx,
                    ))
                    .child(cell(
                        "copy",
                        "Inline Copy Text",
                        InlineCopyText::new("copy-id", "f86b3f10-32e9-4db7-ae95-84a1b2c3d4e5")
                            .labels("Copy database ID", "Database ID copied"),
                        cx,
                    )),
            )
            .child(
                Dialog::new("project-editor", &self.dialog, move |close, _, _| {
                    stack()
                        .p(px(24.))
                        .gap(px(16.))
                        .child(
                            div()
                                .text_size(px(20.))
                                .line_height(px(28.))
                                .font_weight(FontWeight::SEMIBOLD)
                                .child("Delete Resource?"),
                        )
                        .child(Text::new("dialog-copy", "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."))
                        .child(
                            row()
                                .justify_end()
                                .child(close.button(Button::new("cancel", "Cancel")))
                                .child(close.button(Button::new("delete-resource", "Delete").variant(button::Variant::Destructive))),
                        )
                        .into_any_element()
                })
                .description("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."),
            )
    }
    fn data(&self, cx: &mut Context<Self>) -> Div {
        let page = self.pagination.read(cx).page();
        let mut body = TableBody::new("workers");
        let workers = [
            ("Worker 1", "Active"),
            ("Worker 2", "Paused"),
            ("Worker 3", "Active"),
        ];
        for (i, &(name, status)) in workers.iter().enumerate().skip((page - 1) * 2).take(2) {
            let owner = cx.entity().downgrade();
            let selected = self.selected[i];
            body = body.row(
                TableRow::new(format!("worker-{i}"))
                    .variant(if selected {
                        table::RowVariant::Selected
                    } else {
                        table::RowVariant::Default
                    })
                    .cell(
                        TableCheckCell::new("select", format!("Select {name}"))
                            .state(if selected {
                                checkbox::State::Checked
                            } else {
                                checkbox::State::Unchecked
                            })
                            .on_change(move |state, _, _, cx| {
                                let _ = owner.update(cx, |this, cx| {
                                    this.selected[i] = state == checkbox::State::Checked;
                                    cx.notify();
                                });
                            }),
                    )
                    .cell(TableCell::new("name").text(name))
                    .cell(TableCell::new("status").child(Badge::dot(
                        "status",
                        status,
                        if status == "Active" {
                            badge::Variant::Success
                        } else {
                            badge::Variant::Neutral
                        },
                    ))),
            );
        }
        stack()
            .gap(px(8.))
            .w_full()
            .child(
                Table::new("worker-table")
                    .accessibility_label("Workers")
                    .layout(table::Layout::Fixed)
                    .columns([
                        table::ColumnWidth::Pixels(px(40.)),
                        table::ColumnWidth::Auto,
                        table::ColumnWidth::Pixels(px(110.)),
                    ])
                    .header(
                        TableHeader::new("header")
                            .variant(table::HeaderVariant::Compact)
                            .row(
                                TableRow::new("columns")
                                    .cell(TableHead::new("selection").text(""))
                                    .cell(TableHead::new("name").text("Name"))
                                    .cell(TableHead::new("status").text("Status")),
                            ),
                    )
                    .body(body),
            )
            .child(Pagination::new("worker-pages", &self.pagination))
    }
    fn details(&self, cx: &mut Context<Self>) -> Div {
        let toasts = self.toasts.downgrade();
        let focus = self.name.read(cx).focus_handle(cx);
        stack()
            .gap(px(0.))
            .size_full()
            .child(
                grid_row()
                    .child(cell(
                        "input-area",
                        "Input Area",
                        bounded(
                            InputArea::new("release-notes", &self.notes)
                                .rows(3)
                                .show_label(false),
                        ),
                        cx,
                    ))
                    .child(cell(
                        "sensitive-input",
                        "Sensitive Input",
                        bounded(SensitiveInput::new("secret", &self.secret).show_label(false)),
                        cx,
                    ))
                    .child(cell(
                        "field",
                        "Field · Label",
                        bounded(
                            stack()
                                .gap(px(8.))
                                .child(
                                    Label::new("name-label", "Default Label").focus_target(&focus),
                                )
                                .child(
                                    Field::control(
                                        "name-field",
                                        Input::new("field-input", &self.name),
                                        cx,
                                    )
                                    .show_label(false),
                                ),
                        ),
                        cx,
                    )),
            )
            .child(
                grid_row()
                    .child(cell(
                        "banner",
                        "Banner",
                        bounded(
                            stack()
                                .gap(px(8.))
                                .child(
                                    Banner::new("banner-info")
                                        .size(banner::Size::Sm)
                                        .variant(banner::Variant::Info)
                                        .description("This is a default banner."),
                                )
                                .child(
                                    Banner::new("banner-alert")
                                        .size(banner::Size::Sm)
                                        .variant(banner::Variant::Alert)
                                        .title("This is an alert banner."),
                                )
                                .child(
                                    Banner::new("banner-error")
                                        .size(banner::Size::Sm)
                                        .variant(banner::Variant::Error)
                                        .title("This is an error banner."),
                                ),
                        ),
                        cx,
                    ))
                    .child(cell(
                        "loading",
                        "Loader · Skeleton Line",
                        bounded(
                            stack()
                                .items_center()
                                .gap(px(20.))
                                .child(row().justify_center().child(Loader::new("preview-loader")))
                                .child(div().w(px(190.)).h(px(8.)).child(
                                    SkeletonLine::new("preview-skeleton").width_range(100..=100),
                                )),
                        ),
                        cx,
                    ))
                    .child(cell(
                        "meter",
                        "Meter",
                        bounded(Meter::new("storage", "My meter", 75.).custom_value("100 / 5,000")),
                        cx,
                    )),
            )
            .child(
                grid_row()
                    .child(
                        cell("table", "Table · Pagination", self.data(cx), cx)
                            .flex_basis(relative(2. / 3.))
                            .flex_grow(0.)
                            .flex_shrink_0(),
                    )
                    .child(cell(
                        "empty",
                        "Empty · Toast",
                        bounded(
                            stack()
                                .gap(px(12.))
                                .child(
                                    Empty::new("no-results", "No results")
                                        .size(empty::Size::Sm)
                                        .description("Try a different search"),
                                )
                                .child(
                                    row().justify_center().child(
                                        Button::new("toast", "Give me a toast")
                                            .size(button::Size::Sm)
                                            .on_click(move |_, window, cx| {
                                                let _ = toasts.update(cx, |state, cx| {
                                                    state.add(
                                                        Toast::new("saved", "Toast created")
                                                            .description(
                                                                "This is a toast notification.",
                                                            )
                                                            .variant(ToastVariant::Warning),
                                                        window,
                                                        cx,
                                                    );
                                                });
                                            }),
                                    ),
                                ),
                        ),
                        cx,
                    )),
            )
    }
}
impl Render for Showcase {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let page = self.page(cx);
        let content = match page {
            0 => self.controls(cx),
            1 => self.surfaces(cx),
            _ => self.details(cx),
        };
        let t = theme(cx).clone();
        TooltipProvider::new(
            "showcase-tooltips",
            [self.tip.downgrade()],
            div()
                .id("showcase")
                .track_focus(&self.focus)
                .tab_group()
                .w_full()
                .h(window.viewport_size().height)
                .flex()
                .flex_col()
                .bg(t.colors.canvas)
                .text_color(t.text.default)
                .font_family(t.typography.font_family.clone())
                .text_size(t.typography.base.size)
                .line_height(t.typography.base.line_height)
                .on_action(|_: &FocusNext, window, cx| window.focus_next(cx))
                .on_action(|_: &FocusPrevious, window, cx| window.focus_prev(cx))
                .child(
                    row()
                        .h(px(56.))
                        .flex_shrink_0()
                        .px(px(20.))
                        .justify_between()
                        .border_b_1()
                        .border_color(t.colors.hairline)
                        .child(
                            row()
                                .gap(px(12.))
                                .child(div().font_weight(FontWeight::MEDIUM).child("Kumo"))
                                .child(div().text_color(t.text.subtle).child("/ GPUI")),
                        )
                        .child(
                            Tabs::new("showcase-pages", "Showcase pages", &self.navigation)
                                .size(tabs::Size::Sm)
                                .variant(tabs::Variant::Underline),
                        )
                        .child(
                            row()
                                .gap(px(12.))
                                .child(
                                    div()
                                        .text_size(px(12.))
                                        .text_color(t.text.subtle)
                                        .child("0.1.0-rc.2"),
                                )
                                .child(
                                    Button::icon(
                                        "theme",
                                        "Switch theme",
                                        Icon::new(if t.appearance == Appearance::Dark {
                                            "sun.svg"
                                        } else {
                                            "moon.svg"
                                        })
                                        .size(px(16.)),
                                    )
                                    .size(button::Size::Sm)
                                    .variant(button::Variant::Ghost)
                                    .on_click(|_, _, cx| {
                                        let appearance = if theme(cx).appearance == Appearance::Dark
                                        {
                                            Appearance::Light
                                        } else {
                                            Appearance::Dark
                                        };
                                        set_appearance(appearance, cx);
                                    }),
                                ),
                        ),
                )
                .child(div().flex_1().min_h_0().child(content))
                .child(
                    row()
                        .h(px(32.))
                        .flex_shrink_0()
                        .px(px(20.))
                        .justify_between()
                        .child(
                            div()
                                .text_size(px(12.))
                                .text_color(t.text.subtle)
                                .child(self.status.clone()),
                        )
                        .child(
                            div()
                                .text_size(px(12.))
                                .text_color(t.text.subtle)
                                .child(format!("{} / 3", page + 1)),
                        ),
                )
                .child(ToastViewport::new("showcase-toasts", &self.toasts)),
        )
    }
}
/// Launch the paged component showcase.
pub fn run() {
    let args: Vec<_> = std::env::args().collect();
    let width = args
        .iter()
        .find_map(|arg| arg.strip_prefix("--width=")?.parse::<f32>().ok())
        .unwrap_or(1200.);
    gpui_kit::application()
        .with_assets(crate::GalleryAssets)
        .run(move |cx: &mut App| {
            gpui_kumo::init(cx);
            set_appearance(
                if args.iter().any(|arg| arg == "--light") {
                    Appearance::Light
                } else {
                    Appearance::Dark
                },
                cx,
            );
            cx.set_reduce_motion(args.iter().any(|arg| arg == "--reduce-motion"));
            cx.bind_keys([
                KeyBinding::new("tab", FocusNext, None),
                KeyBinding::new("shift-tab", FocusPrevious, None),
                KeyBinding::new("cmd-q", Quit, None),
            ]);
            cx.on_action(|_: &Quit, cx| cx.quit());
            cx.on_window_closed(|cx, _| {
                if cx.windows().is_empty() {
                    cx.quit();
                }
            })
            .detach();
            gpui_kit::open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: Some("Kumo / GPUI".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                        None,
                        size(px(width.max(980.)), px(800.)),
                        cx,
                    ))),
                    window_min_size: Some(size(px(980.), px(800.))),
                    ..Default::default()
                },
                cx,
                |window, cx| cx.new(|cx| Showcase::new(window, cx)),
            )
            .expect("open native showcase");
            cx.activate(true);
        });
}
#[cfg(test)]
mod tests {
    use super::*;
    use gpui_kit::{TestAppContext, test::TestWindowExt};
    #[gpui_kit::test]
    fn page_changes_unmount_components_and_retain_edits(cx: &mut TestAppContext) {
        cx.update(gpui_kumo::init);
        let (view, cx) = cx.add_window_view(Showcase::new);
        cx.update(|window, cx| {
            window.render_frame(cx);
            window.within("name-input").click("control", cx);
            window.press("cmd-a", cx);
            window.input("Native Worker", cx);
            window.click("Surfaces", cx);
            window.render_frame(cx);
            assert!(window.try_find("create").is_none());
            assert!(window.try_find("edit-project").is_some());
            window.click("help", cx);
            window.render_frame(cx);
            assert!(view.read(cx).tip.read(cx).is_open());
            window.click("Details", cx);
            window.render_frame(cx);
            assert!(!view.read(cx).tip.read(cx).is_open());
            assert!(window.try_find("preview-loader").is_some());
            assert_eq!(
                view.read(cx).name.read(cx).value(cx).as_ref(),
                "Native Worker"
            );
            window.click("Controls", cx);
            window.render_frame(cx);
            assert!(window.try_find("preview-loader").is_none());
        });
    }
    #[gpui_kit::test]
    fn table_pages_retain_selection(cx: &mut TestAppContext) {
        cx.update(gpui_kumo::init);
        let (view, cx) = cx.add_window_view(Showcase::new);
        cx.update(|window, cx| {
            view.update(cx, |this, cx| this.navigate(2, window, cx));
            window.render_frame(cx);
            window.within("worker-0").click("select", cx);
            assert!(view.read(cx).selected[0]);
            let pages = view.read(cx).pagination.clone();
            pages.update(cx, |state, cx| state.set_page(2, window, cx));
            view.update(cx, |_, cx| cx.notify());
            window.render_frame(cx);
            assert!(window.try_find("worker-0").is_none());
            assert!(window.try_find("worker-2").is_some());
            assert!(view.read(cx).selected[0]);
        });
    }
}
