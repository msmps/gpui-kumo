# Component coverage inventory

Pinned Kumo `3fd5b648df578cb1ba214dedd30f475009f6a668`, reviewed2026-10-02. This is a count of catalog component families, not subparts or fidelity acceptance checkboxes. Checkbox.Group/Item, Radio.Item, TooltipProvider and Button subsidiary APIs count inside their families. InputArea, SkeletonLine and recommended CodeHighlighted have separate catalog pages and count separately even though some share source directories.

43 in-scope families:18 with working implementations,25 unported. Implemented components have differing documented API/visual/interaction/browser/platform gaps; this count does not declare full fidelity or platform acceptance. [Progress/parity backlog](port-progress.md) and individual matrices remain authoritative for acceptance.

| Catalog component | State |
| --- | --- |
| autocomplete | Unported |
| badge | Implemented; see component acceptance/gaps |
| banner | Implemented; see component acceptance/gaps |
| breadcrumbs | Unported |
| button | Implemented; see component acceptance/gaps |
| button-group | Implemented; see component acceptance/gaps |
| checkbox | Implemented; see component acceptance/gaps |
| clipboard-text | Unported |
| code-highlighted | Unported |
| collapsible | Unported |
| combobox | Unported |
| command-palette | Unported |
| date-picker | Unported |
| dialog | Unported |
| dropdown | Unported |
| empty | Implemented; see component acceptance/gaps |
| field | Implemented; see component acceptance/gaps |
| grid | Unported |
| inline-copy-text | Unported |
| input | Implemented; see component acceptance/gaps |
| input-area | Unported |
| input-group | Implemented; see component acceptance/gaps |
| label | Implemented; see component acceptance/gaps |
| layer-card | Implemented; see component acceptance/gaps |
| layer-dialog | Unported |
| link | Implemented; see component acceptance/gaps |
| loader | Implemented; see component acceptance/gaps |
| meter | Unported |
| pagination | Unported |
| popover | Implemented; see component acceptance/gaps |
| radio | Implemented; see component acceptance/gaps |
| select | Unported |
| sensitive-input | Unported |
| skeleton-line | Unported |
| switch | Implemented; see component acceptance/gaps |
| table | Unported |
| table-of-contents | Unported |
| tabs | Unported |
| tag-input | Unported |
| text | Implemented; see component acceptance/gaps |
| toast | Unported |
| toolbar | Unported |
| tooltip | Implemented; see component acceptance/gaps |

Excluded: charts, Flow, Sidebar/application shells, Cloudflare branding and blocks. Deprecated Code/CodeBlock, Surface, MenuBar and DateRangePicker are omitted under AGENTS/task scope. Their recommended replacements remain in scope: CodeHighlighted, LayerCard, segmented Tabs and DatePicker range mode. CodeHighlighted highlighter dependency policy remains deferred, not excluded. InputArea and SkeletonLine were missing from the coarse early backlog and are explicitly restored here from the pinned catalog.
