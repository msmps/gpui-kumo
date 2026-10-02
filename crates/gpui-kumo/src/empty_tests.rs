use super::*;
use gpui_kit::{Context, Render, TestAppContext, test::TestWindowExt};

struct Harness {
    command: SharedString,
    visible: bool,
    width: f32,
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .w(px(self.width))
            .flex()
            .flex_col()
            .children(self.visible.then(|| {
                Empty::new("first", "No packages")
                    .size(Size::Sm)
                    .description("Create your first package.")
                    .command_line(self.command.clone())
                    .contents(Button::new("consumer-action", "Retry"))
            }))
            .child(
                Empty::new("second", "Nothing here")
                    .size(Size::Sm)
                    .command_line("second"),
            )
    }
}

#[gpui_kit::test]
fn copy_keeps_instances_independent_and_restarts_feedback(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|_, _| Harness {
        command: "echo café 🦀".into(),
        visible: true,
        width: 600.,
    });
    cx.update(|window, cx| {
        window.render_frame(cx);
        window.within("first").click("copy-command", cx);
        assert_eq!(
            cx.read_from_clipboard().unwrap().text().unwrap(),
            "echo café 🦀"
        );
        assert_eq!(
            window.within("first").find("copy-command").label(),
            Some("Command copied")
        );
        assert_eq!(
            window.within("second").find("copy-command").label(),
            Some("Copy command")
        );
    });
    cx.run_until_parked();
    cx.background_executor
        .advance_clock(Duration::from_millis(750));
    cx.update(|window, cx| {
        window.press("enter", cx);
    });
    cx.run_until_parked();
    cx.background_executor
        .advance_clock(Duration::from_millis(500));
    cx.run_until_parked();
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert_eq!(
            window.within("first").find("copy-command").label(),
            Some("Command copied")
        );
        view.update(cx, |view, cx| {
            view.command = "echo changed".into();
            cx.notify();
        });
        window.render_frame(cx);
        assert_eq!(
            window.within("first").find("copy-command").label(),
            Some("Copy command")
        );
        window.press("space", cx);
        assert_eq!(
            cx.read_from_clipboard().unwrap().text().unwrap(),
            "echo changed"
        );
        crate::set_appearance(crate::Appearance::Dark, cx);
        window.render_frame(cx);
        assert_eq!(
            window.within("first").find("copy-command").label(),
            Some("Command copied")
        );
    });
    cx.run_until_parked();
    cx.background_executor
        .advance_clock(Duration::from_millis(1001));
    cx.run_until_parked();
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert_eq!(
            window.within("first").find("copy-command").label(),
            Some("Copy command")
        );
    });
}

#[gpui_kit::test]
fn title_and_container_geometry_follow_minimal_and_described_recipes(cx: &mut TestAppContext) {
    struct Geometry;
    impl Render for Geometry {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div().w(px(600.)).flex().flex_col().children(
                [Size::Sm, Size::Base, Size::Lg]
                    .into_iter()
                    .enumerate()
                    .map(|(i, size)| Empty::new(("empty", i), "Nothing here").size(size)),
            )
        }
    }
    cx.update(crate::init);
    let (_, cx) = cx.add_window_view(|_, _| Geometry);
    cx.update(|window, cx| {
        window.render_frame(cx);
        for (i, padding) in [32., 64., 80.].into_iter().enumerate() {
            let surface = window.find(("empty", i)).bounds();
            let title = window.within(("empty", i)).find("title");
            assert_eq!(surface.size.width, px(600.));
            assert_eq!(title.role(), Some(gpui_kit::Role::Heading));
            assert_eq!(title.label(), Some("Nothing here"));
            assert_eq!(
                surface.size.height,
                title.bounds().size.height + px(2. * padding + 2.)
            );
        }
    });
}

#[gpui_kit::test]
fn narrow_commands_keep_copy_reachable_and_unmount_resets_pending_feedback(
    cx: &mut TestAppContext,
) {
    cx.update(crate::init);
    let command: SharedString =
        "echo café 🦀 with a very long command that extends well beyond the frame".into();
    let (view, cx) = cx.add_window_view(|_, _| Harness {
        command: command.clone(),
        visible: true,
        width: 220.,
    });
    cx.update(|window, cx| {
        window.render_frame(cx);
        let root = window.find("first").bounds();
        let action = window.within("first").find("consumer-action").bounds();
        assert!((f32::from(action.center().x - root.center().x)).abs() < 1.);
        let surface = window.within("first").find("command").bounds();
        let copy = window.within("first").find("copy-command").bounds();
        assert!(surface.contains(&copy.center()));
        assert!(copy.right() <= surface.right() && copy.left() >= surface.left());
        assert!(surface.size.width <= px((220. - 50.) * 0.8 + 1.));
        window.within("first").click("copy-command", cx);
        assert_eq!(
            cx.read_from_clipboard().unwrap().text().unwrap(),
            command.as_ref()
        );
        view.update(cx, |view, cx| {
            view.visible = false;
            cx.notify();
        });
        window.render_frame(cx);
        window.render_frame(cx);
    });
    cx.run_until_parked();
    cx.update(|window, cx| {
        view.update(cx, |view, cx| {
            view.visible = true;
            cx.notify();
        });
        window.render_frame(cx);
        assert_eq!(
            window.within("first").find("copy-command").label(),
            Some("Copy command")
        );
        window.within("first").click("copy-command", cx);
    });
    cx.run_until_parked();
    cx.background_executor
        .advance_clock(Duration::from_millis(500));
    cx.run_until_parked();
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert_eq!(
            window.within("first").find("copy-command").label(),
            Some("Command copied")
        );
    });
}
