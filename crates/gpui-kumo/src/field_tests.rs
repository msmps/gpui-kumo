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
        assert_eq!(window.find("label").value(), Some("Phone (optional)"));
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

struct HelpHarness {
    input: gpui_kit::Entity<crate::InputState>,
    tooltip: gpui_kit::Entity<crate::TooltipState>,
    disabled: bool,
    hide: bool,
    width: f32,
    content: SharedString,
    label: SharedString,
}
impl Render for HelpHarness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focus = self.input.read(cx).focus_handle(cx);
        crate::TooltipProvider::new(
            "provider",
            [self.tooltip.downgrade()],
            div()
                .w(px(self.width))
                .flex()
                .flex_col()
                .tab_group()
                .child(
                    Field::new(
                        "help-field",
                        self.label.clone(),
                        crate::Input::new("help-input", &self.input),
                    )
                    .required(false)
                    .focus_target(&focus)
                    .disabled(self.disabled)
                    .hide_label(self.hide)
                    .label_tooltip(&self.tooltip, self.content.clone())
                    .description("Keep this number up to date."),
                )
                .child(crate::Button::new("after-help", "After")),
        )
    }
}
#[gpui_kit::test]
fn contextual_help_is_independent_of_label_focus_and_control_availability(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|window, cx| HelpHarness {
        input: cx.new(|cx| crate::InputState::new("Phone number", window, cx)),
        tooltip: cx.new(|cx| crate::TooltipState::new(window, cx)),
        disabled: false,
        hide: false,
        width: 300.,
        content: "Used only for account recovery.".into(),
        label: "Phone number".into(),
    });
    let (input, tip) = cx.read(|cx| (view.read(cx).input.clone(), view.read(cx).tooltip.clone()));
    for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
        cx.update(|window, cx| {
            crate::set_appearance(appearance, cx);
            window.activate_window();
            window.render_frame(cx);
            window.render_frame(cx);
            let label = window.find("label").bounds();
            let help = window.find("label-help").bounds();
            assert_eq!(help.size, gpui_kit::size(px(14.), px(14.)));
            assert_eq!(help.center().y, label.center().y);
            assert_eq!(window.find("label-help").label(), Some("More information"));
            assert_eq!(
                window.find("control").bounds().top() - label.bottom(),
                px(8.)
            );
            input.update(cx, |input, cx| input.set_value("", window, cx));
            window.click("label", cx);
            assert!(input.read(cx).focus_handle(cx).is_focused(window));
            window.input("café 🦀", cx);
            window.click("label-help", cx);
            assert_eq!(window.find("label-help").focused(), Some(true));
            assert!(!input.read(cx).focus_handle(cx).is_focused(window));
            assert!(
                !tip.read(cx).is_open(),
                "pointer activation does not open help"
            );
            assert_eq!(input.read(cx).value(cx).as_ref(), "café 🦀");
            window.click("label", cx);
            window.render_frame(cx);
            window.press("shift-tab", cx);
            window.focus_prev(cx);
            window.render_frame(cx);
        });
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.render_frame(cx);
            assert_eq!(window.find("label-help").focused(), Some(true));
            assert!(tip.read(cx).is_open());
            let help_bounds = window.find("label-help").bounds();
            let theme = crate::theme(cx);
            assert!(
                window.painted_quads().iter().any(|quad| {
                    quad.bounds
                        == help_bounds
                            .dilate(theme.effects.keyboard_focus_ring_width)
                            .scale(window.scale_factor())
                        && quad.border_color == theme.colors.brand
                }),
                "help paints its keyboard focus ring"
            );
            window.press("escape", cx);
            assert!(!tip.read(cx).is_open());
            assert_eq!(window.find("label-help").focused(), Some(true));
            input.update(cx, |input, cx| input.set_disabled(true, cx));
            view.update(cx, |view, cx| {
                view.disabled = true;
                view.width = 240.;
                cx.notify();
            });
            window.render_frame(cx);
            window.click("label-help", cx);
            assert_eq!(window.find("label-help").focused(), Some(true));
            window.click("label", cx);
            assert_eq!(
                window.find("label-help").focused(),
                Some(true),
                "disabled label preserves help focus"
            );
            assert!(!input.read(cx).focus_handle(cx).is_focused(window));
            assert_eq!(input.read(cx).value(cx).as_ref(), "café 🦀");
            tip.update(cx, |tip, cx| tip.set_open(true, cx));
            view.update(cx, |view, cx| {
                view.hide = true;
                cx.notify();
            });
            window.render_frame(cx);
            assert!(!tip.read(cx).is_open());
            assert!(window.try_find("label-help").is_none());
            assert!(window.try_find("label").is_none());
            assert!(window.try_find("message").is_some());
            view.update(cx, |view, cx| {
                view.hide = false;
                view.label = "Phone number for recovery with an international dialing code".into();
                view.content = "".into();
                cx.notify();
            });
            tip.update(cx, |tip, cx| tip.set_open(true, cx));
            window.render_frame(cx);
            assert!(!tip.read(cx).is_open());
            assert!(window.try_find("label-help").is_none());
            view.update(cx, |view, cx| {
                view.content = "Used only for account recovery.".into();
                cx.notify();
            });
            window.render_frame(cx);
            let label = window.find("label").bounds();
            assert!(label.size.height > px(21.), "long label wraps");
            assert_eq!(
                window.find("label-help").bounds().center().y,
                label.center().y
            );
            assert_eq!(
                window.find("control").bounds().top() - label.bottom(),
                px(8.)
            );
            view.update(cx, |view, cx| {
                view.label = "Phone number".into();
                cx.notify();
            });
            input.update(cx, |input, cx| input.set_disabled(false, cx));
            view.update(cx, |view, cx| {
                view.disabled = false;
                cx.notify();
            });
        });
    }
}
