# KUMO-059: InputArea native test shortcuts

GitHub: [#44](https://github.com/msmps/gpui-kumo/issues/44). Status: Implemented and verified; checkpoint push pending.

At published33ab766, macOS workspace tests reproduced three failures: empty selected text or absent clipboard. Baseline166 passed/3 failed. Installed Base0.7 binds selection/copy/paste/undo to Command on macOS, Control elsewhere; Control-A means line start on macOS. Existing tests hard-coded Linux shortcuts.

Acceptance: use actual native keystrokes with unchanged assertions and production behavior; four InputArea rendered tests, workspace tests/doctests, fmt, warning-denied Clippy/build and independent review. Test-only platform modifier helper passes all four InputArea regressions and169 library tests/nine doctests; independent review found no blocker. No speech/IME/native clipboard delivery claim.
