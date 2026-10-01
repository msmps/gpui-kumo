# KUMO-017: Components: validate the native gallery and interactions on Windows

Status: Open

GitHub issue: Pending publication

## Problem

The committed slice has macOS bootstrap and Linux native smoke evidence, but the Windows native component workflow and font fallback have not been validated. This task covers native runtime/input/rendering; spoken reader acceptance is separate.

## Evidence

- [Dependency/platform baseline](../gpui-sources.md)
- [Implementation plan](../implementation-plan.md)
- [Native font profile](../kumo-tokens.md#initial-native-implementation)

## Acceptance criteria

- [ ] Build and open the actual Windows gallery with the pinned dependency set and record build/prerequisite/toolchain details.
- [ ] Exercise pointer/Enter/Space activation once, unavailable traversal, Input Unicode/clipboard/undo and Popover inside/outside/nested dismissal with focus restoration.
- [ ] Inspect both appearances, fallback fonts, display scaling and all four native edge-flipping cases.
- [ ] Record supported scope and regressions separately from the Windows screen-reader and real-IME follow-ups.

Baseline: commit `38c2ea8`, GPUI Kit/Base 0.7.0 and GPUI 0.3.7. Where a browser comparison is needed, use Kumo commit `3fd5b648df578cb1ba214dedd30f475009f6a668`.
