//! Kumo compound pagination with a retained native draft and Base controlled bounds.
use crate::{Button, InputEvent, InputGroup, InputState, Theme, theme};
use gpui_kit::base::TestSupportExt;
use gpui_kit::{
    AnyElement, App, AppContext, Context, ElementId, Entity, EventEmitter, FocusHandle, Focusable,
    InteractiveElement, IntoElement, ParentElement, Render, RenderOnce, SharedString,
    StatefulInteractiveElement, Styled, Subscription, Window, base, div, prelude::FluentBuilder,
    px, svg,
};
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PaginationTotal {
    Known(usize),
    /// The application owns cursor/token storage; this is only its next-page signal.
    Unknown {
        has_next_page: bool,
    },
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Controls {
    #[default]
    Full,
    Simple,
}
#[derive(Clone, Debug)]
pub struct PaginationLabels {
    pub navigation: SharedString,
    pub first_page: SharedString,
    pub previous_page: SharedString,
    pub next_page: SharedString,
    pub last_page: SharedString,
    pub page_number: SharedString,
    pub page_size: SharedString,
}
impl Default for PaginationLabels {
    fn default() -> Self {
        Self {
            navigation: "Pagination".into(),
            first_page: "First page".into(),
            previous_page: "Previous page".into(),
            next_page: "Next page".into(),
            last_page: "Last page".into(),
            page_number: "Page number".into(),
            page_size: "Page size".into(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PaginationEvent {
    Page(usize),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PaginationInfoValue {
    pub page: usize,
    pub per_page: usize,
    pub total_count: Option<usize>,
    pub first_item: usize,
    pub last_item: usize,
}
impl PaginationInfoValue {
    pub fn page_showing_range(&self) -> String {
        format!("{}-{}", self.first_item, self.last_item)
    }
}
#[derive(IntoElement)]
pub struct PaginationInfo {
    value: PaginationInfoValue,
    content: Option<AnyElement>,
    text: Option<SharedString>,
}
impl PaginationInfo {
    pub fn value(&self) -> PaginationInfoValue {
        self.value
    }
    /// Localized readable information. Preserves source typography and exposes its full name.
    pub fn text(mut self, text: impl Into<SharedString>) -> Self {
        self.text = Some(text.into());
        self.content = None;
        self
    }
    /// Custom content owns its accessible meaning; no live-region claim.
    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self.text = None;
        self
    }
}
impl RenderOnce for PaginationInfo {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = theme(cx);
        let text = self.text.or_else(|| {
            if self.content.is_some() {
                return None;
            }
            self.value
                .total_count
                .filter(|n| *n > 0)
                .map(|n| format!("Showing {} of {n}", self.value.page_showing_range()).into())
        });
        div()
            .id("pagination-info")
            .test_support()
            .when_some(text.clone(), |this, text| {
                this.role(gpui_kit::Role::Label)
                    .aria_label(text.clone())
                    .aria_value(text)
            })
            .text_size(t.typography.sm.size)
            .line_height(t.typography.sm.line_height)
            .text_color(t.text.subtle)
            .min_w_0()
            .when_some(
                self.content.or_else(|| {
                    text.map(|text| {
                        div()
                            .font_features(gpui_kit::FontFeatures(std::sync::Arc::new(vec![(
                                "tnum".into(),
                                1,
                            )])))
                            .child(text)
                            .into_any_element()
                    })
                }),
                |this, content| this.child(content),
            )
    }
}
#[derive(IntoElement, Default)]
pub struct PaginationSeparator;
impl RenderOnce for PaginationSeparator {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .flex_shrink_0()
            .mx(theme(cx).spacing.eight)
            .h(px(24.))
            .border_l_1()
            .border_color(theme(cx).colors.hairline)
    }
}
/// Fresh parts from the retained root. Reorder/wrap them without retaining UI entities.
pub struct PaginationParts {
    pub info: PaginationInfo,
    pub separator: PaginationSeparator,
    pub controls: AnyElement,
}
type Content = Rc<dyn Fn(PaginationParts, &mut Window, &mut App) -> AnyElement>;
#[derive(Default)]
struct Presentation {
    controls: Controls,
    content: Option<Content>,
}
/// Controlled current page. Accept proposals with set_page; setters emit no events.
/// Retain once and mount once per window. Its Base Input alone owns draft text/selection.
pub struct PaginationState {
    model: base::PaginationState,
    per_page: usize,
    total: PaginationTotal,
    input: Entity<InputState>,
    draft_dirty: bool,
    draft_revision: u64,
    labels: PaginationLabels,
    focuses: [FocusHandle; 4],
    presentation: Presentation,
    _subscriptions: Vec<Subscription>,
}
impl EventEmitter<PaginationEvent> for PaginationState {}
impl PaginationState {
    pub fn new(
        page: usize,
        per_page: usize,
        total: PaginationTotal,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        assert!(per_page > 0, "Pagination page size must be positive");
        let labels = PaginationLabels::default();
        let model = Self::model(page, per_page, total, false, cx);
        let input = cx.new(|cx| {
            let mut s = InputState::new(labels.page_number.clone(), window, cx);
            s.set_value(model.current_page().to_string(), window, cx);
            s
        });
        let events =
            cx.subscribe_in(
                &input,
                window,
                |state: &mut Self, _, event, window, cx| match event {
                    InputEvent::Change => {
                        // Base may report a no-op single-line text event after Enter.
                        // A logical unchanged owner value must not cancel its queued commit.
                        let differs =
                            state.input.read(cx).value(cx).as_ref() != state.page().to_string();
                        if differs || state.draft_dirty {
                            state.draft_revision = state.draft_revision.wrapping_add(1);
                        }
                        state.draft_dirty = differs;
                    }
                    InputEvent::Submit { .. } | InputEvent::Blur => state.commit_draft(window, cx),
                    InputEvent::Focus => {}
                },
            );
        Self {
            model,
            per_page,
            total,
            input,
            draft_dirty: false,
            draft_revision: 0,
            labels,
            focuses: std::array::from_fn(|_| cx.focus_handle()),
            presentation: Presentation::default(),
            _subscriptions: vec![events, cx.observe_global::<Theme>(|_, cx| cx.notify())],
        }
    }
    fn model(
        page: usize,
        per_page: usize,
        total: PaginationTotal,
        disabled: bool,
        cx: &mut Context<Self>,
    ) -> base::PaginationState {
        let page = page.max(1);
        let pages = match total {
            PaginationTotal::Known(n) => n.div_ceil(per_page).max(1),
            PaginationTotal::Unknown { has_next_page } => {
                page.saturating_add(usize::from(has_next_page))
            }
        };
        let target = cx.entity().downgrade();
        base::PaginationState::new(page, pages)
            .disabled(disabled)
            .on_change(move |page, window, cx| {
                let _ = target.update(cx, |state, cx| {
                    // Controlled rejection keeps the displayed owner page coherent.
                    state.reset_draft(window, cx);
                    cx.emit(PaginationEvent::Page(page));
                });
            })
    }
    pub fn page(&self) -> usize {
        self.model.current_page()
    }
    pub fn per_page(&self) -> usize {
        self.per_page
    }
    pub fn total(&self) -> PaginationTotal {
        self.total
    }
    pub fn max_page(&self) -> Option<usize> {
        matches!(self.total, PaginationTotal::Known(_)).then(|| self.model.total_pages())
    }
    pub fn is_disabled(&self) -> bool {
        self.model.is_disabled()
    }
    pub fn info(&self) -> PaginationInfoValue {
        let total_count = match self.total {
            PaginationTotal::Known(n) => Some(n),
            PaginationTotal::Unknown { .. } => None,
        };
        let first_item = if total_count == Some(0) {
            0
        } else {
            self.page()
                .saturating_sub(1)
                .saturating_mul(self.per_page)
                .saturating_add(1)
        };
        let last_item = self
            .page()
            .saturating_mul(self.per_page)
            .min(total_count.unwrap_or(usize::MAX));
        PaginationInfoValue {
            page: self.page(),
            per_page: self.per_page,
            total_count,
            first_item,
            last_item,
        }
    }
    pub fn page_input(&self) -> &Entity<InputState> {
        &self.input
    }
    fn reset_draft(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.draft_dirty = false;
        self.draft_revision = self.draft_revision.wrapping_add(1);
        let text = self.page().to_string();
        if self.input.read(cx).value(cx).as_ref() != text {
            self.input.update(cx, |s, cx| s.set_value(text, window, cx));
        }
    }
    pub fn set_page(&mut self, page: usize, window: &mut Window, cx: &mut Context<Self>) {
        self.model = Self::model(page, self.per_page, self.total, self.is_disabled(), cx);
        self.reset_draft(window, cx);
        cx.notify();
    }
    /// Normalize to the new valid range without a proposal; does not automatically reset to1.
    pub fn set_total(
        &mut self,
        total: PaginationTotal,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.total = total;
        self.set_page(self.page(), window, cx);
        self.reconcile_hidden_focus(window, cx);
    }
    /// The owner chooses any reset-to-first-page policy explicitly.
    pub fn set_per_page(&mut self, per_page: usize, window: &mut Window, cx: &mut Context<Self>) {
        assert!(per_page > 0, "Pagination page size must be positive");
        self.per_page = per_page;
        self.set_page(self.page(), window, cx);
    }
    pub fn set_disabled(&mut self, disabled: bool, window: &mut Window, cx: &mut Context<Self>) {
        self.model = Self::model(self.page(), self.per_page, self.total, disabled, cx);
        self.input.update(cx, |s, cx| s.set_disabled(disabled, cx));
        self.reset_draft(window, cx);
        cx.notify();
    }
    pub fn set_labels(&mut self, labels: PaginationLabels, cx: &mut Context<Self>) {
        for name in [
            &labels.navigation,
            &labels.first_page,
            &labels.previous_page,
            &labels.next_page,
            &labels.last_page,
            &labels.page_number,
            &labels.page_size,
        ] {
            assert!(
                !name.trim().is_empty(),
                "Pagination labels require complete names"
            );
        }
        self.input
            .update(cx, |s, cx| s.set_name(labels.page_number.clone(), cx));
        self.labels = labels;
        cx.notify();
    }
    fn reconcile_hidden_focus(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let full = self.presentation.controls == Controls::Full
            && matches!(self.total, PaginationTotal::Known(_));
        if !full
            && (self.input.read(cx).focus_handle(cx).is_focused(window)
                || self.focuses[0].is_focused(window)
                || self.focuses[3].is_focused(window))
        {
            self.reset_draft(window, cx);
            if self.model.previous_page().is_some() {
                self.focuses[1].focus(window, cx);
            } else if self.model.next_page().is_some() {
                self.focuses[2].focus(window, cx);
            } else {
                window.blur(cx);
            }
        }
    }
    fn request(target: &Entity<Self>, direction: Direction, window: &mut Window, cx: &mut App) {
        let model = target.read(cx).model.clone();
        let requested = match direction {
            Direction::First => Some(1),
            Direction::Previous => model.previous_page(),
            Direction::Next => model.next_page(),
            Direction::Last => Some(model.total_pages()),
            Direction::Page(n) => Some(n.clamp(1, model.total_pages())),
        };
        if let Some(page) = requested {
            model.request_page(page, window, cx);
        }
    }
    fn commit_draft(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.draft_dirty {
            return;
        }
        let parsed = parse_page(self.input.read(cx).value(cx).as_ref());
        self.reset_draft(window, cx);
        if let Some(page) = parsed {
            let target = cx.entity().downgrade();
            let revision = self.draft_revision;
            let handle = window.window_handle();
            cx.defer(move |cx| {
                let _ = handle.update(cx, |_, window, cx| {
                    if let Some(target) = target.upgrade()
                        && target.read(cx).draft_revision == revision
                    {
                        Self::request(&target, Direction::Page(page), window, cx);
                    }
                });
            });
        }
    }
    fn button(
        &self,
        id: &'static str,
        direction: Direction,
        name: SharedString,
        index: usize,
        bytes: &'static [u8],
        cx: &Context<Self>,
    ) -> Button {
        let target = cx.entity().downgrade();
        let disabled = self.is_disabled()
            || match direction {
                Direction::First | Direction::Previous => self.model.previous_page().is_none(),
                Direction::Next | Direction::Last => self.model.next_page().is_none(),
                Direction::Page(_) => false,
            };
        Button::icon(id, name, NavigationGlyph(bytes))
            .variant(crate::button::Variant::Secondary)
            .track_focus(&self.focuses[index])
            .disabled(disabled)
            .on_click(move |_, window, cx| {
                if let Some(target) = target.upgrade() {
                    Self::request(&target, direction, window, cx);
                }
            })
    }
    fn action(
        &self,
        id: &'static str,
        direction: Direction,
        name: SharedString,
        index: usize,
        bytes: &'static [u8],
        cx: &Context<Self>,
    ) -> impl Fn(Button, &mut Window, &mut App) -> Button + 'static {
        let target = cx.entity().downgrade();
        let focus = self.focuses[index].clone();
        move |_, _, cx| {
            let disabled = target.upgrade().is_none_or(|target| {
                let state = target.read(cx);
                state.is_disabled()
                    || match direction {
                        Direction::First | Direction::Previous => {
                            state.model.previous_page().is_none()
                        }
                        Direction::Next | Direction::Last => state.model.next_page().is_none(),
                        Direction::Page(_) => false,
                    }
            });
            let target = target.clone();
            Button::icon(id, name.clone(), NavigationGlyph(bytes))
                .variant(crate::button::Variant::Secondary)
                .track_focus(&focus)
                .disabled(disabled)
                .on_click(move |_, window, cx| {
                    if let Some(target) = target.upgrade() {
                        Self::request(&target, direction, window, cx);
                    }
                })
        }
    }
    fn controls(&self, _: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let full = self.presentation.controls == Controls::Full
            && matches!(self.total, PaginationTotal::Known(_));
        let body = if full {
            div()
                .w(px(190.))
                .child(
                    InputGroup::new("pagination-input-group", &self.input)
                        .editor_width(px(50.))
                        .text_align(gpui_kit::TextAlign::Center)
                        .leading_button(
                            "pagination-first",
                            self.labels.first_page.clone(),
                            crate::button::Variant::Secondary,
                            self.action(
                                "pagination-first",
                                Direction::First,
                                self.labels.first_page.clone(),
                                0,
                                include_bytes!("../assets/pagination-first.svg"),
                                cx,
                            ),
                        )
                        .leading_button(
                            "pagination-previous",
                            self.labels.previous_page.clone(),
                            crate::button::Variant::Secondary,
                            self.action(
                                "pagination-previous",
                                Direction::Previous,
                                self.labels.previous_page.clone(),
                                1,
                                include_bytes!("../assets/pagination-previous.svg"),
                                cx,
                            ),
                        )
                        .button(
                            "pagination-next",
                            self.labels.next_page.clone(),
                            crate::button::Variant::Secondary,
                            self.action(
                                "pagination-next",
                                Direction::Next,
                                self.labels.next_page.clone(),
                                2,
                                include_bytes!("../assets/pagination-next.svg"),
                                cx,
                            ),
                        )
                        .button(
                            "pagination-last",
                            self.labels.last_page.clone(),
                            crate::button::Variant::Secondary,
                            self.action(
                                "pagination-last",
                                Direction::Last,
                                self.labels.last_page.clone(),
                                3,
                                include_bytes!("../assets/pagination-last.svg"),
                                cx,
                            ),
                        ),
                )
                .into_any_element()
        } else {
            let borders = crate::button::JoinedRingQueue::default();
            let prev = self
                .button(
                    "pagination-previous",
                    Direction::Previous,
                    self.labels.previous_page.clone(),
                    1,
                    include_bytes!("../assets/pagination-previous.svg"),
                    cx,
                )
                .input_group_zone(
                    false,
                    crate::input_group::Zone {
                        height: px(36.),
                        radius: theme(cx).radii.lg,
                        first: true,
                        last: false,
                        borders: borders.clone(),
                    },
                );
            let next = self
                .button(
                    "pagination-next",
                    Direction::Next,
                    self.labels.next_page.clone(),
                    2,
                    include_bytes!("../assets/pagination-next.svg"),
                    cx,
                )
                .input_group_zone(
                    false,
                    crate::input_group::Zone {
                        height: px(36.),
                        radius: theme(cx).radii.lg,
                        first: false,
                        last: true,
                        borders: borders.clone(),
                    },
                );
            crate::input_group::Zoned {
                body: div()
                    .flex()
                    .child(prev)
                    .child(div().flex().ml(px(-1.)).child(next))
                    .into_any_element(),
                borders,
            }
            .into_any_element()
        };
        div()
            .flex_1()
            .min_w_0()
            .flex()
            .flex_col()
            .items_end()
            .child(
                base::Pagination::new("pagination-navigation", self.model.clone())
                    .accessibility_label(self.labels.navigation.clone())
                    .child(body),
            )
            .into_any_element()
    }
}
#[derive(IntoElement)]
struct NavigationGlyph(&'static [u8]);
impl RenderOnce for NavigationGlyph {
    fn render(self, window: &mut Window, _: &mut App) -> impl IntoElement {
        svg()
            .data(self.0)
            .size(px(16.))
            .flex_shrink_0()
            .text_color(window.text_style().color)
    }
}
#[derive(Clone, Copy)]
enum Direction {
    First,
    Previous,
    Next,
    Last,
    Page(usize),
}
fn parse_page(text: &str) -> Option<usize> {
    let text = text.trim();
    if text.is_empty() {
        return Some(1);
    }
    let negative = text.starts_with('-');
    let digits = text.strip_prefix(['-', '+']).unwrap_or(text);
    if digits.is_empty() || !digits.bytes().all(|c| c.is_ascii_digit()) {
        return None;
    }
    if negative {
        return Some(1);
    }
    Some(
        digits
            .bytes()
            .fold(0usize, |n, c| {
                n.saturating_mul(10).saturating_add(usize::from(c - b'0'))
            })
            .max(1),
    )
}
#[derive(IntoElement)]
#[must_use]
pub struct Pagination {
    id: ElementId,
    state: Entity<PaginationState>,
    presentation: Presentation,
}
impl Pagination {
    pub fn new(id: impl Into<ElementId>, state: &Entity<PaginationState>) -> Self {
        Self {
            id: id.into(),
            state: state.clone(),
            presentation: Presentation::default(),
        }
    }
    pub fn controls(mut self, controls: Controls) -> Self {
        self.presentation.controls = controls;
        self
    }
    /// Reorder/wrap fresh compound parts. Capture the owner weakly, and never
    /// read/update this PaginationState while it is rendering under a borrow.
    pub fn content(
        mut self,
        content: impl Fn(PaginationParts, &mut Window, &mut App) -> AnyElement + 'static,
    ) -> Self {
        self.presentation.content = Some(Rc::new(content));
        self
    }
}
impl RenderOnce for Pagination {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        self.state.update(cx, |s, cx| {
            s.presentation = self.presentation;
            s.reconcile_hidden_focus(window, cx);
        });
        div().id(self.id).w_full().min_w_0().child(self.state)
    }
}
impl Render for PaginationState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let parts = PaginationParts {
            info: PaginationInfo {
                value: self.info(),
                content: None,
                text: None,
            },
            separator: PaginationSeparator,
            controls: self.controls(window, cx),
        };
        let body = if let Some(content) = &self.presentation.content {
            content(parts, window, cx)
        } else {
            div()
                .flex()
                .items_center()
                .gap(theme(cx).spacing.eight)
                .child(parts.info)
                .child(parts.controls)
                .into_any_element()
        };
        div()
            .id("pagination-root")
            .test_support()
            .w_full()
            .min_w_0()
            .font_family(theme(cx).typography.font_family.clone())
            .child(body)
    }
}

#[cfg(test)]
#[path = "pagination_tests.rs"]
mod tests;
