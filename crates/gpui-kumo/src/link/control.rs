//! Shared semantic link root; GPUI provides pointer/keyboard activation.

use gpui_kit::base::{ObservedElement, TestSupportExt};
use gpui_kit::{
    Div, ElementId, FocusHandle, InteractiveElement, MouseButton, Role, SharedString, Stateful,
    StatefulInteractiveElement, div, prelude::FluentBuilder,
};

pub(crate) fn root(
    id: ElementId,
    label: SharedString,
    href: SharedString,
    focus: &FocusHandle,
    disabled: bool,
    tab_stop: bool,
) -> ObservedElement<Stateful<Div>> {
    div()
        .id(id)
        .test_support()
        .role(Role::Link)
        .aria_label(label)
        .when(!disabled, |root| {
            root.track_focus(&focus.clone().tab_stop(tab_stop))
        })
        .when(disabled, |root| {
            root.on_mouse_down(MouseButton::Left, |_, _, cx| {
                cx.stop_propagation();
            })
        })
        .a11y_synthetic_children(move |builder| {
            builder.parent_node().set_url(href.to_string());
            if disabled {
                builder.parent_node().set_disabled();
            }
        })
}
