//! Enrich Select's actual control node without introducing a second role/focus target.
use gpui_kit::{
    A11ySubtreeBuilder, App, Bounds, Element, ElementId, GlobalElementId, InspectorElementId,
    IntoElement, LayoutId, Pixels, Role, SharedString, Window, accesskit,
};
use std::rc::Rc;

/// Retained only by GPUI's mounted element state, never by SelectState.
pub(super) struct Mount {
    cleanup: Option<Box<dyn FnOnce()>>,
}
impl Mount {
    pub fn new(cleanup: impl FnOnce() + 'static) -> Self {
        Self {
            cleanup: Some(Box::new(cleanup)),
        }
    }
}
impl Drop for Mount {
    fn drop(&mut self) {
        if let Some(cleanup) = self.cleanup.take() {
            cleanup();
        }
    }
}
type MountFactory = dyn Fn(&mut Window, &mut App) -> Rc<Mount>;

pub(super) struct Control<E: Element> {
    pub inner: E,
    pub disabled: bool,
    pub read_only: bool,
    pub invalid: bool,
    pub description: Option<SharedString>,
    pub mount: Box<MountFactory>,
}
impl<E: Element> IntoElement for Control<E> {
    type Element = Self;
    fn into_element(self) -> Self {
        self
    }
}
impl<E: Element> Element for Control<E> {
    type RequestLayoutState = E::RequestLayoutState;
    type PrepaintState = E::PrepaintState;
    fn id(&self) -> Option<ElementId> {
        self.inner.id()
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
        self.inner.request_layout(id, inspector, window, cx)
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
            // GPUI keys persistent state by (element ID, state TypeId), so this
            // does not replace the inner element's own state. Unseen elements release it.
            window.with_element_state(id, |mount: Option<Rc<Mount>>, window| {
                let mount = mount.unwrap_or_else(|| (self.mount)(window, cx));
                ((), mount)
            });
        }
        self.inner
            .prepaint(id, inspector, bounds, layout, window, cx)
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
        self.inner
            .paint(id, inspector, bounds, layout, paint, window, cx)
    }
    fn a11y_role(&self) -> Option<Role> {
        self.inner.a11y_role()
    }
    fn write_a11y_info(&self, node: &mut accesskit::Node) {
        self.inner.write_a11y_info(node);
        if self.disabled {
            node.set_disabled();
        }
        if self.read_only {
            node.set_read_only();
        }
        if self.invalid {
            node.set_invalid(accesskit::Invalid::True);
        }
        if let Some(description) = &self.description {
            node.set_description(description.to_string());
        }
    }
    fn a11y_synthetic_children(
        &mut self,
        paint: &mut Self::PrepaintState,
        builder: &mut A11ySubtreeBuilder,
    ) {
        self.inner.a11y_synthetic_children(paint, builder);
    }
}
