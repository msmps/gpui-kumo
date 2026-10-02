use super::*;
use gpui_kit::{Context, InteractiveElement, Render, Role, TestAppContext, test::TestWindowExt};
struct Harness {
    state: State,
    disabled: bool,
    apply: bool,
    changes: Vec<State>,
    focus: FocusHandle,
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let owner = cx.entity().downgrade();
        div()
            .tab_group()
            .flex()
            .flex_col()
            .w(px(160.))
            .child(Checkbox::new("before", "Before").bare())
            .child(
                Checkbox::new("target", "Accept terms with a long wrapping label")
                    .state(self.state)
                    .disabled(self.disabled)
                    .track_focus(&self.focus)
                    .on_change(move |state, _, _, cx| {
                        let _ = owner.update(cx, |this, cx| {
                            this.changes.push(state);
                            if this.apply {
                                this.state = state;
                            }
                            cx.notify();
                        });
                    }),
            )
            .child(Checkbox::new("after", "After").bare())
    }
}
#[gpui_kit::test]
fn label_pointer_and_keyboard_propose_once_and_controlled_state_remains_owned(
    cx: &mut TestAppContext,
) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|_, cx| Harness {
        state: State::Indeterminate,
        disabled: false,
        apply: true,
        changes: Vec::new(),
        focus: cx.focus_handle(),
    });
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert_eq!(window.find("target").role(), Some(Role::CheckBox));
        assert_eq!(window.find("target").indeterminate(), Some(true));
        window.click("label", cx);
        assert_eq!(view.read(cx).changes, &[State::Checked]);
        assert!(view.read(cx).focus.is_focused(window));
        window.render_frame(cx);
        window.press("space", cx);
        window.render_frame(cx);
        window.press("enter", cx);
        assert_eq!(
            view.read(cx).changes,
            &[State::Checked, State::Unchecked, State::Checked]
        );
        view.update(cx, |view, cx| {
            view.apply = false;
            cx.notify();
        });
        window.render_frame(cx);
        window.press("space", cx);
        window.render_frame(cx);
        window.press("space", cx);
        assert_eq!(view.read(cx).state, State::Checked);
        assert_eq!(
            &view.read(cx).changes[3..],
            &[State::Unchecked, State::Unchecked]
        );
        crate::set_appearance(crate::Appearance::Dark, cx);
        window.render_frame(cx);
        assert_eq!(view.read(cx).state, State::Checked);
        view.update(cx, |view, cx| {
            view.disabled = true;
            cx.notify();
        });
        window.render_frame(cx);
        window.click("label", cx);
        view.read(cx).focus.clone().focus(window, cx);
        window.press("enter", cx);
        window.press("space", cx);
        assert_eq!(view.read(cx).changes.len(), 5);
        // Synthetic parent metadata is not captured by Base test snapshots.
        // Disabled input outcomes above remain the observable regression.
        window.blur(cx);
        window.focus_next(cx);
        window.focus_next(cx);
        assert!(!view.read(cx).focus.is_focused(window));
    });
}

#[test]
#[should_panic(expected = "Checkbox requires a readable name")]
fn blank_names_are_rejected() {
    let _ = Checkbox::new("unnamed", "  ");
}

struct Geometry;
impl Render for Geometry {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(px(12.))
            .child(Checkbox::new("bare", "Bare").bare())
            .child(Checkbox::new("short", "Short"))
            .child(
                div().w(px(100.)).child(
                    Checkbox::new("long", "A long label that wraps within a narrow layout")
                        .control_first(false),
                ),
            )
    }
}
#[gpui_kit::test]
fn bare_square_and_labels_have_source_geometry_in_both_themes(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (_, cx) = cx.add_window_view(|_, _| Geometry);
    cx.update(|window, cx| {
        for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
            crate::set_appearance(appearance, cx);
            window.render_frame(cx);
            let bare = window.find("bare").bounds();
            assert_eq!(bare.size, gpui_kit::size(px(16.), px(16.)));
            assert_eq!(window.find("short").bounds().size.height, px(21.));
            assert!(window.find("long").bounds().size.height > px(21.));
            assert!(window.find("long").bounds().size.width <= px(100.));
            assert_eq!(window.find("bare").label(), Some("Bare"));
        }
    });
}

struct RichLabels {
    state: State,
    changes: usize,
    disabled: bool,
    first: bool,
    width: f32,
}
impl Render for RichLabels {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        use gpui_kit::base::TestSupportExt;
        let owner = cx.entity().downgrade();
        div().w(px(self.width)).child(
            Checkbox::new(
                "rich-control",
                "Accept terms for café 🦀 and international notifications",
            )
            .content(
                div().id("rich-copy").test_support().min_w_0().child(
                    gpui_kit::StyledText::new(
                        "Accept terms for café 🦀 and international notifications",
                    )
                    .with_highlights([(
                        7..12,
                        gpui_kit::HighlightStyle {
                            font_weight: Some(FontWeight::BOLD),
                            ..Default::default()
                        },
                    )]),
                ),
            )
            .state(self.state)
            .disabled(self.disabled)
            .control_first(self.first)
            .required(false)
            .on_change(move |state, _, _, cx| {
                let _ = owner.update(cx, |v, cx| {
                    v.state = state;
                    v.changes += 1;
                    cx.notify();
                });
            }),
        )
    }
}
#[gpui_kit::test]
fn rich_checkbox_label_preserves_name_wrapping_and_single_activation(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|_, _| RichLabels {
        state: State::Indeterminate,
        changes: 0,
        disabled: false,
        first: true,
        width: 180.,
    });
    cx.update(|window, cx| {
        for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
            crate::set_appearance(appearance, cx);
            for first in [true, false] {
                view.update(cx, |v, cx| {
                    v.first = first;
                    v.disabled = false;
                    v.state = State::Indeterminate;
                    cx.notify();
                });
                window.activate_window();
                window.render_frame(cx);
                assert_eq!(window.find("rich-control").role(), Some(Role::CheckBox));
                assert_eq!(
                    window.find("rich-control").label(),
                    Some("Accept terms for café 🦀 and international notifications (optional)")
                );
                let root = window.find("rich-control").bounds();
                let copy = window.find("rich-copy").bounds();
                assert!(root.size.width <= px(180.));
                assert!(copy.size.height > px(21.), "rich text wraps");
                assert!(
                    copy.left() >= root.left() && copy.right() <= root.right(),
                    "rich glyph container stays inside control"
                );
                assert_eq!(copy.top(), root.top());
                let before = view.read(cx).changes;
                window.click("rich-copy", cx);
                window.render_frame(cx);
                assert_eq!(view.read(cx).changes, before + 1);
                assert_eq!(view.read(cx).state, State::Checked);
                window.press("space", cx);
                window.render_frame(cx);
                window.press("enter", cx);
                window.render_frame(cx);
                assert_eq!(view.read(cx).changes, before + 3);
                view.update(cx, |v, cx| {
                    v.disabled = true;
                    cx.notify();
                });
                window.render_frame(cx);
                window.click("rich-copy", cx);
                window.press("space", cx);
                window.press("enter", cx);
                window.render_frame(cx);
                assert_eq!(
                    view.read(cx).changes,
                    before + 3,
                    "disabled rich label cannot activate"
                );
            }
        }
    });
}
