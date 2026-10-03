# Component coverage inventory

Pinned Kumo `3fd5b648df578cb1ba214dedd30f475009f6a668`, reviewed2026-10-03. This is a count of catalog component families, not subparts or fidelity acceptance checkboxes. Checkbox.Group/Item, Radio.Item, TooltipProvider and Button subsidiary APIs count inside their families. InputArea, SkeletonLine and recommended CodeHighlighted have separate catalog pages and count separately even though some share source directories.

43 in-scope families:32 with working implementations,11 unported. Implemented components have differing documented API/visual/interaction/browser/platform gaps; this count does not declare full fidelity or platform acceptance. [Progress/parity backlog](port-progress.md) and individual matrices remain authoritative for acceptance.

| Catalog component | State |
| --- | --- |
| autocomplete | Unported — [#11](https://github.com/msmps/gpui-kumo/issues/11) |
| badge | Implemented; see component acceptance/gaps |
| banner | Implemented; see component acceptance/gaps |
| breadcrumbs | Implemented; [acceptance/gaps](breadcrumbs-validation.md), [#12](https://github.com/msmps/gpui-kumo/issues/12) remains open |
| button | Implemented; see component acceptance/gaps |
| button-group | Implemented; see component acceptance/gaps |
| checkbox | Implemented; see component acceptance/gaps |
| clipboard-text | Unported — [#13](https://github.com/msmps/gpui-kumo/issues/13) |
| code-highlighted | Unported — [#14](https://github.com/msmps/gpui-kumo/issues/14) |
| collapsible | Implemented; [acceptance/gaps](collapsible-validation.md), [#15](https://github.com/msmps/gpui-kumo/issues/15) remains open |
| combobox | Unported — [#16](https://github.com/msmps/gpui-kumo/issues/16) |
| command-palette | Unported — [#17](https://github.com/msmps/gpui-kumo/issues/17) |
| date-picker | Unported — [#18](https://github.com/msmps/gpui-kumo/issues/18) |
| dialog | Retained modal core implemented; [contract and remaining composition/motion](dialog-validation.md) — [#19](https://github.com/msmps/gpui-kumo/issues/19) |
| dropdown | Flat action-menu core implemented; [contract and remaining composition](dropdown-validation.md) — [#20](https://github.com/msmps/gpui-kumo/issues/20) |
| empty | Implemented; see component acceptance/gaps |
| field | Implemented; see component acceptance/gaps |
| grid | Unported — [#21](https://github.com/msmps/gpui-kumo/issues/21) |
| inline-copy-text | Implemented; [acceptance/gaps](inline-copy-text-validation.md), [#22](https://github.com/msmps/gpui-kumo/issues/22) remains open |
| input | Implemented; see component acceptance/gaps |
| input-area | Implemented; [acceptance/gaps](input-area-validation.md), [#23](https://github.com/msmps/gpui-kumo/issues/23) remains open |
| input-group | Implemented; see component acceptance/gaps |
| label | Implemented; see component acceptance/gaps |
| layer-card | Implemented; see component acceptance/gaps |
| layer-dialog | Unported — [#24](https://github.com/msmps/gpui-kumo/issues/24) |
| link | Implemented; see component acceptance/gaps |
| loader | Implemented; see component acceptance/gaps |
| meter | Implemented; [acceptance/gaps](meter-validation.md), [#25](https://github.com/msmps/gpui-kumo/issues/25) remains open |
| pagination | Input/simple, Dropdown, Info/Separator and PageSize implemented; recreated Dropdown validation and platform fidelity in progress, [acceptance/gaps](pagination-validation.md) — [#26](https://github.com/msmps/gpui-kumo/issues/26) |
| popover | Implemented; see component acceptance/gaps |
| radio | Implemented; see component acceptance/gaps |
| select | Core implemented; composition in progress, [acceptance/gaps](select-validation.md) — [#27](https://github.com/msmps/gpui-kumo/issues/27) |
| sensitive-input | Implemented; [acceptance/gaps](sensitive-input-validation.md), [#28](https://github.com/msmps/gpui-kumo/issues/28) remains open |
| skeleton-line | Implemented; [acceptance/gaps](skeleton-line-validation.md), [#29](https://github.com/msmps/gpui-kumo/issues/29) remains open |
| switch | Implemented; see component acceptance/gaps |
| table | Unported — [#30](https://github.com/msmps/gpui-kumo/issues/30) |
| table-of-contents | Unported — [#31](https://github.com/msmps/gpui-kumo/issues/31) |
| tabs | Core implemented; [contract and remaining overflow/motion/fidelity](tabs-validation.md) — [#32](https://github.com/msmps/gpui-kumo/issues/32) |
| tag-input | Unported — [#33](https://github.com/msmps/gpui-kumo/issues/33) |
| text | Implemented; see component acceptance/gaps |
| toast | Implemented core; [acceptance/gaps](toast-validation.md), [#34](https://github.com/msmps/gpui-kumo/issues/34) |
| toolbar | Action/link, retained Input/InputGroup and compact addon action lifecycle implemented; [contract and remaining composition](toolbar-validation.md) — [#35](https://github.com/msmps/gpui-kumo/issues/35) |
| tooltip | Implemented; see component acceptance/gaps |

Excluded: charts, Flow, Sidebar/application shells, Cloudflare branding and blocks. Deprecated Code/CodeBlock, Surface, MenuBar and DateRangePicker are omitted under AGENTS/task scope. Their recommended replacements remain in scope: CodeHighlighted, LayerCard, segmented Tabs and DatePicker range mode. CodeHighlighted highlighter dependency policy remains deferred, not excluded. InputArea and SkeletonLine were missing from the coarse early backlog and are explicitly restored here from the pinned catalog.

The original 25 missing families have individual GitHub issues, with pinned gpui-base 0.7.0 mappings and acceptance criteria. See [tracker](issues/README.md#remaining-component-family-ports). Base mapping: 11 direct counterparts, 7 partial compositions,7 without a dedicated family primitive; source/API limitations are recorded per issue.
