//! Semantic Kumo tables with typed sections, shared column sizing and owner-controlled selection.
//!
//! Tables present data; sorting, filtering, pagination and selection values belong
//! to the application. Headers accept header cells, while bodies and footers
//! accept data cells. Each section/row/cell has a stable ID; accessibility indices
//! are derived from the current composition rather than supplied by the caller.
//!
//! ```
//! use gpui_kumo::{Table, TableBody, TableCell, TableHead, TableHeader, TableRow};
//! let table = Table::new("requests")
//!     .accessibility_label("Recent requests")
//!     .header(TableHeader::new("header").row(
//!         TableRow::new("columns")
//!             .cell(TableHead::new("path").text("Path"))
//!             .cell(TableHead::new("status").text("Status")),
//!     ))
//!     .body(TableBody::new("body").row(
//!         TableRow::new("homepage")
//!             .cell(TableCell::new("path").text("/"))
//!             .cell(TableCell::new("status").text("200")),
//!     ));
//! ```
//!
//! ```compile_fail
//! use gpui_kumo::{TableCell, TableHeader, TableRow};
//! // A data cell cannot accidentally become a column header.
//! let header = TableHeader::new("header")
//!     .row(TableRow::new("row").cell(TableCell::new("data")));
//! ```

#![deny(missing_docs)]

mod layout;
pub(crate) mod resize;
pub use resize::TableResizeHandle;

use gpui_kit::base::TestSupportExt;
use gpui_kit::{
    AnyElement, App, ElementId, IntoElement, ParentElement, Pixels, RenderOnce, ScrollHandle,
    SharedString, StyleRefinement, Styled, Window,
};
use gpui_kit::{InteractiveElement, Role, StatefulInteractiveElement, div};

/// Column sizing strategy. This does not add a data-grid engine.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Layout {
    /// Measure all mounted cells; preserve intrinsic widths and distribute spare space.
    #[default]
    Auto,
    /// Divide remaining width equally between unspecified columns.
    Fixed,
}

/// Native equivalent of a `colgroup` width specification.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ColumnWidth {
    /// Size using the table's layout strategy.
    #[default]
    Auto,
    /// An exact positive, finite width, including cell padding.
    Pixels(Pixels),
}

/// Header presentation; compact reduces vertical padding without shrinking controls.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HeaderVariant {
    /// Base surface with twelve-pixel vertical padding.
    #[default]
    Default,
    /// Elevated surface with eight-pixel vertical padding.
    Compact,
}

/// Row presentation, independent of checkbox value state.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RowVariant {
    /// Alternate base/elevated backgrounds within each section.
    #[default]
    Default,
    /// Tint background; the owner separately configures selection controls.
    Selected,
}

/// Edge at which a cell stays visible during horizontal scrolling.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sticky {
    /// Clamp to the leading edge of the table viewport.
    Left,
    /// Clamp to the trailing edge of the table viewport.
    Right,
}

/// A consumed semantic table. Owns a keyed native scroll viewport, not application data.
/// Styles refine the viewport after the recipe. Use `min_table_width` to constrain
/// the scrolling content independently from the viewport, and `columns` for sizing.
/// Auto layout uses mounted max-content widths; it is not the browser's full
/// min/max-content table algorithm. All rows are mounted; there is no virtualization.
/// Initial mounting and changes to content or viewport size can require a second
/// frame to settle shared widths. Style cell content with `Styled`; use `columns`
/// rather than cell width overrides to keep all rows aligned.
///
/// # Panics
/// Rendering panics if section IDs (including the caption), row IDs within a
/// section, or cell IDs within a row are duplicated; if a row exceeds 65,535
/// columns; or if width specifications outnumber the composed columns.
#[derive(IntoElement)]
#[must_use]
pub struct Table {
    id: ElementId,
    name: Option<SharedString>,
    layout: Layout,
    columns: Vec<ColumnWidth>,
    minimum: Pixels,
    header: Option<TableHeader>,
    bodies: Vec<TableBody>,
    footer: Option<TableFooter>,
    caption: Option<TableCaption>,
    scroll: Option<ScrollHandle>,
    style: StyleRefinement,
}

impl Table {
    /// Create an empty table with stable identity and automatic column sizing.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            name: None,
            layout: Layout::Auto,
            columns: Vec::new(),
            minimum: gpui_kit::px(0.),
            header: None,
            bodies: Vec::new(),
            footer: None,
            caption: None,
            scroll: None,
            style: Default::default(),
        }
    }
    /// Name the table independently of caption/content.
    ///
    /// # Panics
    /// Panics if `name` is blank.
    pub fn accessibility_label(mut self, name: impl Into<SharedString>) -> Self {
        self.name = Some(crate::name::nonblank(name));
        self
    }
    /// Choose intrinsic or fixed column layout.
    pub fn layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }
    /// Supply column widths in display order. Unspecified columns are automatic.
    ///
    /// # Panics
    /// Panics if a pixel width is non-finite or not positive. Rendering also
    /// rejects more width specifications than columns in a nonempty table.
    pub fn columns(mut self, columns: impl IntoIterator<Item = ColumnWidth>) -> Self {
        self.columns = columns.into_iter().collect();
        for column in &self.columns {
            if let ColumnWidth::Pixels(width) = column {
                positive(*width);
            }
        }
        self
    }
    /// Set the minimum content width; narrower viewports scroll horizontally.
    ///
    /// # Panics
    /// Panics for negative or non-finite widths.
    pub fn min_table_width(mut self, width: Pixels) -> Self {
        assert!(
            f32::from(width).is_finite() && width >= gpui_kit::px(0.),
            "Table minimum width must be finite and nonnegative"
        );
        self.minimum = width;
        self
    }
    /// Set/replace the header. Only typed header cells can be inserted here.
    pub fn header(mut self, header: TableHeader) -> Self {
        self.header = Some(header);
        self
    }
    /// Append a body section. Striping restarts in each section.
    pub fn body(mut self, body: TableBody) -> Self {
        self.bodies.push(body);
        self
    }
    /// Set/replace the footer, rendered after every body.
    pub fn footer(mut self, footer: TableFooter) -> Self {
        self.footer = Some(footer);
        self
    }
    /// Set/replace the readable caption above the table.
    pub fn caption(mut self, caption: TableCaption) -> Self {
        self.caption = Some(caption);
        self
    }
    /// Use a caller-retained scroll handle; otherwise the table retains one by ID.
    pub fn scroll_handle(mut self, handle: &ScrollHandle) -> Self {
        self.scroll = Some(handle.clone());
        self
    }
}
impl Styled for Table {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for Table {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        layout::render(self, window, cx)
    }
}
impl std::fmt::Debug for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Table")
            .field("id", &self.id)
            .field("layout", &self.layout)
            .field("columns", &self.columns.len())
            .field("bodies", &self.bodies.len())
            .finish_non_exhaustive()
    }
}

/// A header section with column-header semantics.
#[derive(Debug)]
#[must_use]
pub struct TableHeader {
    id: ElementId,
    rows: Vec<TableRow<TableHead>>,
    variant: HeaderVariant,
    sticky: bool,
}
impl TableHeader {
    /// Create an empty header section.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            rows: Vec::new(),
            variant: HeaderVariant::Default,
            sticky: false,
        }
    }
    /// Append a typed header row.
    pub fn row(mut self, row: TableRow<TableHead>) -> Self {
        self.rows.push(row);
        self
    }
    /// Append typed header rows.
    pub fn rows(mut self, rows: impl IntoIterator<Item = TableRow<TableHead>>) -> Self {
        self.rows.extend(rows);
        self
    }
    /// Select default or compact header presentation.
    pub fn variant(mut self, variant: HeaderVariant) -> Self {
        self.variant = variant;
        self
    }
    /// Keep header rows at the top when the table viewport scrolls vertically.
    pub fn sticky(mut self, sticky: bool) -> Self {
        self.sticky = sticky;
        self
    }
}
macro_rules! section {
    ($name:ident, $docs:literal) => {
        #[doc = $docs]
        #[derive(Debug)]
        #[must_use]
        pub struct $name {
            id: ElementId,
            rows: Vec<TableRow<TableCell>>,
        }
        impl $name {
            /// Create an empty data section.
            pub fn new(id: impl Into<ElementId>) -> Self {
                Self {
                    id: id.into(),
                    rows: Vec::new(),
                }
            }
            /// Append a typed data row.
            pub fn row(mut self, row: TableRow<TableCell>) -> Self {
                self.rows.push(row);
                self
            }
            /// Append typed data rows.
            pub fn rows(mut self, rows: impl IntoIterator<Item = TableRow<TableCell>>) -> Self {
                self.rows.extend(rows);
                self
            }
        }
    };
}
section!(TableBody, "A body section containing data cells.");
section!(
    TableFooter,
    "A footer section containing data cells; rendered after all bodies."
);

/// Typed row composition. Cell type is inferred by `cell` and its containing section.
/// IDs remain stable across reordering; row/column indices are recomputed on render.
#[must_use]
pub struct TableRow<C = TableCell> {
    id: ElementId,
    cells: Vec<C>,
    variant: RowVariant,
}
impl<C> TableRow<C> {
    /// Create an empty row with the default striped presentation.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            cells: Vec::new(),
            variant: RowVariant::Default,
        }
    }
    /// Append a cell of this row's kind, including the matching checkbox cell.
    pub fn cell(mut self, cell: impl Into<C>) -> Self {
        self.cells.push(cell.into());
        self
    }
    /// Append cells of this row's kind.
    pub fn cells(mut self, cells: impl IntoIterator<Item = C>) -> Self {
        self.cells.extend(cells);
        self
    }
    /// Set visual row selection independently of application values.
    pub fn variant(mut self, variant: RowVariant) -> Self {
        self.variant = variant;
        self
    }
}
impl<C> std::fmt::Debug for TableRow<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TableRow")
            .field("id", &self.id)
            .field("cells", &self.cells.len())
            .field("variant", &self.variant)
            .finish_non_exhaustive()
    }
}

struct CellContent {
    id: ElementId,
    children: Vec<AnyElement>,
    sticky: Option<Sticky>,
    span: usize,
    check: Option<crate::Checkbox>,
    resize: Option<TableResizeHandle>,
    style: StyleRefinement,
}
impl CellContent {
    fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            children: Vec::new(),
            sticky: None,
            span: 1,
            check: None,
            resize: None,
            style: Default::default(),
        }
    }
}
macro_rules! cell {
    ($name:ident, $docs:literal) => {
        #[doc = $docs]
        #[must_use]
        pub struct $name {
            content: CellContent,
        }
        impl $name {
            /// Create an empty cell with stable identity; empty content is valid.
            pub fn new(id: impl Into<ElementId>) -> Self {
                Self {
                    content: CellContent::new(id),
                }
            }
            /// Append readable text with native accessibility metadata.
            /// For rich content use `child` and explicitly name its readable nodes.
            pub fn text(mut self, text: impl Into<SharedString>) -> Self {
                let id = format!("text-{}", self.content.children.len());
                self.content.children.push(
                    TableText {
                        id: id.into(),
                        text: text.into(),
                    }
                    .into_any_element(),
                );
                self
            }
            /// Pin this cell to a horizontal viewport edge with an opaque fill and fade.
            /// Adjacent pinned columns are stacked in column order.
            pub fn sticky(mut self, side: Sticky) -> Self {
                self.content.sticky = Some(side);
                self
            }
            /// Span consecutive columns, including padding once.
            ///
            /// # Panics
            /// Panics for zero spans or spans larger than 65,535.
            pub fn column_span(mut self, span: usize) -> Self {
                assert!(
                    (1..=65535).contains(&span),
                    "Table cell span must be between 1 and 65535"
                );
                self.content.span = span;
                self
            }
        }
        impl ParentElement for $name {
            fn extend(&mut self, children: impl IntoIterator<Item = AnyElement>) {
                self.content.children.extend(children);
            }
        }
        impl Styled for $name {
            fn style(&mut self) -> &mut StyleRefinement {
                &mut self.content.style
            }
        }
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("id", &self.content.id)
                    .field("span", &self.content.span)
                    .field("sticky", &self.content.sticky)
                    .finish_non_exhaustive()
            }
        }
    };
}
cell!(
    TableHead,
    "A column header with Kumo typography and bottom border. Content may include sorting buttons or a resize handle."
);
cell!(
    TableCell,
    "A data cell with Kumo padding and inherited typography. Interactive descendants retain their own behavior."
);

impl TableHead {
    /// Set/replace the resize affordance at the cell's outer trailing edge.
    /// The typed slot keeps its hit area independent of content padding/wrapping.
    pub fn resize_handle(mut self, handle: TableResizeHandle) -> Self {
        self.content.resize = Some(handle);
        self
    }
}

// Unlike the standalone Text recipe, cell text inherits the header/row recipe.
#[derive(IntoElement)]
struct TableText {
    id: ElementId,
    text: SharedString,
}
impl RenderOnce for TableText {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .test_support()
            .min_w_0()
            .role(Role::Label)
            .aria_label(self.text.clone())
            .aria_value(self.text.clone())
            .child(self.text)
    }
}

/// Readable native caption. It does not silently replace an explicit table name.
#[derive(Debug)]
#[must_use]
pub struct TableCaption {
    id: ElementId,
    text: SharedString,
}
impl TableCaption {
    /// Create caption content; empty text is permitted.
    pub fn new(id: impl Into<ElementId>, text: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
        }
    }
}

macro_rules! check_cell {
    ($name:ident, $target:ident, $docs:literal) => {
        #[doc = $docs]
        #[must_use]
        pub struct $name { cell: $target, checkbox: crate::Checkbox }
        impl $name {
            /// Create a checkbox cell with an explicit accessible name and no visible label.
            ///
            /// # Panics
            /// Panics if `name` is blank.
            pub fn new(id: impl Into<ElementId>, name: impl Into<SharedString>) -> Self {
                let cell = $target::new(id);
                Self { cell, checkbox: crate::Checkbox::new("checkbox", name).show_label(false) }
            }
            /// Set the owner-controlled checked/mixed/unchecked value.
            pub fn state(mut self, state: crate::checkbox::State) -> Self { self.checkbox = self.checkbox.state(state); self }
            /// Gate checkbox activation, editing no other cells.
            pub fn disabled(mut self, disabled: bool) -> Self { self.checkbox = self.checkbox.disabled(disabled); self }
            /// Override the checkbox's name independently of its default.
            ///
            /// # Panics
            /// Panics if `name` is blank.
            pub fn accessibility_label(mut self, name: impl Into<SharedString>) -> Self { self.checkbox = self.checkbox.accessibility_label(name); self }
            /// Use a caller-retained checkbox focus handle.
            pub fn track_focus(mut self, focus: &gpui_kit::FocusHandle) -> Self { self.checkbox = self.checkbox.track_focus(focus); self }
            /// Propose the next checkbox value once; the owner applies it and redraws.
            pub fn on_change(mut self, handler: impl Fn(crate::checkbox::State, &gpui_kit::ClickEvent, &mut Window, &mut App) + 'static) -> Self {
                self.checkbox = self.checkbox.on_change(handler); self
            }
            /// Pin the complete checkbox cell to a viewport edge.
            pub fn sticky(mut self, side: Sticky) -> Self { self.cell = self.cell.sticky(side); self }
        }
        impl From<$name> for $target {
            fn from(mut check: $name) -> Self { check.cell.content.check = Some(check.checkbox); check.cell }
        }
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($name)).field("cell", &self.cell).field("checkbox", &self.checkbox).finish_non_exhaustive()
            }
        }
    }
}
check_cell!(
    TableCheckHead,
    TableHead,
    "A select-all header checkbox with the whole cell as its pointer hit target."
);
check_cell!(
    TableCheckCell,
    TableCell,
    "A row-selection checkbox with the whole cell as its pointer hit target."
);

fn positive(width: Pixels) {
    assert!(
        f32::from(width).is_finite() && width > gpui_kit::px(0.),
        "Table width must be finite and positive"
    );
}

#[cfg(test)]
mod tests;
