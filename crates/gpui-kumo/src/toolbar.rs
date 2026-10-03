//! Joined Kumo action/link controls over retained native Base behavior.
use crate::link::NavigationRequest;
use crate::{Button, Icon, Input, InputEvent, InputGroup, InputGroupAddon, InputState, theme};
use gpui_kit::{
    App, ClickEvent, Context, ElementId, Entity, EventEmitter, FocusHandle, Focusable, FontWeight,
    InteractiveElement, IntoElement, ParentElement, Render, RenderOnce, SharedString,
    StatefulInteractiveElement, Styled, Subscription, Window, accesskit, base, canvas, div,
    prelude::FluentBuilder, px, quad,
};
use std::{cell::Cell, rc::Rc};
pub(crate) type FocusHandler = Rc<dyn Fn(&mut Window, &mut App)>;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}
#[derive(Clone)]
enum Kind {
    Button,
    Link(SharedString),
    Input(Editor),
}
#[derive(Clone)]
struct Editor {
    state: Entity<InputState>,
    width: gpui_kit::Pixels,
    group: Option<EditorGroup>,
}
#[derive(Clone, Default)]
struct EditorGroup {
    start: Option<InputGroupAddon>,
    end: Option<InputGroupAddon>,
    suffix: Option<SharedString>,
}
/// A stable action, native destination or retained editor. IDs are unique per list.
#[derive(Clone)]
pub struct ToolbarItem {
    id: ElementId,
    name: SharedString,
    kind: Kind,
    icon: Option<SharedString>,
    icon_only: bool,
    disabled: bool,
    loading: bool,
    focusable_when_disabled: bool,
}
impl ToolbarItem {
    pub fn button(id: impl Into<ElementId>, name: impl Into<SharedString>) -> Self {
        let name = name.into();
        assert!(
            !name.trim().is_empty(),
            "Toolbar controls require a readable name"
        );
        Self {
            id: id.into(),
            name,
            kind: Kind::Button,
            icon: None,
            icon_only: false,
            disabled: false,
            loading: false,
            focusable_when_disabled: true,
        }
    }
    /// Navigation is emitted to the owner; destinations never launch a browser automatically.
    pub fn link(
        id: impl Into<ElementId>,
        name: impl Into<SharedString>,
        href: impl Into<SharedString>,
    ) -> Self {
        let mut item = Self::button(id, name);
        item.kind = Kind::Link(href.into());
        item
    }
    /// Mount one application-retained single-line editor. Width is the whole control.
    /// The editor owns its value, selection, history, readable name and notifications.
    pub fn input(
        id: impl Into<ElementId>,
        state: &Entity<InputState>,
        width: gpui_kit::Pixels,
    ) -> Self {
        assert!(
            f32::from(width).is_finite() && width > px(0.),
            "Toolbar editor width must be finite and positive"
        );
        let mut item = Self::button(id, "Editor");
        item.kind = Kind::Input(Editor {
            state: state.clone(),
            width,
            group: None,
        });
        item
    }
    /// A shared editor with passive addons and independently tabbable compact actions.
    pub fn input_group(
        id: impl Into<ElementId>,
        state: &Entity<InputState>,
        width: gpui_kit::Pixels,
    ) -> Self {
        let mut item = Self::input(id, state, width);
        if let Kind::Input(editor) = &mut item.kind {
            editor.group = Some(EditorGroup::default());
        }
        item
    }
    fn group_mut(&mut self) -> &mut EditorGroup {
        let Kind::Input(editor) = &mut self.kind else {
            panic!("addons require Toolbar InputGroup")
        };
        editor
            .group
            .as_mut()
            .expect("addons require Toolbar InputGroup")
    }
    /// Text, icon or compact action addons; nested parts are supported.
    pub fn start(mut self, addon: InputGroupAddon) -> Self {
        self.group_mut().start = Some(addon);
        self
    }
    /// Trailing addons under the same InputGroup contract as `start`.
    pub fn end(mut self, addon: InputGroupAddon) -> Self {
        self.group_mut().end = Some(addon);
        self
    }
    /// A suffix follows the displayed editor text using the existing InputGroup recipe.
    pub fn suffix(mut self, text: impl Into<SharedString>) -> Self {
        self.group_mut().suffix = Some(text.into());
        self
    }
    /// Decorative SVG using the application's AssetSource. Icon-only controls keep `name`.
    pub fn icon(mut self, path: impl Into<SharedString>, icon_only: bool) -> Self {
        assert!(
            !matches!(self.kind, Kind::Input(_)),
            "use InputGroup addons for editor icons"
        );
        self.icon = Some(path.into());
        self.icon_only = icon_only;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    /// A Button loading indicator also gates activation. Links do not expose loading.
    pub fn loading(mut self, loading: bool) -> Self {
        assert!(
            matches!(self.kind, Kind::Button),
            "Toolbar links do not load"
        );
        self.loading = loading;
        self
    }
    /// Source Buttons/Inputs default to focusable when unavailable. Disabled native links leave traversal.
    pub fn focusable_when_disabled(mut self, focusable: bool) -> Self {
        assert!(
            !matches!(self.kind, Kind::Link(_)),
            "Toolbar.Link has no focusableWhenDisabled prop"
        );
        self.focusable_when_disabled = focusable;
        self
    }
}
/// Actual activation data is retained for owner routing and input-dependent behavior.
#[derive(Clone, Debug)]
pub enum ToolbarEvent {
    Activate {
        id: ElementId,
        activation: ClickEvent,
    },
    Navigate {
        id: ElementId,
        request: NavigationRequest,
    },
}
struct Item {
    control: ToolbarItem,
    focus: FocusHandle,
    bounds: Rc<Cell<Option<gpui_kit::Bounds<gpui_kit::Pixels>>>>,
}
/// Caller-owned collection and roving-focus authority, independent of application values.
pub struct ToolbarState {
    items: Vec<Item>,
    active: Option<ElementId>,
    disabled: bool,
    orientation: Orientation,
    loop_focus: bool,
    name: SharedString,
    scroll: gpui_kit::ScrollHandle,
    subscriptions: Vec<Subscription>,
}
impl EventEmitter<ToolbarEvent> for ToolbarState {}
impl ToolbarState {
    pub fn new(items: Vec<ToolbarItem>, cx: &mut Context<Self>) -> Self {
        Self::validate(&items);
        let mut state = Self {
            active: None,
            items: items
                .into_iter()
                .map(|control| Item {
                    focus: Self::control_focus(&control, cx),
                    control,
                    bounds: Rc::new(Cell::new(None)),
                })
                .collect(),
            disabled: false,
            orientation: Orientation::Horizontal,
            loop_focus: true,
            name: "Toolbar".into(),
            scroll: gpui_kit::ScrollHandle::new(),
            subscriptions: Vec::new(),
        };
        state.watch_inputs(cx);
        state
    }
    /// Initial group availability. Runtime changes use `set_disabled` for focus recovery.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    fn control_focus(control: &ToolbarItem, cx: &mut Context<Self>) -> FocusHandle {
        match &control.kind {
            Kind::Input(editor) => editor.state.read(cx).focus_handle(cx),
            _ => cx.focus_handle(),
        }
    }
    fn sync_inputs(&self, cx: &mut Context<Self>) {
        for item in &self.items {
            if let Kind::Input(editor) = &item.control.kind {
                editor.state.update(cx, |s, cx| {
                    s.set_toolbar_disabled(self.disabled || item.control.disabled, cx)
                });
            }
        }
    }
    fn watch_inputs(&mut self, cx: &mut Context<Self>) {
        self.subscriptions.clear();
        for item in &self.items {
            if let Kind::Input(editor) = &item.control.kind {
                self.subscriptions
                    .push(cx.observe(&editor.state, |_, _, cx| cx.notify()));
                let id = item.control.id.clone();
                self.subscriptions.push(cx.subscribe(
                    &editor.state,
                    move |s, _, event: &InputEvent, cx| {
                        if matches!(event, InputEvent::Focus) {
                            s.active = Some(id.clone());
                            cx.notify();
                        }
                    },
                ));
            }
        }
        self.sync_inputs(cx);
    }
    fn validate(items: &[ToolbarItem]) {
        for (index, item) in items.iter().enumerate() {
            if let Kind::Input(editor) = &item.kind {
                assert!(
                    !items[..index].iter().any(
                        |old| matches!(&old.kind, Kind::Input(other) if other.state == editor.state)
                    ),
                    "one retained editor may only mount once in a Toolbar"
                );
            }
            assert!(
                !items[..index].iter().any(|old| old.id == item.id),
                "Toolbar IDs must be unique"
            );
        }
    }
    fn unavailable(&self, item: &ToolbarItem, cx: &App) -> bool {
        item.disabled
            || item.loading
            || (self.disabled && !matches!(item.kind, Kind::Link(_)))
            || matches!(&item.kind, Kind::Input(editor) if editor.state.read(cx).is_disabled())
    }
    fn eligible(&self, item: &ToolbarItem, cx: &App) -> bool {
        if matches!(&item.kind, Kind::Input(editor) if editor.group.is_some() && editor.state.read(cx).is_disabled())
        {
            return false;
        }
        !self.unavailable(item, cx)
            || (!matches!(item.kind, Kind::Link(_)) && item.focusable_when_disabled)
    }
    fn entry(&self, window: &Window, cx: &App) -> Option<usize> {
        self.items
            .iter()
            .position(|i| self.eligible(&i.control, cx) && i.focus.is_focused(window))
            .or_else(|| {
                self.items.iter().position(|i| {
                    self.eligible(&i.control, cx) && Some(&i.control.id) == self.active.as_ref()
                })
            })
            .or_else(|| {
                self.items
                    .iter()
                    .position(|i| self.eligible(&i.control, cx))
            })
    }
    fn owns_focus(item: &Item, window: &Window, cx: &App) -> bool {
        item.focus.is_focused(window)
            || matches!(&item.control.kind, Kind::Input(editor) if editor.group.is_some() && editor.state.read(cx).toolbar_group_contains_focus(window, cx))
    }
    fn leave(&self, window: &mut Window, cx: &mut Context<Self>, removed: &[Item]) {
        for _ in 0..self.items.len() + removed.len() + 2 {
            window.focus_next(cx);
            if !self
                .items
                .iter()
                .chain(removed)
                .any(|i| Self::owns_focus(i, window, cx))
            {
                return;
            }
        }
        window.blur(cx);
    }
    fn recover(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self
            .items
            .iter()
            .any(|i| Self::owns_focus(i, window, cx) && !self.eligible(&i.control, cx))
        {
            if let Some(index) = self.entry(window, cx) {
                self.focus_item(&self.items[index].control.id.clone(), window, cx);
            } else {
                self.leave(window, cx, &[]);
            }
        }
    }
    /// Root disabled gates Buttons and editors. Links retain source routing availability.
    pub fn set_disabled(&mut self, disabled: bool, window: &mut Window, cx: &mut Context<Self>) {
        self.disabled = disabled;
        self.sync_inputs(cx);
        self.recover(window, cx);
        cx.notify();
    }
    /// Preserve focus by ID on reorder; removing a focused item recovers to the next eligible entry.
    pub fn set_items(
        &mut self,
        items: Vec<ToolbarItem>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        Self::validate(&items);
        let focused = self
            .items
            .iter()
            .find(|i| Self::owns_focus(i, window, cx))
            .map(|i| i.control.id.clone());
        for item in &self.items {
            if let Kind::Input(editor) = &item.control.kind {
                editor
                    .state
                    .update(cx, |s, cx| s.set_toolbar_disabled(false, cx));
            }
        }
        let mut old = std::mem::take(&mut self.items);
        self.items = items
            .into_iter()
            .map(|control| {
                if let Some(index) = old.iter().position(|i| i.control.id == control.id) {
                    let old = old.remove(index);
                    let focus = match (&old.control.kind, &control.kind) {
                        (_, Kind::Input(editor)) => editor.state.read(cx).focus_handle(cx),
                        (Kind::Input(_), _) => cx.focus_handle(),
                        _ => old.focus,
                    };
                    Item {
                        control,
                        focus,
                        bounds: Rc::new(Cell::new(None)),
                    }
                } else {
                    Item {
                        focus: Self::control_focus(&control, cx),
                        control,
                        bounds: Rc::new(Cell::new(None)),
                    }
                }
            })
            .collect();
        if let Some(id) = focused
            && !self.items.iter().any(|i| {
                i.control.id == id
                    && self.eligible(&i.control, cx)
                    && Self::owns_focus(i, window, cx)
            })
        {
            if let Some(index) = self
                .items
                .iter()
                .position(|i| i.control.id == id && self.eligible(&i.control, cx))
                .or_else(|| self.entry(window, cx))
            {
                self.focus_item(&self.items[index].control.id.clone(), window, cx);
            } else {
                self.leave(window, cx, &old);
            }
        }
        self.watch_inputs(cx);
        cx.notify();
    }
    fn reveal_focused(&self, window: &Window, cx: &mut Context<Self>) {
        let Some(item) = self.items.iter().find(|i| Self::owns_focus(i, window, cx)) else {
            return;
        };
        let Some(mut bounds) = item.bounds.get() else {
            return;
        };
        let old = self.scroll.offset();
        bounds.origin += old;
        let viewport = self.scroll.bounds();
        let left = viewport.left() + px(2.);
        let right = viewport.right() - px(2.);
        let delta = if bounds.left() < left {
            left - bounds.left()
        } else if bounds.right() > right {
            right - bounds.right()
        } else {
            px(0.)
        };
        let next = (old.x + delta).clamp(-self.scroll.max_offset().x, px(0.));
        if next != old.x {
            self.scroll.set_offset(gpui_kit::point(next, px(0.)));
            cx.notify();
        }
    }
    fn focus_item(&mut self, id: &ElementId, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(item) = self
            .items
            .iter()
            .find(|i| &i.control.id == id && self.eligible(&i.control, cx))
        {
            item.focus.focus(window, cx);
            self.active = Some(id.clone());
            self.reveal_focused(window, cx);
            cx.notify();
        }
    }
    fn activate(
        &mut self,
        id: &ElementId,
        activation: &ClickEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(control) = self
            .items
            .iter()
            .find(|i| &i.control.id == id && !self.unavailable(&i.control, cx))
            .map(|i| i.control.clone())
        else {
            return;
        };
        self.focus_item(id, window, cx);
        match control.kind {
            Kind::Button => cx.emit(ToolbarEvent::Activate {
                id: id.clone(),
                activation: activation.clone(),
            }),
            Kind::Input(_) => {}
            Kind::Link(href) => cx.emit(ToolbarEvent::Navigate {
                id: id.clone(),
                request: NavigationRequest {
                    href,
                    activation: activation.clone(),
                },
            }),
        }
    }
    fn editor_arrow(&mut self, key: &str, window: &mut Window, cx: &mut Context<Self>) {
        let m = window.modifiers();
        if m.control
            || m.alt
            || m.platform
            || m.shift
            || m.function
            || !self.navigate(key, window, cx)
        {
            cx.propagate();
        } else {
            cx.stop_propagation();
        }
    }
    fn navigate(&mut self, key: &str, window: &mut Window, cx: &mut Context<Self>) -> bool {
        let forward = match (self.orientation, key) {
            (Orientation::Horizontal, "right") | (Orientation::Vertical, "down") => true,
            (Orientation::Horizontal, "left") | (Orientation::Vertical, "up") => false,
            _ => return false,
        };
        if let Some(item) = self.items.iter().find(|i| i.focus.is_focused(window))
            && let Kind::Input(editor) = &item.control.kind
            && !self.unavailable(&item.control, cx)
            && !editor
                .state
                .update(cx, |s, cx| s.toolbar_arrow_at_boundary(forward, window, cx))
        {
            return false;
        }
        let eligible: Vec<_> = self
            .items
            .iter()
            .filter(|i| self.eligible(&i.control, cx))
            .collect();
        let current = eligible
            .iter()
            .position(|i| i.focus.is_focused(window))
            .or_else(|| {
                self.items
                    .iter()
                    .any(|i| Self::owns_focus(i, window, cx))
                    .then(|| {
                        eligible
                            .iter()
                            .position(|i| Some(&i.control.id) == self.active.as_ref())
                            .or_else(|| (!eligible.is_empty()).then_some(0))
                    })
                    .flatten()
            });
        let Some(current) = current else {
            return false;
        };
        let next = if forward {
            current + 1
        } else {
            current.checked_sub(1).unwrap_or(eligible.len())
        };
        let next = if next >= eligible.len() {
            if !self.loop_focus {
                return true;
            }
            if forward { 0 } else { eligible.len() - 1 }
        } else {
            next
        };
        let id = eligible[next].control.id.clone();
        self.focus_item(&id, window, cx);
        true
    }
}
/// Consumed appearance and identity over one caller-retained ToolbarState per mount.
#[derive(IntoElement)]
pub struct Toolbar {
    id: ElementId,
    name: SharedString,
    state: Entity<ToolbarState>,
    orientation: Orientation,
    loop_focus: bool,
}
impl Toolbar {
    pub fn new(
        id: impl Into<ElementId>,
        name: impl Into<SharedString>,
        state: &Entity<ToolbarState>,
    ) -> Self {
        let name = name.into();
        assert!(!name.trim().is_empty(), "Toolbar requires a readable name");
        Self {
            id: id.into(),
            name,
            state: state.clone(),
            orientation: Orientation::Horizontal,
            loop_focus: true,
        }
    }
    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }
    pub fn loop_focus(mut self, loop_focus: bool) -> Self {
        self.loop_focus = loop_focus;
        self
    }
}
impl RenderOnce for Toolbar {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        self.state.update(cx, |s, _| {
            s.name = self.name;
            s.orientation = self.orientation;
            s.loop_focus = self.loop_focus;
        });
        div().id(self.id).flex().min_w_0().child(self.state)
    }
}
pub(crate) struct InputFocus {
    pub on_action_removed: FocusHandler,
    pub tab_stop: bool,
    pub first: bool,
    pub last: bool,
    pub rings: crate::button::JoinedRingQueue,
    pub on_focus: FocusHandler,
}
pub(crate) struct ButtonFocus {
    pub tab_stop: bool,
    pub icon_width: Option<gpui_kit::Pixels>,
    pub focusable_when_disabled: bool,
    pub unavailable: bool,
    pub on_focus: FocusHandler,
}
impl Render for ToolbarState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.sync_inputs(cx);
        self.recover(window, cx);
        let theme = theme(cx).clone();
        let entry = self.entry(window, cx);
        let total = self.items.len();
        let rings = crate::button::JoinedRingQueue::default();
        let mut root = base::Toolbar::new("toolbar")
            .disabled(true)
            .aria_label(self.name.clone())
            .flex()
            .flex_none()
            .items_stretch()
            .relative()
            .rounded(px(8.))
            .bg(theme.colors.control)
            .shadow(theme.effects.shadow_xs.clone())
            .capture_action(
                cx.listener(|s, _: &base::input::MoveLeft, w, cx| s.editor_arrow("left", w, cx)),
            )
            .capture_action(
                cx.listener(|s, _: &base::input::MoveRight, w, cx| s.editor_arrow("right", w, cx)),
            )
            .capture_action(
                cx.listener(|s, _: &base::input::MoveUp, w, cx| s.editor_arrow("up", w, cx)),
            )
            .capture_action(
                cx.listener(|s, _: &base::input::MoveDown, w, cx| s.editor_arrow("down", w, cx)),
            )
            .capture_key_down(
                cx.listener(|s, event: &gpui_kit::KeyDownEvent, window, cx| {
                    let m = &event.keystroke.modifiers;
                    if m.control || m.alt || m.platform || m.shift || m.function {
                        return;
                    }
                    if s.navigate(&event.keystroke.key, window, cx) {
                        window.prevent_default();
                        cx.stop_propagation();
                    }
                }),
            );
        let orientation = match self.orientation {
            Orientation::Horizontal => accesskit::Orientation::Horizontal,
            Orientation::Vertical => accesskit::Orientation::Vertical,
        };
        root = root.a11y_synthetic_children(move |builder| {
            builder.parent_node().set_orientation(orientation)
        });
        for (index, item) in self.items.iter().enumerate() {
            let control = &item.control;
            let unavailable = self.unavailable(control, cx);
            let id = control.id.clone();
            let owner = cx.entity().downgrade();
            let focus_id = id.clone();
            let focus_owner = owner.clone();
            let on_focus: FocusHandler = Rc::new(move |window, cx| {
                let _ = focus_owner.update(cx, |s, cx| s.focus_item(&focus_id, window, cx));
            });
            let child = match &control.kind {
                Kind::Button => {
                    let button = if control.icon_only {
                        Button::icon(
                            id.clone(),
                            control.name.clone(),
                            Icon::new(control.icon.clone().expect("icon-only path")).size(px(14.)),
                        )
                    } else {
                        Button::new(id.clone(), control.name.clone())
                            .when_some(control.icon.clone(), |b, path| {
                                b.leading_icon(Icon::new(path).size(px(14.)))
                            })
                    };
                    button
                        .disabled(control.disabled)
                        .loading(control.loading)
                        .track_focus(&item.focus)
                        .toolbar_focus(ButtonFocus {
                            tab_stop: entry == Some(index),
                            icon_width: control
                                .icon_only
                                .then(|| px(if index > 0 { 35. } else { 36. })),
                            focusable_when_disabled: control.focusable_when_disabled,
                            unavailable,
                            on_focus,
                        })
                        .group_join(index == 0, index + 1 == total, false, rings.clone())
                        .on_click(move |activation, window, cx| {
                            let _ =
                                owner.update(cx, |s, cx| s.activate(&id, activation, window, cx));
                        })
                        .into_any_element()
                }
                Kind::Input(editor) => {
                    let recovery_owner = cx.entity().downgrade();
                    let recovery_id = control.id.clone();
                    let hooks = InputFocus {
                        on_action_removed: Rc::new(move |window, cx| {
                            let _ = recovery_owner.update(cx, |s, cx| {
                                let index = s
                                    .items
                                    .iter()
                                    .position(|i| {
                                        i.control.id == recovery_id && s.eligible(&i.control, cx)
                                    })
                                    .or_else(|| s.entry(window, cx));
                                if let Some(index) = index {
                                    let id = s.items[index].control.id.clone();
                                    s.focus_item(&id, window, cx);
                                } else {
                                    s.leave(window, cx, &[]);
                                }
                            });
                        }),
                        tab_stop: entry == Some(index),
                        first: index == 0,
                        last: index + 1 == total,
                        rings: rings.clone(),
                        on_focus,
                    };
                    let input = if let Some(group) = &editor.group {
                        let mut input = InputGroup::new("editor", &editor.state);
                        if let Some(start) = &group.start {
                            input = input.start(start.clone());
                        }
                        if let Some(end) = &group.end {
                            input = input.end(end.clone());
                        }
                        if let Some(suffix) = &group.suffix {
                            input = input.suffix(suffix.clone());
                        }
                        input.toolbar_focus(hooks).into_any_element()
                    } else {
                        Input::new("editor", &editor.state)
                            .toolbar_focus(hooks)
                            .into_any_element()
                    };
                    div()
                        .flex_none()
                        .w((editor.width - px(if index > 0 { 1. } else { 0. })).max(px(0.)))
                        .child(input)
                        .into_any_element()
                }
                Kind::Link(href) => LinkControl {
                    control: control.clone(),
                    href: href.clone(),
                    focus: item.focus.clone(),
                    paint_focus: item.focus.clone(),
                    entry: entry == Some(index),
                    disabled: unavailable,
                    first: index == 0,
                    last: index + 1 == total,
                    rings: rings.clone(),
                    on_focus,
                    owner,
                    id,
                }
                .into_any_element(),
            };
            let measured = item.bounds.clone();
            let scroll = self.scroll.clone();
            root = root.child(
                div()
                    .id(control.id.clone())
                    .relative()
                    .flex()
                    .flex_none()
                    .when(index > 0, |v| {
                        v.border_l_1().border_color(theme.colors.line)
                    })
                    .child(child)
                    .child(
                        canvas(
                            move |bounds, _, _| {
                                measured.set(Some(gpui_kit::Bounds {
                                    origin: bounds.origin - scroll.offset(),
                                    size: bounds.size,
                                }));
                            },
                            |_, _, _, _| {},
                        )
                        .absolute()
                        .inset_0()
                        .size_full(),
                    ),
            );
        }
        let border = theme.colors.line;
        let queue = rings.clone();
        let owner = cx.entity().downgrade();
        let root = root.child(
            canvas(
                |_, _, _| (),
                move |bounds, _, window, cx| {
                    if total > 0 {
                        window.paint_quad(quad(
                            bounds.dilate(px(1.)),
                            px(9.),
                            border.alpha(0.),
                            px(1.),
                            border,
                            Default::default(),
                        ));
                    }
                    for (_, graphic, mask) in queue.borrow_mut().drain(..) {
                        window.with_content_mask(Some(mask), |window| window.paint_quad(graphic));
                    }
                    let owner = owner.clone();
                    window.defer(cx, move |window, cx| {
                        let _ = owner.update(cx, |s, cx| s.reveal_focused(window, cx));
                    });
                },
            )
            .absolute()
            .inset_0()
            .size_full(),
        );
        div()
            .id("toolbar-viewport")
            .flex()
            .min_w_0()
            .max_w_full()
            .p(px(2.))
            .overflow_x_scroll()
            .overflow_y_hidden()
            .track_scroll(&self.scroll)
            .child(root)
    }
}
#[derive(IntoElement)]
struct LinkControl {
    control: ToolbarItem,
    href: SharedString,
    focus: FocusHandle,
    paint_focus: FocusHandle,
    entry: bool,
    disabled: bool,
    first: bool,
    last: bool,
    rings: crate::button::JoinedRingQueue,
    on_focus: FocusHandler,
    owner: gpui_kit::WeakEntity<ToolbarState>,
    id: ElementId,
}
impl RenderOnce for LinkControl {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = theme(cx).clone();
        let color = theme.colors.brand;
        let first = self.first;
        let last = self.last;
        let focus = self.paint_focus;
        let rings = self.rings;
        let id = self.id;
        let owner = self.owner;
        let mut link = crate::link::control::root(
            id.clone(),
            self.control.name.clone(),
            self.href,
            &self.focus,
            self.disabled,
            self.entry,
        )
        .h(px(36.))
        .flex()
        .items_center()
        .justify_center()
        .gap(px(6.))
        .px(px(if self.control.icon_only { 0. } else { 12. }))
        .font_family(theme.typography.font_family.clone())
        .text_size(px(14.))
        .line_height(px(21.))
        .font_weight(FontWeight::MEDIUM)
        .text_color(theme.text.default)
        .relative()
        .rounded(px(8.))
        .when(!first, |v| v.rounded_l(px(0.)))
        .when(!last, |v| v.rounded_r(px(0.)))
        .when(self.control.icon_only, |v| {
            v.w(px(if first { 36. } else { 35. }))
        })
        .when(!self.disabled, |v| {
            v.cursor_pointer().hover(|v| v.bg(theme.colors.tint))
        })
        .on_mouse_down(gpui_kit::MouseButton::Left, move |_, w, cx| {
            (self.on_focus)(w, cx)
        })
        .when(!self.disabled, |link| {
            link.on_click(move |event, w, cx| {
                let _ = owner.update(cx, |s, cx| s.activate(&id, event, w, cx));
            })
        });
        if let Some(path) = self.control.icon {
            link = link.child(Icon::new(path).size(px(14.)));
        }
        if !self.control.icon_only {
            link = link.child(self.control.name);
        }
        link.child(
            canvas(
                |_, _, _| (),
                move |bounds, _, window, _| {
                    if focus.is_focused(window) && window.last_input_was_keyboard() {
                        let mut corners = gpui_kit::Corners::all(px(2.));
                        if first {
                            corners.top_left = px(10.);
                            corners.bottom_left = px(10.);
                        }
                        if last {
                            corners.top_right = px(10.);
                            corners.bottom_right = px(10.);
                        }
                        rings.borrow_mut().push((
                            true,
                            quad(
                                bounds.dilate(px(2.)),
                                corners,
                                color.alpha(0.),
                                px(2.),
                                color,
                                Default::default(),
                            ),
                            window.content_mask(),
                        ));
                    }
                },
            )
            .absolute()
            .inset_0()
            .size_full(),
        )
    }
}
#[cfg(test)]
mod tests;
