//! Shared-container InputGroup presentation over an existing retained editor.
use crate::{Input, InputState, Theme, input::Size};
use gpui_kit::base::TestSupportExt;
use gpui_kit::{
    App, ElementId, Entity, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, Styled, Window, div, px,
};

type ActionFactory = dyn Fn(Size, &mut Window, &mut App) -> crate::Button;
pub(crate) struct AddonContext<'a> {
    /// Theme.
    pub theme: &'a Theme,
    /// Size.
    pub size: Size,
    /// Disabled.
    pub disabled: bool,
    /// Start.
    pub start: bool,
    /// Focus.
    pub focus: &'a mut AddonFocus,
}

/// Retain compact action identity without retaining rendered buttons or callbacks.
#[derive(Default)]
pub(crate) struct AddonFocus {
    handles: std::collections::HashMap<(bool, ElementId), gpui_kit::FocusHandle>,
    seen: std::collections::HashSet<(bool, ElementId)>,
    available: Vec<gpui_kit::FocusHandle>,
}
impl AddonFocus {
    /// Configure begin.
    pub fn begin(&mut self, window: &Window) -> Option<gpui_kit::FocusHandle> {
        self.seen.clear();
        self.available.clear();
        self.handles
            .values()
            .find(|h| h.is_focused(window))
            .cloned()
    }
    pub(crate) fn button(
        &mut self,
        button: crate::Button,
        start: bool,
        disabled: bool,
        cx: &mut App,
    ) -> crate::Button {
        let key = (start, button.id().clone());
        assert!(
            self.seen.insert(key.clone()),
            "InputGroup addon action IDs must be unique within each addon"
        );
        let handle = button
            .provided_focus()
            .or_else(|| self.handles.get(&key).cloned())
            .unwrap_or_else(|| cx.focus_handle());
        self.handles.insert(key, handle.clone());
        if !disabled && !button.is_unavailable() {
            self.available.push(handle.clone());
        }
        button.track_focus(&handle)
    }
    /// Report whether available applies to this component.
    pub fn is_available(&self, handle: &gpui_kit::FocusHandle) -> bool {
        self.available.contains(handle)
    }
    /// Configure finish.
    pub fn finish(
        &mut self,
        previous: Option<gpui_kit::FocusHandle>,
    ) -> Option<gpui_kit::FocusHandle> {
        self.handles.retain(|id, _| self.seen.contains(id));
        previous.filter(|handle| !self.available.contains(handle))
    }
}

#[derive(Clone)]
/// Passive decoration or an independently focusable compact action.
pub enum InputGroupAddon {
    /// Passive text addon.
    Text(SharedString),
    /// Decorative SVG asset path.
    Icon(SharedString),
    /// Nested addon parts, flattened during rendering.
    Parts(Vec<InputGroupAddon>),
    /// Requested by an application action.
    Action(std::rc::Rc<ActionFactory>),
}
impl InputGroupAddon {
    /// Create a passive text addon.
    pub fn text(text: impl Into<SharedString>) -> Self {
        Self::Text(text.into())
    }
    /// Supply a decorative icon; icons do not replace the control’s accessible name.
    pub fn icon(path: impl Into<SharedString>) -> Self {
        Self::Icon(path.into())
    }
    /// Several addon items share source padding and a 6px gap.
    /// Nested collections are flattened; empty collections render no addon.
    pub fn parts(parts: impl IntoIterator<Item = Self>) -> Self {
        Self::Parts(parts.into_iter().collect())
    }
    pub(crate) fn is_empty(&self) -> bool {
        match self {
            Self::Parts(parts) => parts.iter().all(Self::is_empty),
            _ => false,
        }
    }
    fn items<'a>(&'a self, output: &mut Vec<&'a Self>) {
        match self {
            Self::Parts(parts) => parts.iter().for_each(|part| part.items(output)),
            _ => output.push(self),
        }
    }
    /// Configure a source-sized ghost button each render.
    ///
    /// This factory is retained by InputState. Capture WeakEntity handles for
    /// that state and its owner: strong captures form a reference cycle.
    /// ```
    /// # use gpui_kumo::{InputGroup, InputGroupAddon, InputState};
    /// # use gpui_kit::Entity;
    /// # fn clear_group(input: &Entity<InputState>) -> InputGroup {
    /// let target = input.downgrade();
    /// InputGroup::new("search", input).end(InputGroupAddon::button(
    ///     "clear", "Clear", move |button, _, _| {
    ///         let target = target.clone();
    ///         button.on_click(move |_, window, cx| {
    ///             let _ = target.update(cx, |state, cx| state.set_value("", window, cx));
    ///         })
    ///     },
    /// ))
    /// # }
    /// ```
    ///
    /// # Panics
    /// Panics during rendering when the label is blank or action identity repeats
    /// within the same addon.
    pub fn button(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        configure: impl Fn(crate::Button, &mut Window, &mut App) -> crate::Button + 'static,
    ) -> Self {
        let id = id.into();
        let label = label.into();
        Self::Action(std::rc::Rc::new(move |size, window, cx| {
            configure(
                crate::Button::new(id.clone(), label.clone())
                    .variant(crate::button::Variant::Ghost)
                    .size(compact_size(size)),
                window,
                cx,
            )
        }))
    }
    /// Icon-only action; the factory follows the same weak-capture contract as `button`.
    ///
    /// # Panics
    /// Panics when the required accessible name is blank.
    pub fn icon_button(
        id: impl Into<ElementId>,
        name: impl Into<SharedString>,
        path: impl Into<SharedString>,
        configure: impl Fn(crate::Button, &mut Window, &mut App) -> crate::Button + 'static,
    ) -> Self {
        let id = id.into();
        let name = name.into();
        let path = path.into();
        Self::Action(std::rc::Rc::new(move |size, window, cx| {
            let icon_size = match size {
                Size::Xs => 10.,
                Size::Sm => 13.,
                Size::Base => 18.,
                Size::Lg => 20.,
            };
            configure(
                crate::Button::icon(
                    id.clone(),
                    name.clone(),
                    crate::Icon::new(path.clone()).size(px(icon_size)),
                )
                .variant(crate::button::Variant::Ghost)
                .size(compact_size(size)),
                window,
                cx,
            )
        }))
    }
    pub(crate) fn render(
        &self,
        id: &'static str,
        context: AddonContext<'_>,
        window: &mut Window,
        cx: &mut App,
    ) -> gpui_kit::AnyElement {
        let AddonContext {
            theme,
            size,
            disabled,
            start,
            focus,
        } = context;
        let (outer, icon_size) = match size {
            Size::Xs => (6., 10.),
            Size::Sm => (6., 13.),
            Size::Base => (8., 18.),
            Size::Lg => (10., 20.),
        };
        let mut items = Vec::new();
        self.items(&mut items);
        if items.is_empty() {
            return div().into_any_element();
        }
        let outer = if items.iter().any(|item| matches!(item, Self::Action(_))) {
            match (size, start) {
                (Size::Lg, true) => 6.,
                (Size::Lg, false) => 2.,
                _ => 4.,
            }
        } else {
            outer
        };
        let addon = div()
            .id(id)
            .test_support()
            .flex()
            .items_center()
            .flex_shrink_0()
            .gap(theme.spacing.six)
            .text_color(theme.text.subtle);
        let addon = if start {
            addon.pl(px(outer))
        } else {
            addon.pr(px(outer))
        };
        addon
            .children(items.into_iter().enumerate().map(|(index, item)| {
                let child = div().flex().items_center().child(match item {
                    Self::Action(render) => div()
                        .child(
                            focus
                                .button(render(size, window, cx), start, disabled, cx)
                                .input_group_action(disabled),
                        )
                        .into_any_element(),
                    Self::Text(text) => div().child(text.clone()).into_any_element(),
                    Self::Icon(path) => crate::Icon::new(path.clone())
                        .size(px(icon_size))
                        .into_any_element(),
                    Self::Parts(_) => {
                        unreachable!("collections are flattened before rendering")
                    }
                });
                // Action IDs are caller-owned; positional ancestor IDs would
                // recreate Base's keyed focus when passive parts move.
                if matches!(item, Self::Action(_)) {
                    child.into_any_element()
                } else {
                    child
                        .id(("addon-item", index))
                        .test_support()
                        .into_any_element()
                }
            }))
            .into_any_element()
    }
}
#[derive(Default)]
pub(crate) struct Container {
    /// Start.
    pub start: Option<InputGroupAddon>,
    /// End.
    pub end: Option<InputGroupAddon>,
    /// Suffix.
    pub suffix: Option<SharedString>,
    /// Buttons.
    pub buttons: Vec<std::rc::Rc<ActionFactory>>,
    /// Leading buttons.
    pub leading_buttons: Vec<std::rc::Rc<ActionFactory>>,
    /// Editor width.
    pub editor_width: Option<gpui_kit::Pixels>,
    /// Text align.
    pub text_align: gpui_kit::TextAlign,
}
/// Initial shared-container slice. The retained InputState owns editing and availability.
/// Passive addons use source padding and icon sizes; suffixes follow displayed text.
/// Compact actions retain Base activation and shared focus-within.
/// Direct non-ghost actions use individual or hybrid joined borders.
/// Individual mode preserves leading/editor/trailing order. Hybrid mode puts
/// all direct actions after the shared editor zone, matching Kumo partitioning.
#[derive(IntoElement)]
#[must_use]
pub struct InputGroup {
    input: Input,
    container: Container,
}
impl InputGroup {
    /// Present one retained editor with optional decorations and compact actions.
    pub fn new(id: impl Into<ElementId>, state: &Entity<InputState>) -> Self {
        Self {
            input: Input::new(id, state),
            container: Container::default(),
        }
    }
    pub(crate) fn toolbar_focus(mut self, hooks: crate::toolbar::InputFocus) -> Self {
        self.input = self.input.toolbar_focus(hooks);
        self
    }
    /// Select the component dimensions and corresponding spacing and typography.
    pub fn size(mut self, size: Size) -> Self {
        self.input = self.input.size(size);
        self
    }
    /// Set visible text and its default accessible name.
    ///
    /// # Panics
    /// Panics when `text` is blank.
    pub fn label(mut self, text: impl Into<SharedString>) -> Self {
        self.input = self.input.label(text);
        self
    }
    /// Override the accessible name regardless of builder ordering.
    ///
    /// # Panics
    /// Panics when `name` is blank.
    pub fn accessibility_label(mut self, name: impl Into<SharedString>) -> Self {
        self.input = self.input.accessibility_label(name);
        self
    }
    /// Choose visible label presentation without changing the accessible name.
    pub fn show_label(mut self, show: bool) -> Self {
        self.input = self.input.show_label(show);
        self
    }
    /// Independent contextual help beside the group's visible label.
    pub fn label_tooltip(
        mut self,
        state: &Entity<crate::TooltipState>,
        content: impl Into<SharedString>,
    ) -> Self {
        self.input = self.input.label_tooltip(state, content);
        self
    }
    /// Explicit false adds the optional indicator to a visible label.
    /// This is presentation; validation and editor required semantics stay owner-controlled.
    pub fn required(mut self, required: bool) -> Self {
        self.input = self.input.required(required);
        self
    }
    /// An error suppresses helper text even when its message is hidden.
    /// The group remains invalid; `show` replaces the browser match result.
    pub fn error_visible(mut self, text: impl Into<SharedString>, show: bool) -> Self {
        self.input = self.input.error_visible(text.into(), show);
        self
    }
    /// Supply supporting text; errors take precedence even when their message is hidden.
    pub fn description(mut self, text: impl Into<SharedString>) -> Self {
        self.input = self.input.description(text);
        self
    }
    /// Display an application-owned error; values are retained and helper text is suppressed.
    pub fn error(mut self, text: impl Into<SharedString>) -> Self {
        self.input = self.input.error(text);
        self
    }
    /// Supply leading InputGroup content under the shared addon contract.
    pub fn start(mut self, addon: InputGroupAddon) -> Self {
        self.container.start = Some(addon);
        self
    }
    /// Text immediately follows the displayed value; long suffixes truncate.
    pub fn suffix(mut self, text: impl Into<SharedString>) -> Self {
        self.container.suffix = Some(text.into());
        self
    }
    /// Append a direct action. Non-ghost variants select joined border zones.
    /// The group controls sizing; use addon buttons for compact ghost actions.
    /// Factories are retained: capture weak handles for the editor or its owner.
    ///
    /// # Panics
    /// Panics during rendering when the label is blank or action identity repeats
    /// within the same addon.
    pub fn button(
        mut self,
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        variant: crate::button::Variant,
        configure: impl Fn(crate::Button, &mut Window, &mut App) -> crate::Button + 'static,
    ) -> Self {
        let id = id.into();
        let label = label.into();
        self.container
            .buttons
            .push(std::rc::Rc::new(move |size, window, cx| {
                configure(
                    crate::Button::new(id.clone(), label.clone())
                        .variant(variant)
                        .size(button_size(size)),
                    window,
                    cx,
                )
            }));
        self
    }
    /// A direct action before the editor in individual mode. With addons,
    /// Kumo hybrid partitioning puts all direct actions after the editor zone.
    /// Factories are retained; capture editor/owner entities weakly.
    ///
    /// # Panics
    /// Panics during rendering for blank labels or duplicate direct-action IDs.
    pub fn leading_button(
        mut self,
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        variant: crate::button::Variant,
        configure: impl Fn(crate::Button, &mut Window, &mut App) -> crate::Button + 'static,
    ) -> Self {
        self = self.button(id, label, variant, configure);
        self.container
            .leading_buttons
            .push(self.container.buttons.pop().unwrap());
        self
    }
    /// The complete editor surface width, including its source padding/border.
    /// Suffix sizing and narrow parents can still constrain the editor.
    ///
    /// # Panics
    /// Panics when width is nonfinite or nonpositive.
    pub fn editor_width(mut self, width: gpui_kit::Pixels) -> Self {
        assert!(
            f32::from(width).is_finite() && width > px(0.),
            "editor width must be finite and positive"
        );
        self.container.editor_width = Some(width);
        self
    }
    /// Native single-line alignment retains Base caret, selection and scrolling.
    pub fn text_align(mut self, align: gpui_kit::TextAlign) -> Self {
        self.container.text_align = align;
        self
    }
    /// Supply trailing InputGroup content under the shared addon contract.
    pub fn end(mut self, addon: InputGroupAddon) -> Self {
        self.container.end = Some(addon);
        self
    }
}
impl RenderOnce for InputGroup {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        self.input.group(self.container)
    }
}

// Installed Base 0.7.0 reserves this right-side caret-scroll margin.
// Glyph/hitbox clipping excludes the overlap; reserve the platform caret width
// as a small native gap so an End caret remains visible.
pub(crate) const EDITOR_CARET_MARGIN: gpui_kit::Pixels = px(10.);

#[cfg(not(target_os = "macos"))]
pub(crate) const SUFFIX_OVERLAP: gpui_kit::Pixels = px(8.);
#[cfg(target_os = "macos")]
pub(crate) const SUFFIX_OVERLAP: gpui_kit::Pixels = px(8.5);

pub(crate) fn compact_size(size: Size) -> crate::button::Size {
    match size {
        Size::Xs | Size::Sm => crate::button::Size::Xs,
        Size::Base => crate::button::Size::Sm,
        Size::Lg => crate::button::Size::Base,
    }
}

pub(crate) fn height(size: Size) -> gpui_kit::Pixels {
    px(match size {
        Size::Xs => 24.,
        Size::Sm => 28.,
        Size::Base => 36.,
        Size::Lg => 44.,
    })
}
pub(crate) fn button_size(size: Size) -> crate::button::Size {
    match size {
        Size::Xs => crate::button::Size::Xs,
        Size::Sm => crate::button::Size::Sm,
        Size::Base => crate::button::Size::Base,
        Size::Lg => crate::button::Size::Lg,
    }
}
#[derive(Clone)]
pub(crate) struct Zone {
    /// Height.
    pub height: gpui_kit::Pixels,
    /// Radius.
    pub radius: gpui_kit::Pixels,
    /// First.
    pub first: bool,
    /// Last.
    pub last: bool,
    /// Borders.
    pub borders: crate::button::JoinedRingQueue,
}
// Preserve native layout/prepaint/Tab order; paint each inside border and one
// seam after sibling surfaces. No global deferred paint escapes a popup.
pub(crate) struct Zoned {
    /// Body.
    pub body: gpui_kit::AnyElement,
    /// Borders.
    pub borders: crate::button::JoinedRingQueue,
}
impl IntoElement for Zoned {
    type Element = Self;
    fn into_element(self) -> Self {
        self
    }
}
impl gpui_kit::Element for Zoned {
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
        (self.body.request_layout(window, cx), ())
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
        self.body.prepaint(window, cx);
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
        self.body.paint(window, cx);
        let mut borders = std::mem::take(&mut *self.borders.borrow_mut());
        borders.sort_by(|a, b| a.1.bounds.left().partial_cmp(&b.1.bounds.left()).unwrap());
        for (index, (_, quad, mask)) in borders.iter().enumerate() {
            let mut quad = quad.clone();
            if index > 0 {
                quad.border_widths.left = px(0.);
            }
            if index + 1 < borders.len() {
                quad.border_widths.right = px(0.);
            }
            window.with_content_mask(Some(*mask), |window| window.paint_quad(quad));
        }
        for pair in borders.windows(2) {
            let chosen = if pair[0].0 { &pair[0] } else { &pair[1] };
            let seam = gpui_kit::quad(
                gpui_kit::Bounds::new(
                    pair[1].1.bounds.origin,
                    gpui_kit::size(px(1.), pair[1].1.bounds.size.height),
                ),
                px(0.),
                chosen.1.border_color,
                px(0.),
                chosen.1.border_color.alpha(0.),
                Default::default(),
            );
            window.with_content_mask(Some(chosen.2), |window| window.paint_quad(seam));
        }
    }
}

impl crate::field::sealed::Sealed for InputGroup {}
impl crate::field::FieldControl for InputGroup {
    fn into_field_parts(
        self,
        cx: &App,
    ) -> (SharedString, gpui_kit::FocusHandle, gpui_kit::AnyElement) {
        let (label, focus) = self.input.field_identity(cx);
        (label, focus, self.show_label(false).into_any_element())
    }
}

impl std::fmt::Debug for InputGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_struct("InputGroup");
        debug.field("input", &self.input);
        debug.field("has_start", &self.container.start.is_some());
        debug.field("has_end", &self.container.end.is_some());
        debug.field(
            "actions",
            &(self.container.buttons.len() + self.container.leading_buttons.len()),
        );
        debug.finish_non_exhaustive()
    }
}

impl std::fmt::Debug for InputGroupAddon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(_) => f.debug_tuple("Text").field(&"<text>").finish(),
            Self::Icon(path) => f.debug_tuple("Icon").field(path).finish(),
            Self::Parts(parts) => f.debug_tuple("Parts").field(parts).finish(),
            Self::Action(_) => f.debug_tuple("Action").field(&"<factory>").finish(),
        }
    }
}

#[cfg(test)]
mod tests;
