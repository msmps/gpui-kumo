use super::*;
use gpui_kit::{
    AppContext, Context, Focusable, ParentElement, Render, TestAppContext, px, test::TestWindowExt,
};
struct Harness {
    input: gpui_kit::Entity<crate::InputState>,
    disabled: bool,
    error: Option<bool>,
    layout: Layout,
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focus = self.input.read(cx).focus_handle(cx);
        div().w(px(300.)).child(
            Field::new("field", "Phone", crate::Input::new("input", &self.input))
                .required(false)
                .focus_target(&focus)
                .disabled(self.disabled)
                .layout(self.layout)
                .description("Account recovery only")
                .when_some(self.error, |field, show| field.error("Invalid phone", show)),
        )
    }
}
#[gpui_kit::test]
fn labels_forward_focus_without_duplicating_control_value_and_errors_replace_help(
    cx: &mut TestAppContext,
) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|window, cx| Harness {
        input: cx.new(|cx| crate::InputState::new("Phone", window, cx)),
        disabled: false,
        error: None,
        layout: Layout::Stacked,
    });
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert_eq!(window.find("label").label(), Some("Phone (optional)"));
        assert_eq!(
            window.find("message").label(),
            Some("Account recovery only")
        );
        window.click("label", cx);
        assert!(
            view.read(cx)
                .input
                .read(cx)
                .focus_handle(cx)
                .is_focused(window)
        );
        window.input("café 🦀", cx);
        let input = view.read(cx).input.clone();
        assert_eq!(input.read(cx).value(cx).as_ref(), "café 🦀");
        view.update(cx, |view, cx| {
            view.error = Some(true);
            cx.notify();
        });
        window.render_frame(cx);
        assert_eq!(window.find("message").label(), Some("Invalid phone"));
        let value = window.find("control").bounds();
        let label = window.find("label").bounds();
        assert_eq!(value.top() - label.bottom(), px(8.));
        view.update(cx, |view, cx| {
            view.error = Some(false);
            cx.notify();
        });
        window.render_frame(cx);
        assert!(window.try_find("message").is_none());
        window.blur(cx);
        view.update(cx, |view, cx| {
            view.disabled = true;
            cx.notify();
        });
        window.render_frame(cx);
        window.click("label", cx);
        assert!(!input.read(cx).focus_handle(cx).is_focused(window));
        crate::set_appearance(crate::Appearance::Dark, cx);
        window.render_frame(cx);
        assert_eq!(input.read(cx).value(cx).as_ref(), "café 🦀");
    });
}

#[gpui_kit::test]
fn control_first_grows_label_from_the_leading_control_at_wide_and_narrow_widths(
    cx: &mut TestAppContext,
) {
    struct Example;
    impl Render for Example {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .flex()
                .flex_col()
                .gap(px(20.))
                .children([400., 120.].into_iter().enumerate().map(|(i, width)| {
                    div().w(px(width)).child(
                        Field::new(
                            ("field", i),
                            "Allow notifications",
                            crate::Button::new("toggle", "On"),
                        )
                        .layout(Layout::ControlFirst),
                    )
                }))
        }
    }
    cx.update(crate::init);
    let (_, cx) = cx.add_window_view(|_, _| Example);
    cx.update(|window, cx| {
        window.render_frame(cx);
        for i in 0usize..2 {
            let label = window.within(("field", i)).find("label").bounds();
            let control = window.within(("field", i)).find("toggle").bounds();
            assert_eq!(control.left(), px(0.));
            assert_eq!(label.left() - control.right(), px(8.));
            assert!(label.size.width > px(0.));
        }
    });
}
