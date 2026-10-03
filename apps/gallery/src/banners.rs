use gpui_kit::{
    Context, IntoElement, ParentElement, Render, Styled, Subscription, Window, div, px,
};
use gpui_kumo::{
    Banner, Button, Icon, Link, Text,
    banner::{Action, ActionVariant, Size, Variant},
    theme,
};

pub struct Banners {
    activations: usize,
    disabled: bool,
    loading: bool,
    dismissed: bool,
    _theme: Subscription,
}

impl Banners {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            activations: 0,
            disabled: false,
            loading: false,
            dismissed: false,
            _theme: cx.observe_global::<gpui_kumo::Theme>(|_, cx| cx.notify()),
        }
    }
}

impl Render for Banners {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut panel = super::panel(theme(cx), "Banner · contextual messages and actions");
        for (variant, id, title) in [
            (Variant::Info, "info", "Update available"),
            (Variant::Alert, "alert", "Review required"),
            (Variant::Error, "error", "Save failed"),
            (Variant::Secondary, "secondary", "Maintenance scheduled"),
        ] {
            panel = panel.child(
                Banner::new(id)
                    .variant(variant)
                    .title(title)
                    .description("Contextual description — café 🦀 — with three action treatments.")
                    .icon(Icon::new("workspace.svg"))
                    .action(
                        Action::new("primary", "Continue")
                            .disabled(self.disabled)
                            .loading(self.loading)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.activations += 1;
                                cx.notify();
                            })),
                    )
                    .action(
                        Action::new("outline", "Details")
                            .variant(ActionVariant::Secondary)
                            .disabled(self.disabled)
                            .loading(self.loading)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.activations += 1;
                                cx.notify();
                            })),
                    )
                    .action(
                        Action::new("ghost", "Later")
                            .variant(ActionVariant::Ghost)
                            .disabled(self.disabled)
                            .loading(self.loading)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.activations += 1;
                                cx.notify();
                            })),
                    ),
            );
        }
        panel = panel.child(
            Banner::new("compact-link")
                .size(Size::Sm)
                .title("Compact message")
                .description("Learn about this update.")
                .link_action(
                    Link::new("learn", "Documentation", "app://documentation").on_navigate(
                        cx.listener(|this, _: &gpui_kumo::link::NavigationRequest, _, cx| {
                            this.activations += 1;
                            cx.notify();
                        }),
                    ),
                ),
        );
        if !self.dismissed {
            panel = panel.child(
                Banner::new("dismissible")
                    .size(Size::Sm)
                    .variant(Variant::Alert)
                    .description("Dismissal is owned by the gallery.")
                    .action(
                        Action::icon(
                            "dismiss",
                            "Dismiss message",
                            Icon::new("workspace.svg").size(px(10.)),
                        )
                        .variant(ActionVariant::Ghost)
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.dismissed = true;
                            cx.notify();
                        })),
                    ),
            );
        }
        panel
            .child(
                div()
                    .flex()
                    .gap(px(8.))
                    .child(
                        Button::new(
                            "toggle-disabled",
                            if self.disabled {
                                "Enable banner actions"
                            } else {
                                "Disable banner actions"
                            },
                        )
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.disabled = !this.disabled;
                            cx.notify();
                        })),
                    )
                    .child(
                        Button::new(
                            "toggle-loading",
                            if self.loading {
                                "Stop banner loading"
                            } else {
                                "Start banner loading"
                            },
                        )
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.loading = !this.loading;
                            cx.notify();
                        })),
                    ),
            )
            .child(Text::new(
                "action-count",
                format!("{} banner activations", self.activations),
            ))
    }
}
