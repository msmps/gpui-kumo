# KUMO-051: Correct disabled Button state export on Linux

Status: In progress

GitHub issue: https://github.com/msmps/gpui-kumo/issues/36

## Measured defect

During Pagination #26 validation on Linux X11, unavailable Next/Last Buttons correctly reject activation and remove the accessible Click action, but AT-SPI still reports ENABLED and SENSITIVE. Kumo's disabled meaning is therefore incompletely exported. This is not a claim about spoken Orca output.

## Exact dependency cause

Pinned GPUI 0.3.7 uses accesskit_atspi_common 0.19.1. Its src/node.rs state mapping inserts ReadOnly only when is_read_only_supported() && is_read_only_or_disabled(); otherwise it inserts Enabled | Sensitive. accesskit_consumer's is_read_only_supported() excludes Role::Button, so a Button's actual AccessKit disabled flag does not clear Enabled/Sensitive. Kumo Button enriches the actual Base node with set_disabled(); Base correctly removes Click when unavailable.

## Acceptance

- [ ] Reproduce with native enabled/disabled Buttons, including Pagination boundary directions, busy Buttons and transitions, and record actual AccessKit versus AT-SPI state/actions.
- [ ] Evaluate a narrowly scoped upstream correction or pinned patch; preserve existing Base behavior and Kumo styling, and do not alter a Button's role to conceal the export bug.
- [ ] Verify Enabled/Sensitive are absent when disabled and restored when enabled; ReadOnly applies only to supported roles. Verify pointer/Space/Enter and accessible action remain guarded, focus/Tab policy unchanged.
- [ ] Run required Rust checks and actual X11 AT-SPI regression; record Wayland/Orca/macOS/Windows scope independently.

Related Linux workflow: #2. Component evidence will be recorded in docs/pagination-validation.md. Platform state export remains a fidelity gap; working implementation does not imply full platform parity.
