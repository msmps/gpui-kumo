use gpui_kit::{
    Context, IntoElement, ParentElement, Render, StyleRefinement, Styled, Window, div, px,
};
use gpui_kumo::{Button, Meter, meter::IndicatorColor, theme};

pub struct Meters {
    value: f64,
}
impl Default for Meters {
    fn default() -> Self {
        Self { value: 65. }
    }
}
impl Render for Meters {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = theme(cx);
        crate::panel(theme, "Meter · measured quotas and owner-driven updates")
            .child(Meter::new("meter-storage", "Storage used", self.value))
            .child(
                Meter::new("meter-styled", "Caller-styled track and fill", self.value)
                    .track_style(
                        StyleRefinement::default()
                            .h(px(12.))
                            .rounded(px(4.))
                            .bg(theme.colors.tint),
                    )
                    .indicator_style(
                        StyleRefinement::default()
                            .rounded(px(4.))
                            .bg(theme.colors.success),
                    ),
            )
            .child(
                Meter::new("meter-requests", "API requests", 750.)
                    .range(0.0..=1000.0)
                    .custom_value("750 / 1,000")
                    .indicator_color(IndicatorColor::Success),
            )
            .child(Meter::new("meter-hidden", "Label without visible value", 40.).show_value(false))
            .child(Meter::new("meter-empty", "Empty allocation", 0.))
            .child(Meter::new("meter-tiny", "Tiny allocation", 0.5))
            .child(
                Meter::new("meter-full", "Quota reached", 100.)
                    .indicator_color(IndicatorColor::Danger),
            )
            .child(
                Meter::new(
                    "meter-long",
                    "Long Unicode quota label · archive café 🦀 retained for a regional account",
                    42.,
                )
                .custom_value("42 / 100 GB retained across regional archives"),
            )
            .child(
                Meter::new("meter-format", "Temperature (native formatter)", 27.)
                    .range(-20.0..=40.0)
                    .indicator_color(IndicatorColor::Warning)
                    .format(|value| format!("{:.0} °C", value.value).into())
                    .value_text(|formatted, _| format!("Temperature {formatted}").into()),
            )
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap(theme.spacing.eight)
                    .child(
                        Button::new("meter-decrease", "Decrease").on_click(cx.listener(
                            |this, _, _, cx| {
                                this.value = (this.value - 25.).max(0.);
                                cx.notify();
                            },
                        )),
                    )
                    .child(
                        Button::new("meter-increase", "Increase").on_click(cx.listener(
                            |this, _, _, cx| {
                                this.value = (this.value + 25.).min(100.);
                                cx.notify();
                            },
                        )),
                    )
                    .child(
                        Button::new("meter-reduced", "Toggle reduced motion").on_click(
                            |_, _, cx| {
                                cx.set_reduce_motion(!cx.reduce_motion());
                            },
                        ),
                    ),
            )
    }
}
