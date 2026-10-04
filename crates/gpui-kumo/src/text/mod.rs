//! Read-only typography from the pinned Kumo Text recipe.
//! Visual heading size and semantic heading level are independent.

#![deny(missing_docs)]

use gpui_kit::base::TestSupportExt;
use gpui_kit::{
    AnyElement, App, ElementId, FontWeight, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, Role, SharedString, StatefulInteractiveElement, Styled, Window, div,
};

use crate::theme;

/// Tone of copy text. Success intentionally uses Kumo's link color.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Tone {
    /// Primary readable text.
    #[default]
    Default,
    /// Supporting text.
    Secondary,
    /// Success text, authored with the link token upstream.
    Success,
    /// Error text.
    Error,
}

/// Kumo's four copy sizes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Size {
    /// 12px.
    Xs,
    /// 13px.
    Sm,
    /// 14px.
    #[default]
    Base,
    /// 16px.
    Lg,
}

/// Heading presentation, independent of document hierarchy.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HeadingSize {
    /// Standard heading: 16px / 24px.
    #[default]
    Standard,
    /// Modern large heading: 20px / 28px.
    Large,
}

/// Monospace tone.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MonoTone {
    /// Primary readable text.
    #[default]
    Default,
    /// Supporting text.
    Secondary,
}

/// Monospace sizes are optically smaller than corresponding copy.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MonoSize {
    /// 13px.
    #[default]
    Standard,
    /// 14px.
    Large,
}

/// Valid typography combinations. Only copy supports the upstream bold option.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Style {
    /// Copy inherits line height and, unless bold, font weight from its parent.
    Copy {
        /// Semantic foreground tone.
        tone: Tone,
        /// Copy font size.
        size: Size,
        /// Kumo's "bold" copy uses medium (500), not weight 700.
        bold: bool,
    },
    /// Semibold heading presentation with its authored line height.
    Heading(HeadingSize),
    /// Monospace inherits line height and font weight from its parent.
    Mono {
        /// Default or supporting foreground.
        tone: MonoTone,
        /// Optically adjusted font size.
        size: MonoSize,
    },
}

impl Default for Style {
    fn default() -> Self {
        Self::Copy {
            tone: Tone::Default,
            size: Size::Base,
            bold: false,
        }
    }
}

/// Semantic heading hierarchy; styling never implies a level.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeadingLevel {
    /// First level.
    One = 1,
    /// Second level.
    Two,
    /// Third level.
    Three,
    /// Fourth level.
    Four,
    /// Fifth level.
    Five,
    /// Sixth level.
    Six,
}

/// Consumed read-only text with stable identity and full accessible content.
/// The owning view observes Theme when retaining cached presentation.
///
/// ```
/// use gpui_kumo::{Text, text::{HeadingLevel, HeadingSize, Style}};
/// let title = Text::new("page-title", "Project settings")
///     .style(Style::Heading(HeadingSize::Large))
///     .heading_level(HeadingLevel::One);
/// ```
#[derive(IntoElement)]
#[must_use]
pub struct Text {
    id: ElementId,
    content: SharedString,
    rich_content: Option<AnyElement>,
    style: Style,
    heading_level: Option<HeadingLevel>,
    truncate: bool,
}

impl Text {
    /// Create default copy. IDs must be unique within their ancestor scope.
    pub fn new(id: impl Into<ElementId>, content: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            content: content.into(),
            rich_content: None,
            style: Style::default(),
            heading_level: None,
            truncate: false,
        }
    }

    /// Select a valid copy, heading or monospace presentation.
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Replace visual content with decorative inline composition, such as
    /// GPUI `StyledText`. The string passed to `new` remains the complete
    /// accessible text. Keep interactive controls outside this component.
    pub fn rich_content(mut self, content: impl IntoElement) -> Self {
        self.rich_content = Some(content.into_any_element());
        self
    }

    /// Opt into heading semantics without changing visual presentation.
    pub fn heading_level(mut self, level: HeadingLevel) -> Self {
        self.heading_level = Some(level);
        self
    }

    /// Use one clipped line with ellipsis. Accessible content remains complete.
    pub fn truncate(mut self, truncate: bool) -> Self {
        self.truncate = truncate;
        self
    }
}

impl RenderOnce for Text {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = theme(cx);
        let mut element = div()
            .id(self.id)
            .test_support()
            .role(Role::Label)
            .aria_label(self.content.clone())
            .text_color(theme.text.default)
            .min_w_0();
        match self.style {
            Style::Copy { tone, size, bold } => {
                let size = match size {
                    Size::Xs => theme.typography.xs.size,
                    Size::Sm => theme.typography.sm.size,
                    Size::Base => theme.typography.base.size,
                    Size::Lg => theme.typography.lg.size,
                };
                let color = match tone {
                    Tone::Default => theme.text.default,
                    Tone::Secondary => theme.text.subtle,
                    Tone::Success => theme.text.link,
                    Tone::Error => theme.text.danger,
                };
                element = element.text_size(size).text_color(color);
                if bold {
                    element = element.font_weight(FontWeight::MEDIUM);
                }
            }
            Style::Heading(size) => {
                let style = match size {
                    HeadingSize::Standard => theme.typography.heading,
                    HeadingSize::Large => theme.typography.heading_lg,
                };
                element = element
                    .text_size(style.size)
                    .line_height(style.line_height)
                    .font_weight(style.weight);
            }
            Style::Mono { tone, size } => {
                element = element
                    .font_family(theme.typography.mono_font_family.clone())
                    .text_size(match size {
                        MonoSize::Standard => theme.typography.sm.size,
                        MonoSize::Large => theme.typography.base.size,
                    })
                    .text_color(match tone {
                        MonoTone::Default => theme.text.default,
                        MonoTone::Secondary => theme.text.subtle,
                    });
            }
        }
        if let Some(level) = self.heading_level {
            element = element.role(Role::Heading).aria_level(level as usize);
        } else {
            // AccessKit derives a Label's platform name from its readable value.
            element = element.aria_value(self.content.clone());
        }
        if self.truncate {
            element = element.truncate();
        }
        element.child(
            self.rich_content
                .unwrap_or_else(|| self.content.into_any_element()),
        )
    }
}

debug_struct!(Text { id, truncate });

#[cfg(test)]
mod tests;
