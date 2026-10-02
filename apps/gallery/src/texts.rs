use gpui_kit::{Div, FontWeight, HighlightStyle, ParentElement, Styled, StyledText, div, px};
use gpui_kumo::{
    Text, Theme,
    text::{HeadingLevel, HeadingSize, MonoSize, MonoTone, Size, Style, Tone},
};

pub(super) fn panel(theme: &Theme) -> Div {
    super::panel(theme, "Text")
        .child(Text::new("rich", "Rich inline emphasis").rich_content(
            StyledText::new("Rich inline emphasis").with_highlights([(
                12..20,
                HighlightStyle {
                    font_weight: Some(FontWeight::SEMIBOLD),
                    ..Default::default()
                },
            )]),
        ))
        .child(
            Text::new("semantic-heading", "Typography with heading semantics")
                .style(Style::Heading(HeadingSize::Large))
                .heading_level(HeadingLevel::Two),
        )
        .child(
            div().flex().flex_wrap().gap(px(24.)).children(
                [
                    ("heading", "Heading", Style::Heading(HeadingSize::Standard)),
                    (
                        "heading-large",
                        "Heading large",
                        Style::Heading(HeadingSize::Large),
                    ),
                    ("body", "Body", Style::default()),
                    (
                        "bold",
                        "Body bold (500)",
                        Style::Copy {
                            tone: Tone::Default,
                            size: Size::Base,
                            bold: true,
                        },
                    ),
                    (
                        "xs",
                        "Body xs (12px)",
                        Style::Copy {
                            tone: Tone::Default,
                            size: Size::Xs,
                            bold: false,
                        },
                    ),
                    (
                        "sm",
                        "Body sm (13px)",
                        Style::Copy {
                            tone: Tone::Default,
                            size: Size::Sm,
                            bold: false,
                        },
                    ),
                    (
                        "lg",
                        "Body lg (16px)",
                        Style::Copy {
                            tone: Tone::Default,
                            size: Size::Lg,
                            bold: false,
                        },
                    ),
                    (
                        "secondary",
                        "Secondary",
                        Style::Copy {
                            tone: Tone::Secondary,
                            size: Size::Base,
                            bold: false,
                        },
                    ),
                    (
                        "success",
                        "Success uses link",
                        Style::Copy {
                            tone: Tone::Success,
                            size: Size::Base,
                            bold: false,
                        },
                    ),
                    (
                        "error",
                        "Error",
                        Style::Copy {
                            tone: Tone::Error,
                            size: Size::Base,
                            bold: false,
                        },
                    ),
                    (
                        "mono",
                        "Monospace (13px)",
                        Style::Mono {
                            tone: MonoTone::Default,
                            size: MonoSize::Standard,
                        },
                    ),
                    (
                        "mono-lg",
                        "Monospace (14px)",
                        Style::Mono {
                            tone: MonoTone::Default,
                            size: MonoSize::Large,
                        },
                    ),
                    (
                        "mono-secondary",
                        "Mono secondary",
                        Style::Mono {
                            tone: MonoTone::Secondary,
                            size: MonoSize::Standard,
                        },
                    ),
                ]
                .map(|(id, label, style)| {
                    div().w(px(220.)).child(Text::new(id, label).style(style))
                }),
            ),
        )
        .child(
            div()
                .w(px(220.))
                .flex()
                .flex_col()
                .gap(px(8.))
                .child(
                    Text::new(
                        "truncated",
                        "café 🦀 — long content keeps its complete accessible name",
                    )
                    .truncate(true),
                )
                .child(Text::new(
                    "wrapped",
                    "café 🦀 — long content wraps to the available width",
                )),
        )
}
