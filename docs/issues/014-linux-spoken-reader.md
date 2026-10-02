# KUMO-014: Components: validate spoken screen-reader workflows on Linux

Status: Open

GitHub issue: https://github.com/msmps/gpui-kumo/issues/2

## Problem

Passing headless metadata and Linux AT-SPI tree/action/focus checks do not establish spoken navigation and announcements. Complete an actual Orca workflow on Linux/AT-SPI for the implemented catalog, starting with Button, Input and Popover.

## Evidence

- [Current verification scope](../popover-validation.md#concrete-limits)
- [Component contracts](../kumo-component-recipes.md)
- [Verification requirements](../gpui-verification.md)

## Acceptance criteria

- [ ] Record X11/Wayland scope explicitly; current native evidence covers X11.
- [ ] Check named Button activation, icon-only names, loading/unavailable announcements and traversal.
- [ ] Check Input label/help/error meaning, value/caret/selection announcements and editing, with read-only and disabled states.
- [ ] Check Popover expanded/dialog meaning, initial focus, nested navigation, Escape/outside dismissal and focus restoration.
- [ ] Verify Checkbox/Radio/Switch state and grouping, Field label/help/error meaning, InputGroup control order and availability, ButtonGroup navigation, and Tooltip trigger-to-help announcements; report unsupported relationships explicitly.
- [ ] Attach a results matrix with passed/failed/blocked outcomes and reproducible steps; raise distinct discovered defects.

## Scope

Coordinate with the dedicated metadata/text-interface issues. Do not mark unsupported capabilities verified from descriptions or tree presence alone.

Baseline: commit `38c2ea8`, GPUI Kit/Base 0.7.0 and GPUI 0.3.7. Where a browser comparison is needed, use Kumo commit `3fd5b648df578cb1ba214dedd30f475009f6a668`.

Current coverage extension: 18 implemented families at `d07646f8d24acf72248f954a03c8da056d84848d`; use [coverage](../component-coverage.md) and individual validation matrices to mark role/state/name exposure as supported, unsupported or unverified. No screen-reader workflow is claimed from a headless role name alone.
