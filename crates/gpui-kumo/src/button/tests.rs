use super::{Button, Shape, Size};
use gpui_kit::{
    Context, Entity, FocusHandle, InputEvent, InteractiveElement, IntoElement, KeyDownEvent,
    KeyUpEvent, Keystroke, ParentElement, Render, Role, Styled, TestAppContext, VisualTestContext,
    Window, div, px, test::TestWindowExt,
};
use std::{cell::Cell, rc::Rc};

struct Harness {
    disabled: bool,
    loading: bool,
    target: FocusHandle,
    before: FocusHandle,
    after: FocusHandle,
    activations: Rc<Cell<usize>>,
}

impl Render for Harness {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let activations = self.activations.clone();
        div()
            .id("harness")
            .tab_group()
            .flex()
            .flex_col()
            .gap(px(12.))
            .child(Button::new("before", "Before").track_focus(&self.before))
            .child(
                Button::new("target", "Save")
                    .track_focus(&self.target)
                    .disabled(self.disabled)
                    .loading(self.loading)
                    .on_click(move |_, _, _| activations.set(activations.get() + 1)),
            )
            .child(Button::new("after", "After").track_focus(&self.after))
    }
}

fn harness(cx: &mut TestAppContext) -> (Entity<Harness>, &mut VisualTestContext) {
    cx.update(|cx| {
        crate::init(cx);
        cx.set_reduce_motion(true);
    });
    let (view, cx) = cx.add_window_view(|_, cx| Harness {
        disabled: false,
        loading: false,
        target: cx.focus_handle(),
        before: cx.focus_handle(),
        after: cx.focus_handle(),
        activations: Rc::new(Cell::new(0)),
    });
    cx.update(|window, cx| window.render_frame(cx));
    (view, cx)
}

#[gpui_kit::test]
fn pointer_enter_and_space_activate_once_and_expose_the_name(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    cx.update(|window, cx| {
        assert_eq!(window.find("target").label(), Some("Save"));
        assert_eq!(window.find("target").role(), Some(Role::Button));
        window.click("target", cx);
        assert_eq!(view.read(cx).activations.get(), 1);
        window.press("enter", cx);
        assert_eq!(view.read(cx).activations.get(), 2);
        window.press("space", cx);
        assert_eq!(view.read(cx).activations.get(), 3);
        assert!(view.read(cx).target.is_focused(window));
    });
}

#[gpui_kit::test]
fn unavailable_controls_reject_activation_skip_traversal_and_can_reenable(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    for (disabled, loading) in [(true, false), (false, true), (true, true)] {
        cx.update(|window, cx| {
            view.update(cx, |view, cx| {
                view.disabled = disabled;
                view.loading = loading;
                cx.notify();
            });
            window.render_frame(cx);
            window.click("target", cx);
            view.read(cx).target.clone().focus(window, cx);
            window.press("enter", cx);
            window.press("space", cx);
            assert_eq!(view.read(cx).activations.get(), 0);
            view.read(cx).before.clone().focus(window, cx);
            window.focus_next(cx);
            assert!(view.read(cx).after.is_focused(window));
        });
    }
    cx.update(|window, cx| {
        view.update(cx, |view, cx| {
            view.disabled = false;
            view.loading = false;
            cx.notify();
        });
        window.render_frame(cx);
        window.click("target", cx);
        window.press("enter", cx);
        assert_eq!(view.read(cx).activations.get(), 2);
    });
}

#[gpui_kit::test]
fn disabling_between_key_down_and_key_up_cancels_activation(cx: &mut TestAppContext) {
    let (view, cx) = harness(cx);
    cx.update(|window, cx| {
        view.read(cx).target.clone().focus(window, cx);
        window.render_frame(cx);
        let key = Keystroke::parse("space").unwrap();
        window.dispatch_event(
            KeyDownEvent {
                keystroke: key.clone(),
                is_held: false,
                prefer_character_input: false,
            }
            .to_platform_input(),
            cx,
        );
        view.update(cx, |view, cx| {
            view.disabled = true;
            cx.notify();
        });
        window.render_frame(cx);
        window.dispatch_event(KeyUpEvent { keystroke: key }.to_platform_input(), cx);
        assert_eq!(view.read(cx).activations.get(), 0);
    });
}

struct GeometryHarness;
impl Render for GeometryHarness {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(px(12.))
            .child(div().w(px(240.)).child(Button::new("intrinsic", "Short")))
            .child(
                div()
                    .w(px(240.))
                    .child(Button::new("longer", "A longer label")),
            )
            .children(
                [Size::Xs, Size::Sm, Size::Base, Size::Lg]
                    .into_iter()
                    .enumerate()
                    .map(|(i, size)| {
                        div()
                            .flex()
                            .gap(px(12.))
                            .child(Button::new(("standard", i), "Size").size(size))
                            .child(
                                Button::icon(("square", i), "Add", div().size(px(8.))).size(size),
                            )
                            .child(
                                Button::icon(("circle", i), "Add round", div().size(px(8.)))
                                    .size(size)
                                    .shape(Shape::Circle),
                            )
                    }),
            )
    }
}

#[gpui_kit::test]
fn rendered_size_and_shape_geometry_matches_the_recipe(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (_, cx) = cx.add_window_view(|_, _| GeometryHarness);
    cx.update(|window, cx| {
        window.render_frame(cx);
        let short = window.find("intrinsic").bounds().size.width;
        let longer = window.find("longer").bounds().size.width;
        assert!(short < longer && longer < px(240.));
        for (i, (height, compact)) in [(20., 14.), (26., 26.), (36., 36.), (40., 40.)]
            .into_iter()
            .enumerate()
        {
            assert_eq!(
                window.find(("standard", i)).bounds().size.height,
                px(height)
            );
            for shape in ["square", "circle"] {
                let size = window.find((shape, i)).bounds().size;
                assert_eq!(size.width, px(compact));
                assert_eq!(size.height, px(compact));
            }
            assert_eq!(window.find(("square", i)).label(), Some("Add"));
        }
    });
}

#[gpui_kit::test]
fn wrapper_respects_parent_cross_axis_centering_for_every_size(cx: &mut TestAppContext) {
    struct Centered;
    impl Render for Centered {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div().flex().flex_col().children(
                [Size::Xs, Size::Sm, Size::Base, Size::Lg]
                    .into_iter()
                    .enumerate()
                    .map(|(i, size)| {
                        use gpui_kit::base::TestSupportExt;
                        div()
                            .id(("row", i))
                            .test_support()
                            .flex()
                            .items_center()
                            .h(px(60.))
                            .child(
                                Button::icon(
                                    ("button", i),
                                    "Copy",
                                    div().id("glyph").test_support().size(px(16.)),
                                )
                                .size(size)
                                .variant(super::Variant::Ghost),
                            )
                    }),
            )
        }
    }
    cx.update(crate::init);
    let (_, cx) = cx.add_window_view(|_, _| Centered);
    cx.update(|window, cx| {
        for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
            crate::set_appearance(appearance, cx);
            window.render_frame(cx);
            for i in 0usize..4 {
                let row = window.find(("row", i)).bounds();
                let scope = window.within(("row", i));
                let button = scope.find(("button", i)).bounds();
                let glyph = scope.find("glyph").bounds();
                assert_eq!(button.center().y, row.center().y);
                assert_eq!(glyph.center(), button.center());
            }
        }
    });
}

#[gpui_kit::test]
fn stretching_column_does_not_stretch_button_control_or_hitbox(cx: &mut TestAppContext) {
    struct Column;
    impl Render for Column {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .w(px(400.))
                .flex()
                .flex_col()
                .child(Button::icon("column-action", "Copy", div().size(px(16.))).size(Size::Sm))
        }
    }
    cx.update(crate::init);
    let (_, cx) = cx.add_window_view(|_, _| Column);
    cx.update(|window, cx| {
        window.render_frame(cx);
        let action = window.find("column-action").bounds();
        assert_eq!(action.size, gpui_kit::size(px(26.), px(26.)));
        assert_eq!(action.left(), px(0.));
    });
}
