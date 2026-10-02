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
| KUMO-009 | Popover | [export expanded/collapsed state through Linux AT-SPI](009-popover-expanded-state.md) | Open |
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

## Maintaining the tracker

- Keep IDs and filenames stable. Add new issues using the next unused ID.
- Update the issue's status and this index together. Use Open, In progress, Blocked or Resolved; record the reason when blocked.
- When resolving an issue, add the relevant commit or PR and verification evidence. Keep the file as history.
- When GitHub access is restored, check for existing issues before publishing. Record the resulting URL in the individual file and keep its local ID in the GitHub issue body for traceability.
- Continue tracking work here until publication is confirmed. A prepared draft or failed API request does not mean an issue exists on GitHub.
