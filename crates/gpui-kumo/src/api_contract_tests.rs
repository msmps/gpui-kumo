use crate::{
    Button, Field, Input, InputArea, InputAreaState, InputGroup, InputState, Select, SelectOption,
    SelectState, SelectValue, SensitiveInput, SensitiveInputState,
};
use gpui_kit::{
    AppContext, Context, Entity, Focusable, IntoElement, ParentElement, Render, SharedString,
    Styled, TestAppContext, Window, div, px, test::TestWindowExt,
};

struct Names {
    input: Entity<InputState>,
    area: Entity<InputAreaState>,
    secret: Entity<SensitiveInputState>,
    select: Entity<SelectState<usize>>,
    group: Entity<InputState>,
    override_name: Option<SharedString>,
    show: bool,
}
impl Render for Names {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let input = Input::new("input", &self.input).show_label(self.show);
        let input = if let Some(name) = &self.override_name {
            // The later text builder must not replace this semantic override.
            input
                .accessibility_label(name.clone())
                .label("Visible label")
        } else {
            input
        };
        div()
            .flex()
            .flex_col()
            .w(px(350.))
            .child(input)
            .child(InputArea::new("area", &self.area).show_label(self.show))
            .child(SensitiveInput::new("secret", &self.secret).show_label(self.show))
            .child(Select::new("select", &self.select).show_label(self.show))
            .child(
                Field::control(
                    "associated-field",
                    InputGroup::new("group", &self.group),
                    cx,
                )
                .show_label(self.show),
            )
            .child(
                Button::new("override-first", "Old")
                    .accessibility_label("Explicit")
                    .label("New")
                    .show_label(self.show),
            )
            .child(
                Button::new("override-last", "Old")
                    .label("New")
                    .accessibility_label("Explicit")
                    .show_label(self.show),
            )
            .child(Button::icon("icon", "Open settings", div()))
    }
}

#[gpui_kit::test]
fn rendered_names_follow_defaults_overrides_visibility_and_field_association(
    cx: &mut TestAppContext,
) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|window, cx| Names {
        input: cx.new(|cx| InputState::new("Input default", window, cx)),
        area: cx.new(|cx| InputAreaState::new("Area default", window, cx)),
        secret: cx.new(|cx| SensitiveInputState::new("Secret default", "", window, cx)),
        select: cx.new(|cx| {
            SelectState::new(
                "Select default",
                SelectValue::Single(None),
                vec![SelectOption::new("one", 1, "One")],
                cx,
            )
        }),
        group: cx.new(|cx| InputState::new("Field default", window, cx)),
        override_name: None,
        show: true,
    });
    cx.update(|window, cx| {
        window.render_frame(cx);
        for (scope, name, node) in [
            ("input", "Input default", "control"),
            ("area", "Area default", "control"),
            ("secret", "Secret default", "control"),
            ("select", "Select default", "trigger"),
            ("group", "Field default", "control"),
        ] {
            assert_eq!(window.within(scope).find(node).label(), Some(name));
        }
        assert_eq!(
            window.within("associated-field").find("label").value(),
            Some("Field default")
        );
        let secret = view.read(cx).secret.clone();
        secret.update(cx, |state, cx| {
            state.set_value("DO_NOT_LOG_SECRET", window, cx)
        });
        assert!(!format!("{:?}", secret.read(cx)).contains("DO_NOT_LOG_SECRET"));
        secret.update(cx, |state, cx| state.set_value("", window, cx));
        let field = view.read(cx).group.clone();
        window.within("associated-field").click("label", cx);
        assert!(field.read(cx).focus_handle(cx).is_focused(window));
        field.update(cx, |state, cx| state.set_name("Champ localisé", cx));
        window.render_frame(cx);
        assert_eq!(
            window.within("associated-field").find("label").value(),
            Some("Champ localisé")
        );
        let area = view.read(cx).area.clone();
        area.update(cx, |state, cx| state.set_name("Zone localisée", cx));
        let secret = view.read(cx).secret.clone();
        secret.update(cx, |state, cx| state.set_name("Secret localisé", cx));
        let select = view.read(cx).select.clone();
        select.update(cx, |state, cx| state.set_open(true, window, cx));
        select.update(cx, |state, cx| state.set_name("Choix localisé", cx));
        assert!(select.read(cx).is_open());
        view.update(cx, |view, cx| {
            view.override_name = Some("Record-specific override".into());
            view.show = false;
            cx.notify();
        });
        for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
            crate::set_appearance(appearance, cx);
            window.render_frame(cx);
            assert_eq!(
                window.within("input").find("control").label(),
                Some("Record-specific override")
            );
            assert_eq!(
                window.within("area").find("control").label(),
                Some("Zone localisée")
            );
            assert_eq!(
                window.within("secret").find("control").label(),
                Some("Secret localisé")
            );
            assert_eq!(
                window.within("select").find("trigger").label(),
                Some("Choix localisé")
            );
            assert_eq!(
                window.within("group").find("control").label(),
                Some("Champ localisé")
            );
            for id in ["override-first", "override-last"] {
                assert_eq!(window.find(id).label(), Some("Explicit"));
            }
            assert_eq!(window.find("icon").label(), Some("Open settings"));
            assert!(select.read(cx).is_open());
        }
    });
}

#[test]
fn external_names_are_fallible() {
    assert!(crate::AccessibleName::try_from(" \t\n").is_err());
    assert!(crate::AccessibleName::try_from("café 🦀").is_ok());
}

struct ControlledNames;
impl Render for ControlledNames {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .child(
                crate::Checkbox::new("checkbox", "Old")
                    .accessibility_label("Checkbox override")
                    .label("New")
                    .show_label(false)
                    .required(false),
            )
            .child(
                crate::Switch::new("switch", "Old")
                    .label("New")
                    .accessibility_label("Switch override")
                    .show_label(false),
            )
            .child(
                crate::CheckboxGroup::new("checks", "Old", &[])
                    .accessibility_label("Checks override")
                    .label("New")
                    .show_label(false)
                    .item(
                        crate::CheckboxItem::new("item", "Old")
                            .accessibility_label("Item override")
                            .label("New"),
                    )
                    .description("Hidden help")
                    .error_visible("Hidden error", false),
            )
            .child(
                crate::SwitchGroup::new("switches", "Old")
                    .label("New")
                    .accessibility_label("Switches override")
                    .show_label(false)
                    .item(crate::Switch::new("item", "Item"))
                    .error_visible("Hidden error", false)
                    .description("Hidden help"),
            )
            .child(
                crate::RadioGroup::new("radios", "Old", Some(1usize))
                    .accessibility_label("Radios override")
                    .label("New")
                    .show_label(false)
                    .item(
                        crate::RadioItem::new("radio-item", 1, "Old")
                            .accessibility_label("Radio override")
                            .label("New"),
                    )
                    .description("Hidden help")
                    .error_visible("Hidden error", false),
            )
            .child(
                crate::Link::new("link", "Old", "/route")
                    .accessibility_label("Link override")
                    .label("New"),
            )
            .child(
                crate::Meter::new("meter", "Old", 25.)
                    .accessibility_label("Meter override")
                    .label("New"),
            )
    }
}
#[gpui_kit::test]
fn controlled_names_and_hidden_errors_follow_the_shared_contract(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (_, cx) = cx.add_window_view(|_, _| ControlledNames);
    cx.update(|window, cx| {
        for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
            crate::set_appearance(appearance, cx);
            window.render_frame(cx);
            for (id, name) in [
                ("checkbox", "Checkbox override"),
                ("switch", "Switch override"),
                ("checks", "Checks override"),
                ("switches", "Switches override"),
                ("radio-item", "Radio override"),
                ("link", "Link override"),
                ("meter", "Meter override"),
            ] {
                assert_eq!(window.find(id).label(), Some(name));
            }
            assert_eq!(
                window.within("checks").find("item:item").label(),
                Some("Item override")
            );
            for id in ["checks", "switches"] {
                let group = window.within(id);
                assert!(group.try_find("legend").is_none());
                assert!(group.try_find("description").is_none());
                assert!(group.try_find("error").is_none());
            }
            // Base RadioGroup has an opaque root and repeated identity ancestry;
            // its native group name is checked in the AT-SPI fixture. Rows are observed here.
            assert!(window.try_find("legend").is_none());
            assert!(window.try_find("description").is_none());
            assert!(window.try_find("error").is_none());
            assert!(window.within("checkbox").try_find("label").is_none());
            assert!(window.within("switch").try_find("label").is_none());
        }
    });
}
#[test]
#[should_panic(expected = "nonblank accessible name")]
fn icon_only_buttons_reject_blank_names() {
    let _ = Button::icon("icon", " \t", div());
}
