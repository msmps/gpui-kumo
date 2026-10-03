//! Kumo design-system components for GPUI.
//!
//! Component contracts and visual recipes belong here; GPUI Base provides the
//! initial behavior foundation. See `docs/README.md` for the slice.

use gpui_kit::App;

mod color;
mod icon;

pub mod assets;
pub mod badge;
pub mod banner;
pub mod breadcrumbs;
pub mod button;
pub mod button_group;
pub mod checkbox;
pub mod checkbox_group;
pub mod collapsible;
pub mod dialog;
pub mod dropdown;
pub mod empty;
pub mod field;
pub mod inline_copy_text;
pub mod input;
pub mod input_area;
pub mod input_group;
pub mod label;
pub mod layer_card;
pub mod link;
pub mod loader;
pub mod meter;
pub mod pagination;
pub mod popover;
pub mod radio;
pub mod select;
pub mod sensitive_input;
pub mod skeleton_line;
pub mod switch;
pub mod tabs;
pub mod text;
pub mod theme;
pub mod toast;
pub mod toolbar;
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
pub use field::Field;
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
pub use toolbar::{Toolbar, ToolbarEvent, ToolbarItem, ToolbarState};
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
