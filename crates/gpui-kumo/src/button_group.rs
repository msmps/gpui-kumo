//! A horizontal join of independently owned Base-backed Kumo Buttons.
use crate::Button;
use gpui_kit::StatefulInteractiveElement;
use gpui_kit::{
    App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce, Role, SharedString,
    Styled, Window, base::TestSupportExt, div, prelude::FluentBuilder, px,
};
#[derive(IntoElement)]
#[must_use]
pub struct ButtonGroup {
    id: ElementId,
    name: SharedString,
    items: Vec<Button>,
}
impl ButtonGroup {
    pub fn new(id: impl Into<ElementId>, name: impl Into<SharedString>) -> Self {
        let name = name.into();
        assert!(
            !name.trim().is_empty(),
            "ButtonGroup requires a readable name"
        );
        Self {
            id: id.into(),
            name,
            items: Vec::new(),
        }
    }
    pub fn item(mut self, item: Button) -> Self {
        assert!(
            !self.items.iter().any(|i| i.id() == item.id()),
            "ButtonGroup item IDs must be unique"
        );
        self.items.push(item);
        self
    }
}
impl RenderOnce for ButtonGroup {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let total = self.items.len();
        let columns = u16::try_from(total.max(1)).expect("ButtonGroup exceeds GPUI grid columns");
        let has_ring: Vec<_> = self
            .items
            .iter()
            .map(|item| item.has_group_ring(crate::theme(cx)))
            .collect();
        let rings = crate::button::JoinedRingQueue::default();
        let children = self
            .items
            .into_iter()
            .enumerate()
            .map(|(index, item)| {
                div()
                    .flex()
                    .flex_shrink_0()
                    .when(index > 0, |this| this.ml(px(-1.)))
                    .child(item.group_join(
                        index == 0,
                        index + 1 == total,
                        has_ring.get(index + 1).copied().unwrap_or(false),
                        rings.clone(),
                    ))
                    .into_any_element()
            })
            .collect();
        div().flex().child(
            div()
                .id(self.id)
                .test_support()
                .role(Role::Group)
                .aria_label(self.name)
                .flex()
                .flex_none()
                .child(Joined {
                    children,
                    rings,
                    columns,
                }),
        )
    }
}
// Preserve original layout/prepaint order for traversal, then lift only the
// focused sibling's paint inside this container. Global deferral would escape
// the group and could paint below a containing popup's background.
struct Joined {
    children: Vec<gpui_kit::AnyElement>,
    rings: crate::button::JoinedRingQueue,
    columns: u16,
}
impl gpui_kit::IntoElement for Joined {
    type Element = Self;
    fn into_element(self) -> Self {
        self
    }
}
impl gpui_kit::Element for Joined {
    type RequestLayoutState = ();
    type PrepaintState = ();
    fn id(&self) -> Option<ElementId> {
        None
    }
    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }
    fn request_layout(
        &mut self,
        _: Option<&gpui_kit::GlobalElementId>,
        _: Option<&gpui_kit::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (gpui_kit::LayoutId, ()) {
        let ids: Vec<_> = self
            .children
            .iter_mut()
            .map(|child| child.request_layout(window, cx))
            .collect();
        let style = gpui_kit::Style {
            display: gpui_kit::Display::Grid,
            grid_cols: Some(gpui_kit::GridTemplate {
                repeat: self.columns,
                min_size: gpui_kit::GridTemplateMinSize::MaxContent,
            }),
            ..Default::default()
        };
        (window.request_layout(style, ids, cx), ())
    }
    fn prepaint(
        &mut self,
        _: Option<&gpui_kit::GlobalElementId>,
        _: Option<&gpui_kit::InspectorElementId>,
        _: gpui_kit::Bounds<gpui_kit::Pixels>,
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        for child in &mut self.children {
            child.prepaint(window, cx);
        }
    }
    fn paint(
        &mut self,
        _: Option<&gpui_kit::GlobalElementId>,
        _: Option<&gpui_kit::InspectorElementId>,
        _: gpui_kit::Bounds<gpui_kit::Pixels>,
        _: &mut (),
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        for child in &mut self.children {
            child.paint(window, cx);
        }
        let (focus, dividers): (Vec<_>, Vec<_>) = std::mem::take(&mut *self.rings.borrow_mut())
            .into_iter()
            .partition(|(focus, _, _)| *focus);
        for (_, quad, mask) in dividers.into_iter().chain(focus) {
            window.with_content_mask(Some(mask), |window| window.paint_quad(quad));
        }
    }
}
#[cfg(test)]
mod tests;
