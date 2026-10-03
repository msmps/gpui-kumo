//! Focused native toolbar review: --dark and --width=520 (default1040).
#[path = "../src/panel.rs"]
mod panel;
use panel::panel;
#[path = "../src/toolbar_editors.rs"]
mod toolbar_editors;
use gpui_kit::{
    App, AppContext, Bounds, Context, InteractiveElement, IntoElement, ParentElement, Render,
    Styled, Window, WindowBounds, WindowOptions, div, px, size,
};
use gpui_kumo::{Appearance, set_appearance};
gpui_kit::actions!(
    toolbar_review,
    [Next, Previous, ToggleItems, ToggleDisabled, ToggleReadonly]
);
struct Review {
    focus: gpui_kit::FocusHandle,
    toolbar: gpui_kit::Entity<toolbar_editors::ToolbarEditors>,
}
impl Render for Review {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = gpui_kumo::theme(cx);
        div()
            .id("review")
            .track_focus(&self.focus)
            .tab_group()
            .on_action(|_: &Next, window, cx| window.focus_next(cx))
            .on_action(|_: &Previous, window, cx| window.focus_prev(cx))
            .on_action(cx.listener(|this, _: &ToggleItems, window, cx| {
                this.toolbar
                    .update(cx, |toolbar, cx| toolbar.toggle_items(window, cx));
            }))
            .on_action(cx.listener(|this, _: &ToggleDisabled, window, cx| {
                this.toolbar
                    .update(cx, |s, cx| s.toggle_disabled(window, cx));
            }))
            .on_action(cx.listener(|this, _: &ToggleReadonly, _, cx| {
                this.toolbar.update(cx, |s, cx| s.toggle_readonly(cx));
            }))
            .flex()
            .flex_col()
            .gap(px(12.))
            .size_full()
            .p(px(32.))
            .bg(theme.colors.canvas)
            .text_color(theme.text.default)
            .font_family(theme.typography.font_family.clone())
            .text_size(theme.typography.base.size)
            .line_height(theme.typography.base.line_height)
            .child(gpui_kumo::Button::new("before", "Before toolbar"))
            .child(self.toolbar.clone())
            .child(gpui_kumo::Button::new("after", "After toolbar"))
    }
}
struct Assets;
impl gpui_kit::AssetSource for Assets {
    fn load(&self, path: &str) -> gpui_kit::Result<Option<std::borrow::Cow<'static, [u8]>>> {
        Ok((path == "toolbar-settings.svg").then(|| {
            std::borrow::Cow::Borrowed(include_bytes!("../assets/toolbar-settings.svg").as_slice())
        }))
    }
    fn list(&self, _: &str) -> gpui_kit::Result<Vec<gpui_kit::SharedString>> {
        Ok(vec!["toolbar-settings.svg".into()])
    }
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let width = args
        .iter()
        .find_map(|arg| {
            arg.strip_prefix("--width=")
                .and_then(|value| value.parse::<f32>().ok())
        })
        .unwrap_or(1040.);
    let dark = args.iter().any(|arg| arg == "--dark");
    gpui_kit::application()
        .with_assets(Assets)
        .run(move |cx: &mut App| {
            gpui_kumo::init(cx);
            cx.bind_keys([
                gpui_kit::KeyBinding::new("tab", Next, None),
                gpui_kit::KeyBinding::new("shift-tab", Previous, None),
                gpui_kit::KeyBinding::new("alt-t", ToggleItems, None),
                gpui_kit::KeyBinding::new("alt-d", ToggleDisabled, None),
                gpui_kit::KeyBinding::new("alt-r", ToggleReadonly, None),
            ]);
            set_appearance(
                if dark {
                    Appearance::Dark
                } else {
                    Appearance::Light
                },
                cx,
            );
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                        None,
                        size(px(width), px(800.)),
                        cx,
                    ))),
                    ..Default::default()
                },
                |window, cx| {
                    cx.new(|cx| Review {
                        toolbar: cx.new(|cx| toolbar_editors::ToolbarEditors::new(window, cx)),
                        focus: cx.focus_handle(),
                    })
                },
            )
            .expect("open toolbar review window");
            cx.activate(true);
        });
}
