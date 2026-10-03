//! Kumo design-system components for GPUI.
//!
//! Component contracts and visual recipes belong here; GPUI Base provides the
//! initial behavior foundation. See `docs/implementation-plan.md` for the slice.

use gpui_kit::App;

pub mod badge;
pub mod banner;
pub mod button;
mod color;
pub mod empty;
pub mod field;
mod icon;
pub mod input;
pub mod label;
pub mod layer_card;
pub mod link;
pub mod loader;
pub mod popover;
pub mod text;
pub mod theme;

pub use badge::Badge;
pub use banner::Banner;
pub use button::Button;
pub use empty::Empty;
pub use field::Field;
pub use icon::Icon;
pub use input::{Input, InputEvent, InputState};
pub use label::Label;
pub use layer_card::LayerCard;
pub use link::Link;
pub use loader::Loader;
pub use popover::{Popover, PopoverEvent, PopoverState};
pub use text::Text;
pub use theme::{Appearance, Theme, set_appearance, set_theme, theme};

/// Initialize the design system before opening application windows.
pub fn init(cx: &mut App) {
    gpui_kit::init(cx);
    popover::init(cx);
    tooltip::init(cx);
    select::init(cx);
    set_theme(Theme::new(Appearance::Light), cx);
}

pub mod checkbox;
pub use checkbox::Checkbox;

pub mod checkbox_group;
pub use checkbox_group::{CheckboxGroup, CheckboxItem};

pub mod radio;
pub use radio::{RadioGroup, RadioItem};

pub mod switch;
pub use switch::{Switch, SwitchGroup};

pub mod button_group;
pub use button_group::ButtonGroup;

pub mod input_group;
pub use input_group::{InputGroup, InputGroupAddon};

pub mod tooltip;
pub use tooltip::{Tooltip, TooltipEvent, TooltipProvider, TooltipState};

pub mod sensitive_input;
pub use sensitive_input::{SensitiveInput, SensitiveInputEvent, SensitiveInputState};

pub mod input_area;
pub use input_area::{InputArea, InputAreaEvent, InputAreaState, Textarea};

pub mod inline_copy_text;
pub use inline_copy_text::InlineCopyText;

pub mod collapsible;
pub use collapsible::{Collapsible, CollapsiblePanel};

pub mod skeleton_line;
pub use skeleton_line::SkeletonLine;

pub mod meter;
pub use meter::{Meter, MeterValue};

pub mod breadcrumbs;
pub use breadcrumbs::{BreadcrumbClipboard, BreadcrumbCurrent, Breadcrumbs};

#[cfg(test)]
mod select_base_tests;

pub mod select;
pub use select::{
    Select, SelectEvent, SelectGroup, SelectOption, SelectPart, SelectState, SelectValue,
    SelectValueContent,
};

pub mod pagination;
pub use pagination::{
    Pagination, PaginationEvent, PaginationInfo, PaginationInfoValue, PaginationLabels,
    PaginationPageSize, PaginationParts, PaginationSeparator, PaginationState, PaginationTotal,
};

#[cfg(test)]
mod readable_label_tests;

#[cfg(test)]
mod animation_clock_tests;

pub mod tabs;
pub use tabs::{TabItem, Tabs, TabsEvent, TabsLabels, TabsState};

pub mod toolbar;
pub use toolbar::{Toolbar, ToolbarEvent, ToolbarItem, ToolbarState};

pub mod dropdown;
pub use dropdown::{
    Dropdown, DropdownEvent, DropdownItem, DropdownPart, DropdownState, DropdownVariant,
};
