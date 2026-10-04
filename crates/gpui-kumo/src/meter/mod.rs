//! Controlled Kumo measurements with native Meter semantics and Base parts.
#![deny(missing_docs)]

use crate::{Text, text, theme};
use base::TestSupportExt;
use gpui_kit::{
    App, ElementId, FontFeatures, InteractiveElement, IntoElement, ParentElement, Refineable,
    RenderOnce, Role, SharedString, StatefulInteractiveElement, StyleRefinement, Styled, Window,
    base, div, prelude::FluentBuilder, relative,
};
use std::{ops::RangeInclusive, sync::Arc, time::Duration};

/// Current owner-supplied measurement and source-normalized outputs.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MeterValue {
    /// Original owner value, before clamping.
    pub raw: f64,
    /// Range-clamped value exposed to accessibility.
    pub value: f64,
    /// Inclusive range start.
    pub min: f64,
    /// Inclusive range end.
    pub max: f64,
    /// Source-normalized percentage in0–100.
    pub percentage: f64,
}
impl MeterValue {
    fn new(raw: f64, min: f64, max: f64) -> Self {
        let percentage = ((raw - min) * 100.) / (max - min);
        Self {
            raw,
            min,
            max,
            value: if raw.is_nan() {
                min
            } else {
                raw.clamp(min, max)
            },
            percentage: if percentage.is_nan() {
                0.
            } else {
                percentage.clamp(0., 100.)
            },
        }
    }
}
/// Semantic indicator fills, including Kumo's success-style example.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IndicatorColor {
    #[default]
    /// Default brand fill.
    Brand,
    /// Success surface fill.
    Success,
    /// Warning surface fill.
    Warning,
    /// Danger surface fill.
    Danger,
}
type Formatter = Box<dyn Fn(MeterValue) -> SharedString>;
type ValueText = Box<dyn Fn(&str, MeterValue) -> SharedString>;

/// A read-only measurement. Owner values update semantics immediately;
/// Base retains only the source 300ms width transition per stable unique ID.
#[derive(IntoElement)]
#[must_use]
pub struct Meter {
    id: ElementId,
    label: SharedString,
    accessible_name: Option<SharedString>,
    value: f64,
    range: RangeInclusive<f64>,
    custom_value: Option<SharedString>,
    show_value: bool,
    color: IndicatorColor,
    format: Option<Formatter>,
    value_text: Option<ValueText>,
    track_style: StyleRefinement,
    indicator_style: StyleRefinement,
}
impl Meter {
    /// Create a named measurement with a unique stable ID and default0–100 range.
    /// Set visible text and its default accessible name; explicit overrides remain authoritative.
    ///
    /// # Panics
    /// Panics when `text` is blank.
    pub fn label(mut self, text: impl Into<SharedString>) -> Self {
        self.label = crate::name::nonblank(text);
        self
    }
    /// Override the accessible name independently of visible text and builder order.
    ///
    /// # Panics
    /// Panics when `name` is blank.
    pub fn accessibility_label(mut self, name: impl Into<SharedString>) -> Self {
        self.accessible_name = Some(crate::name::nonblank(name));
        self
    }
    /// Create a named measurement with an application-owned raw value.
    ///
    /// # Panics
    /// Panics when the required name or label is blank.
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>, value: f64) -> Self {
        let label = label.into();
        assert!(!label.trim().is_empty(), "Meter requires a nonempty label");
        Self {
            id: id.into(),
            label,
            accessible_name: None,
            value,
            range: 0.0..=100.0,
            custom_value: None,
            show_value: true,
            color: IndicatorColor::Brand,
            format: None,
            value_text: None,
            track_style: StyleRefinement::default(),
            indicator_style: StyleRefinement::default(),
        }
    }
    /// Set a finite ordered range. Equal endpoints follow the source normalization.
    ///
    /// # Panics
    /// Panics when endpoints are nonfinite or the range is reversed.
    pub fn range(mut self, range: RangeInclusive<f64>) -> Self {
        assert!(
            range.start().is_finite() && range.end().is_finite() && range.start() <= range.end(),
            "Meter range must be finite and ordered"
        );
        self.range = range;
        self
    }
    /// Visual text only, matching Kumo. Use value_text for an accessible override.
    pub fn custom_value(mut self, value: impl Into<SharedString>) -> Self {
        self.custom_value = Some(value.into());
        self
    }
    /// Show the formatted value unless a nonempty custom value takes precedence.
    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }
    /// Select an indicator fill from the shared semantic palette.
    pub fn indicator_color(mut self, color: IndicatorColor) -> Self {
        self.color = color;
        self
    }
    /// Refine the track's default presentation, equivalent to trackClassName.
    pub fn track_style(mut self, style: StyleRefinement) -> Self {
        self.track_style = style;
        self
    }
    /// Refine the fill's default presentation. Normalized width remains owner-driven.
    pub fn indicator_style(mut self, style: StyleRefinement) -> Self {
        self.indicator_style = style;
        self
    }
    /// Native equivalent of Intl formatting. Default is a whole percentage.
    pub fn format(mut self, format: impl Fn(MeterValue) -> SharedString + 'static) -> Self {
        self.format = Some(Box::new(format));
        self
    }
    /// Receives formatted text and source-normalized/raw measurement.
    pub fn value_text(mut self, text: impl Fn(&str, MeterValue) -> SharedString + 'static) -> Self {
        self.value_text = Some(Box::new(text));
        self
    }
}
impl RenderOnce for Meter {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let value = MeterValue::new(self.value, *self.range.start(), *self.range.end());
        let formatted = self.format.map_or_else(
            || format!("{}%", value.percentage.round() as u8).into(),
            |format| format(value),
        );
        let semantic = self
            .value_text
            .map_or_else(|| formatted.clone(), |text| text(&formatted, value));
        let custom = self
            .custom_value
            .as_ref()
            .is_some_and(|text| !text.is_empty());
        let visible = self
            .custom_value
            .filter(|text| !text.is_empty())
            .or_else(|| self.show_value.then_some(formatted));
        let fraction = window.with_id(self.id.clone(), |window| {
            base::transition(
                "meter-width",
                value.percentage as f32 / 100.,
                base::Transition::new(Duration::from_millis(300))
                    .ease(|p| base::Easing::EaseOut.sample(p)),
                window,
                cx,
            )
        });
        let theme = theme(cx);
        let color = match self.color {
            IndicatorColor::Brand => theme.colors.brand,
            IndicatorColor::Success => theme.colors.success,
            IndicatorColor::Warning => theme.colors.warning,
            IndicatorColor::Danger => theme.colors.danger,
        };
        div()
            .id(self.id)
            .test_support()
            .role(Role::Meter)
            .aria_label(
                self.accessible_name
                    .clone()
                    .unwrap_or_else(|| self.label.clone()),
            )
            .aria_min_numeric_value(value.min)
            .aria_max_numeric_value(value.max)
            .aria_numeric_value(value.value)
            .aria_value(semantic)
            .font_family(theme.typography.font_family.clone())
            .text_color(theme.text.default)
            .flex()
            .flex_col()
            .gap(theme.spacing.eight)
            .min_w_0()
            .w_full()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .gap(theme.spacing.sixteen)
                    .min_w_0()
                    .child(
                        div()
                            .min_w_0()
                            .line_height(theme.typography.xs.line_height)
                            .child(Text::new("label", self.label).style(text::Style::Copy {
                                tone: text::Tone::Secondary,
                                size: text::Size::Xs,
                                bold: false,
                            })),
                    )
                    .when_some(visible, |header, value| {
                        header.child(
                            div()
                                .min_w_0()
                                .line_height(theme.typography.sm.line_height)
                                .font_features(FontFeatures(Arc::new(vec![("tnum".into(), 1)])))
                                .child(if custom {
                                    Text::new("value", value)
                                        .style(text::Style::Copy {
                                            tone: text::Tone::Default,
                                            size: text::Size::Sm,
                                            bold: true,
                                        })
                                        .into_any_element()
                                } else {
                                    // Base UI Value is aria-hidden; plain strings paint
                                    // without creating an accessible child in this GPUI.
                                    div()
                                        .text_size(theme.typography.sm.size)
                                        .font_weight(gpui_kit::FontWeight::MEDIUM)
                                        .child(value)
                                        .into_any_element()
                                }),
                        )
                    }),
            )
            .child(
                base::ProgressTrack::new()
                    .relative()
                    .w_full()
                    .h(theme.spacing.eight)
                    .rounded_full()
                    .bg(theme.colors.fill)
                    .overflow_hidden()
                    .map(|mut track| {
                        track.style().refine(&self.track_style);
                        track
                    })
                    .child(
                        div()
                            .id("indicator")
                            .test_support()
                            .absolute()
                            .left_0()
                            .top_0()
                            .h_full()
                            .w(relative(fraction))
                            .child(
                                base::ProgressIndicator::new()
                                    .size_full()
                                    .rounded_full()
                                    .bg(color)
                                    .map(|mut indicator| {
                                        indicator.style().refine(&self.indicator_style);
                                        indicator
                                    }),
                            ),
                    ),
            )
    }
}

impl std::fmt::Debug for Meter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Meter")
            .field("id", &self.id)
            .field("label", &self.label)
            .field("show_value", &self.show_value)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests;
