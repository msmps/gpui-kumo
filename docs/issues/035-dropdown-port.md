# KUMO-035: Dropdown: finish menu parts and overlay composition

Status: In progress

GitHub issue: https://github.com/msmps/gpui-kumo/issues/20

Local tracker: KUMO-035

## Current scope — 2026-10-03

**Working native implementation; remaining API/fidelity work.** Flat action menu, stable row identity, navigation/typeahead, current activation guards, dismissal/restoration and lifetime cleanup are implemented.

Audited at `c1026fe042c133929eea2ef15da6e6a05b57c617`: 32/43 working families, 11 unported. This issue no longer tracks an unstarted port.

## Remaining acceptance

- [ ] Port CheckboxItem, RadioGroup/Item/Indicator and LinkItem navigation.
- [ ] Port submenus and nested boundaries, semantic groups and joined/rich trigger composition.
- [ ] Complete placement/alignment and parent Popover integration.
- [ ] Match source motion, group-header font and focus-visible modality; verify RTL.

Browser comparison belongs to #10; OS IME to #1 where applicable; spoken Linux/VoiceOver/Windows acceptance to #2–#4. Native tests and metadata do not establish spoken or full platform fidelity. Existing native adaptations and dependency limitations remain in the linked contract.

## Evidence

- [Component contract and measured evidence](https://github.com/msmps/gpui-kumo/blob/work/docs/dropdown-validation.md)
- [Current progress](https://github.com/msmps/gpui-kumo/blob/work/docs/port-progress.md)
- [Coverage](https://github.com/msmps/gpui-kumo/blob/work/docs/component-coverage.md)

Latest shared locked gate passes226 library/1gallery/9doctests and adapter12/14, fmt/warnings-denied lint/builds. [Announcement checkpoint macOS CI37139317488](https://github.com/msmps/gpui-kumo/actions/runs/37139317488) passes. That gate establishes build/regression health, not this issue's remaining visual/platform acceptance.

Kumo `3fd5b648df578cb1ba214dedd30f475009f6a668`, Kit/Base0.7.0, GPUI0.3.7 and Rust1.99.0 remain pinned; existing vendor provenance stays authoritative. Historical implementation checkpoints remain in the contract and Git history.
