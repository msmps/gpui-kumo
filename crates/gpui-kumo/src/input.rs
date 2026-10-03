//! Single-line Input with retained editing state and Kumo-owned presentation.

use gpui_kit::{
    App, AppContext, Context, ElementId, Entity, EntityInputHandler, EventEmitter, FocusHandle,
    Focusable, FontWeight, InteractiveElement, IntoElement, ParentElement, Render, RenderOnce,
    SharedString, StatefulInteractiveElement, Styled, Subscription, Window, base, canvas, div,
    prelude::FluentBuilder, px, quad,
};

use crate::{Theme, theme};
use gpui_kit::base::TestSupportExt;

mod accessibility;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Size {
    Xs,
    Sm,
    #[default]
    Base,
    Lg,
}

/// Shared source input dimensions; group overrides remain in InputGroup.
pub(crate) fn metrics(
    size: Size,
    theme: &Theme,
) -> (
    f32,
    gpui_kit::Pixels,
    gpui_kit::Pixels,
    crate::theme::TextStyle,
) {
    match size {
        Size::Xs => (20., theme.spacing.six, theme.radii.sm, theme.typography.xs),
        Size::Sm => (
            26.,
            theme.spacing.eight,
            theme.radii.md,
            theme.typography.xs,
        ),
        Size::Base => (
            36.,
            theme.spacing.twelve,
            theme.radii.lg,
            theme.typography.base,
        ),
        Size::Lg => (
            40.,
            theme.spacing.sixteen,
            theme.radii.lg,
            theme.typography.base,
        ),
    }
}

/// Notifications from the editing engine. Read the value from the emitting state.
#[derive(Clone, Debug)]
pub enum InputEvent {
    Change,
    Submit { secondary: bool, shift: bool },
    Focus,
    Blur,
}

/// Durable value, selection, clipboard/IME handling, history and focus.
/// Create once with `cx.new`, retain the entity, and mount it in one Input.
/// Programmatic `set_value` resets selection/history without emitting Change.
pub struct InputState {
    editor: Entity<base::input::InputState>,
    name: SharedString,
    disabled: bool,
    read_only: bool,
    toolbar_disabled: bool,
    presentation: Presentation,
    accessibility: accessibility::Bridge,
    group_focus: Option<FocusHandle>,
    group_zone_focus: Option<FocusHandle>,
    group_button_focus: std::collections::HashMap<ElementId, FocusHandle>,
    _subscriptions: Vec<Subscription>,
}

impl EventEmitter<InputEvent> for InputState {}

impl InputState {
    /// The accessible name must be nonempty; a placeholder is not a label.
    pub fn new(name: impl Into<SharedString>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let name = name.into();
        assert!(!name.trim().is_empty(), "Input requires an accessible name");
        let editor = cx.new(|cx| base::input::InputState::new(window, cx));
        let events = cx.subscribe(&editor, |_, _, event, cx| {
            let event = match event {
                base::input::InputEvent::Change => InputEvent::Change,
                base::input::InputEvent::PressEnter { secondary, shift } => InputEvent::Submit {
                    secondary: *secondary,
                    shift: *shift,
                },
                base::input::InputEvent::Focus => InputEvent::Focus,
                base::input::InputEvent::Blur => InputEvent::Blur,
            };
            cx.emit(event);
            cx.notify();
        });
        let theme = cx.observe_global::<Theme>(|_, cx| cx.notify());
        // Selection-only notifications do not emit InputEvent::Change. Refresh
        // the containing accessibility node when Base moves its caret as well.
        let editor_observer = cx.observe(&editor, |_, _, cx| cx.notify());
        Self {
            editor,
            name,
            disabled: false,
            read_only: false,
            toolbar_disabled: false,
            presentation: Presentation::default(),
            accessibility: accessibility::Bridge::default(),
            group_focus: None,
            group_zone_focus: None,
            group_button_focus: std::collections::HashMap::new(),
            _subscriptions: vec![events, theme, editor_observer],
        }
    }

    pub(crate) fn set_name(&mut self, name: SharedString, cx: &mut Context<Self>) {
        assert!(!name.trim().is_empty(), "Input requires an accessible name");
        self.name = name;
        cx.notify();
    }

    pub fn value(&self, cx: &App) -> SharedString {
        self.editor.read(cx).value()
    }

    pub fn selected_value(&self, cx: &App) -> SharedString {
        self.editor.read(cx).selected_value()
    }

    pub fn set_value(
        &mut self,
        value: impl Into<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let value = value.into();
        self.editor
            .update(cx, |editor, cx| editor.set_value(value, window, cx));
        cx.notify();
    }

    pub fn set_placeholder(
        &mut self,
        placeholder: impl Into<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let placeholder = placeholder.into();
        self.editor.update(cx, |editor, cx| {
            editor.set_placeholder(placeholder, window, cx)
        });
        cx.notify();
    }

    pub fn is_disabled(&self) -> bool {
        self.disabled
    }
    pub fn is_read_only(&self) -> bool {
        self.read_only
    }

    pub(crate) fn set_masked(&mut self, masked: bool, window: &mut Window, cx: &mut Context<Self>) {
        if self.editor.read(cx).presentation().is_masked() != masked {
            self.editor
                .update(cx, |editor, cx| editor.set_masked(masked, window, cx));
            cx.notify();
        }
    }

    /// Reject user edits and exclude the control from Tab traversal.
    /// Existing focus is retained; the owner may move it explicitly.
    pub fn set_disabled(&mut self, disabled: bool, cx: &mut Context<Self>) {
        if self.disabled == disabled {
            return;
        }
        self.disabled = disabled;
        self.sync_editing_availability(cx);
        self.focus_handle(cx).tab_stop(!disabled);
        cx.notify();
    }

    pub(crate) fn set_toolbar_disabled(&mut self, disabled: bool, cx: &mut Context<Self>) {
        if self.toolbar_disabled != disabled {
            self.toolbar_disabled = disabled;
            self.sync_editing_availability(cx);
            cx.notify();
        }
    }
    fn sync_editing_availability(&self, cx: &mut Context<Self>) {
        let unavailable = self.effectively_disabled();
        let toolbar = self.presentation.toolbar.is_some();
        // Source's focusable aria-disabled Inputs remain opaque. Use Base's
        // existing edit guard and keep authored disabled semantics on the facade.
        self.editor.update(cx, |editor, cx| {
            editor.set_disabled(unavailable && !toolbar, cx);
            editor.set_readonly(self.read_only || (unavailable && toolbar), cx);
        });
    }
    pub(super) fn effectively_disabled(&self) -> bool {
        self.disabled || self.toolbar_disabled
    }
    pub(crate) fn toolbar_group_contains_focus(&self, window: &Window, cx: &App) -> bool {
        self.group_focus
            .as_ref()
            .is_some_and(|scope| scope.contains_focused(window, cx))
    }
    pub(crate) fn toolbar_arrow_at_boundary(
        &mut self,
        forward: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        self.editor.update(cx, |editor, cx| {
            editor.marked_text_range(window, cx).is_none()
                && editor.selected_range().is_empty()
                && if forward {
                    editor.cursor() == editor.value().len()
                } else {
                    editor.cursor() == 0
                }
        })
    }
    /// Preserve focus, selection and copying while rejecting user edits.
    pub fn set_read_only(&mut self, read_only: bool, cx: &mut Context<Self>) {
        if self.read_only == read_only {
            return;
        }
        self.read_only = read_only;
        self.sync_editing_availability(cx);
        cx.notify();
    }
}

impl Focusable for InputState {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.read(cx).focus_handle(cx)
    }
}

#[derive(Default)]
struct Presentation {
    size: Size,
    label: bool,
    tooltip: Option<(Entity<crate::TooltipState>, SharedString)>,
    optional: bool,
    description: Option<SharedString>,
    error: Option<(SharedString, bool)>,
    group: Option<crate::input_group::Container>,
    end_reserve: gpui_kit::Pixels,
    focus_scope: Option<FocusHandle>,
    toolbar: Option<crate::toolbar::InputFocus>,
}

/// Consumed presentation over an application-retained `Entity<InputState>`.
/// Width fills the parent; use a bounded parent for narrower fields.
/// Errors are application-owned display state and do not reject editing.
#[derive(IntoElement)]
#[must_use]
pub struct Input {
    id: ElementId,
    state: Entity<InputState>,
    presentation: Presentation,
}

impl Input {
    pub fn new(id: impl Into<ElementId>, state: &Entity<InputState>) -> Self {
        Self {
            id: id.into(),
            state: state.clone(),
            presentation: Presentation::default(),
        }
    }
    pub(crate) fn toolbar_focus(mut self, hooks: crate::toolbar::InputFocus) -> Self {
        self.presentation.toolbar = Some(hooks);
        self
    }
    pub(crate) fn group(mut self, group: crate::input_group::Container) -> Self {
        self.presentation.group = Some(group);
        self
    }
    pub fn size(mut self, size: Size) -> Self {
        self.presentation.size = size;
        self
    }
    /// Show the state's accessible name as a clickable field label.
    pub fn label(mut self, show: bool) -> Self {
        self.presentation.label = show;
        self
    }
    /// Contextual help beside a visible label. Retain the TooltipState once.
    /// Editing availability does not disable the independent help trigger.
    pub fn label_tooltip(
        mut self,
        state: &Entity<crate::TooltipState>,
        content: impl Into<SharedString>,
    ) -> Self {
        self.presentation.tooltip = Some((state.clone(), content.into()));
        self
    }
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.presentation.description = Some(description.into());
        self
    }
    /// Error text replaces the description and selects the danger ring.
    pub fn error(mut self, error: impl Into<SharedString>) -> Self {
        self.presentation.error = Some((error.into(), true));
        self
    }
    pub(crate) fn required(mut self, required: bool) -> Self {
        self.presentation.optional = !required;
        self
    }
    pub(crate) fn sensitive(mut self, reserve: gpui_kit::Pixels, scope: &FocusHandle) -> Self {
        self.presentation.end_reserve = reserve;
        self.presentation.focus_scope = Some(scope.clone());
        self
    }
    pub(crate) fn error_visible(mut self, text: SharedString, show: bool) -> Self {
        self.presentation.error = Some((text, show));
        self
    }
}

impl RenderOnce for Input {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        // Presentation is refreshed by the owner; the editing entity is never recreated.
        self.state.update(cx, |state, cx| {
            if let Some((old, _)) = &state.presentation.tooltip
                && self
                    .presentation
                    .tooltip
                    .as_ref()
                    .is_none_or(|(new, _)| old != new)
            {
                old.update(cx, |tooltip, cx| tooltip.set_open(false, cx));
            }
            let leaving_toolbar =
                state.presentation.toolbar.is_some() && self.presentation.toolbar.is_none();
            state.presentation = self.presentation;
            state.sync_editing_availability(cx);
            if leaving_toolbar {
                state.focus_handle(cx).tab_stop(!state.disabled);
            }
            if !state.presentation.label
                && let Some((tooltip, _)) = &state.presentation.tooltip
            {
                tooltip.update(cx, |tooltip, cx| tooltip.set_open(false, cx));
            }
        });
        div().id(self.id).w_full().min_w_0().child(self.state)
    }
}

impl Render for InputState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.presentation.toolbar.is_none() {
            self.set_toolbar_disabled(false, cx);
        }
        let disabled = self.effectively_disabled();
        let group_disabled = self.disabled;
        let surface_disabled = if self.presentation.group.is_some() {
            group_disabled
        } else {
            disabled
        };
        let toolbar = self.presentation.toolbar.as_ref();
        let focus = self.focus_handle(cx);
        if let Some(hooks) = toolbar {
            focus.tab_stop(hooks.tab_stop);
        }
        let theme = theme(cx).clone();
        let (mut height, padding, radius, text) = metrics(self.presentation.size, &theme);
        if self.presentation.group.is_some() && self.group_focus.is_none() {
            let handle = cx.focus_handle().tab_stop(false);
            self._subscriptions
                .push(cx.on_focus_in(&handle, window, |_, _, cx| cx.notify()));
            self._subscriptions
                .push(cx.on_focus_out(&handle, window, |_, _, _, cx| cx.notify()));
            self.group_focus = Some(handle);
        }
        let group = self.presentation.group.as_ref();
        let group_focus = group.and(self.group_focus.clone());
        let buttons: Vec<_> = group
            .into_iter()
            .flat_map(|g| g.leading_buttons.iter().chain(&g.buttons))
            .map(|render| render(self.presentation.size, window, cx))
            .collect();
        for (index, button) in buttons.iter().enumerate() {
            assert!(
                !buttons[..index]
                    .iter()
                    .any(|previous| previous.id() == button.id()),
                "InputGroup direct button IDs must be unique"
            );
        }
        self.group_button_focus
            .retain(|id, _| buttons.iter().any(|button| button.id() == id));
        let joined = buttons.iter().any(|button| !button.is_ghost());
        if joined && self.group_zone_focus.is_none() {
            let handle = cx.focus_handle().tab_stop(false);
            self._subscriptions
                .push(cx.on_focus_in(&handle, window, |_, _, cx| cx.notify()));
            self._subscriptions
                .push(cx.on_focus_out(&handle, window, |_, _, _, cx| cx.notify()));
            self.group_zone_focus = Some(handle);
        }
        let zone_focus = joined.then(|| self.group_zone_focus.clone()).flatten();
        let borders = crate::button::JoinedRingQueue::default();
        let input_borders = borders.clone();
        let count = buttons.len();
        let hybrid = joined
            && group.is_some_and(|g| {
                g.start.as_ref().is_some_and(|a| !a.is_empty())
                    || g.end.as_ref().is_some_and(|a| !a.is_empty())
            });
        let leading_count = if hybrid {
            0
        } else {
            group.map_or(0, |g| g.leading_buttons.len())
        };
        let trailing_count = count - leading_count;
        let editor_width = group.and_then(|g| g.editor_width);
        let text_align = group.map_or(gpui_kit::TextAlign::Left, |g| g.text_align);
        self.editor
            .update(cx, |editor, cx| editor.set_text_align(text_align, cx));
        let mut direct_buttons: Vec<_> = buttons
            .into_iter()
            .enumerate()
            .map(|(index, button)| {
                let focus = button.provided_focus().unwrap_or_else(|| {
                    self.group_button_focus
                        .entry(button.id().clone())
                        .or_insert_with(|| cx.focus_handle())
                        .clone()
                });
                let button = button.track_focus(&focus);
                let button = if joined {
                    button
                        .size(crate::input_group::button_size(self.presentation.size))
                        .input_group_zone(
                            self.disabled,
                            crate::input_group::Zone {
                                height: crate::input_group::height(self.presentation.size),
                                radius,
                                first: leading_count > 0 && index == 0,
                                last: trailing_count > 0 && index + 1 == count,
                                borders: borders.clone(),
                            },
                        )
                } else {
                    button
                        .size(crate::input_group::compact_size(self.presentation.size))
                        .input_group_action(self.disabled)
                };
                div()
                    .flex()
                    .flex_shrink_0()
                    .when(joined && (index > 0 || leading_count == 0), |this| {
                        this.ml(px(-1.))
                    })
                    .on_mouse_down(gpui_kit::MouseButton::Left, |_, _, cx| {
                        cx.stop_propagation()
                    })
                    .child(button)
                    .into_any_element()
            })
            .collect();

        let leading_buttons: Vec<_> = direct_buttons.drain(..leading_count).collect();

        if group.is_some() {
            height = f32::from(crate::input_group::height(self.presentation.size));
        }
        let seam = match self.presentation.size {
            Size::Xs => 4.,
            Size::Sm => 6.,
            Size::Base => 8.,
            Size::Lg => 10.,
        };
        let start = group
            .and_then(|g| g.start.as_ref())
            .filter(|a| !a.is_empty())
            .map(|a| {
                a.render(
                    "addon-start",
                    crate::input_group::AddonContext {
                        theme: &theme,
                        size: self.presentation.size,
                        disabled: self.disabled,
                        start: true,
                    },
                    window,
                    cx,
                )
            });
        let end = group
            .and_then(|g| g.end.as_ref())
            .filter(|a| !a.is_empty())
            .map(|a| {
                a.render(
                    "addon-end",
                    crate::input_group::AddonContext {
                        theme: &theme,
                        size: self.presentation.size,
                        disabled: self.disabled,
                        start: false,
                    },
                    window,
                    cx,
                )
            });
        let suffix = group.and_then(|g| g.suffix.clone());
        let editor_content_width = suffix.as_ref().map(|_| {
            let editor = self.editor.read(cx);
            let displayed = if editor.value().is_empty() {
                editor.presentation().placeholder().clone()
            } else {
                editor.value()
            };
            // shape_line requires one line; sanitize programmatic placeholder
            // line breaks for measurement without changing editor-owned text.
            let displayed: SharedString = displayed.replace(['\n', '\r'], " ").into();
            let run = gpui_kit::TextRun {
                len: displayed.len(),
                font: gpui_kit::font(theme.typography.font_family.clone()),
                color: theme.text.default,
                background_color: None,
                underline: None,
                strikethrough: None,
            };
            let line = window
                .text_system()
                .shape_line(displayed, text.size, &[run], None);
            line.width.max(px(1.))
                + crate::input_group::EDITOR_CARET_MARGIN
                + if start.is_some() { px(seam) } else { padding }
        });
        let editor_body = if suffix.is_some() {
            // Base needs its scroll safety width, but glyphs and pointer selection
            // must stop before the suffix even when Home exposes the text start.
            div()
                .id("editor-clip")
                .test_support()
                .relative()
                .flex_1()
                .min_w_0()
                .h_full()
                .mr(crate::input_group::SUFFIX_OVERLAP)
                .overflow_hidden()
                .child(
                    div()
                        .absolute()
                        .left_0()
                        .right(-crate::input_group::SUFFIX_OVERLAP)
                        .top_0()
                        .h_full()
                        .flex()
                        .items_center()
                        .child(base::input::Input::new(&self.editor)),
                )
                .into_any_element()
        } else {
            base::input::Input::new(&self.editor).into_any_element()
        };
        let focus = self.focus_handle(cx);
        let focused = (!disabled || toolbar.is_some())
            && (if joined {
                zone_focus.as_ref()
            } else {
                group_focus
                    .as_ref()
                    .or(self.presentation.focus_scope.as_ref())
            })
            .map_or_else(
                || focus.is_focused(window),
                |scope| scope.contains_focused(window, cx),
            );
        let invalid = self.presentation.error.is_some();
        let ring_color = if invalid && !joined {
            theme.colors.danger
        } else if focused {
            theme.colors.focus.opacity(0.5)
        } else {
            theme.colors.line
        };
        let ring_width = if focused && !joined {
            theme.effects.input_focus_ring_width
        } else {
            theme.effects.control_ring_width
        };
        let foreground = if disabled && toolbar.is_none() {
            theme.native.disabled_input_foreground
        } else {
            theme.text.default
        };
        self.editor.update(cx, |editor, _| {
            editor.set_editor_style(base::input::InputEditorStyle {
                foreground,
                muted_foreground: theme.text.placeholder,
                selection: theme.native.selection,
                caret: theme.text.default,
                ..Default::default()
            })
        });
        let editor = self.editor.read(cx);
        let placeholder = editor.presentation().placeholder().clone();
        let value = editor.value();
        let masked = editor.presentation().is_masked();
        let description = crate::field::resolve_message(
            self.presentation.description.clone(),
            self.presentation.error.clone(),
        )
        .map(|(text, _)| text);
        let semantic_description = [
            description.as_deref(),
            self.disabled.then_some("Unavailable"),
            self.read_only.then_some("Read only"),
            invalid.then_some("Invalid"),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join(". ");
        let label_focus = focus.clone();
        let read_only = self.read_only;
        let accessibility = self.accessibility.clone();
        let text_prepaint = accessibility.clone();
        let text_editor = self.editor.downgrade();
        let selection_state = cx.entity().downgrade();
        let value_state = selection_state.clone();
        let replace_state = selection_state.clone();
        let paint_state = selection_state.clone();
        let target = focus.clone();
        let editor_semantics = base::input::InputBase::new("control")
            .role(if masked {
                gpui_kit::Role::PasswordInput
            } else {
                gpui_kit::Role::TextInput
            })
            .accessibility_label(self.name.clone())
            .aria_value(if masked && !value.is_empty() {
                "••••••••".into()
            } else {
                value
            })
            .aria_placeholder(placeholder)
            .aria_description(semantic_description)
            .a11y_synthetic_children(move |builder| {
                let node = builder.parent_node();
                if disabled {
                    node.set_disabled();
                }
                if read_only {
                    node.set_read_only();
                }
                if invalid {
                    node.set_invalid(gpui_kit::accesskit::Invalid::True);
                }
                if !masked {
                    accessibility.build(builder);
                }
            })
            .when(!disabled, |this| {
                let run_id = text_prepaint.run_id.clone();
                this.on_a11y_action(
                    gpui_kit::AccessibleAction::SetTextSelection,
                    move |data, window, cx| {
                        let _ = selection_state.update(cx, |state, cx| {
                            state.accessibility_action(run_id.get(), data, window, cx);
                        });
                    },
                )
            })
            .when(!disabled && !self.read_only, |this| {
                this.on_a11y_action(
                    gpui_kit::AccessibleAction::SetValue,
                    move |data, window, cx| {
                        let _ = value_state.update(cx, |state, cx| {
                            state.accessibility_action(None, data, window, cx);
                        });
                    },
                )
                .on_a11y_action(
                    gpui_kit::AccessibleAction::ReplaceSelectedText,
                    move |data, window, cx| {
                        let _ = replace_state.update(cx, |state, cx| {
                            state.accessibility_replace_selection(data, window, cx);
                        });
                    },
                )
            })
            .track_focus(&focus)
            .w_full()
            .min_w_0()
            .h_full()
            .flex()
            .items_center()
            .child(editor_body);

        let editor_element = div()
            .id("editor-zone")
            .test_support()
            .flex()
            .items_center()
            .flex_1()
            .min_w_0()
            .h_full()
            .pl(if start.is_some() { px(seam) } else { padding })
            .pr(if suffix.is_some() {
                px(0.)
            } else if end.is_some() {
                px(seam)
            } else {
                padding + self.presentation.end_reserve
            })
            .when_some(editor_width.filter(|_| !joined), |this, width| {
                this.flex_initial().w(width).max_w_full()
            })
            .when_some(editor_content_width, |this, width| {
                this.flex_initial().w(width).max_w_full()
            })
            .capture_any_mouse_down(move |_, window, cx| {
                if disabled {
                    window.prevent_default();
                    cx.stop_propagation();
                }
            })
            .child(editor_semantics);
        let (container_buttons, joined_buttons) = if joined {
            (Vec::new(), direct_buttons)
        } else {
            (direct_buttons, Vec::new())
        };
        let (container_leading, joined_leading) = if joined {
            (Vec::new(), leading_buttons)
        } else {
            (leading_buttons, Vec::new())
        };
        let toolbar_ring = toolbar.map(|hooks| (hooks.first, hooks.last, hooks.rings.clone()));
        let surface = div()
            .id("surface")
            .test_support()
            .w_full()
            .min_w_0()
            .h(px(height))
            .px(px(0.))
            .flex()
            .items_center()
            .rounded(radius)
            .relative()
            .bg(if toolbar.is_some() {
                gpui_kit::transparent_black()
            } else {
                theme.colors.control
            })
            .when_some(toolbar, |this, hooks| {
                this.rounded_l(if hooks.first { radius } else { px(0.) })
                    .rounded_r(if hooks.last { radius } else { px(0.) })
            })
            .text_color(foreground)
            .font_family(theme.typography.font_family.clone())
            .text_size(text.size)
            .line_height(text.line_height)
            .font_weight(FontWeight::NORMAL)
            .when(group.is_some() && !joined, |this| {
                this.opacity(if surface_disabled {
                    0.5
                } else {
                    1.
                })
            })
            .when(joined, |this| {
                this.flex_1()
                    .border_1()
                    .border_color(theme.colors.line.alpha(0.))
                    .when(leading_count > 0, |this| {
                        this.ml(px(-1.)).rounded_tl(px(0.)).rounded_bl(px(0.))
                    })
                    .when(trailing_count > 0, |this| {
                        this.rounded_tr(px(0.)).rounded_br(px(0.))
                    })
            })
            .when_some(editor_width.filter(|_| joined), |this, width| {
                this.flex_initial().w(width).max_w_full()
            })
            .when_some(zone_focus, |this, focus| this.track_focus(&focus))
            .when_some(toolbar, |this, hooks| {
                let on_focus = hooks.on_focus.clone();
                this.on_mouse_down(gpui_kit::MouseButton::Left, move |_, window, cx| {
                    if !disabled && !window.default_prevented() {
                        on_focus(window, cx);
                    }
                })
            })
            .capture_any_mouse_down(move |_, window, cx| {
                if surface_disabled {
                    window.prevent_default();
                    cx.stop_propagation();
                }
            })
            .when(toolbar.is_some(), |mut this| {
                // GPUI resolves bound editor actions before raw key events. Guard
                // the actual action path as well as text insertion; Tab remains host-owned.
                macro_rules! guard {
                    ($($action:ty),* $(,)?) => { $( {
                        let owner = cx.entity().downgrade();
                        this = this.capture_action(move |_: &$action, window, cx| {
                            if owner.upgrade().is_some_and(|state| { let state = state.read(cx); !state.focus_handle(cx).is_focused(window) || !state.effectively_disabled() }) { cx.propagate(); } else { cx.stop_propagation(); }
                        });
                    } )* };
                }
                guard!(
                    base::input::Backspace, base::input::Delete,
                    base::input::DeleteToBeginningOfLine, base::input::DeleteToEndOfLine,
                    base::input::DeleteToPreviousWordStart, base::input::DeleteToNextWordEnd,
                    base::input::MoveLeft, base::input::MoveRight, base::input::MoveUp, base::input::MoveDown,
                    base::input::MoveHome, base::input::MoveEnd, base::input::MovePageUp, base::input::MovePageDown,
                    base::input::MoveToStartOfLine, base::input::MoveToEndOfLine,
                    base::input::MoveToStart, base::input::MoveToEnd,
                    base::input::MoveToPreviousWord, base::input::MoveToNextWord,
                    base::input::SelectAll, base::actions::SelectLeft, base::actions::SelectRight,
                    base::actions::SelectUp, base::actions::SelectDown,
                    base::input::SelectToStartOfLine, base::input::SelectToEndOfLine,
                    base::input::SelectToStart, base::input::SelectToEnd,
                    base::input::SelectToPreviousWordStart, base::input::SelectToNextWordEnd,
                    base::input::Copy, base::input::Cut, base::input::Paste,
                    base::input::Undo, base::input::Redo, base::input::Enter,
                    base::input::Escape, base::input::ShowCharacterPalette,
                );
                this
            })
            .capture_key_down({
                let focus = focus.clone();
                move |event, window, cx| {
                    if disabled && focus.is_focused(window) && event.keystroke.key != "tab" {
                        window.prevent_default();
                        cx.stop_propagation();
                    }
                }
            })
            .on_mouse_down(gpui_kit::MouseButton::Left, move |_, window, cx| {
                if !disabled && !window.default_prevented() {
                    target.focus(window, cx);
                }
                window.prevent_default();
            })
            .child(
                div()
                    .id("content-row")
                    .test_support()
                    .flex()
                    .items_center()
                    .w_full()
                    .min_w_0()
                    .h_full()
                    .when(group.is_some(), |this| this.overflow_hidden())
                    .children(container_leading)
                    .when_some(start, |this, addon| this.child(addon))
                    .child(editor_element)
                    .when_some(suffix, |this, suffix| {
                        this.child(
                            div()
                                .id("suffix")
                                .test_support()
                                .ml(-crate::input_group::SUFFIX_OVERLAP)
                                .flex()
                                .items_center()
                                .flex_auto()
                                .min_w_0()
                                .pr(padding)
                                .text_color(theme.text.subtle)
                                .child(div().min_w_0().truncate().child(suffix)),
                        )
                    })
                    .when_some(end, |this, addon| this.child(addon))
                    .children(container_buttons),
            )
            .child(
                canvas(
                    |_, _, _| (),
                    move |bounds, _, window, cx| {
                        // Base publishes range geometry during paint. Retain
                        // that complete snapshot and refresh the tree next frame.
                        if window.is_a11y_active()
                            && let Some(editor) = text_editor.upgrade()
                            && text_prepaint.capture(editor.read(cx))
                        {
                            let owner = paint_state.clone();
                            window.defer(cx, move |_, cx| {
                                let _ = owner.update(cx, |_, cx| cx.notify());
                            });
                        }
                        if let Some((first, last, rings)) = &toolbar_ring {
                            if focused {
                                let corners = gpui_kit::Corners {
                                    top_left: if *first {
                                        radius + ring_width
                                    } else {
                                        ring_width
                                    },
                                    bottom_left: if *first {
                                        radius + ring_width
                                    } else {
                                        ring_width
                                    },
                                    top_right: if *last {
                                        radius + ring_width
                                    } else {
                                        ring_width
                                    },
                                    bottom_right: if *last {
                                        radius + ring_width
                                    } else {
                                        ring_width
                                    },
                                };
                                rings.borrow_mut().push((
                                    true,
                                    quad(
                                        bounds.dilate(ring_width),
                                        corners,
                                        ring_color.alpha(0.),
                                        ring_width,
                                        ring_color,
                                        Default::default(),
                                    ),
                                    window.content_mask(),
                                ));
                            }
                            return;
                        }
                        if joined {
                            let radii = gpui_kit::Corners {
                                top_left: if leading_count == 0 { radius } else { px(0.) },
                                bottom_left: if leading_count == 0 { radius } else { px(0.) },
                                top_right: if trailing_count == 0 { radius } else { px(0.) },
                                bottom_right: if trailing_count == 0 { radius } else { px(0.) },
                            };
                            input_borders.borrow_mut().push((
                                focused,
                                quad(
                                    bounds.dilate(px(1.)),
                                    radii,
                                    ring_color.alpha(0.),
                                    px(1.),
                                    ring_color,
                                    Default::default(),
                                ),
                                window.content_mask(),
                            ));
                            return;
                        }
                        window.paint_quad(quad(
                            bounds.dilate(ring_width),
                            radius + ring_width,
                            ring_color.alpha(0.),
                            ring_width,
                            ring_color,
                            Default::default(),
                        ));
                    },
                )
                .absolute()
                .top_0()
                .left_0()
                .size_full(),
            );
        let content = if joined {
            div()
                .w_full()
                .min_w_0()
                .opacity(if disabled && toolbar.is_none() {
                    0.5
                } else {
                    1.
                })
                .child(crate::input_group::Zoned {
                    body: div()
                        .flex()
                        .w_full()
                        .min_w_0()
                        .h(px(height))
                        .children(joined_leading)
                        .child(surface)
                        .children(joined_buttons)
                        .into_any_element(),
                    borders,
                })
                .into_any_element()
        } else {
            surface.into_any_element()
        };
        let content = div()
            .id("group-content")
            .test_support()
            .w_full()
            .min_w_0()
            .when_some(group_focus, |this, handle| {
                this.track_focus(&handle)
                    .role(gpui_kit::Role::Group)
                    .aria_label(self.name.clone())
                    .a11y_synthetic_children(move |builder| {
                        if group_disabled {
                            builder.parent_node().set_disabled();
                        }
                    })
            })
            .when(group.is_some(), |this| {
                // Only controls belong to focus-within and disabled Group
                // semantics; contextual label help remains independent.
                this.on_mouse_down(gpui_kit::MouseButton::Left, |_, window, _| {
                    window.prevent_default()
                })
            })
            .child(content);
        div()
            .id("field-root")
            .test_support()
            .flex()
            .flex_col()
            .w_full()
            .min_w_0()
            .gap(theme.spacing.eight)
            .when(self.presentation.label, |this| {
                this.child(
                    crate::Label::new("label", self.name.clone())
                        .optional(self.presentation.optional)
                        .focus_target(&label_focus)
                        .disabled(disabled)
                        .when_some(
                            self.presentation.tooltip.clone(),
                            |label, (state, content)| label.tooltip(&state, content),
                        ),
                )
            })
            .child(content)
            .when_some(description, |this, description| {
                this.child(crate::field::message_element(&theme, description, invalid))
            })
    }
}

#[cfg(test)]
#[path = "input_tests.rs"]
mod tests;
