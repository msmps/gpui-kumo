# Pagination dropdown recreation evidence

Issue [#26](https://github.com/msmps/gpui-kumo/issues/26), local tracker KUMO-041. This is a recreation over verified `work` at `6c316ab533e2ab7cb5d54e694d2c9c3f10b3686e`; the earlier unpublished candidate's browser/native results are not evidence for this code.

| Revision / run | Measured result |
| --- | --- |
| `46f919ef94fb8e4e11672e0ace5115c44992f23c`, [37117276643 / job111186393342](https://github.com/msmps/gpui-kumo/actions/runs/37117276643/job/111186393342) | Compiled; 173/175 library tests passed. Two new harness assertions failed; later stages did not execute |
| `dfbbdc06bb598026416f42eb3736dd7c5efccb65` | Test host now registers the gallery's global Tab actions; exact outer Button border bounds retain the full-opacity assertion. No CI run appeared for this intermediate push |
| `d3bfbe96c170417294f0cc6f2f1ad3aedfbdd33b`, [37117625758 / job111187386063](https://github.com/msmps/gpui-kumo/actions/runs/37117625758/job/111187386063) | Full pinned macOS script passed: 175 library + 1 gallery + 9 doctests; workspace and adapter formatting/lint/build/default gallery; adapter 12 default / 14 all-feature tests. Added actual activation-to-root-removal cancellation regression passes |
| `033ccfde78c5fdf1f925fc3e03ba8a9671f45278`, [37117805585 / job111187887143](https://github.com/msmps/gpui-kumo/actions/runs/37117805585/job/111187887143) | Final source review retains the standalone middle Select's shadow-xs. Full pinned macOS gate passed with the same 175 library / 1 gallery / 9 doctest and 12 / 14 adapter counts |

[Initial failing excerpt](initial-test-failure.log) and [repaired full gate excerpt](repaired-rust-gate.log) and [final source gate excerpt](final-rust-gate.log) preserve raw Actions timestamps/results. Job success confirms the complete `scripts/check-rust.sh`, including its silent formatting checks and warning-denied lint. Existing dependency warnings remain visible in full logs; versions and vendor patches are unchanged.

Local pinned Rust 1.99 formatting passes for workspace and adapter. PageSize probe Python syntax passes. The Linux test command stops before execution at missing `hdrhistogram` metadata; an HTTPS probe cannot connect to configured `proxy:8080`. No local tests/lint/build, new native captures, browser rendering, OS IME or spoken-reader acceptance is claimed. Both-theme geometry/paint tests are headless regression evidence, not native glyph/adapter proof.

Remaining acceptance: native Light/Dark at 1040/520, actual labels/glyphs/centres/baselines/gaps/padding, joined corners/borders/fills/focus/shadows, open/hover paint, placement/collision/scrolling and PageSize native regression; pinned browser comparison and supported-platform gates. #26 stays open. [Current contract](../../../pagination-dropdown-checkpoint.md).
