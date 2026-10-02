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

mod badges;
mod banners;
mod button_groups;
mod buttons;
mod cards;
mod checkboxes;
mod empties;
mod fields;
mod input_groups;
mod inputs;
mod links;
mod loaders;
mod popovers;
mod radios;
mod sensitive_inputs;
mod switches;
mod texts;
mod tooltips;

#[cfg(feature = "frame-profiler")]
mod performance;

gpui_kit::actions!(gallery, [Quit, ToggleAppearance, FocusNext, FocusPrevious]);

struct GalleryAssets;

impl AssetSource for GalleryAssets {
    fn load(&self, path: &str) -> gpui_kit::Result<Option<Cow<'static, [u8]>>> {
        Ok(match path {
            "empty-copy.svg" => Some(Cow::Borrowed(include_bytes!(
                "../../../crates/gpui-kumo/assets/empty-copy.svg"
            ))),
            "caret-down.svg" => Some(Cow::Borrowed(include_bytes!(
                "../../../crates/gpui-kumo/assets/caret-down.svg"
            ))),
            "workspace.svg" => Some(Cow::Borrowed(include_bytes!("../assets/workspace.svg"))),
            _ => None,
        })
    }

    fn list(&self, path: &str) -> gpui_kit::Result<Vec<SharedString>> {
        Ok(if path.is_empty() {
            vec![
                "workspace.svg".into(),
                "caret-down.svg".into(),
                "empty-copy.svg".into(),
            ]
        } else {
            vec![]
        })
    }
}

struct Gallery {
    focus_handle: FocusHandle,
    _theme_subscription: Subscription,
    inputs: gpui_kit::Entity<inputs::Inputs>,
    fields: gpui_kit::Entity<fields::Fields>,
    checkboxes: gpui_kit::Entity<checkboxes::Checkboxes>,
    cards: gpui_kit::Entity<cards::Cards>,
    links: gpui_kit::Entity<links::Links>,
    banners: gpui_kit::Entity<banners::Banners>,
    popovers: gpui_kit::Entity<popovers::Popovers>,
    radios: gpui_kit::Entity<radios::Radios>,
    switches: gpui_kit::Entity<switches::Switches>,
    button_groups: gpui_kit::Entity<button_groups::ButtonGroups>,
    input_groups: gpui_kit::Entity<input_groups::InputGroups>,
    tooltips: gpui_kit::Entity<tooltips::Tooltips>,
    sensitive_inputs: gpui_kit::Entity<sensitive_inputs::SensitiveInputs>,
    activations: usize,
    disabled: bool,
    loading: bool,
    #[cfg(feature = "frame-profiler")]
    measurement: performance::Measurement,
}

impl Render for Gallery {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = current_theme(cx).clone();
        let root = div()
            .id("gallery")
            .track_focus(&self.focus_handle)
            .on_action(|_: &FocusNext, window, cx| window.focus_next(cx))
            .on_action(|_: &FocusPrevious, window, cx| window.focus_prev(cx))
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
            .line_height(theme.typography.base.line_height);
        #[cfg(feature = "frame-profiler")]
        let root = root
            .on_action(cx.listener(|this, _: &performance::Start, window, cx| {
                this.measurement.start(window, cx);
            }))
            .on_action(cx.listener(|this, _: &performance::Stop, window, cx| {
                this.measurement.stop(window, cx);
            }))
            .on_action(|_: &performance::ToggleMotion, _, cx| {
                cx.set_reduce_motion(!cx.reduce_motion());
            });
        root.child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .flex_wrap()
                .gap(theme.spacing.twelve)
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
                                        .child("Text · Button · Input · Popover · foundations"),
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
        .child(self.sensitive_inputs.clone())
        .child(self.checkboxes.clone())
        .child(self.fields.clone())
        .child(self.tooltips.clone())
        .child(self.input_groups.clone())
        .child(self.button_groups.clone())
        .child(self.switches.clone())
        .child(self.radios.clone())
        .child(empties::panel(&theme))
        .child(self.banners.clone())
        .child(badges::panel(&theme))
        .child(self.links.clone())
        .child(self.cards.clone())
        .child(loaders::panel(&theme))
        .child(texts::panel(&theme))
        .child(self.popovers.clone())
        .child(self.inputs.clone())
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
                .child("System font · ⌘L switches appearance · Text · Button · Input · Popover"),
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
                KeyBinding::new("tab", FocusNext, None),
                KeyBinding::new("shift-tab", FocusPrevious, None),
                KeyBinding::new("cmd-q", Quit, None),
                KeyBinding::new("cmd-l", ToggleAppearance, None),
            ]);
            #[cfg(feature = "frame-profiler")]
            cx.bind_keys([
                KeyBinding::new("cmd-shift-p", performance::Start, None),
                KeyBinding::new("cmd-shift-o", performance::Stop, None),
                KeyBinding::new("cmd-shift-m", performance::ToggleMotion, None),
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
            if let Err(error) = gpui_kit::open_window(
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
                            inputs: cx.new(|cx| inputs::Inputs::new(window, cx)),
                            fields: cx.new(|cx| fields::Fields::new(window, cx)),
                            checkboxes: cx.new(checkboxes::Checkboxes::new),
                            cards: cx.new(|cx| cards::Cards::new(window, cx)),
                            links: cx.new(links::Links::new),
                            banners: cx.new(banners::Banners::new),
                            popovers: cx.new(|cx| popovers::Popovers::new(window, cx)),
                            radios: cx.new(radios::Radios::new),
                            switches: cx.new(switches::Switches::new),
                            button_groups: cx.new(button_groups::ButtonGroups::new),
                            input_groups: cx.new(|cx| input_groups::InputGroups::new(window, cx)),
                            tooltips: cx.new(|cx| tooltips::Tooltips::new(window, cx)),
                            sensitive_inputs: cx
                                .new(|cx| sensitive_inputs::SensitiveInputs::new(window, cx)),
                            activations: 0,
                            disabled: false,
                            loading: false,
                            #[cfg(feature = "frame-profiler")]
                            measurement: performance::Measurement::default(),
                        }
                    })
                },
            ) {
                eprintln!("Failed to open the Kumo gallery window: {error:#}");
                cx.quit();
                return;
            }
            cx.activate(true);
        });
}
