//! Kumo compound pagination with a retained native draft and Base controlled bounds.
use crate::{
    Button, InputEvent, InputGroup, InputState, Select, SelectOption, SelectState, SelectValue,
    SelectValueContent, Theme, theme,
};
use gpui_kit::base::TestSupportExt;
use gpui_kit::{
    AnyElement, App, AppContext, Context, ElementId, Entity, EventEmitter, FocusHandle, Focusable,
    InteractiveElement, IntoElement, ParentElement, Render, RenderOnce, SharedString,
    StatefulInteractiveElement, Styled, Subscription, Window, base, div, prelude::FluentBuilder,
    px, svg,
};
use std::{cell::Cell, rc::Rc};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Known item count or application-owned next-page availability.
pub enum PaginationTotal {
    /// A known total number of items.
    Known(usize),
    /// The application owns cursor/token storage; this is only its next-page signal.
    Unknown {
        /// Has next page.
        has_next_page: bool,
    },
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
/// Navigation button set.
pub enum Controls {
    #[default]
    /// Include first, previous, next and last controls.
    Full,
    /// Include previous and next controls.
    Simple,
}
/// The source dropdown enumerates every page; use Input for large datasets.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PageSelector {
    #[default]
    /// An editable page-number draft.
    Input,
    /// An enumerated page selector.
    Dropdown,
}
#[derive(Clone, Debug)]
/// Localisable names for pagination and its controls.
pub struct PaginationLabels {
    /// Complete text for navigation.
    pub navigation: SharedString,
    /// Complete text for first page.
    pub first_page: SharedString,
    /// Complete text for previous page.
    pub previous_page: SharedString,
    /// Complete text for next page.
    pub next_page: SharedString,
    /// Complete text for last page.
    pub last_page: SharedString,
    /// Complete text for page number.
    pub page_number: SharedString,
    /// Complete text for page size.
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
#[non_exhaustive]
/// Owner-controlled page or page-size proposal.
pub enum PaginationEvent {
    /// Proposed one-based page number.
    Page(usize),
    /// Proposed size; only the owner chooses acceptance and reset-to-first policy.
    PageSize(usize),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Current one-based page and inclusive item range.
pub struct PaginationInfoValue {
    /// Page.
    pub page: usize,
    /// Per page.
    pub per_page: usize,
    /// Total count.
    pub total_count: Option<usize>,
    /// First item.
    pub first_item: usize,
    /// Last item.
    pub last_item: usize,
}
impl PaginationInfoValue {
    /// Format the inclusive first-to-last item range.
    pub fn page_showing_range(&self) -> String {
        format!("{}-{}", self.first_item, self.last_item)
    }
}
#[derive(IntoElement)]
/// Readable pagination summary, optionally replaced by custom content.
pub struct PaginationInfo {
    value: PaginationInfoValue,
    content: Option<AnyElement>,
    text: Option<SharedString>,
}
impl PaginationInfo {
    /// Read the current value from its owner; this does not request a change.
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
/// Decorative divider between pagination parts.
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
/// Source PageSize composition. Label presentation does not rename the Select.
#[derive(IntoElement)]
pub struct PaginationPageSize {
    show_label: bool,
    state: Entity<SelectState<usize>>,
    value: usize,
    label: Option<AnyElement>,
    text: Option<SharedString>,
}
impl PaginationPageSize {
    /// Localized visible label; an empty label is hidden.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        let label = label.into();
        self.text = (!label.is_empty()).then_some(label);
        self.label = None;
        self
    }
    /// Rich label content owns its accessible meaning.
    pub fn label_content(mut self, content: impl IntoElement) -> Self {
        self.label = Some(content.into_any_element());
        self.text = None;
        self
    }
    /// Choose visible label presentation without changing the accessible name.
    pub fn show_label(mut self, show: bool) -> Self {
        self.show_label = show;
        self
    }
}
impl RenderOnce for PaginationPageSize {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = theme(cx);
        let (_, padding, _, style) = crate::input::metrics(crate::input::Size::Base, t);
        let text: SharedString = self.value.to_string().into();
        let run = gpui_kit::TextRun {
            len: text.len(),
            font: gpui_kit::font(t.typography.font_family.clone()),
            color: t.text.default,
            background_color: None,
            underline: None,
            strikethrough: None,
        };
        // Kumo's Select trigger is intrinsically sized here. Reuse its actual
        // text/padding/gap/caret recipe without changing full-width form Selects.
        let width = window
            .text_system()
            .shape_line(text, style.size, &[run], None)
            .width
            + padding * 2.
            + t.spacing.six
            + px(16.);
        div()
            .id("pagination-page-size")
            .test_support()
            .flex()
            .items_center()
            .gap(t.spacing.eight)
            .min_w_0()
            .when_some(self.text.filter(|_| self.show_label), |this, text| {
                this.child(crate::Text::new("pagination-page-size-label", text).style(
                    crate::text::Style::Copy {
                        tone: crate::text::Tone::Secondary,
                        size: crate::text::Size::Sm,
                        bold: false,
                    },
                ))
            })
            .children(self.label.filter(|_| self.show_label))
            .child(
                div()
                    .w(width)
                    .flex_shrink_0()
                    .child(Select::new("pagination-size-select", &self.state)),
            )
    }
}
/// Fresh parts from the retained root. Reorder/wrap them without retaining UI entities.
pub struct PaginationParts {
    /// Info.
    pub info: PaginationInfo,
    /// Separator.
    pub separator: PaginationSeparator,
    /// Page size.
    pub page_size: PaginationPageSize,
    /// Controls.
    pub controls: AnyElement,
}
type Content = Rc<dyn Fn(PaginationParts, &mut Window, &mut App) -> AnyElement>;
#[derive(Default)]
struct Presentation {
    controls: Controls,
    page_selector: PageSelector,
    page_size: bool,
    content: Option<Content>,
}
/// Controlled current page. Accept proposals with set_page; setters emit no events.
/// Retain once and mount once per window. Its Base Input alone owns draft text/selection.
pub struct PaginationState {
    model: base::PaginationState,
    per_page: usize,
    total: PaginationTotal,
    input: Entity<InputState>,
    page_size: Entity<SelectState<usize>>,
    page_select: Entity<SelectState<usize>>,
    page_options: Option<usize>,
    proposal_revision: Rc<Cell<u64>>,
    draft_dirty: bool,
    draft_revision: u64,
    labels: PaginationLabels,
    focuses: [FocusHandle; 4],
    presentation: Presentation,
    _subscriptions: Vec<Subscription>,
}
impl EventEmitter<PaginationEvent> for PaginationState {}
impl PaginationState {
    /// Create retained page navigation with one-based page bounds and an editor draft. Retain with `cx.new`.
    ///
    /// # Panics
    /// Panics when the page size is zero.
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
        let proposal_revision = Rc::new(Cell::new(0));
        let owner = cx.entity().downgrade();
        let configure_proposals = |select: &mut SelectState<usize>, page: bool| {
            let owner = owner.clone();
            let revision = proposal_revision.clone();
            select.set_proposal_handler(move |value, window, cx| {
                let SelectValue::Single(Some(value)) = value else {
                    return;
                };
                let value = *value;
                let expected = revision.get();
                let revision = revision.clone();
                let owner = owner.clone();
                let handle = window.window_handle();
                // Select is still borrowed here. Stamp now, deliver after that
                // borrow ends, and discard if the owner changed in between.
                cx.defer(move |cx| {
                    let _ = handle.update(cx, |_, window, cx| {
                        if revision.get() != expected {
                            return;
                        }
                        let Some(owner) = owner.upgrade() else { return };
                        let mounted = {
                            let state = owner.read(cx);
                            let select = if page {
                                &state.page_select
                            } else {
                                &state.page_size
                            };
                            select.read(cx).is_mounted()
                        };
                        if !mounted {
                            return;
                        }
                        if page {
                            if owner.read(cx).page_options.is_some() {
                                Self::request(&owner, Direction::Page(value), window, cx);
                            }
                        } else {
                            owner.update(cx, |state, cx| {
                                if !state.is_disabled() && value != state.per_page {
                                    cx.emit(PaginationEvent::PageSize(value));
                                }
                            });
                        }
                    });
                });
            });
        };
        let page_select = cx.new(|cx| {
            let mut state = SelectState::new(
                labels.page_number.clone(),
                SelectValue::Single(Some(model.current_page())),
                vec![],
                cx,
            );
            state.set_controlled(true, cx);
            state.set_joined_middle();
            configure_proposals(&mut state, true);
            state
        });
        let page_size = cx.new(|cx| {
            let mut state = SelectState::new(
                labels.page_size.clone(),
                SelectValue::Single(Some(per_page)),
                size_options(vec![25, 50, 100, 250]),
                cx,
            );
            state.set_controlled(true, cx);
            configure_proposals(&mut state, false);
            // External owner values need not occur in the current option list.
            // Keep the actual size readable without adding an invented option.
            state.set_value_content(
                |value, _, _| match value {
                    SelectValue::Single(Some(size)) => Some(SelectValueContent::new(
                        size.to_string(),
                        div().child(size.to_string()),
                    )),
                    _ => None,
                },
                cx,
            );
            state
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
            page_size,
            page_select,
            page_options: None,
            proposal_revision,
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
    /// Read the current one-based page.
    pub fn page(&self) -> usize {
        self.model.current_page()
    }
    /// Read the current page size.
    pub fn per_page(&self) -> usize {
        self.per_page
    }
    /// Read the known count or next-page availability.
    pub fn total(&self) -> PaginationTotal {
        self.total
    }
    /// Read the last page when the total is known.
    pub fn max_page(&self) -> Option<usize> {
        matches!(self.total, PaginationTotal::Known(_)).then(|| self.model.total_pages())
    }
    /// Report whether the owner has disabled this control.
    pub fn is_disabled(&self) -> bool {
        self.model.is_disabled()
    }
    /// Read the current page and inclusive item range.
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
    /// Access the retained page-number editor.
    pub fn page_input(&self) -> &Entity<InputState> {
        &self.input
    }
    fn reset_draft(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.proposal_revision
            .set(self.proposal_revision.get().wrapping_add(1));
        self.draft_dirty = false;
        self.draft_revision = self.draft_revision.wrapping_add(1);
        let text = self.page().to_string();
        if self.input.read(cx).value(cx).as_ref() != text {
            self.input.update(cx, |s, cx| s.set_value(text, window, cx));
        }
    }
    /// Update page and refresh the retained component.
    pub fn set_page(&mut self, page: usize, window: &mut Window, cx: &mut Context<Self>) {
        self.model = Self::model(page, self.per_page, self.total, self.is_disabled(), cx);
        self.reset_draft(window, cx);
        self.reconcile_page_options(cx);
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
    ///
    /// # Panics
    /// Panics when the page size is zero.
    pub fn set_per_page(&mut self, per_page: usize, window: &mut Window, cx: &mut Context<Self>) {
        assert!(per_page > 0, "Pagination page size must be positive");
        self.per_page = per_page;
        self.page_size.update(cx, |s, cx| {
            s.set_value(SelectValue::Single(Some(per_page)), cx)
        });
        self.set_page(self.page(), window, cx);
    }
    /// Update availability and notify presentation while retaining the value.
    pub fn set_disabled(&mut self, disabled: bool, window: &mut Window, cx: &mut Context<Self>) {
        self.model = Self::model(self.page(), self.per_page, self.total, disabled, cx);
        self.input.update(cx, |s, cx| s.set_disabled(disabled, cx));
        self.page_size
            .update(cx, |s, cx| s.set_disabled(disabled, window, cx));
        self.page_select
            .update(cx, |s, cx| s.set_disabled(disabled, window, cx));
        self.reset_draft(window, cx);
        cx.notify();
    }
    /// Update labels and refresh the retained component.
    ///
    /// # Panics
    /// Panics when any supplied control name is blank.
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
        self.page_size
            .update(cx, |s, cx| s.set_name(labels.page_size.clone(), cx));
        self.page_select
            .update(cx, |s, cx| s.set_name(labels.page_number.clone(), cx));
        self.labels = labels;
        cx.notify();
    }
    /// Replace source options without changing the owner's size or emitting a proposal.
    /// Empty lists are allowed. Positive, unique whole sizes have stable numeric IDs.
    ///
    /// # Panics
    /// Panics when page sizes are zero or duplicated.
    pub fn set_page_size_options(&mut self, options: Vec<usize>, cx: &mut Context<Self>) {
        self.proposal_revision
            .set(self.proposal_revision.get().wrapping_add(1));
        self.page_size
            .update(cx, |s, cx| s.set_options(size_options(options), cx));
    }
    fn full_controls(&self) -> bool {
        self.presentation.controls == Controls::Full
            && matches!(self.total, PaginationTotal::Known(_))
    }
    fn dropdown(&self) -> bool {
        self.full_controls() && self.presentation.page_selector == PageSelector::Dropdown
    }
    fn reconcile_page_options(&mut self, cx: &mut Context<Self>) {
        let pages = self.dropdown().then(|| self.model.total_pages());
        if self.page_options != pages {
            self.page_options = pages;
            self.page_select.update(cx, |state, cx| {
                state.set_options(
                    pages.map_or_else(Vec::new, |pages| {
                        (1..=pages)
                            .map(|page| {
                                SelectOption::new(("pagination-page", page), page, page.to_string())
                            })
                            .collect()
                    }),
                    cx,
                );
            });
        }
        let page = self.page();
        if self.page_select.read(cx).value() != &SelectValue::Single(Some(page)) {
            self.page_select.update(cx, |state, cx| {
                state.set_value(SelectValue::Single(Some(page)), cx);
            });
        }
    }
    fn reconcile_hidden_focus(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let full = self.full_controls();
        let dropdown = self.dropdown();
        let select_focus = self.page_select.read(cx).focus_handle();
        let removed_select =
            !dropdown && (select_focus.is_focused(window) || self.page_select.read(cx).is_open());
        // Close while still mounted so content focus can return to its trigger
        // before the removed-control fallback chooses a surviving action.
        if !dropdown {
            self.page_select
                .update(cx, |state, cx| state.set_open(false, window, cx));
        }
        if ((!full || dropdown) && self.input.read(cx).focus_handle(cx).is_focused(window))
            || (removed_select && select_focus.is_focused(window))
            || (!full && (self.focuses[0].is_focused(window) || self.focuses[3].is_focused(window)))
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
            .shape(crate::button::Shape::Standard)
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
                .shape(crate::button::Shape::Standard)
                .track_focus(&focus)
                .disabled(disabled)
                .on_click(move |_, window, cx| {
                    if let Some(target) = target.upgrade() {
                        Self::request(&target, direction, window, cx);
                    }
                })
        }
    }
    fn controls(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let full = self.full_controls();
        let dropdown = self.dropdown();
        let mut minimum = if full { px(214.) } else { px(83.) };
        let body = if dropdown {
            let t = theme(cx);
            let text: SharedString = self.page().to_string().into();
            let run = gpui_kit::TextRun {
                len: text.len(),
                font: gpui_kit::font(t.typography.font_family.clone()),
                color: t.text.default,
                background_color: None,
                underline: None,
                strikethrough: None,
            };
            let (_, padding, _, _) = crate::input::metrics(crate::input::Size::Base, t);
            let width = window
                .text_system()
                .shape_line(text, t.typography.base.size, &[run], None)
                .width
                + padding * 2.
                + t.spacing.six
                + px(16.);
            // Select is not an InputGroup child in pinned Kumo: it has no
            // leading -1px overlap; the other three button seams do.
            minimum = px(4. * 42. - 3.) + width;
            let borders = crate::button::JoinedRingQueue::default();
            let navigation = |id, direction, name, index, bytes, first, last| {
                self.button(id, direction, name, index, bytes, cx)
                    .input_group_zone(
                        false,
                        crate::input_group::Zone {
                            height: px(36.),
                            radius: t.radii.lg,
                            first,
                            last,
                            borders: borders.clone(),
                        },
                    )
            };
            crate::input_group::Zoned {
                body: div()
                    .flex()
                    .items_center()
                    .child(navigation(
                        "pagination-first",
                        Direction::First,
                        self.labels.first_page.clone(),
                        0,
                        include_bytes!("../../assets/pagination-first.svg"),
                        true,
                        false,
                    ))
                    .child(div().flex().ml(px(-1.)).child(navigation(
                        "pagination-previous",
                        Direction::Previous,
                        self.labels.previous_page.clone(),
                        1,
                        include_bytes!("../../assets/pagination-previous.svg"),
                        false,
                        false,
                    )))
                    .child(
                        div().w(width).flex_shrink_0().child(
                            Select::new("pagination-page-select", &self.page_select)
                                .joined_middle(borders.clone()),
                        ),
                    )
                    .child(div().flex().ml(px(-1.)).child(navigation(
                        "pagination-next",
                        Direction::Next,
                        self.labels.next_page.clone(),
                        2,
                        include_bytes!("../../assets/pagination-next.svg"),
                        false,
                        false,
                    )))
                    .child(div().flex().ml(px(-1.)).child(navigation(
                        "pagination-last",
                        Direction::Last,
                        self.labels.last_page.clone(),
                        3,
                        include_bytes!("../../assets/pagination-last.svg"),
                        false,
                        true,
                    )))
                    .into_any_element(),
                borders,
            }
            .into_any_element()
        } else if full {
            div()
                .w(px(214.))
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
                                include_bytes!("../../assets/pagination-first.svg"),
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
                                include_bytes!("../../assets/pagination-previous.svg"),
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
                                include_bytes!("../../assets/pagination-next.svg"),
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
                                include_bytes!("../../assets/pagination-last.svg"),
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
                    include_bytes!("../../assets/pagination-previous.svg"),
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
                    include_bytes!("../../assets/pagination-next.svg"),
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
            // Preserve the intrinsic joined control width when the source parts
            // are composed in a wrapping native row; never paint over siblings.
            .min_w(minimum)
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
fn size_options(options: Vec<usize>) -> Vec<SelectOption<usize>> {
    let mut seen = std::collections::HashSet::with_capacity(options.len());
    options
        .into_iter()
        .map(|size| {
            assert!(size > 0, "Pagination page size options must be positive");
            assert!(
                seen.insert(size),
                "Pagination page size options must be unique"
            );
            SelectOption::new(("pagination-size", size), size, size.to_string())
        })
        .collect()
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
/// Compound page navigation over a retained owner-controlled state.
pub struct Pagination {
    id: ElementId,
    state: Entity<PaginationState>,
    presentation: Presentation,
}
impl Pagination {
    /// Present the retained pagination state under a stable element identity.
    pub fn new(id: impl Into<ElementId>, state: &Entity<PaginationState>) -> Self {
        Self {
            id: id.into(),
            state: state.clone(),
            presentation: Presentation::default(),
        }
    }
    /// Choose the pagination navigation button set.
    pub fn controls(mut self, controls: Controls) -> Self {
        self.presentation.controls = controls;
        self
    }
    /// Source page control. Dropdown allocates all pages only in Full + Known
    /// mode; Input is recommended for large counts.
    pub fn page_selector(mut self, selector: PageSelector) -> Self {
        self.presentation.page_selector = selector;
        self
    }
    /// Include the default PageSize part. Custom content chooses its own parts.
    pub fn page_size(mut self, show: bool) -> Self {
        self.presentation.page_size = show;
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
            if s.presentation.controls != self.presentation.controls
                || s.presentation.page_selector != self.presentation.page_selector
            {
                s.reset_draft(window, cx);
            }
            s.presentation = self.presentation;
            s.reconcile_hidden_focus(window, cx);
            s.reconcile_page_options(cx);
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
            page_size: PaginationPageSize {
                show_label: true,
                state: self.page_size.clone(),
                value: self.per_page,
                label: None,
                text: Some("Per page:".into()),
            },
            controls: self.controls(window, cx),
        };
        let body = if let Some(content) = &self.presentation.content {
            content(parts, window, cx)
        } else {
            div()
                .flex()
                .items_center()
                .flex_wrap()
                .gap(theme(cx).spacing.eight)
                .child(parts.info)
                .when(self.presentation.page_size, |this| {
                    this.child(parts.separator).child(parts.page_size)
                })
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

debug_struct!(PaginationInfo { value });

debug_struct!(PaginationSeparator {});

debug_struct!(PaginationPageSize { show_label, value });

debug_struct!(PaginationParts {});

debug_struct!(PaginationState {
    per_page,
    draft_dirty
});

debug_struct!(Pagination { id });

#[cfg(test)]
mod tests;
