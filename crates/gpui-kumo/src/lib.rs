//! Kumo design-system components for GPUI.
//!
//! Components own design-system presentation and semantics; GPUI Base stays behind
//! the library boundary. Applications retain editor and overlay entities across renders.
//!
//! The RC targets GPUI Kit 0.7.0 and its GPUI 0.3.7 family. Consume GPUI types
//! through `gpui_kit`; unrelated GPUI packages/sources are not interchangeable.
//! Normal installation uses published dependencies, with known reverse-Tab,
//! Textarea auto-growth and Linux accessibility-state limitations. Development
//! workspace patches are optional and do not propagate to consumers. See the
//! [installation guide](https://github.com/msmps/gpui-kumo/blob/work/README.md#installation)
//! for setup, supported dependency versions and optional corrections.
//!
//! Element IDs are stable identities, independent of localised labels. Textual controls
//! use `label(text)`, `show_label(bool)` and `accessibility_label(text)` where supported.
//! Explicit accessible names win over default labels in either builder order.
//! [`AccessibleName`] validates external names; infallible named-control APIs reject blanks.
//!
//! Standalone form labels are visible by default; `optional_indicator(true)` adds “(optional)”.
//! Use [`Field::control`] to carry label/help/feedback into one associated wrapper.
//! Field errors update supported controls; label availability follows their retained state.
//! Name changes notify without resetting its value, focus, selection or editing history:
//! ```no_run
//! use gpui_kit::{App, Entity};
//! use gpui_kumo::{Field, Input, InputState};
//! fn project_field(input: &Entity<InputState>, cx: &App) -> Field {
//!     Field::control("project-field", Input::new("project", input), cx)
//! }
//! fn localise(input: &Entity<InputState>, cx: &mut App) {
//!     input.update(cx, |state, cx| state.set_name("Nom du projet", cx));
//! }
//! ```
//!
//! Toolbar constructors return kind-specific builders; finish them with `build()` to
//! store a common [`ToolbarItem`]. Unsupported operations fail at compile time.
//! The maintained contract and capability exceptions live in
//! [component construction](https://github.com/msmps/gpui-kumo/blob/work/docs/design-system-components.md#public-api-contract).

#![deny(missing_docs, missing_debug_implementations)]

use gpui_kit::App;

mod color;
mod icon;
mod name;
pub use name::{AccessibleName, BlankName};

/// Embedded assets available to application asset sources.
pub mod assets;
/// Supported Kumo status/category labels with typed filled and dot composition.
pub mod badge;
/// Structured Kumo messages with typed, context-resolved actions.
pub mod banner;
/// Kumo responsive navigation trails over Base-backed Link and Button.
pub mod breadcrumbs;
/// Kumo Button: application-owned props over Base's activation and focus behavior.
pub mod button;
/// A horizontal join of independently owned Base-backed Kumo Buttons.
pub mod button_group;
/// Kumo Checkbox presentation over Base's controlled toggle and focus behavior.
pub mod checkbox;
/// Controlled checkbox selection and Kumo group presentation.
pub mod checkbox_group;
/// Kumo disclosure presentation over Base Collapsible and Button behavior.
pub mod collapsible;
/// Retained Kumo modal lifecycle over Base's unstyled dialog parts.
pub mod dialog;
/// Retained action menus over Base's unstyled popup positioning.
pub mod dropdown;
/// Empty-state presentation with retained, native command-copy feedback.
pub mod empty;
/// Application-owned Field layout and message presentation.
pub mod field;
/// Compact copy controls with Kumo Text recipes over Base Button activation.
pub mod inline_copy_text;
/// Single-line Input with retained editing state and Kumo-owned presentation.
pub mod input;
/// Multiline InputArea over a retained Base Textarea, with Kumo-owned presentation.
pub mod input_area;
/// Shared-container InputGroup presentation over an existing retained editor.
pub mod input_group;
/// Form labels with explicit native focus association.
pub mod label;
/// Simple and layered Kumo card containers with typed section composition.
pub mod layer_card;
/// Kumo navigation links with GPUI-owned focus and activation.
pub mod link;
/// Kumo's circular loading status, shared with Button's decorative indicator.
pub mod loader;
/// Controlled Kumo measurements with native Meter semantics and Base parts.
pub mod meter;
/// Kumo compound pagination with a retained native draft and Base controlled bounds.
pub mod pagination;
/// Nonmodal Popover: Kumo lifecycle and semantics over Base popup positioning.
pub mod popover;
/// Typed Kumo Radio composition over Base Radio and semantic RadioGroup.
pub mod radio;
/// Retained typed Kumo Select with library-owned disclosure and Base positioning.
pub mod select;
/// Retained secret editing with Kumo's reveal, hide and copy presentation.
pub mod sensitive_input;
/// Decorative loading lines with retained sampling and Base CSS timing.
pub mod skeleton_line;
/// Kumo Switch presentation and grouping over Base's controlled binary behavior.
pub mod switch;
/// Typed horizontal Kumo Tabs over Base Tab/TabList semantics and activation.
pub mod tabs;
/// Read-only typography from the pinned Kumo Text recipe.
pub mod text;
/// Application-owned Kumo tokens and theme updates.
pub mod theme;
/// Retained Kumo notification presentation over Base's manager and stack.
pub mod toast;
/// Joined Kumo action/link controls over retained native Base behavior.
pub mod toolbar;
/// Kumo Tooltip over Base popup semantics and positioning.
pub mod tooltip;

pub use badge::Badge;
pub use banner::Banner;
pub use breadcrumbs::{BreadcrumbClipboard, BreadcrumbCurrent, Breadcrumbs};
pub use button::Button;
pub use button_group::ButtonGroup;
pub use checkbox::Checkbox;
pub use checkbox_group::{CheckboxGroup, CheckboxItem};
pub use collapsible::{Collapsible, CollapsiblePanel};
pub use dialog::{
    Dialog, DialogClose, DialogCloseReason, DialogEvent, DialogRole, DialogSize, DialogState,
    DialogTrigger,
};
pub use dropdown::{
    Dropdown, DropdownEvent, DropdownItem, DropdownPart, DropdownState, DropdownVariant,
};
pub use empty::Empty;
pub use field::{Field, FieldControl};
pub use icon::Icon;
pub use inline_copy_text::InlineCopyText;
pub use input::{Input, InputEvent, InputState};
pub use input_area::{InputArea, InputAreaEvent, InputAreaState, Textarea};
pub use input_group::{InputGroup, InputGroupAddon};
pub use label::Label;
pub use layer_card::LayerCard;
pub use link::Link;
pub use loader::Loader;
pub use meter::{Meter, MeterValue};
pub use pagination::{
    Pagination, PaginationEvent, PaginationInfo, PaginationInfoValue, PaginationLabels,
    PaginationPageSize, PaginationParts, PaginationSeparator, PaginationState, PaginationTotal,
};
pub use popover::{Popover, PopoverEvent, PopoverState};
pub use radio::{RadioGroup, RadioItem};
pub use select::{
    Select, SelectEvent, SelectGroup, SelectOption, SelectPart, SelectState, SelectValue,
    SelectValueContent,
};
pub use sensitive_input::{SensitiveInput, SensitiveInputEvent, SensitiveInputState};
pub use skeleton_line::SkeletonLine;
pub use switch::{Switch, SwitchGroup};
pub use tabs::{TabItem, Tabs, TabsEvent, TabsLabels, TabsState};
pub use text::Text;
pub use theme::{Appearance, Theme, set_appearance, set_theme, theme};
pub use toast::{
    Toast, ToastAction, ToastContent, ToastDismissReason, ToastEvent, ToastState, ToastVariant,
    ToastViewport,
};
pub use toolbar::{
    Toolbar, ToolbarButton, ToolbarEvent, ToolbarInput, ToolbarInputGroup, ToolbarItem,
    ToolbarLink, ToolbarState,
};
pub use tooltip::{Tooltip, TooltipEvent, TooltipProvider, TooltipState};

/// Initialize the design system before opening application windows.
pub fn init(cx: &mut App) {
    gpui_kit::init(cx);
    popover::init(cx);
    tooltip::init(cx);
    select::init(cx);
    dialog::init(cx);
    set_theme(Theme::new(Appearance::Light), cx);
}

#[cfg(test)]
mod animation_clock_tests;
#[cfg(test)]
mod readable_label_tests;

#[cfg(test)]
mod api_contract_tests;
