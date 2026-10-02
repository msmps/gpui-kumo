use super::*;
use crate::{Button, Input, InputState, Text};

#[gpui_kit::test]
fn secondary_background_does_not_cover_the_roots_rounded_top_corners(cx: &mut TestAppContext) {
    struct Corner;
    impl Render for Corner {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            LayerCard::new("card")
                .w(px(200.))
                .section(Section::secondary("header").child(Text::new("title", "Header")))
                .section(Section::primary("body").child(Text::new("copy", "Body")))
        }
    }
    cx.update(crate::init);
    let (_, cx) = cx.add_window_view(|_, _| Corner);
    cx.update(|window, cx| {
        window.render_frame(cx);
        let root = window.find("card").bounds();
        let sample = (root.origin + gpui_kit::point(px(1.), px(1.))).scale(window.scale_factor());
        let elevated = crate::theme(cx).colors.elevated;
        assert!(
            !window.painted_quads().iter().any(|quad| {
                quad.background.as_solid() == Some(elevated)
                    && quad.bounds.contains(&sample)
                    && quad.content_mask.bounds.contains(&sample)
                    && quad.corner_radii.top_left == px(0.).scale(window.scale_factor())
            }),
            "The secondary fill covers the root's rounded top corner with a square quad"
        );
    });
}
#[gpui_kit::test]
fn secondary_fills_follow_root_radius_and_section_bounds_in_both_themes(cx: &mut TestAppContext) {
    struct Edges;
    impl Render for Edges {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            LayerCard::new("card")
                .w(px(200.))
                .rounded(px(16.))
                .bg(gpui_kit::red())
                .section(Section::secondary("header").child(Text::new("title", "Header")))
                .section(Section::primary("body").child(Text::new("copy", "Body")))
                .section(Section::secondary("footer").child(Text::new("end", "Footer")))
        }
    }
    cx.update(crate::init);
    let (_, cx) = cx.add_window_view(|_, _| Edges);
    for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
        cx.update(|window, cx| {
            crate::set_appearance(appearance, cx);
            window.refresh();
            window.render_frame(cx);
            let scale = window.scale_factor();
            let root = window.find("card").bounds().scale(scale);
            let elevated = crate::theme(cx).colors.elevated;
            let fills: Vec<_> = window
                .painted_quads()
                .into_iter()
                .filter(|quad| quad.background.as_solid() == Some(elevated))
                .collect();
            assert_eq!(fills.len(), 2, "Both secondary sections must still paint");
            for (fill, id) in fills.into_iter().zip(["header", "footer"]) {
                assert_eq!(
                    fill.bounds, root,
                    "Round relative to the actual root, not the negative section margin"
                );
                assert_eq!(fill.corner_radii, Corners::all(px(16.).scale(scale)));
                assert_eq!(
                    fill.content_mask.bounds,
                    window.find(id).bounds().scale(scale).intersect(&root)
                );
            }
        });
    }
}

use gpui_kit::{AppContext, Entity, FocusHandle, Focusable, Pixels, Role};
use gpui_kit::{Context, Render, Subscription, TestAppContext, test::TestWindowExt};

struct Form {
    input: Entity<InputState>,
    button: FocusHandle,
    activations: usize,
    disabled: bool,
    width: Pixels,
    _theme: Subscription,
}

impl Render for Form {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().id("form").tab_group().child(
            LayerCard::new("card")
                .w(self.width)
                .section(
                    Section::secondary("header").child(
                        Button::new("save", "Save")
                            .track_focus(&self.button)
                            .disabled(self.disabled)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.activations += 1;
                                cx.notify();
                            })),
                    ),
                )
                .section(
                    Section::primary("body")
                        .child(Input::new("field", &self.input))
                        .child(Text::new(
                            "copy",
                            "café 🦀 — content wraps in a narrow card",
                        )),
                ),
        )
    }
}

#[gpui_kit::test]
fn nested_controls_preserve_input_focus_and_single_activation_across_theme_and_width_changes(
    cx: &mut TestAppContext,
) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|window, cx| Form {
        input: cx.new(|cx| InputState::new("Card name", window, cx)),
        button: cx.focus_handle(),
        activations: 0,
        disabled: false,
        width: px(240.),
        _theme: cx.observe_global::<crate::Theme>(|_, cx| cx.notify()),
    });
    let input = cx.read(|cx| view.read(cx).input.clone());
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.click("save", cx);
        window.press("enter", cx);
        window.press("space", cx);
        assert_eq!(view.read(cx).activations, 3);
        window.focus_next(cx);
        assert!(input.read(cx).focus_handle(cx).is_focused(window));
        assert_eq!(window.find("control").role(), Some(Role::TextInput));
        assert_eq!(window.find("control").label(), Some("Card name"));
    });
    cx.simulate_input("café 🦀");
    cx.update(|window, cx| {
        view.update(cx, |view, cx| {
            view.width = px(120.);
            cx.notify();
        });
        crate::set_appearance(crate::Appearance::Dark, cx);
        window.render_frame(cx);
        assert_eq!(input.read(cx).value(cx).as_ref(), "café 🦀");
        assert!(input.read(cx).focus_handle(cx).is_focused(window));
        assert!(window.find("copy").bounds().size.height > px(21.));
        assert!(window.find("control").bounds().size.width <= px(92.));
        window.press("backspace", cx);
        assert_eq!(input.read(cx).value(cx).as_ref(), "café ");
        view.update(cx, |view, cx| {
            view.disabled = true;
            cx.notify();
        });
        window.render_frame(cx);
        window.click("save", cx);
        view.read(cx).button.clone().focus(window, cx);
        window.press("enter", cx);
        assert_eq!(view.read(cx).activations, 3);
    });
}

#[gpui_kit::test]
fn empty_cards_and_oversized_content_keep_layout_and_paint_boundaries(cx: &mut TestAppContext) {
    struct Edges;
    impl Render for Edges {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .flex()
                .flex_col()
                .gap(px(8.))
                .child(LayerCard::new("empty").w(px(80.)))
                .child(
                    LayerCard::new("empty-section")
                        .w(px(80.))
                        .section(Section::primary("primary")),
                )
                .child(
                    LayerCard::new("clipped").w(px(80.)).h(px(40.)).child(
                        div()
                            .w(px(200.))
                            .h(px(100.))
                            .flex_shrink_0()
                            .bg(gpui_kit::rgb(0xff00ff)),
                    ),
                )
        }
    }
    cx.update(crate::init);
    let (_, cx) = cx.add_window_view(|_, _| Edges);
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert_eq!(window.find("empty").bounds().size.height, px(0.));
        assert_eq!(window.find("empty-section").bounds().size.height, px(32.));
        let card = window.find("clipped").bounds().scale(window.scale_factor());
        let painted = window.painted_quads();
        let child = painted
            .iter()
            .find(|quad| quad.background.as_solid() == Some(gpui_kit::rgb(0xff00ff).into()))
            .expect("Oversized child paints");
        assert!(child.bounds.size.width > card.size.width);
        assert_eq!(child.content_mask.bounds, card);
    });
}

struct Harness {
    _theme: Subscription,
}

#[gpui_kit::test]
fn simple_cards_inherit_line_height_but_layered_parts_apply_the_base_recipe(
    cx: &mut TestAppContext,
) {
    struct Typography;
    impl Render for Typography {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .flex()
                .flex_col()
                .line_height(px(31.))
                .child(LayerCard::new("simple").child(Text::new("simple-copy", "Simple")))
                .child(
                    LayerCard::new("layered")
                        .w(px(200.))
                        .section(
                            Section::secondary("header")
                                .child(Text::new("secondary-copy", "Header")),
                        )
                        .section(Section::primary("body").child(Text::new("primary-copy", "Body"))),
                )
        }
    }
    cx.update(crate::init);
    let (_, cx) = cx.add_window_view(|_, _| Typography);
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert_eq!(window.find("simple-copy").bounds().size.height, px(31.));
        assert_eq!(window.find("secondary-copy").bounds().size.height, px(21.));
        assert_eq!(window.find("primary-copy").bounds().size.height, px(21.));
    });
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(px(20.))
            .child(
                LayerCard::new("simple").w(px(200.)).p(px(16.)).child(
                    div()
                        .id("simple-content")
                        .test_support()
                        .h(px(20.))
                        .w_full(),
                ),
            )
            .child(
                LayerCard::new("layered")
                    .w(px(200.))
                    .section(Section::secondary("secondary").child(div().h(px(20.))))
                    .section(
                        Section::primary("primary").child(
                            div()
                                .id("primary-content")
                                .test_support()
                                .h(px(20.))
                                .w_full(),
                        ),
                    ),
            )
            .child(
                LayerCard::new("override").w(px(200.)).section(
                    Section::primary("override-primary")
                        .p(px(4.))
                        .child(div().h(px(20.))),
                ),
            )
    }
}

#[gpui_kit::test]
fn sections_select_layered_layout_without_ring_insets_and_caller_padding_wins(
    cx: &mut TestAppContext,
) {
    cx.update(crate::init);
    let (_, cx) = cx.add_window_view(|_, cx| Harness {
        _theme: cx.observe_global::<crate::Theme>(|_, cx| cx.notify()),
    });
    cx.update(|window, cx| {
        for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
            crate::set_appearance(appearance, cx);
            window.render_frame(cx);
            let simple = window.find("simple").bounds();
            assert_eq!(simple.size.width, px(200.));
            assert_eq!(simple.size.height, px(52.));
            assert_eq!(window.find("simple-content").bounds().size.width, px(168.));
            let layered = window.find("layered").bounds();
            assert_eq!(layered.size.height, px(88.));
            assert_eq!(
                window.find("secondary").bounds().origin.y,
                layered.origin.y - px(8.)
            );
            assert_eq!(window.find("primary-content").bounds().size.width, px(172.));
            assert_eq!(
                window.find("override-primary").bounds().size.height,
                px(28.)
            );
            let base = crate::theme(cx).colors.base;
            assert!(
                window
                    .painted_quads()
                    .iter()
                    .any(|quad| quad.background.as_solid() == Some(base))
            );
        }
    });
}
