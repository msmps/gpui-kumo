# Component issues

This is the local issue tracker for component fidelity and the shared foundation. The first 18 follow-ups were identified at commit `38c2ea8`; later findings extend that baseline. None of these local IDs is a GitHub issue number.

Each issue has a stable `KUMO-NNN` ID, a problem statement, evidence and acceptance criteria. Known implementation gaps, native policy tradeoffs and unverified acceptance work are distinguished in the issue descriptions. The individual file is the source of truth for its status.

| ID | Area | Issue | Status |
| --- | --- | --- | --- |
| KUMO-001 | Button | [expose disabled and loading accessibility state](001-button-availability-semantics.md) | In progress |
| KUMO-002 | Button | [implement the authored 100ms outline color transition](002-button-outline-transition.md) | Open |
| KUMO-003 | Button | [evaluate faithful shadow rendering on transparent outlines](003-button-outline-shadow.md) | Open |
| KUMO-004 | Button | [complete pinned browser visual and narrow-layout comparisons](004-button-browser-parity.md) | Open |
| KUMO-005 | Input | [expose disabled, read-only and invalid accessibility metadata](005-input-state-semantics.md) | In progress |
| KUMO-006 | Input | [provide accessible text ranges, selection and editing actions](006-input-accessible-text.md) | Blocked |
| KUMO-007 | Input | [validate composition through operating-system IMEs](007-input-os-ime.md) | Open |
| KUMO-008 | Input | [complete pinned browser and Field visual comparisons](008-input-browser-parity.md) | Open |
| KUMO-009 | Popover | [export expanded/collapsed state through Linux AT-SPI](009-popover-expanded-state.md) | Resolved by #41 |
| KUMO-010 | Popover | [evaluate scale and exit-motion parity with Kumo](010-popover-motion-parity.md) | Open |
| KUMO-011 | Popover | [validate scrolling, resizing and extreme collision geometry](011-popover-geometry-stress.md) | In progress |
| KUMO-012 | Popover | [complete browser/native pixel comparisons](012-popover-browser-pixel-parity.md) | Open |
| KUMO-013 | Components | [measure browser color gamut and gradient parity](013-component-color-parity.md) | Open |
| KUMO-014 | Components | [validate spoken screen-reader workflows on Linux](014-linux-spoken-reader.md) | Open |
| KUMO-015 | Components | [validate Button, Input and Popover with VoiceOver](015-macos-voiceover.md) | Open |
| KUMO-016 | Components | [validate Button, Input and Popover with a Windows screen reader](016-windows-screen-reader.md) | Open |
| KUMO-017 | Components | [validate the native gallery and interactions on Windows](017-windows-native-components.md) | Open |
| KUMO-018 | Components | [record the Base dependency decision after the first slice](018-base-foundation-decision.md) | Resolved |
| KUMO-019 | LayerCard | [preserve rounded clipping for arbitrary content](019-layer-card-rounded-clipping.md) | Open |
| KUMO-020 | Components | [reconcile native frames with updated state](020-native-frame-consistency.md) | In progress |
| KUMO-021 | Tooltip | [match Kumo motion and grouped instant switching](021-tooltip-motion-provider.md) | Open |
| KUMO-022 | Tooltip | [support rich content and native trigger composition](022-tooltip-rich-content-triggers.md) | Open |
| KUMO-023 | Checkbox/Switch | [complete contextual help and Checkbox.Item mixed presentation](023-checkbox-switch-help-item-indeterminate.md) | Open |
| KUMO-024 | InputGroup | [support leading actions, multiple editors and rich parts](024-input-group-composition.md) | Open |
| KUMO-025 | Components | [complete pinned browser comparisons for the implemented catalog](025-implemented-catalog-browser-parity.md) | Open |
| KUMO-026 | Autocomplete | [port Autocomplete](026-autocomplete-port.md) ([#11](https://github.com/msmps/gpui-kumo/issues/11)) | Open |
| KUMO-027 | Breadcrumbs | [port Breadcrumbs](027-breadcrumbs-port.md) ([#12](https://github.com/msmps/gpui-kumo/issues/12)) | In progress |
| KUMO-028 | ClipboardText | [port ClipboardText](028-clipboard-text-port.md) ([#13](https://github.com/msmps/gpui-kumo/issues/13)) | Open |
| KUMO-029 | CodeHighlighted | [port CodeHighlighted](029-code-highlighted-port.md) ([#14](https://github.com/msmps/gpui-kumo/issues/14)) | Open |
| KUMO-030 | Collapsible | [port Collapsible](030-collapsible-port.md) ([#15](https://github.com/msmps/gpui-kumo/issues/15)) | In progress |
| KUMO-031 | Combobox | [port Combobox](031-combobox-port.md) ([#16](https://github.com/msmps/gpui-kumo/issues/16)) | Open |
| KUMO-032 | CommandPalette | [port CommandPalette](032-command-palette-port.md) ([#17](https://github.com/msmps/gpui-kumo/issues/17)) | Open |
| KUMO-033 | DatePicker | [port DatePicker](033-date-picker-port.md) ([#18](https://github.com/msmps/gpui-kumo/issues/18)) | Open |
| KUMO-034 | Dialog | [port Dialog](034-dialog-port.md) ([#19](https://github.com/msmps/gpui-kumo/issues/19)) | Open |
| KUMO-035 | Dropdown | [port Dropdown](035-dropdown-port.md) ([#20](https://github.com/msmps/gpui-kumo/issues/20)) | Open |
| KUMO-036 | Grid | [port Grid](036-grid-port.md) ([#21](https://github.com/msmps/gpui-kumo/issues/21)) | Open |
| KUMO-037 | InlineCopyText | [port InlineCopyText](037-inline-copy-text-port.md) ([#22](https://github.com/msmps/gpui-kumo/issues/22)) | In progress |
| KUMO-038 | InputArea | [port InputArea](038-input-area-port.md) ([#23](https://github.com/msmps/gpui-kumo/issues/23)) | In progress |
| KUMO-039 | LayerDialog | [port LayerDialog](039-layer-dialog-port.md) ([#24](https://github.com/msmps/gpui-kumo/issues/24)) | Open |
| KUMO-040 | Meter | [port Meter](040-meter-port.md) ([#25](https://github.com/msmps/gpui-kumo/issues/25)) | In progress |
| KUMO-041 | Pagination | [port Pagination](041-pagination-port.md) ([#26](https://github.com/msmps/gpui-kumo/issues/26)) | In progress |
| KUMO-042 | Select | [port Select](042-select-port.md) ([#27](https://github.com/msmps/gpui-kumo/issues/27)) | Core implemented; parts in progress |
| KUMO-043 | SensitiveInput | [port SensitiveInput](043-sensitive-input-port.md) ([#28](https://github.com/msmps/gpui-kumo/issues/28)) | In progress |
| KUMO-044 | SkeletonLine | [port SkeletonLine](044-skeleton-line-port.md) ([#29](https://github.com/msmps/gpui-kumo/issues/29)) | In progress |
| KUMO-045 | Table | [port Table](045-table-port.md) ([#30](https://github.com/msmps/gpui-kumo/issues/30)) | Open |
| KUMO-046 | TableOfContents | [port TableOfContents](046-table-of-contents-port.md) ([#31](https://github.com/msmps/gpui-kumo/issues/31)) | Open |
| KUMO-047 | Tabs | [port Tabs](047-tabs-port.md) ([#32](https://github.com/msmps/gpui-kumo/issues/32)) | Open |
| KUMO-048 | TagInput | [port TagInput](048-tag-input-port.md) ([#33](https://github.com/msmps/gpui-kumo/issues/33)) | Open |
| KUMO-049 | Toast | [port Toast](049-toast-port.md) ([#34](https://github.com/msmps/gpui-kumo/issues/34)) | Open |
| KUMO-050 | Toolbar | [port Toolbar](050-toolbar-port.md) ([#35](https://github.com/msmps/gpui-kumo/issues/35)) | Open |

## Published GitHub issues

The following fidelity gaps are published on 2026-10-02. GitHub numbers differ from local KUMO IDs; other historical local issues remain unpublished. No implementation or platform-validation gap is resolved by publication.

| Local ID | GitHub issue | Category |
| --- | --- | --- |
| KUMO-007 | [#1 — Input: validate composition through operating-system IMEs](https://github.com/msmps/gpui-kumo/issues/1) | Outstanding validation |
| KUMO-014 | [#2 — Components: validate spoken screen-reader workflows on Linux](https://github.com/msmps/gpui-kumo/issues/2) | Outstanding validation |
| KUMO-015 | [#3 — Components: validate implemented controls with VoiceOver](https://github.com/msmps/gpui-kumo/issues/3) | Outstanding validation |
| KUMO-016 | [#4 — Components: validate implemented controls with a Windows screen reader](https://github.com/msmps/gpui-kumo/issues/4) | Outstanding validation |
| KUMO-019 | [#5 — LayerCard: preserve rounded clipping for arbitrary descendants](https://github.com/msmps/gpui-kumo/issues/5) | Dependency limitation |
| KUMO-021 | [#6 — Tooltip: match Kumo motion and grouped instant switching](https://github.com/msmps/gpui-kumo/issues/6) | Missing implementation |
| KUMO-022 | [#7 — Tooltip: support rich content and native trigger composition](https://github.com/msmps/gpui-kumo/issues/7) | Missing implementation |
| KUMO-023 | [#8 — Checkbox/Switch: complete contextual help and Checkbox.Item mixed presentation](https://github.com/msmps/gpui-kumo/issues/8) | Missing implementation |
| KUMO-024 | [#9 — InputGroup: support leading actions, multiple editors and rich parts](https://github.com/msmps/gpui-kumo/issues/9) | Missing implementation |
| KUMO-025 | [#10 — Components: complete pinned browser comparisons for the implemented catalog](https://github.com/msmps/gpui-kumo/issues/10) | Outstanding validation |

## Remaining component family ports

The original 25-family queue has one confirmed GitHub issue per family;16 remain unported. Family issues each (#11–#35), linked above and in [coverage](../component-coverage.md). Local IDs KUMO-026–050 distinguish these family ports from earlier fidelity follow-ups. Issues describe the pinned Base primitives, dependency order, limitations and acceptance gates: 11 direct counterparts, 7 partial compositions and 7 without a dedicated Base counterpart. A direct name does not establish complete behavior; Tabs keyboard navigation, Dropdown menu semantics and Meter range semantics require particular care. Publication does not change implementation coverage.

## Maintaining the tracker

Shared follow-up: [KUMO-051 — disabled Buttons export Enabled/Sensitive on Linux](051-linux-disabled-button-state.md) ([#36](https://github.com/msmps/gpui-kumo/issues/36)), Resolved. Found through Pagination's actual AT-SPI gate; applies to all Buttons.

Completed foundation: [KUMO-052 — readable values on Label nodes](052-readable-label-values.md) ([#37](https://github.com/msmps/gpui-kumo/issues/37)), Resolved. Same native gate exposed the distinction between authored aria_label and platform names; affects Text/Label/Badge/Banner/BreadcrumbCurrent.

- Keep IDs and filenames stable. Add new issues using the next unused ID.
- Update the issue's status and this index together. Use Open, In progress, Blocked or Resolved; record the reason when blocked.
- When resolving an issue, add the relevant commit or PR and verification evidence. Keep the file as history.
- When GitHub access is restored, check for existing issues before publishing. Record the resulting URL in the individual file and keep its local ID in the GitHub issue body for traceability.
- Continue tracking work here until publication is confirmed. A prepared draft or failed API request does not mean an issue exists on GitHub.

Visual follow-up: [KUMO-053 — rich Text highlight font fidelity](053-rich-text-highlight-font.md) ([#38](https://github.com/msmps/gpui-kumo/issues/38)), Open. Native review discovered an existing serif-looking highlight; inspect pinned shaping/font resolution before changing typography.

Shared follow-up: [KUMO-054 — authored busy-state export on Linux](054-linux-busy-state.md) ([#39](https://github.com/msmps/gpui-kumo/issues/39)), Resolved. Authored Busy now exports independently of availability, with actual state/event and four-combination native checks.

Necessary example repair: [KUMO-055 — Button gallery narrow composition](055-button-gallery-narrow-layout.md) ([#40](https://github.com/msmps/gpui-kumo/issues/40)), Resolved. Intrinsic Button geometry preserved; counter, variant and size-row wrapping verified natively in all four combinations.

[ KUMO-056 — Linux expansion export](056-linux-expansion-state.md) ([#41](https://github.com/msmps/gpui-kumo/issues/41)), Resolved. Exact pinned adapter exports authored optional expansion; actual Adapter regressions and strict native both-theme wide/narrow checks pass.

[KUMO-057 — narrow foundation gallery](057-foundation-gallery-narrow-layout.md) ([#42](https://github.com/msmps/gpui-kumo/issues/42)), Open. Actual swatch-column overflow below Button panels remains a separate gallery repair.

[KUMO-058 — reproducible CI](058-reproducible-ci.md) ([#43](https://github.com/msmps/gpui-kumo/issues/43)), In progress.

[KUMO-059 — native InputArea test shortcuts](059-input-area-platform-shortcuts.md) ([#44](https://github.com/msmps/gpui-kumo/issues/44)), Resolved in744354a.
