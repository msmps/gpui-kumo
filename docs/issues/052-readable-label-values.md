# KUMO-052: Expose readable values on actual Label nodes

Status: Resolved

GitHub issue: https://github.com/msmps/gpui-kumo/issues/37

## Problem and evidence

Pagination #26 native AT-SPI validation found a visible info Label had an empty exported name despite its aria_label. Installed accesskit_consumer0.38.0 Node::label_comes_from_value() is true for Role::Label, and accesskit_atspi_common0.19.1 NodeWrapper::name reads value for that role. An aria_label alone is insufficient. Pagination now authors both full name and value; native retest is in progress.

Source audit found the same omission in Text, Label (including optional names), Badge, Banner title/plain description, and BreadcrumbCurrent. Their source nodes have Role::Label + aria_label and no aria_value. Existing headless name assertions did not establish platform reading. Decorative/as-content labels and loading Breadcrumbs must retain their existing semantics policy.

## Acceptance

- [x] Reproduce actual exported names on the pinned Linux adapter for the affected components, including Unicode, long/truncated text and optional labels; retain before/after evidence.
- [x] Author full readable values on actual Label nodes using installed GPUI APIs. Preserve heading semantics, caller-owned rich content, loading/hidden policies, focus/activation and Kumo presentation; do not manufacture a live region.
- [x] Add observable value/name regressions and verify multiple instances and owner/theme updates.
- [x] Run fmt, workspace tests/doctests, warning-denied all-target/all-feature Clippy/build and native both-theme wide/narrow names/appearance review. Independently review the diff.
- [x] Keep OS speech/VoiceOver/Windows workflows separate; Linux metadata is not proof of spoken behavior.

Related #2; discovered while finishing #26. Fix this shared foundation, then #36 disabled Button export, then resume Pagination dropdown/PageSize. No upstream version chase or unrelated rewrite is needed.

Results:166 workspace tests/nine doctests and required Rust gates pass. Actual Linux four-theme/width probe now exports all143 Label names, zero unnamed; all nine samples match complete values, heading preserved, all bounds identical before/after. Independent review found no blocker. See [acceptance and evidence](../readable-label-validation.md).
