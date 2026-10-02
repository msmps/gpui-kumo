# Component coverage inventory

Pinned Kumo `3fd5b648df578cb1ba214dedd30f475009f6a668`, reviewed2026-10-02. This is a count of catalog component families, not subparts or fidelity acceptance checkboxes. Checkbox.Group/Item, Radio.Item, TooltipProvider and Button subsidiary APIs count inside their families. InputArea, SkeletonLine and recommended CodeHighlighted have separate catalog pages and count separately even though some share source directories.

43 in-scope families:18 with working implementations,25 unported. Implemented components have differing documented API/visual/interaction/browser/platform gaps; this count does not declare full fidelity or platform acceptance. [Progress/parity backlog](port-progress.md) and individual matrices remain authoritative for acceptance.

| Catalog component | State |
| --- | --- |
| autocomplete | Unported — [#11](https://github.com/msmps/gpui-kumo/issues/11) |
| badge | Implemented; see component acceptance/gaps |
| banner | Implemented; see component acceptance/gaps |
| breadcrumbs | Unported — [#12](https://github.com/msmps/gpui-kumo/issues/12) |
| button | Implemented; see component acceptance/gaps |
| button-group | Implemented; see component acceptance/gaps |
| checkbox | Implemented; see component acceptance/gaps |
| clipboard-text | Unported — [#13](https://github.com/msmps/gpui-kumo/issues/13) |
| code-highlighted | Unported — [#14](https://github.com/msmps/gpui-kumo/issues/14) |
| collapsible | Unported — [#15](https://github.com/msmps/gpui-kumo/issues/15) |
| combobox | Unported — [#16](https://github.com/msmps/gpui-kumo/issues/16) |
| command-palette | Unported — [#17](https://github.com/msmps/gpui-kumo/issues/17) |
| date-picker | Unported — [#18](https://github.com/msmps/gpui-kumo/issues/18) |
| dialog | Unported — [#19](https://github.com/msmps/gpui-kumo/issues/19) |
| dropdown | Unported — [#20](https://github.com/msmps/gpui-kumo/issues/20) |
| empty | Implemented; see component acceptance/gaps |
| field | Implemented; see component acceptance/gaps |
| grid | Unported — [#21](https://github.com/msmps/gpui-kumo/issues/21) |
| inline-copy-text | Unported — [#22](https://github.com/msmps/gpui-kumo/issues/22) |
| input | Implemented; see component acceptance/gaps |
| input-area | Unported — [#23](https://github.com/msmps/gpui-kumo/issues/23) |
| input-group | Implemented; see component acceptance/gaps |
| label | Implemented; see component acceptance/gaps |
| layer-card | Implemented; see component acceptance/gaps |
| layer-dialog | Unported — [#24](https://github.com/msmps/gpui-kumo/issues/24) |
| link | Implemented; see component acceptance/gaps |
| loader | Implemented; see component acceptance/gaps |
| meter | Unported — [#25](https://github.com/msmps/gpui-kumo/issues/25) |
| pagination | Unported — [#26](https://github.com/msmps/gpui-kumo/issues/26) |
| popover | Implemented; see component acceptance/gaps |
| radio | Implemented; see component acceptance/gaps |
| select | Unported — [#27](https://github.com/msmps/gpui-kumo/issues/27) |
| sensitive-input | Unported — [#28](https://github.com/msmps/gpui-kumo/issues/28) |
| skeleton-line | Unported — [#29](https://github.com/msmps/gpui-kumo/issues/29) |
| switch | Implemented; see component acceptance/gaps |
| table | Unported — [#30](https://github.com/msmps/gpui-kumo/issues/30) |
| table-of-contents | Unported — [#31](https://github.com/msmps/gpui-kumo/issues/31) |
| tabs | Unported — [#32](https://github.com/msmps/gpui-kumo/issues/32) |
| tag-input | Unported — [#33](https://github.com/msmps/gpui-kumo/issues/33) |
| text | Implemented; see component acceptance/gaps |
| toast | Unported — [#34](https://github.com/msmps/gpui-kumo/issues/34) |
| toolbar | Unported — [#35](https://github.com/msmps/gpui-kumo/issues/35) |
| tooltip | Implemented; see component acceptance/gaps |

Excluded: charts, Flow, Sidebar/application shells, Cloudflare branding and blocks. Deprecated Code/CodeBlock, Surface, MenuBar and DateRangePicker are omitted under AGENTS/task scope. Their recommended replacements remain in scope: CodeHighlighted, LayerCard, segmented Tabs and DatePicker range mode. CodeHighlighted highlighter dependency policy remains deferred, not excluded. InputArea and SkeletonLine were missing from the coarse early backlog and are explicitly restored here from the pinned catalog.

All 25 unported families now have individual GitHub issues, with pinned gpui-base 0.7.0 mappings and acceptance criteria. See [tracker](issues/README.md#remaining-component-family-ports). Base mapping: 11 direct counterparts, 7 partial compositions,7 without a dedicated family primitive; source/API limitations are recorded per issue.
