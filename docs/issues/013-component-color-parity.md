# KUMO-013: Components: measure browser color gamut and gradient parity

Status: Open

GitHub issue: Pending publication

## Problem

The native color conversion reduces out-of-gamut chroma at fixed lightness/hue, intentionally differing from CSS local-MINDE. It maps gradient stops before GPUI interpolation; browsers may gamut-map individual interpolated colors. Button/Input/Popover color fidelity is not fully measured.

## Evidence

- [Native token/color policy](../kumo-tokens.md#initial-native-implementation)
- [Color conversion](../../crates/gpui-kumo/src/color.rs)
- [Theme and emphasis gradients](../../crates/gpui-kumo/src/theme.rs)

## Acceptance criteria

- [ ] Measure the selected semantic roles, emphasis mixes and intermediate gradient colors against the pinned browser in both appearances.
- [ ] Control browser version, color space and display profile; distinguish CSS conversion from font/rasterization differences.
- [ ] Decide whether to adopt closer gamut mapping/interpolation or retain the native algorithm, backed by measured error and contrast implications.
- [ ] Add numerical regression coverage for changed conversion/interpolation behavior and update the native policy.

Baseline: commit `38c2ea8`, GPUI Kit/Base 0.7.0 and GPUI 0.3.7. Where a browser comparison is needed, use Kumo commit `3fd5b648df578cb1ba214dedd30f475009f6a668`.
