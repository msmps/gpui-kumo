use super::*;
use gpui_kit::{Context, Render, TestAppContext, test::TestWindowExt};
struct Harness {
    selected: Option<u32>,
    changes: Vec<u32>,
    disabled: bool,
    apply: bool,
    reversed: bool,
    navigation_events: usize,
    activation_events: usize,
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let owner = cx.entity().downgrade();
        let mut group = RadioGroup::new("group", "Page size", self.selected)
            .disabled(self.disabled)
            .on_change(move |value, event, _, cx| {
                let _ = owner.update(cx, |this, cx| {
                    match event {
                        ChangeEvent::Activation(_) => this.activation_events += 1,
                        ChangeEvent::Navigation(_) => this.navigation_events += 1,
                    };
                    this.changes.push(value);
                    if this.apply {
                        this.selected = Some(value);
                    }
                    cx.notify();
                });
            });
        for (id, value, label, disabled) in if self.reversed {
            vec![
                ("three", 30, "Thirty", false),
                ("two", 20, "Twenty", true),
                ("one", 10, "Ten", false),
            ]
        } else {
            vec![
                ("one", 10, "Ten", false),
                ("two", 20, "Twenty", true),
                ("three", 30, "Thirty", false),
            ]
        } {
            group = group.item(RadioItem::new(id, value, label).disabled(disabled));
        }
        div()
            .tab_group()
            .flex()
            .flex_col()
            .w(px(300.))
            .child(crate::Button::new("before", "Before"))
            .child(group)
            .child(crate::Button::new("after", "After"))
    }
}
#[gpui_kit::test]
fn typed_selection_and_roving_focus_skip_disabled_and_exit_group(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|_, _| Harness {
        selected: Some(10),
        changes: Vec::new(),
        disabled: false,
        apply: true,
        reversed: false,
        navigation_events: 0,
        activation_events: 0,
    });
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.click("one", cx);
        assert!(view.read(cx).changes.is_empty());
        assert_eq!(window.find("one").focused(), Some(true));
        window.press("down", cx);
        window.render_frame(cx);
        assert_eq!(view.read(cx).changes, vec![30]);
        assert_eq!(window.find("three").focused(), Some(true));
        assert_eq!(window.find("three").checked(), Some(true));
        window.press("right", cx);
        window.render_frame(cx);
        assert_eq!(view.read(cx).changes, vec![30, 10]);
        window.press("end", cx);
        window.render_frame(cx);
        assert_eq!(view.read(cx).changes, vec![30, 10, 30]);
        window.press("space", cx);
        window.render_frame(cx);
        assert_eq!(view.read(cx).changes.len(), 3);
        window.focus_next(cx);
        window.render_frame(cx);
        assert_eq!(
            window.find("after").focused(),
            Some(true),
            "focus: {:?}",
            ["before", "one", "two", "three", "after"].map(|id| (id, window.find(id).focused()))
        );
        window.click("two", cx);
        assert_eq!(view.read(cx).changes.len(), 3);
        window.click("one", cx);
        window.render_frame(cx);
        view.update(cx, |this, cx| {
            this.reversed = true;
            cx.notify();
        });
        window.render_frame(cx);
        assert_eq!(window.find("one").focused(), Some(true));
        crate::set_appearance(crate::Appearance::Dark, cx);
        window.render_frame(cx);
        assert_eq!(window.find("one").checked(), Some(true));
        view.update(cx, |this, cx| {
            this.disabled = true;
            cx.notify();
        });
        window.render_frame(cx);
        window.press("space", cx);
        window.press("down", cx);
        window.click("three", cx);
        assert_eq!(view.read(cx).changes.len(), 4);
        view.update(cx, |this, cx| {
            this.disabled = false;
            this.apply = false;
            cx.notify();
        });
        window.render_frame(cx);
        window.click("three", cx);
        window.render_frame(cx);
        window.press("space", cx);
        assert_eq!(view.read(cx).selected, Some(10));
        assert_eq!(view.read(cx).changes.len(), 6);
        assert_eq!(view.read(cx).navigation_events, 3);
        assert_eq!(view.read(cx).activation_events, 3);
    });
}

struct Geometry {
    width: f32,
    position: ControlPosition,
}
impl Render for Geometry {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .w(px(self.width))
            .flex()
            .flex_col()
            .gap(px(20.))
            .child(
                RadioGroup::new("cards", "Plans", Some(1u32))
                    .appearance(Appearance::Card)
                    .orientation(Orientation::Horizontal)
                    .control_position(self.position)
                    .item(
                        RadioItem::new("card", 1, "A long label").description("Details wrap here"),
                    )
                    .item(RadioItem::new("other", 2, "Other").description("More")),
            )
            .child(
                RadioGroup::new("inline", "Inline", Some(1u32))
                    .item(RadioItem::new("inline-item", 1, "Inline label").description("Hidden")),
            )
    }
}
#[gpui_kit::test]
fn card_geometry_corner_paint_and_whole_item_hover_follow_source(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|_, _| Geometry {
        width: 400.,
        position: ControlPosition::End,
    });
    cx.update(|window, cx| {
        for width in [400., 180.] {
            for position in [ControlPosition::Start, ControlPosition::End] {
                for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
                    view.update(cx, |view, cx| {
                        view.width = width;
                        view.position = position;
                        cx.notify();
                    });
                    crate::set_appearance(appearance, cx);
                    window.render_frame(cx);
                    let card = window.find("card").bounds();
                    let other = window.find("other").bounds();
                    assert!((f32::from(card.size.width - (px(width) - px(12.)) / 2.)).abs() <= 0.5);
                    assert_eq!(other.left() - card.right(), px(12.));
                    let scope = window.within("card");
                    let label = scope.find("label").bounds();
                    let indicator = scope.find("indicator").bounds();
                    assert_eq!(indicator.top() - label.top(), px(2.));
                    assert_eq!(scope.find("dot").bounds().center(), indicator.center());
                    assert_eq!(label.top() - card.top(), px(13.));
                    if position == ControlPosition::Start {
                        assert_eq!(indicator.left() - card.left(), px(13.));
                        assert_eq!(label.left() - indicator.right(), px(12.));
                    } else {
                        assert_eq!(card.right() - indicator.right(), px(13.));
                        assert_eq!(indicator.left() - label.right(), px(12.));
                    }
                    let scale = window.scale_factor();
                    let corner = window
                        .painted_quads()
                        .into_iter()
                        .find(|q| {
                            q.bounds == card.scale(scale)
                                && q.border_color == theme(cx).colors.interact
                        })
                        .unwrap();
                    assert_eq!(
                        corner.corner_radii,
                        gpui_kit::Corners::all(theme(cx).radii.lg.scale(scale))
                    );
                    assert!(
                        window
                            .painted_quads()
                            .into_iter()
                            .any(|q| q.background.as_solid() == Some(theme(cx).colors.tint)
                                && q.bounds.contains(&card.center().scale(scale))),
                        "selected card must paint its tint surface"
                    );
                    assert!(
                        window
                            .within("inline-item")
                            .try_find("item-description")
                            .is_none()
                    );
                    window.blur(cx);
                    window.within("card").hover("label", cx);
                    window.render_frame(cx);
                    let ring = window
                        .painted_quads()
                        .into_iter()
                        .find(|q| {
                            q.bounds == indicator.dilate(px(2.)).scale(scale)
                                && q.border_color == theme(cx).colors.hairline
                        })
                        .unwrap();
                    assert_eq!(
                        ring.corner_radii,
                        gpui_kit::Corners::all(px(10.).scale(scale))
                    );
                }
            }
        }
    });
}

#[test]
#[should_panic(expected = "Radio item IDs and values must be unique")]
fn duplicate_typed_values_are_rejected() {
    let _ = RadioGroup::new("g", "Group", Some(1u32))
        .item(RadioItem::new("one", 1, "First"))
        .item(RadioItem::new("two", 1, "Duplicate"));
}

#[gpui_kit::test]
fn empty_or_unknown_selection_enters_first_enabled_and_disabled_group_is_skipped(
    cx: &mut TestAppContext,
) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|_, _| Harness {
        selected: None,
        changes: Vec::new(),
        disabled: false,
        apply: true,
        reversed: false,
        navigation_events: 0,
        activation_events: 0,
    });
    cx.update(|window, cx| {
        for selected in [None, Some(999)] {
            view.update(cx, |view, cx| {
                view.selected = selected;
                cx.notify();
            });
            window.render_frame(cx);
            window.click("before", cx);
            window.focus_next(cx);
            window.render_frame(cx);
            assert_eq!(window.find("one").focused(), Some(true));
            assert_eq!(window.find("one").checked(), Some(false));
            window.focus_next(cx);
            window.render_frame(cx);
            assert_eq!(window.find("after").focused(), Some(true));
            assert!(view.read(cx).changes.is_empty());
        }
        view.update(cx, |view, cx| {
            view.disabled = true;
            cx.notify();
        });
        window.render_frame(cx);
        window.click("before", cx);
        window.focus_next(cx);
        window.render_frame(cx);
        assert_eq!(window.find("after").focused(), Some(true));
    });
}
