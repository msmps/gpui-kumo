use gpui_kit::{App, IntoElement, Pixels, RenderOnce, SharedString, Styled, Window, px, svg};

/// A decorative SVG asset tinted with the surrounding text foreground.
/// The application supplies the asset through its `AssetSource`.
#[derive(IntoElement)]
#[must_use]
pub struct Icon {
    path: SharedString,
    size: Pixels,
}

impl Icon {
    /// Create a decorative SVG from the application’s asset path.
    pub fn new(path: impl Into<SharedString>) -> Self {
        Self {
            path: path.into(),
            size: px(16.),
        }
    }

    /// Set the decorative icon’s width and height in logical pixels.
    pub fn size(mut self, size: Pixels) -> Self {
        self.size = size;
        self
    }
}

impl RenderOnce for Icon {
    fn render(self, window: &mut Window, _: &mut App) -> impl IntoElement {
        svg()
            .path(self.path)
            .size(self.size)
            .flex_shrink_0()
            .text_color(window.text_style().color)
    }
}

impl std::fmt::Debug for Icon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Icon")
            .field("path", &self.path)
            .finish_non_exhaustive()
    }
}
