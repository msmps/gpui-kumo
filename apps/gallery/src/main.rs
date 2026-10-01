use std::borrow::Cow;

use gpui_kit::{
    App, AppContext, AssetSource, Bounds, Context, Div, FocusHandle, FontWeight,
    InteractiveElement, IntoElement, KeyBinding, Menu, MenuItem, ParentElement, Render,
    SharedString, StatefulInteractiveElement, Styled, Subscription, TitlebarOptions, Window,
    WindowBounds, WindowOptions, div, px, size, svg,
};
use gpui_kumo::{
    Appearance, Button, Theme, button::Variant, set_appearance, theme as current_theme,
};

mod buttons;

gpui_kit::actions!(gallery, [Quit, ToggleAppearance]);

struct GalleryAssets;

impl AssetSource for GalleryAssets {
    fn load(&self, path: &str) -> gpui_kit::Result<Option<Cow<'static, [u8]>>> {
        Ok(match path {
            "workspace.svg" => Some(Cow::Borrowed(include_bytes!("../assets/workspace.svg"))),
            _ => None,
        })
    }

    fn list(&self, path: &str) -> gpui_kit::Result<Vec<SharedString>> {
        Ok(if path.is_empty() {
            vec!["workspace.svg".into()]
        } else {
            vec![]
        })
    }
}

struct Gallery {
    focus_handle: FocusHandle,
    _theme_subscription: Subscription,
    activations: usize,
    disabled: bool,
    loading: bool,
}

impl Render for Gallery {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = current_theme(cx).clone();
        div()
            .id("gallery")
            .track_focus(&self.focus_handle)
            .on_action(|_: &Quit, _, cx| cx.quit())
            .on_action(|_: &ToggleAppearance, _, cx| {
                set_appearance(current_theme(cx).appearance.opposite(), cx);
            })
            .size_full()
            .overflow_y_scroll()
            .flex()
            .flex_col()
            .p(px(32.))
            .gap(px(24.))
            .bg(theme.colors.canvas)
            .text_color(theme.text.default)
            .font_family(theme.typography.font_family.clone())
            .text_size(theme.typography.base.size)
            .line_height(theme.typography.base.line_height)
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(theme.spacing.twelve)
                            .child(
                                svg()
                                    .path("workspace.svg")
                                    .size_8()
                                    .text_color(theme.text.brand),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(theme.spacing.four)
                                    .child(
                                        div()
                                            .text_size(px(24.))
                                            .line_height(px(32.))
                                            .font_weight(FontWeight::MEDIUM)
                                            .child("Kumo component gallery"),
                                    )
                                    .child(
                                        div()
                                            .text_color(theme.text.subtle)
                                            .child("Button · foundations"),
                                    ),
                            ),
                    )
                    .child(div().flex().gap(theme.spacing.eight).children(
                        [Appearance::Light, Appearance::Dark].map(|appearance| {
                            let selected = theme.appearance == appearance;
                            Button::new(
                                if appearance == Appearance::Light {
                                    "light"
                                } else {
                                    "dark"
                                },
                                if appearance == Appearance::Light {
                                    "Light"
                                } else {
                                    "Dark"
                                },
                            )
                            .accessibility_label(if appearance == Appearance::Light {
                                "Light appearance"
                            } else {
                                "Dark appearance"
                            })
                            .variant(if selected {
                                Variant::Primary
                            } else {
                                Variant::Secondary
                            })
                            .on_click(move |_, _, cx| set_appearance(appearance, cx))
                        }),
                    )),
            )
            .child(buttons::interaction_panel(self, &theme, cx))
            .child(buttons::variant_panel(&theme))
            .child(buttons::size_panel(&theme))
            .child(
                div().flex().gap(px(24.)).child(color_panel(&theme)).child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(24.))
                        .flex_1()
                        .child(typography_panel(&theme))
                        .child(effects_panel(&theme)),
                ),
            )
            .child(
                div()
                    .text_color(theme.text.subtle)
                    .text_size(theme.typography.xs.size)
                    .child("System font · ⌘L switches appearance · Input and Popover are next"),
            )
    }
}

fn panel(theme: &Theme, title: &'static str) -> Div {
    div()
        .flex()
        .flex_col()
        .flex_shrink_0()
        .gap(theme.spacing.sixteen)
        .p(px(24.))
        .bg(theme.colors.base)
        .rounded(theme.radii.lg)
        .border_1()
        .border_color(theme.colors.hairline)
        .child(div().font_weight(FontWeight::MEDIUM).child(title))
}

fn color_panel(theme: &Theme) -> Div {
    panel(theme, "Semantic colors").flex_1().children(
        [
            ("Canvas", theme.colors.canvas),
            ("Base", theme.colors.base),
            ("Control", theme.colors.control),
            ("Tint", theme.colors.tint),
            ("Brand", theme.colors.brand),
            ("Danger", theme.colors.danger),
            ("Line", theme.colors.line),
            ("Focus", theme.colors.focus),
            ("Text / default", theme.text.default),
            ("Text / subtle", theme.text.subtle),
            ("Text / brand", theme.text.brand),
        ]
        .map(|(label, color)| {
            div()
                .flex()
                .items_center()
                .gap(theme.spacing.twelve)
                .child(
                    div()
                        .w(px(48.))
                        .h(px(24.))
                        .flex_shrink_0()
                        .rounded(theme.radii.sm)
                        .bg(color)
                        .border_1()
                        .border_color(theme.colors.hairline),
                )
                .child(label)
        }),
    )
}

fn typography_panel(theme: &Theme) -> Div {
    panel(theme, "Typography").children(
        [
            ("Extra small · 12 / 16", theme.typography.xs),
            ("Small · 13 / 15.29", theme.typography.sm),
            ("Base · 14 / 21", theme.typography.base),
            ("Large · 16 / 24", theme.typography.lg),
        ]
        .map(|(label, style)| {
            div()
                .text_size(style.size)
                .line_height(style.line_height)
                .font_weight(style.weight)
                .child(label)
        }),
    )
}

fn effects_panel(theme: &Theme) -> Div {
    panel(theme, "Gradients and elevation")
        .child(
            div().flex().gap(theme.spacing.sixteen).children(
                [
                    ("Primary", &theme.primary),
                    ("Destructive", &theme.destructive),
                ]
                .map(|(label, emphasis)| {
                    div()
                        .flex()
                        .flex_col()
                        .gap(theme.spacing.eight)
                        .flex_1()
                        .child(
                            div()
                                .text_color(theme.text.subtle)
                                .text_size(theme.typography.xs.size)
                                .child(label),
                        )
                        .child(
                            div()
                                .h(px(40.))
                                .rounded(theme.radii.lg)
                                .bg(emphasis.gradient(false))
                                .shadow(vec![emphasis.inset_highlight()])
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_color(gpui_kit::rgb(0xffffff))
                                .child("Rest"),
                        )
                        .child(
                            div()
                                .h(px(40.))
                                .rounded(theme.radii.lg)
                                .bg(emphasis.gradient(true))
                                .shadow(vec![emphasis.inset_highlight()])
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_color(gpui_kit::rgb(0xffffff))
                                .child("Hover"),
                        )
                }),
            ),
        )
        .child(
            div()
                .flex()
                .gap(theme.spacing.sixteen)
                .pt(theme.spacing.eight)
                .child(
                    div()
                        .flex_1()
                        .p(theme.spacing.twelve)
                        .rounded(theme.radii.lg)
                        .bg(theme.colors.control)
                        .shadow(theme.effects.shadow_xs.clone())
                        .child("Shadow / xs"),
                )
                .child(
                    div()
                        .flex_1()
                        .p(theme.spacing.twelve)
                        .rounded(theme.radii.lg)
                        .bg(theme.colors.control)
                        .shadow(theme.effects.shadow_md.clone())
                        .child("Shadow / md"),
                ),
        )
}

fn main() {
    gpui_kit::application()
        .with_assets(GalleryAssets)
        .run(|cx: &mut App| {
            gpui_kumo::init(cx);
            cx.bind_keys([
                KeyBinding::new("cmd-q", Quit, None),
                KeyBinding::new("cmd-l", ToggleAppearance, None),
            ]);
            cx.on_action(|_: &Quit, cx| cx.quit());
            cx.set_menus([Menu::new("Kumo Gallery").items([MenuItem::action("Quit", Quit)])]);
            cx.on_window_closed(|cx, _| {
                if cx.windows().is_empty() {
                    cx.quit();
                }
            })
            .detach();

            let bounds = Bounds::centered(None, size(px(1040.), px(800.)), cx);
            gpui_kit::open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: Some("GPUI Kumo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                cx,
                |window, cx| {
                    cx.new(|cx| {
                        let focus_handle = cx.focus_handle();
                        focus_handle.focus(window, cx);
                        let subscription = cx.observe_global::<Theme>(|_, cx| cx.notify());
                        Gallery {
                            focus_handle,
                            _theme_subscription: subscription,
                            activations: 0,
                            disabled: false,
                            loading: false,
                        }
                    })
                },
            )
            .expect("failed to open the Kumo gallery window");
            cx.activate(true);
        });
}
