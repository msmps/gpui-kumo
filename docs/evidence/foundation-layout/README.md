# Foundation layout acceptance (#42)

Native Linux acceptance completed on 2026-10-03 over `3eee1325af367f03f9481fdd04fce284ebbf9027`. No further gallery recipe change was needed. [Current captures, reproduction and review](linux/README.md). Focused and full-gallery Light/Dark at 1040/520, plus actual 737/738 resize transitions, preserve readable text, panel containment and complete shadow samples. The full pinned Linux Rust script passes: 175 library / 1 gallery / 9 doctests and 12 default / 14 all-feature standalone adapter tests, formatting, warning-denied lint and locked builds. [Raw gate](../cloud-native-2026-10-03/linux-rust-gate.log).

The bounded gallery repair is accepted and published inf03da05; successful remote CI37119510684 is verified and GitHub#42 checklist is reconciled/closed. No browser parity, arbitrary-descendant clipping, speech, OS IME or other-platform acceptance is inferred.

## Earlier handoff evidence (historical)


Implementation/source review and rendered geometry regression pass for Light/Dark1040/520 and737/738 threshold resize transitions. Published native checkpoint remains incomplete: only `light-1040.png` is a saved reviewed final capture. A Light520 review showed contained stacked labels/gradients but used the earlier1000px-height preview and cut off Shadow samples; it is not a complete native gate and was not saved as final evidence. Final preview now uses1200px height. Both-theme full captures, narrow shadows/text bounds and final skeptical visual review remain required before closing#42.

Reproduce: `cargo run -p kumo-gallery --example foundations --locked -- --width=520 --dark` (omit `--dark` for Light;1040 for wide). Preview shares the exact gallery foundation renderer; resize actual windows and also inspect full-gallery composition. No claim about native accessibility/speech from the preview: macOS UI automation exposed only the window chrome during this session.

Source review preserved token values, gradient/rest/hover samples, padding, radii, fills/borders/shadows. Wide Light captured review checked label/swatch centres/baselines, sibling spacing, edge padding and rounded/layer agreement. Regression checks panel and effect-column bounds, not glyph or shadow paint bounds. No arbitrary-descendant clipping repair is implied.
