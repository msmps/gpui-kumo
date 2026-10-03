use std::borrow::Cow;

use gpui_kit::{
    App, AppContext, AssetSource, Bounds, Context, FocusHandle, FontWeight, InteractiveElement,
    IntoElement, KeyBinding, Menu, MenuItem, ParentElement, Render, SharedString,
    StatefulInteractiveElement, Styled, Subscription, TitlebarOptions, Window, WindowBounds,
    WindowOptions, div, px, size, svg,
};
use gpui_kumo::{
    Appearance, Button, Theme, button::Variant, set_appearance, theme as current_theme,
};

mod badges;
mod banners;
mod breadcrumbs;
mod button_groups;
mod buttons;
mod cards;
mod checkboxes;
mod collapsibles;
mod empties;
mod fields;
mod foundations;
use foundations::panel;
mod inline_copies;
mod input_areas;
mod input_groups;
mod inputs;
mod links;
mod loaders;
mod meters;
mod paginations;
mod popovers;
mod radios;
mod selects;
mod sensitive_inputs;
mod skeletons;
mod switches;
mod tabs;
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
    input_areas: gpui_kit::Entity<input_areas::InputAreas>,
    inline_copies: gpui_kit::Entity<inline_copies::InlineCopies>,
    collapsibles: gpui_kit::Entity<collapsibles::Collapsibles>,
    skeletons: gpui_kit::Entity<skeletons::Skeletons>,
    meters: gpui_kit::Entity<meters::Meters>,
    breadcrumbs: gpui_kit::Entity<breadcrumbs::Trails>,
    selects: gpui_kit::Entity<selects::Selects>,
    paginations: gpui_kit::Entity<paginations::Paginations>,
    tabs: gpui_kit::Entity<tabs::TabExamples>,
    activations: usize,
    disabled: bool,
    loading: bool,
    #[cfg(feature = "frame-profiler")]
    measurement: performance::Measurement,
}

impl Render for Gallery {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
        .child(self.paginations.clone())
        .child(self.tabs.clone())
        .child(self.input_groups.clone())
        .child(self.selects.clone())
        .child(self.breadcrumbs.clone())
        .child(self.meters.clone())
        .child(self.skeletons.clone())
        .child(self.collapsibles.clone())
        .child(self.inline_copies.clone())
        .child(self.input_areas.clone())
        .child(self.sensitive_inputs.clone())
        .child(self.checkboxes.clone())
        .child(self.fields.clone())
        .child(self.tooltips.clone())
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
        .child(buttons::variant_panel(
            &theme,
            // 160px label + three200px cells + three16px gaps,
            // plus32px gallery and24px panel padding on each side.
            window.viewport_size().width < px(920.),
        ))
        .child(buttons::size_panel(&theme))
        .child(foundations::panels(&theme, window.viewport_size().width))
        .child(
            div()
                .text_color(theme.text.subtle)
                .text_size(theme.typography.xs.size)
                .child("System font · ⌘L switches appearance · Text · Button · Input · Popover"),
        )
    }
}

fn main() {
    gpui_kit::application()
        .with_assets(GalleryAssets)
        .run(|cx: &mut App| {
            gpui_kumo::init(cx);
            if std::env::args().any(|arg| arg == "--reduce-motion") {
                cx.set_reduce_motion(true);
            }
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
                            input_areas: cx.new(|cx| input_areas::InputAreas::new(window, cx)),
                            inline_copies: cx.new(|_| inline_copies::InlineCopies::new()),
                            skeletons: cx.new(|_| skeletons::Skeletons),
                            collapsibles: cx.new(|cx| collapsibles::Collapsibles::new(window, cx)),
                            meters: cx.new(|_| meters::Meters::default()),
                            breadcrumbs: cx.new(|_| breadcrumbs::Trails::default()),
                            selects: cx.new(|cx| selects::Selects::new(window, cx)),
                            paginations: cx.new(|cx| paginations::Paginations::new(window, cx)),
                            tabs: cx.new(tabs::TabExamples::new),
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
