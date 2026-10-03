//! Select and Link review: --dark and --width=520 (default760).
use gpui_kit::{
    App, AppContext, Bounds, Context, Entity, FocusHandle, InteractiveElement, IntoElement,
    KeyBinding, ParentElement, Render, StatefulInteractiveElement, Styled, Subscription, Window,
    WindowBounds, WindowOptions, div, px, size,
};
use gpui_kumo::{
    Appearance, Button, Link, Select, SelectEvent, SelectOption, SelectState, SelectValue, Text,
    Toolbar, ToolbarEvent, ToolbarItem, ToolbarState, set_appearance, theme,
};

gpui_kit::actions!(select_link_review, [Next, Previous, Quit]);

struct Review {
    focus: FocusHandle,
    select: Entity<SelectState<u32>>,
    toolbar: Entity<ToolbarState>,
    disabled: bool,
    feedback: String,
    activations: usize,
    _subscriptions: Vec<Subscription>,
}
impl Review {
    fn new(cx: &mut Context<Self>) -> Self {
        let select = cx.new(|cx| {
            SelectState::new(
                "Destination",
                SelectValue::Single(Some(1)),
                vec![
                    SelectOption::new("overview", 1, "Overview"),
                    SelectOption::new("unavailable", 2, "Unavailable destination").disabled(true),
                    SelectOption::new("settings", 3, "Settings"),
                ],
                cx,
            )
        });
        let toolbar = cx.new(|cx| {
            ToolbarState::new(
                vec![
                    ToolbarItem::button("previous", "Previous"),
                    ToolbarItem::link("documentation", "Documentation", "/docs"),
                    ToolbarItem::button("next", "Next"),
                ],
                cx,
            )
        });
        let subscriptions = vec![
            cx.subscribe(
                &select,
                |this: &mut Self, _, event: &SelectEvent<u32>, cx| {
                    this.feedback = format!("Selected {:?}", event.value);
                    cx.notify();
                },
            ),
            cx.subscribe(&toolbar, |this: &mut Self, _, event: &ToolbarEvent, cx| {
                this.activations += 1;
                this.feedback = match event {
                    ToolbarEvent::Navigate { request, .. } => format!("Navigate {}", request.href),
                    ToolbarEvent::Activate { .. } => "Toolbar action".into(),
                };
                cx.notify();
            }),
        ];
        Self {
            focus: cx.focus_handle(),
            select,
            toolbar,
            disabled: false,
            feedback: "Choose a destination or activate a link.".into(),
            activations: 0,
            _subscriptions: subscriptions,
        }
    }
}
impl Render for Review {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = theme(cx);
        div()
            .id("review")
            .track_focus(&self.focus)
            .tab_group()
            .on_action(|_: &Next, window, cx| window.focus_next(cx))
            .on_action(|_: &Previous, window, cx| window.focus_prev(cx))
            .on_action(|_: &Quit, _, cx| cx.quit())
            .size_full()
            .overflow_y_scroll()
            .flex()
            .flex_col()
            .p(px(24.))
            .gap(px(16.))
            .bg(theme.colors.canvas)
            .text_color(theme.text.default)
            .font_family(theme.typography.font_family.clone())
            .text_size(theme.typography.base.size)
            .line_height(theme.typography.base.line_height)
            .child(Text::new("title", "Select and Link"))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap(px(8.))
                    .child(
                        Button::new("appearance", "Switch theme").on_click(|_, _, cx| {
                            set_appearance(gpui_kumo::theme(cx).appearance.opposite(), cx);
                        }),
                    )
                    .child(
                        Button::new(
                            "availability",
                            if self.disabled {
                                "Enable controls"
                            } else {
                                "Disable controls"
                            },
                        )
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.disabled = !this.disabled;
                            this.select
                                .update(cx, |s, cx| s.set_disabled(this.disabled, window, cx));
                            this.toolbar
                                .update(cx, |s, cx| s.set_disabled(this.disabled, window, cx));
                            cx.notify();
                        })),
                    ),
            )
            .child(
                kumo_gallery::panel(theme, "Select · open, confirm and restore focus")
                    .child(Select::new("destination", &self.select)),
            )
            .child(
                kumo_gallery::panel(theme, "Link · pointer and keyboard activation").child(
                    Link::new("standalone", "Open documentation", "/docs")
                        .disabled(self.disabled)
                        .external_icon(true)
                        .on_navigate(cx.listener(
                            |this, request: &gpui_kumo::link::NavigationRequest, _, cx| {
                                this.activations += 1;
                                this.feedback = format!("Navigate {}", request.href);
                                cx.notify();
                            },
                        )),
                ),
            )
            .child(
                kumo_gallery::panel(theme, "Toolbar · arrows across a link, then Tab")
                    .child(Toolbar::new("navigation", "Navigation", &self.toolbar)),
            )
            .child(Button::new("after", "After controls"))
            .child(Text::new(
                "feedback",
                format!("{} · {} activations", self.feedback, self.activations),
            ))
    }
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let width = args
        .iter()
        .find_map(|arg| arg.strip_prefix("--width=")?.parse::<f32>().ok())
        .unwrap_or(760.);
    let dark = args.iter().any(|arg| arg == "--dark");
    gpui_kit::application().run(move |cx: &mut App| {
        gpui_kumo::init(cx);
        set_appearance(
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
            KeyBinding::new("cmd-q", Quit, None),
        ]);
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.on_window_closed(|cx, _| cx.quit()).detach();
        if let Err(error) = gpui_kit::open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(width), px(720.)),
                    cx,
                ))),
                ..Default::default()
            },
            cx,
            |window, cx| {
                window.set_window_title("Select and Link review");
                cx.new(Review::new)
            },
        ) {
            eprintln!("Failed to open review: {error:#}");
            cx.quit();
            return;
        }
        cx.activate(true);
    });
}
