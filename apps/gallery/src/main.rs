use std::borrow::Cow;

use gpui_kit::{
    App, AppContext, AssetSource, Bounds, Context, FocusHandle, InteractiveElement, IntoElement,
    KeyBinding, Menu, MenuItem, ParentElement, Render, SharedString, Styled, TitlebarOptions,
    Window, WindowBounds, WindowOptions, div, px, rgb, size, svg,
};

gpui_kit::actions!(gallery, [Quit]);

struct GalleryAssets;

impl AssetSource for GalleryAssets {
    fn load(&self, path: &str) -> gpui_kit::Result<Option<Cow<'static, [u8]>>> {
        Ok(match path {
            "workspace.svg" => Some(Cow::Borrowed(include_bytes!("../assets/workspace.svg"))),
            _ => None,
        })
    }

    fn list(&self, path: &str) -> gpui_kit::Result<Vec<SharedString>> {
        Ok(if path.is_empty() {
            vec!["workspace.svg".into()]
        } else {
            vec![]
        })
    }
}

struct Gallery {
    focus_handle: FocusHandle,
}

impl Render for Gallery {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("gallery")
            .track_focus(&self.focus_handle)
            .on_action(|_: &Quit, _, cx| cx.quit())
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_4()
            .bg(rgb(0xf8f8f8))
            .text_color(rgb(0x202020))
            .font_family(".SystemUIFont")
            .child(
                svg()
                    .path("workspace.svg")
                    .size_8()
                    .text_color(rgb(0xf6821f)),
            )
            .child(
                div()
                    .text_size(px(24.))
                    .child(gpui_kit::text!("Kumo component gallery")),
            )
            .child(
                div()
                    .text_size(px(14.))
                    .child(gpui_kit::text!("Button · Input · Popover")),
            )
    }
}

fn main() {
    gpui_kit::application()
        .with_assets(GalleryAssets)
        .run(|cx: &mut App| {
            gpui_kumo::init(cx);
            cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
            cx.on_action(|_: &Quit, cx| cx.quit());
            cx.set_menus([Menu::new("Kumo Gallery").items([MenuItem::action("Quit", Quit)])]);
            cx.on_window_closed(|cx, _| {
                if cx.windows().is_empty() {
                    cx.quit();
                }
            })
            .detach();

            let bounds = Bounds::centered(None, size(px(960.), px(640.)), cx);
            gpui_kit::open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: Some("GPUI Kumo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                cx,
                |window, cx| {
                    cx.new(|cx| {
                        let focus_handle = cx.focus_handle();
                        focus_handle.focus(window, cx);
                        Gallery { focus_handle }
                    })
                },
            )
            .expect("failed to open the Kumo gallery window");
            cx.activate(true);
        });
}
