use super::*;
use gpui_kit::{Context, Render, TestAppContext, test::TestWindowExt};
struct Harness {
    selected: Vec<SharedString>,
    changes: usize,
    disabled: bool,
    apply: bool,
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let owner = cx.entity().downgrade();
        CheckboxGroup::new("preferences", "Preferences", &self.selected)
            .item(CheckboxItem::new("email", "Email"))
            .item(CheckboxItem::new("sms", "SMS").disabled(true))
            .select_all("Select all", &["email".into(), "sms".into()])
            .disabled(self.disabled)
            .description("Delivery choices")
            .error("Choose carefully")
            .on_change(move |values, _, cx| {
                let _ = owner.update(cx, |this, cx| {
                    this.changes += 1;
                    if this.apply {
                        this.selected = values;
                    }
                    cx.notify();
                });
            })
    }
}
#[gpui_kit::test]
fn selection_proposals_preserve_unrelated_values_and_aggregate_mixed_state(
    cx: &mut TestAppContext,
) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|_, _| Harness {
        selected: vec!["other".into(), "email".into()],
        changes: 0,
        disabled: false,
        apply: true,
    });
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert_eq!(window.find("preferences").role(), Some(Role::Group));
        assert_eq!(window.find("select-all").indeterminate(), Some(true));
        window.click("select-all", cx);
        window.render_frame(cx);
        assert_eq!(
            view.read(cx).selected,
            vec![
                SharedString::from("other"),
                SharedString::from("email"),
                SharedString::from("sms")
            ]
        );
        assert_eq!(view.read(cx).changes, 1);
        assert_eq!(window.find("select-all").checked(), Some(true));
        window.press("space", cx);
        window.render_frame(cx);
        assert_eq!(view.read(cx).selected, vec![SharedString::from("other")]);
        assert_eq!(view.read(cx).changes, 2);
        window.click("item:sms", cx);
        assert_eq!(view.read(cx).changes, 2);
        window.click("item:email", cx);
        window.render_frame(cx);
        assert_eq!(view.read(cx).changes, 3);
        assert_eq!(window.find("select-all").indeterminate(), Some(true));
        view.update(cx, |view, cx| {
            view.disabled = true;
            cx.notify();
        });
        window.render_frame(cx);
        window.click("select-all", cx);
        window.press("space", cx);
        window.click("item:email", cx);
        assert_eq!(view.read(cx).changes, 3);
        crate::set_appearance(crate::Appearance::Dark, cx);
        window.render_frame(cx);
        assert_eq!(window.find("item:email").checked(), Some(true));
        view.update(cx, |view, cx| {
            view.disabled = false;
            view.apply = false;
            view.selected.clear();
            cx.notify();
        });
        window.render_frame(cx);
        window.click("item:email", cx);
        window.render_frame(cx);
        window.press("space", cx);
        assert_eq!(view.read(cx).changes, 5);
        assert!(view.read(cx).selected.is_empty());
    });
}
#[test]
#[should_panic(expected = "Checkbox item values must be unique")]
fn duplicate_values_are_rejected() {
    let _ = CheckboxGroup::new("g", "Group", &[])
        .item(CheckboxItem::new("same", "First"))
        .item(CheckboxItem::new("same", "Second"));
}

struct EdgeHarness {
    changes: usize,
}
impl Render for EdgeHarness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let owner = cx.entity().downgrade();
        let second_owner = owner.clone();
        div()
            .flex()
            .flex_col()
            .gap(gpui_kit::px(16.))
            .child(
                CheckboxGroup::new("first", "First", &[])
                    .select_all("Select first", &["select-all".into()])
                    .item(CheckboxItem::new("select-all", "First item"))
                    .legend("Custom legend")
                    .error("Error")
                    .description("Helper")
                    .on_change(move |_, _, cx| {
                        let _ = owner.update(cx, |this, cx| {
                            this.changes += 1;
                            cx.notify();
                        });
                    }),
            )
            .child(
                CheckboxGroup::new("second", "Second", &[])
                    .hide_legend(true)
                    .select_all("Empty selection", &[])
                    .item(CheckboxItem::new("select-all", "Second item"))
                    .on_change(move |_, _, cx| {
                        let _ = second_owner.update(cx, |this, cx| {
                            this.changes += 1;
                            cx.notify();
                        });
                    }),
            )
    }
}
#[gpui_kit::test]
fn repeated_groups_keep_scoped_ids_and_empty_aggregate_is_inert(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|_, _| EdgeHarness { changes: 0 });
    cx.update(|window, cx| {
        for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
            crate::set_appearance(appearance, cx);
            window.render_frame(cx);
            let mut first = window.within("first");
            assert_eq!(first.find("legend").label(), Some("First"));
            assert_eq!(first.find("error").label(), Some("Error"));
            assert_eq!(first.find("description").label(), Some("Helper"));
            let expected = theme(cx).typography.sm.line_height;
            // GPUI snaps text layout to device pixels (15.5px at this test scale);
            // half a logical pixel still rejects Field's 17.875px line height.
            assert!(
                f32::from(first.find("error").bounds().size.height - expected).abs() <= 0.5,
                "actual {:?}, expected {:?}",
                first.find("error").bounds().size.height,
                expected
            );
            assert!(
                f32::from(first.find("description").bounds().size.height - expected).abs() <= 0.5
            );
            first.click("item:select-all", cx);
            assert!(first.find("item:select-all").focused().unwrap());
            first.click("select-all", cx);
            assert!(first.find("select-all").focused().unwrap());
            window.blur(cx);
            let mut second = window.within("second");
            assert!(second.try_find("legend").is_none());
            assert_eq!(second.find("select-all").checked(), Some(false));
            second.click("select-all", cx);
            assert_eq!(second.find("select-all").focused(), None);
        }
        assert_eq!(view.read(cx).changes, 4);
    });
}
