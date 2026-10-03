# GPUI Kumo continuation

## Goal and current state

Continue porting supported, recommended pinned Kumo to native Rust/GPUI. The user authorized implementation and coherent frequent commits/pushes. Complete required local checks, preserve measured evidence and distinguish working implementations from full fidelity. The current request paused component work for repository/issue cleanup; [audit](issue-audit-2026-10-03.md) and [tracker](issues/README.md) record its outcome.

`work` code checkpoint **c1026fe042c133929eea2ef15da6e6a05b57c617** composes Dropdown → Dialog → confirmed operation → Toast. Local gate passes226library/1gallery/9doctests and adapter12/14, formatting/warnings-denied lint/locked builds. Toast94351f4 and previewc1026fe both pass remote macOS CI. [Progress](port-progress.md), [announcement preview/evidence](evidence/announcement/README.md).

Coverage **32/43 working families,11unported**. Missing: Autocomplete, ClipboardText, CodeHighlighted, Combobox, CommandPalette, DatePicker, Grid, LayerDialog, Table, TableOfContents, TagInput. All their issues stay open. Implemented-family issues describe concrete remaining work; SkeletonLine implementation #29 is closed, with browser/OS preference validation in #10. The demo path needs no new family, but final intended-platform visual recording remains. The13-second movie at `/workspace/artifacts/gpui-kumo-announcement-rehearsal.mp4` is a Linux rehearsal, not macOS visual/speech/IME acceptance or a published announcement.

## Next work

After cleanup, resume dependency-ready LayerDialog #24 over existing Dialog/LayerCard. Source is responsive Drawer composition: desktop width448/576/672/768, center/top alignment, mobile bottom sheet capped85dvh, sticky title/condensing description, typed strict body/actions slots and user-dismissal policy. Verify full pinned source/docs/demos/tests and Base APIs before implementation; do not treat it as a renamed plain Dialog. Preserve existing family/composition/motion/platform gaps. See [coverage](component-coverage.md), [LayerDialog issue](issues/039-layer-dialog-port.md).

## Pins and workflow

- Kumo `3fd5b648df578cb1ba214dedd30f475009f6a668`; Kit/Base0.7.0; GPUI family0.3.7; Rust1.99.0.
- Read root AGENTS.md, [docs entry point](README.md) and [CONTEXT](../CONTEXT.md); preserve all vendored patch provenance and downstream obligations.
- Run `bash scripts/check-rust.sh` for implementation. Validation/tracker-only changes need relevant consistency/link/diff checks, not speculative test additions.
- Gallery: `cargo run -p kumo-gallery --locked`; focused preview: `cargo run -p kumo-gallery --example announcement --locked`, optional `-- --dark` / `-- --width=520`.
- Commit/publish coherent milestones and reconcile GitHub/local issues using concrete evidence. Keep remaining implementation distinct from #10 browser/preference validation, #1 OS IME and #2–#4 spoken platform acceptance.

## Managed environment

Repository `/workspace/gpui-kumo`; setup `source /workspace/.kumo-setup/activate.sh`. Xvfb100, D-Bus/AT-SPI and Lavapipe run real native probes; component scripts document their required runtime/Python/driver paths. Network was restored; obsolete proxy/dependency blocker reports are historical.

Shell Git write credentials expired during preview publication. Authorized GitHub connector blob/tree/commit + non-force ref update published the identical reviewed tree; ordinary anonymous Git fetch worked. Local/remote trees were compared and the local checkpoint retained before reconciling `work`. Never expose credentials or force-push to bypass a failure.

Earlier candidate-recovery, failed gates, platform diagnoses and old checkpoint queues are preserved in [historical handoff](history/cloud-handoff-before-cleanup-2026-10-03.md). Use current contracts/evidence and the audit rather than resuming a superseded blocked milestone.
