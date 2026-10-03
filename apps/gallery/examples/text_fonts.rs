//! Native rich-font comparison: --dark and --width=520 (default1040).
#[path = "../src/panel.rs"]
mod panels;
#[path = "../src/texts.rs"]
mod texts;
use gpui_kit::{
    App, AppContext, Bounds, Context, FontWeight, HighlightStyle, IntoElement, ParentElement,
    Render, Styled, StyledText, Window, WindowBounds, WindowOptions, div, px, size,
};
use gpui_kumo::{Appearance, Text, set_appearance};
use panels::panel;

struct Review {
    traced: bool,
}
impl Render for Review {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = gpui_kumo::theme(cx);
        let mut root = div()
            .size_full()
            .p(px(32.))
            .flex()
            .flex_col()
            .gap(px(24.))
            .bg(theme.colors.canvas)
            .text_color(theme.text.default)
            .font_family(theme.typography.font_family.clone())
            .text_size(theme.typography.base.size)
            .line_height(theme.typography.base.line_height)
            .child(texts::panel(theme));
        for family in ["DejaVu Sans", "Liberation Sans", "DejaVu Serif"] {
            if !self.traced {
                let make_run = |len, weight| {
                    let mut font = gpui_kit::font(family);
                    font.weight = weight;
                    gpui_kit::TextRun {
                        len,
                        font,
                        color: theme.text.default,
                        background_color: None,
                        underline: None,
                        strikethrough: None,
                    }
                };
                let mixed = window.text_system().shape_line(
                    "Rich inline emphasis".into(),
                    px(14.),
                    &[
                        make_run(12, FontWeight::NORMAL),
                        make_run(8, FontWeight::SEMIBOLD),
                    ],
                    None,
                );
                let plain = window.text_system().shape_line(
                    "emphasis".into(),
                    px(14.),
                    &[make_run(8, FontWeight::SEMIBOLD)],
                    None,
                );
                let highlighted = mixed
                    .runs
                    .iter()
                    .flat_map(|run| {
                        run.glyphs
                            .iter()
                            .filter(|glyph| glyph.index >= 12)
                            .map(move |glyph| (run.font_id, glyph))
                    })
                    .collect::<Vec<_>>();
                let expected = plain
                    .runs
                    .iter()
                    .flat_map(|run| run.glyphs.iter().map(move |glyph| (run.font_id, glyph)))
                    .collect::<Vec<_>>();
                assert_eq!(highlighted.len(), expected.len(), "{family} glyph count");
                let offset = highlighted[0].1.position.x;
                for ((actual_font, actual), (expected_font, expected)) in
                    highlighted.iter().zip(&expected)
                {
                    assert_eq!(
                        actual_font, expected_font,
                        "{family} face changed in rich run"
                    );
                    assert_eq!(actual.id, expected.id, "{family} glyph changed in rich run");
                    assert!(
                        (f32::from(actual.position.x - offset - expected.position.x)).abs() < 0.01,
                        "{family} glyph advance changed in rich run"
                    );
                }
                println!("FONT {family}: mixed={mixed:?}; plain={plain:?}");
            }
            root = root.child(
                panel(theme, family).child(
                    div()
                        .font_family(family)
                        .flex()
                        .flex_col()
                        .gap(px(8.))
                        .child(Text::new((family, 0usize), "Rich inline emphasis"))
                        .child(
                            div()
                                .font_weight(FontWeight::SEMIBOLD)
                                .child(Text::new((family, 1usize), "Rich inline emphasis")),
                        )
                        .child(
                            Text::new((family, 2usize), "Rich inline emphasis").rich_content(
                                StyledText::new("Rich inline emphasis").with_highlights([(
                                    12..20,
                                    HighlightStyle {
                                        font_weight: Some(FontWeight::SEMIBOLD),
                                        ..Default::default()
                                    },
                                )]),
                            ),
                        ),
                ),
            );
        }
        self.traced = true;
        root
    }
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let width = args
        .iter()
        .find_map(|arg| arg.strip_prefix("--width=")?.parse::<f32>().ok())
        .unwrap_or(1040.);
    let dark = args.iter().any(|arg| arg == "--dark");
    gpui_kit::application().run(move |cx: &mut App| {
        gpui_kumo::init(cx);
        set_appearance(
            if dark {
                Appearance::Dark
            } else {
                Appearance::Light
            },
            cx,
        );
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(width), px(1850.)),
                    cx,
                ))),
                ..Default::default()
            },
            |_, cx| cx.new(|_| Review { traced: false }),
        )
        .expect("open text font review");
        cx.activate(true);
    });
}
