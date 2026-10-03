//! Consumer regression for Base confirmation focus ownership.
use gpui_kit::{
    Context, FocusHandle, IntoElement, ParentElement, Render, Styled, TestAppContext, Window, base,
    div, px, test::TestWindowExt,
};
struct Harness {
    open: bool,
    outside: bool,
    confirmations: usize,
    trigger: FocusHandle,
    content: FocusHandle,
    other: FocusHandle,
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let owner = cx.entity().downgrade();
        let confirm = owner.clone();
        div()
            .flex()
            .flex_col()
            .child(
                base::Select::new("select")
                    .open(self.open)
                    .focus_handle(&self.trigger)
                    .content_focus_handle(&self.content)
                    .accessibility_label("Choose language")
                    .on_open_change(move |open, _, cx| {
                        let _ = owner.update(cx, |v, cx| {
                            v.open = open;
                            cx.notify();
                        });
                    })
                    .on_confirm(move |window, cx| {
                        let _ = confirm.update(cx, |v, cx| {
                            v.confirmations += 1;
                            v.open = false;
                            if v.outside {
                                v.other.focus(window, cx);
                            } else {
                                v.trigger.focus(window, cx);
                            }
                            cx.notify();
                        });
                    })
                    .child(if self.open {
                        use gpui_kit::InteractiveElement;
                        div()
                            .track_focus(&self.content)
                            .w(px(200.))
                            .h(px(100.))
                            .child("Options")
                    } else {
                        div().w(px(200.)).h(px(36.)).child("Choose language")
                    }),
            )
            .child({
                use gpui_kit::InteractiveElement;
                div().track_focus(&self.other).size(px(36.)).child("Other")
            })
    }
}
#[gpui_kit::test]
fn base_confirm_open_preserves_consumer_close_and_focus(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|_, cx| Harness {
        open: false,
        outside: false,
        confirmations: 0,
        trigger: cx.focus_handle(),
        content: cx.focus_handle(),
        other: cx.focus_handle(),
    });
    for outside in [false, true] {
        view.update(cx, |v, cx| {
            v.outside = outside;
            cx.notify();
        });
        cx.update(|window, cx| {
            let trigger = view.read(cx).trigger.clone();
            trigger.focus(window, cx);
            window.render_frame(cx);
            window.press("enter", cx);
            assert!(view.read(cx).open);
            assert!(view.read(cx).content.is_focused(window));
            window.render_frame(cx);
            window.press("enter", cx);
            assert!(!view.read(cx).open);
            let owner = view.read(cx);
            let target = if outside {
                owner.other.clone()
            } else {
                owner.trigger.clone()
            };
            assert!(
                target.is_focused(window),
                "Base must not overwrite focus chosen by its open-state confirm callback"
            );
            window.render_frame(cx);
            assert!(target.is_focused(window));
        });
    }
    assert_eq!(view.read_with(cx, |v, _| v.confirmations), 2);
}
