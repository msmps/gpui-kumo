//! Mount-scoped transient cleanup; retaining the owner does not retain a drag.
use super::*;
use gpui_kit::{
    A11ySubtreeBuilder, Bounds, Element, GlobalElementId, InspectorElementId, LayoutId, Pixels,
    Role, accesskit,
};
struct Mount {
    drag: Rc<RefCell<Option<overflow::Drag>>>,
    scroll: Rc<RefCell<Option<overflow::ScrollMotion>>>,
    indicator: Rc<RefCell<motion::IndicatorMotion>>,
    valid: Rc<Cell<bool>>,
}
impl Drop for Mount {
    fn drop(&mut self) {
        self.valid.set(false);
        self.drag.borrow_mut().take();
        self.scroll.borrow_mut().take();
        *self.indicator.borrow_mut() = motion::IndicatorMotion::default();
    }
}
pub(super) struct Mounted {
    pub inner: AnyElement,
    pub drag: Rc<RefCell<Option<overflow::Drag>>>,
    pub scroll: Rc<RefCell<Option<overflow::ScrollMotion>>>,
    pub indicator: Rc<RefCell<motion::IndicatorMotion>>,
    pub valid: Rc<Cell<bool>>,
}
impl IntoElement for Mounted {
    type Element = Self;
    fn into_element(self) -> Self {
        self
    }
}
impl Element for Mounted {
    type RequestLayoutState = <AnyElement as Element>::RequestLayoutState;
    type PrepaintState = <AnyElement as Element>::PrepaintState;
    fn id(&self) -> Option<ElementId> {
        Some("tabs-lifecycle".into())
    }
    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        self.inner.source_location()
    }
    fn request_layout(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        Element::request_layout(&mut self.inner, id, inspector, window, cx)
    }
    fn prepaint(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        if let Some(id) = id {
            window.with_element_state(id, |mount: Option<Rc<Mount>>, _| {
                (
                    (),
                    mount.unwrap_or_else(|| {
                        Rc::new(Mount {
                            drag: self.drag.clone(),
                            scroll: self.scroll.clone(),
                            indicator: self.indicator.clone(),
                            valid: self.valid.clone(),
                        })
                    }),
                )
            });
        }
        Element::prepaint(&mut self.inner, id, inspector, bounds, layout, window, cx)
    }
    fn paint(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        layout: &mut Self::RequestLayoutState,
        paint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        Element::paint(
            &mut self.inner,
            id,
            inspector,
            bounds,
            layout,
            paint,
            window,
            cx,
        )
    }
    fn a11y_role(&self) -> Option<Role> {
        self.inner.a11y_role()
    }
    fn write_a11y_info(&self, node: &mut accesskit::Node) {
        self.inner.write_a11y_info(node)
    }
    fn a11y_synthetic_children(
        &mut self,
        paint: &mut Self::PrepaintState,
        builder: &mut A11ySubtreeBuilder,
    ) {
        self.inner.a11y_synthetic_children(paint, builder)
    }
}
