//! Resize an open long popup, scroll to its last action, and inspect both themes.
use gpui_kit::{
    App, AppContext, Bounds, Context, Entity, IntoElement, KeyBinding, Menu, MenuItem,
    ParentElement, Render, Styled, TitlebarOptions, Window, WindowBounds, WindowOptions, div, px,
    size,
};
use gpui_kumo::{Button, Popover, PopoverState, theme};

gpui_kit::actions!(popover_stress, [Quit]);

struct Stress {
    popup: Entity<PopoverState>,
    activations: usize,
}

impl Render for Stress {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = theme(cx);
        let viewport = window.viewport_size();
        let owner = cx.entity().downgrade();
        div()
            .size_full()
            .bg(theme.colors.canvas)
            .text_color(theme.text.default)
            .font_family(theme.typography.font_family.clone())
            .text_size(theme.typography.base.size)
            .child(
                div()
                    .p(px(24.))
                    .flex()
                    .gap(px(24.))
                    .child(format!("Last action activations: {}", self.activations))
                    .child(
                        Button::new("theme", "Switch appearance").on_click(cx.listener(
                            |_, _, _, cx| {
                                let appearance = gpui_kumo::theme(cx).appearance.opposite();
                                gpui_kumo::set_appearance(appearance, cx);
                                cx.notify();
                            },
                        )),
                    ),
            )
            .child(
                div()
                    .absolute()
                    .left(viewport.width / 2. - px(50.))
                    .top(viewport.height - px(52.))
                    .child(
                        Popover::new("stress", &self.popup, "Open long popup")
                            .width(px(600.))
                            .arrow(true)
                            .content(move |_, _, _| {
                                let owner = owner.clone();
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.))
                                    .children((0..30).map(|index| {
                                        div()
                                            .h(px(36.))
                                            .flex_shrink_0()
                                            .child(format!("Long content row {index}"))
                                    }))
                                    .child(Button::new("last", "Last action").on_click(
                                        move |_, _, cx| {
                                            let _ = owner.update(cx, |state, cx| {
                                                state.activations += 1;
                                                cx.notify();
                                            });
                                        },
                                    ))
                            }),
                    ),
            )
    }
}

fn main() {
    gpui_kit::application().run(|cx: &mut App| {
        gpui_kumo::init(cx);
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.set_menus([Menu::new("Popover Stress").items([MenuItem::action("Quit", Quit)])]);
        cx.on_window_closed(|cx, _| cx.quit()).detach();
        if let Err(error) = gpui_kit::open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("Popover stress".into()),
                    ..Default::default()
                }),
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(640.), px(480.)),
                    cx,
                ))),
                ..Default::default()
            },
            cx,
            |_, cx| {
                cx.new(|cx| Stress {
                    popup: cx.new(|cx| PopoverState::new("Long popup", cx)),
                    activations: 0,
                })
            },
        ) {
            eprintln!("Failed to open Popover stress example: {error:#}");
            cx.quit();
            return;
        }
        cx.activate(true);
    });
}
