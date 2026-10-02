use super::*;
use crate::{
    Appearance,
    button::{Size, Variant},
    theme,
};
use gpui_kit::{Bounds, Context, Corners, Render, TestAppContext, test::TestWindowExt};
struct Harness {
    changes: Vec<&'static str>,
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let a = cx.entity().downgrade();
        let b = a.clone();
        div()
            .tab_group()
            .flex()
            .flex_col()
            .w(px(400.))
            .child(
                ButtonGroup::new("joined", "Actions")
                    .item(Button::new("first", "Save").on_click(move |_, _, cx| {
                        let _ = a.update(cx, |this, cx| {
                            this.changes.push("save");
                            cx.notify();
                        });
                    }))
                    .item(Button::new("middle", "Disabled").disabled(true))
                    .item(Button::new("last", "More").on_click(move |_, _, cx| {
                        let _ = b.update(cx, |this, cx| {
                            this.changes.push("more");
                            cx.notify();
                        });
                    })),
            )
            .child(
                ButtonGroup::new("one", "One").item(
                    Button::new("solo", "Solo")
                        .size(Size::Sm)
                        .variant(Variant::Primary),
                ),
            )
            .child(ButtonGroup::new("empty", "Empty"))
    }
}
#[gpui_kit::test]
fn joins_have_source_corners_and_overlap_and_callbacks_remain_independent(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|_, _| Harness { changes: vec![] });
    cx.update(|window, cx| {
        for appearance in [Appearance::Light, Appearance::Dark] {
            crate::set_appearance(appearance, cx);
            window.render_frame(cx);
            let first = window.find("first").bounds();
            let middle = window.find("middle").bounds();
            let last = window.find("last").bounds();
            assert_eq!(middle.left(), first.right() - px(1.));
            assert_eq!(last.left(), middle.right() - px(1.));
            assert_eq!(
                window.find("joined").bounds().size.width,
                first.size.width + middle.size.width + last.size.width - px(2.)
            );
            // Translucent disabled/loading fills must not expose two overlapping
            // outside borders. Each join is one opaque source-role divider.
            let quads = window.painted_quads();
            for (bounds, opacity) in [(middle, 0.5), (last, 1.)] {
                let seam = Bounds::new(bounds.origin, gpui_kit::size(px(1.), bounds.size.height))
                    .scale(window.scale_factor());
                let dividers: Vec<_> = quads.iter().filter(|q| q.bounds == seam).collect();
                assert_eq!(dividers.len(), 1, "one divider at each join");
                let color = dividers[0].background.as_solid().unwrap();
                assert_eq!(
                    color,
                    theme(cx).colors.line.opacity(opacity),
                    "divider retains child disabled opacity"
                );
            }
            assert_eq!(window.find("joined").role(), Some(Role::Group));
            assert_eq!(window.find("joined").label(), Some("Actions"));
            for (id, start, end) in [
                ("first", true, false),
                ("middle", false, false),
                ("last", false, true),
            ] {
                let bounds = window.find(id).bounds().scale(window.scale_factor());
                let fill = window
                    .painted_quads()
                    .into_iter()
                    .find(|q| q.bounds == bounds && q.background.as_solid().is_some())
                    .unwrap();
                let radius = theme(cx).radii.lg.scale(window.scale_factor());
                let zero = px(0.).scale(window.scale_factor());
                assert_eq!(
                    fill.corner_radii,
                    Corners {
                        top_left: if start { radius } else { zero },
                        bottom_left: if start { radius } else { zero },
                        top_right: if end { radius } else { zero },
                        bottom_right: if end { radius } else { zero }
                    }
                );
            }
        }
        window.click("first", cx);
        window.render_frame(cx);
        window.press("space", cx);
        window.render_frame(cx);
        // A keyboard-focused first child must paint after its later neighbour.
        let first = window.find("first").bounds();
        let last = window.find("last").bounds();
        let quads = window.painted_quads();
        let focus = quads
            .iter()
            .rposition(|q| {
                q.border_color == theme(cx).colors.brand
                    && q.bounds == first.dilate(px(2.)).scale(window.scale_factor())
            })
            .unwrap();
        let neighbour = quads
            .iter()
            .rposition(|q| q.bounds == last.scale(window.scale_factor()))
            .unwrap();
        assert!(focus > neighbour);
        window.focus_next(cx);
        window.render_frame(cx);
        window.press("enter", cx);
        window.render_frame(cx);
        window.click("middle", cx);
        assert_eq!(view.read(cx).changes, &["save", "save", "more"]);
        window.click("last", cx);
        assert_eq!(view.read(cx).changes.last(), Some(&"more"));
    });
}

struct Popup {
    state: gpui_kit::Entity<crate::PopoverState>,
    focus: gpui_kit::FocusHandle,
}
impl Render for Popup {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let focus = self.focus.clone();
        crate::Popover::new("popover", &self.state, "Open joined actions").content(
            move |_, _, _| {
                ButtonGroup::new("popup-group", "Popup actions")
                    .item(
                        Button::new("inside-first", "First")
                            .track_focus(&focus)
                            .opacity(0.5),
                    )
                    .item(Button::new("inside-last", "Last"))
            },
        )
    }
}
#[gpui_kit::test]
fn joined_focus_ring_stays_above_containing_popup_without_reordering_tab(cx: &mut TestAppContext) {
    use gpui_kit::AppContext;
    cx.update(crate::init);
    cx.update(|cx| cx.set_reduce_motion(true));
    let (view, cx) = cx.add_window_view(|_, cx| Popup {
        state: cx.new(|cx| crate::PopoverState::new("Joined actions", cx)),
        focus: cx.focus_handle(),
    });
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.render_frame(cx);
        window.click("trigger", cx);
        window.render_frame(cx);
        window.render_frame(cx);
        view.read(cx).focus.clone().focus(window, cx);
        window.press("space", cx);
        window.render_frame(cx);
        assert!(view.read(cx).state.read(cx).is_open());
        let bounds = window.find("inside-first").bounds();
        let surface = window.find("surface").bounds();
        let quads = window.painted_quads();
        let border = theme(cx).colors.brand.opacity(0.5);
        let focus = quads
            .iter()
            .rposition(|q| {
                q.bounds == bounds.dilate(px(2.)).scale(window.scale_factor())
                    && q.border_color == border
            })
            .unwrap();
        let background = quads
            .iter()
            .position(|q| {
                q.bounds == surface.scale(window.scale_factor())
                    && q.background.as_solid() == Some(theme(cx).colors.base)
            })
            .unwrap();
        let neighbour = quads
            .iter()
            .rposition(|q| {
                q.bounds
                    == window
                        .find("inside-last")
                        .bounds()
                        .scale(window.scale_factor())
            })
            .unwrap();
        assert!(
            focus > background && focus > neighbour,
            "focus must paint locally over popup and adjacent control"
        );
        window.focus_next(cx);
        window.render_frame(cx);
        assert_eq!(window.find("inside-last").focused(), Some(true));
        window.press("escape", cx);
        assert!(!view.read(cx).state.read(cx).is_open());
    });
}
