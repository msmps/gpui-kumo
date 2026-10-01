# KUMO-015: Components: validate Button, Input and Popover with VoiceOver

Status: Open

GitHub issue: Pending publication

## Problem

Passing headless metadata and Linux AT-SPI tree/action/focus checks do not establish spoken navigation and announcements. Complete an actual VoiceOver workflow on macOS for the three implemented components.

## Evidence

- [Current verification scope](../popover-validation.md#concrete-limits)
- [Component contracts](../kumo-component-recipes.md)
- [Verification requirements](../gpui-verification.md)

## Acceptance criteria

- [ ] Record OS, architecture and screen-reader settings; ordinary macOS gallery smoke is not spoken-reader acceptance.
- [ ] Check named Button activation, icon-only names, loading/unavailable announcements and traversal.
- [ ] Check Input label/help/error meaning, value/caret/selection announcements and editing, with read-only and disabled states.
- [ ] Check Popover expanded/dialog meaning, initial focus, nested navigation, Escape/outside dismissal and focus restoration.
- [ ] Attach a results matrix with passed/failed/blocked outcomes and reproducible steps; raise distinct discovered defects.

## Scope

Coordinate with the dedicated metadata/text-interface issues. Do not mark unsupported capabilities verified from descriptions or tree presence alone.

Baseline: commit `38c2ea8`, GPUI Kit/Base 0.7.0 and GPUI 0.3.7. Where a browser comparison is needed, use Kumo commit `3fd5b648df578cb1ba214dedd30f475009f6a668`.
