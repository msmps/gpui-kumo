//! Kumo design-system components for GPUI.
//!
//! Component contracts and visual recipes belong here; GPUI Base provides the
//! initial behavior foundation. See `docs/implementation-plan.md` for the slice.

use gpui_kit::App;

/// Initialize the design system before opening application windows.
pub fn init(cx: &mut App) {
    gpui_kit::init(cx);
}
