//! Kumo design-system components for GPUI.
//!
//! Component contracts and visual recipes belong here; GPUI Base provides the
//! initial behavior foundation. See `docs/implementation-plan.md` for the slice.

use gpui_kit::App;

pub mod button;
mod color;
mod icon;
pub mod input;
pub mod popover;
pub mod text;
pub mod theme;

pub use button::Button;
pub use icon::Icon;
pub use input::{Input, InputEvent, InputState};
pub use popover::{Popover, PopoverEvent, PopoverState};
pub use text::Text;
pub use theme::{Appearance, Theme, set_appearance, set_theme, theme};

/// Initialize the design system before opening application windows.
pub fn init(cx: &mut App) {
    gpui_kit::init(cx);
    popover::init(cx);
    set_theme(Theme::new(Appearance::Light), cx);
}
