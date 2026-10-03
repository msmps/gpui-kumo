# Pinned Linux disabled-state correction

Tracker [#36/KUMO-051](issues/051-linux-disabled-button-state.md). Selected after the readable Label foundation #37. Applies to every disabled or busy Button; no appearance, role, Base interaction or public component API change.

## Source and scope

`vendor/accesskit_atspi_common-0.19.1` comes from the exact published19-file crate archive, SHA256 `023da0e5097f46df7092d5280b02efb9bbf8d93298daeced42652463e357d636`, upstream revision `c88605b96d04431f9c3c792464a0f2f253480e94`, path `platforms/atspi-common`. Consumer0.38.0 and AccessKit0.24.1 remain pinned by Cargo.lock. The archive omits license files; MIT/Apache licenses are copied from that exact upstream revision, preserving source copyright headers.

Only `src/node.rs` production mapping changes: unsupported read-only roles previously entered the unconditional Enabled/Sensitive branch even when disabled. Guard that branch with `!state.is_disabled()`. Keep supported read-only/disabled roles' existing ReadOnly mapping. Three adapter regressions in `src/adapter.rs` verify Button/MenuItem/TextInput/Switch enable→disable→enable state queries, supported/unsupported read-only behavior and emitted Button Enabled/Sensitive change events in both directions. No role substitution or invented accessible action. Root Cargo patch redirects just this version, and an exclude allows standalone feature tests without modifying its manifest; root lock only drops the registry source/checksum for that package.

Linux uses this adapter; this does not alter or establish other-platform metadata, Orca speech or Wayland workflows. Kumo Button already authors AccessKit disabled/busy flags, Base removes unavailable Click, and existing pointer/keyboard/focus guards remain authoritative.

## Acceptance and validation

| Area | Acceptance / method |
| --- | --- |
| Disabled and busy | Actual Linux Enabled/Sensitive absent; Click absent; six busy variant nodes unavailable. Strict committed native Pagination probe in both themes1040/520 |
| Transitions | Enabled/Sensitive restored after owner reset, exported change events report false/true; upstream actual Adapter state/event tests |
| Read-only | Supported roles retain ReadOnly; unsupported Button does not gain ReadOnly; upstream role regressions |
| Input/focus | Pointer/Space/Enter guarded at boundary, normal activation once, retained input editing/clipboard and focus/Tab behavior preserved; native probe and existing rendered Button/Pagination tests |
| Appearance | Both-theme wide/narrow pointer/keyboard and boundary captures; inspect glyph/control centres, joined borders/rings, corners/fills/layers. Metadata-only production patch must preserve presentation |
| N/A | New variants/sizes, controlled event contracts, popup dismissal/restoration, layout ownership, IME implementation; no component implementation changes |
| Gate | Failing prepatch regression, upstream default/all-feature tests, explicit vendor formatting/Clippy, required workspace gates, native strict probe and independent review |

Baseline: actual Linux Pagination unavailable Next/Last rejects activation/removes Click but exposes Enabled/Sensitive. A private exact-source regression fails on disabled Button Enabled; the one-line candidate guard passes11 all-feature tests before the emitted-event regression is added. Final integrated results are recorded below after checks finish; do not count this candidate experiment as consumer/native validation.

## Consumer and removal policy

Cargo patches apply at the consumer workspace root, not through dependency manifests. A downstream application requiring this pinned Linux repair must copy/reference this vendored crate and add `accesskit_atspi_common = { path = "…/vendor/accesskit_atspi_common-0.19.1" }` under its own `[patch.crates-io]`, alongside the already documented GPUI/Base patches. Keep its ordinary0.19.1 version and matching lock baseline. Remove the patch only when the selected upstream version corrects disabled state export and the same state/event/native gates pass; do not silently upgrade during this port.

Native fixture review: the pinned adapter does not map the authored AccessKit busy flag to AT-SPI Busy. Identify the six loading Buttons by their existing native `Loading` descriptions to check unavailable state/actions independently; do not treat absence of Busy as absence of loading samples. Busy-state export is a separate shared follow-up, not repaired by this disabled-state guard.

## Integrated results

Passed:166 workspace tests/nine doctests, workspace formatting, warning-denied all-target/all-feature Clippy/build, default native gallery build; upstream10 default and12 all-feature tests (three added); explicit changed-vendor formatting and warning-denied all-target/all-feature Clippy. Logs `/tmp/disabled-state-{fmt,test,clippy,build,native-build,upstream-default,upstream-all,vendor-fmt,vendor-clippy}.log`. Archive comparison confirms only node.rs/adapter.rs differ from published files. Production diff is exactly the disabled guard; tests and copied exact licenses are separate.

The committed strict native probe `scripts/validate-pagination-accessibility.py --require-disabled-state` passed all four light/dark1040/520 combinations. Owner reset restores Enabled/Sensitive; boundary Next exposes only Showing/Visible, no Click, and real pointer/Space/Enter cannot activate. Seven loading Buttons, including all six variant samples, likewise omit Enabled/Sensitive/Click. Existing normal pointer/Space/Enter progresses once6→7→8; huge draft clamps10, malformed restores10, blur commits3, real clipboard contains3, controlled rejection and unknown server boundary remain correct. [Raw results and eight keyboard/boundary captures](evidence/linux-disabled-state/) preserve observations. Busy flag is measured false and explicitly remains #39; this gate passes disabled meaning only. No speech claim.

Independent source review found no blocker. Seven captures independently reviewed and final light520 boundary reviewed by primary agent: actual themes, glyph/control centres, page digit placement/info baselines, joins/focus borders/outer corners/disabled fills show no new regression. Initial busy-based fixture identification failed because Busy is unexported; corrected identification uses Loading descriptions without weakening availability/action assertions. This recurring evidence rule is now explicit: identify fixtures by known exported properties and report each unsupported semantic separately.

Next selected #39: map the already-authored busy flag, test Busy transitions while preserving enabled versus disabled meaning, then strict native both-theme/width loading checks. Then resume #26 Pagination dropdown/PageSize; font finding #38 remains in the backlog.

Historical scope: the results above record the #36 checkpoint4144c60. The subsequent [#39 Busy repair](linux-busy-state-validation.md) adds the independent authored Busy mapping to the same pinned source, retaining this disabled-state guard and its regressions.
