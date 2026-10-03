use crate::{
    Badge, Banner, BreadcrumbCurrent, Breadcrumbs, Label, Text, Theme, text::HeadingLevel,
};
use gpui_kit::{
    Context, IntoElement, ParentElement, Render, Role, SharedString, Styled, Subscription,
    TestAppContext, Window, div, px, test::TestWindowExt,
};

struct Names {
    text: SharedString,
    optional: bool,
    loading: bool,
    narrow: bool,
    _theme: Subscription,
}
impl Render for Names {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w(px(if self.narrow { 150. } else { 320. }))
            .child(Text::new("text", self.text.clone()).truncate(true))
            .child(Text::new("heading", self.text.clone()).heading_level(HeadingLevel::Two))
            .child(Label::new("label", self.text.clone()).optional(self.optional))
            .child(Label::new("decorative", self.text.clone()).as_content())
            .child(Badge::new("badge", self.text.clone()).rich_content(div().child("Decoration")))
            .child(
                Banner::new("banner")
                    .title(self.text.clone())
                    .description(self.text.clone()),
            )
            .child(Breadcrumbs::new("trail").current(
                BreadcrumbCurrent::new("current", self.text.clone()).loading(self.loading),
            ))
    }
}

#[gpui_kit::test]
fn readable_values_follow_owner_updates_without_changing_role_policies(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|_, cx| Names {
        text: "café 🦀 initial".into(),
        optional: true,
        loading: false,
        narrow: false,
        _theme: cx.observe_global::<Theme>(|_, cx| cx.notify()),
    });
    for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
        for (text, optional, loading, narrow) in [
            ("café 🦀 initial", true, false, false),
            (
                "日本 🦀 updated long text retains its complete readable value",
                false,
                true,
                true,
            ),
            ("Restored café 🦀", true, false, true),
        ] {
            cx.update(|window, cx| {
                crate::set_appearance(appearance, cx);
                view.update(cx, |view, cx| {
                    view.text = text.into();
                    view.optional = optional;
                    view.loading = loading;
                    view.narrow = narrow;
                    cx.notify();
                });
                window.render_frame(cx);
                for id in ["text", "badge"] {
                    let node = window.find(id);
                    assert_eq!(node.role(), Some(Role::Label));
                    assert_eq!(node.label(), Some(text));
                    assert_eq!(node.value(), Some(text));
                }
                for id in ["title", "description"] {
                    let node = window.within("banner").find(id);
                    assert_eq!(node.role(), Some(Role::Label));
                    assert_eq!(node.label(), Some(text));
                    assert_eq!(node.value(), Some(text));
                }
                let label = if optional {
                    format!("{text} (optional)")
                } else {
                    text.into()
                };
                assert_eq!(window.find("label").value(), Some(label.as_str()));
                assert_eq!(window.find("label").label(), Some(label.as_str()));
                assert_eq!(window.find("heading").role(), Some(Role::Heading));
                assert_eq!(window.find("heading").label(), Some(text));
                assert_eq!(window.find("heading").value(), None);
                assert_eq!(window.find("decorative").role(), None);
                assert_eq!(window.find("decorative").value(), None);
                let current = window.find("current");
                assert_eq!(current.role(), (!loading).then_some(Role::Label));
                assert_eq!(current.value(), (!loading).then_some(text));
            });
        }
    }
}
