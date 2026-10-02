//! Run with bottom/top/left/right to inspect collisions at that window edge.
use gpui_kit::{
    App, AppContext, Bounds, Context, Entity, IntoElement, KeyBinding, Menu, MenuItem,
    ParentElement, Render, Styled, TitlebarOptions, Window, WindowBounds, WindowOptions, div, px,
    size,
};
use gpui_kumo::{Popover, PopoverState, popover::Placement, theme};

gpui_kit::actions!(popover_edges, [Quit]);

struct EdgeDemo {
    state: Entity<PopoverState>,
    placement: Placement,
    side: &'static str,
}

impl Render for EdgeDemo {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = theme(cx);
        let viewport = window.viewport_size();
        let (x, y) = match self.placement {
            Placement::Bottom => (viewport.width / 2. - px(65.), viewport.height - px(52.)),
            Placement::Top => (viewport.width / 2. - px(65.), px(8.)),
            Placement::Left => (px(8.), viewport.height / 2.),
            Placement::Right => (viewport.width - px(150.), viewport.height / 2.),
        };
        div()
            .size_full()
            .bg(theme.colors.canvas)
            .text_color(theme.text.default)
            .font_family(theme.typography.font_family.clone())
            .text_size(theme.typography.base.size)
            .child(
                div()
                    .absolute()
                    .left(px(24.))
                    .top(px(24.))
                    .w(px(160.))
                    .flex()
                    .flex_col()
                    .gap(px(8.))
                    .text_color(theme.text.subtle)
                    .child(format!("Preferred side: {}", self.side))
                    .child("Click the edge trigger."),
            )
            .child(
                div().absolute().left(x).top(y).child(
                    Popover::new("edge", &self.state, format!("Request {}", self.side))
                        .placement(self.placement)
                        .width(px(240.))
                        .arrow(true)
                        .content(|_, _, _| {
                            div()
                                .h(px(80.))
                                .flex()
                                .flex_col()
                                .gap(px(8.))
                                .child("Popover content")
                                .child("The arrow points back to the trigger.")
                        }),
                ),
            )
    }
}

fn main() {
    let (placement, side) = match std::env::args().nth(1).as_deref().unwrap_or("bottom") {
        "bottom" => (Placement::Bottom, "bottom"),
        "top" => (Placement::Top, "top"),
        "left" => (Placement::Left, "left"),
        "right" => (Placement::Right, "right"),
        _ => {
            eprintln!("Usage: popover_edges [bottom|top|left|right]");
            std::process::exit(2);
        }
    };
    gpui_kit::application().run(move |cx: &mut App| {
        gpui_kumo::init(cx);
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.set_menus([Menu::new("Popover Edges").items([MenuItem::action("Quit", Quit)])]);
        cx.on_window_closed(|cx, _| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();
        if let Err(error) = gpui_kit::open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some(format!("Popover collision: {side}").into()),
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
                cx.new(|cx| EdgeDemo {
                    state: cx.new(|cx| PopoverState::new("Edge popup", cx)),
                    placement,
                    side,
                })
            },
        ) {
            eprintln!("Failed to open the edge demonstration: {error:#}");
            cx.quit();
            return;
        }
        cx.activate(true);
    });
}
