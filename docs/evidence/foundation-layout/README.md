# Foundation layout checkpoint (#42)

Implementation/source review and rendered geometry regression pass for Light/Dark1040/520 and737/738 threshold resize transitions. Published native checkpoint remains incomplete: only `light-1040.png` is a saved reviewed final capture. A Light520 review showed contained stacked labels/gradients but used the earlier1000px-height preview and cut off Shadow samples; it is not a complete native gate and was not saved as final evidence. Final preview now uses1200px height. Both-theme full captures, narrow shadows/text bounds and final skeptical visual review remain required before closing#42.

Reproduce: `cargo run -p kumo-gallery --example foundations --locked -- --width=520 --dark` (omit `--dark` for Light;1040 for wide). Preview shares the exact gallery foundation renderer; resize actual windows and also inspect full-gallery composition. No claim about native accessibility/speech from the preview: macOS UI automation exposed only the window chrome during this session.

Source review preserved token values, gradient/rest/hover samples, padding, radii, fills/borders/shadows. Wide Light captured review checked label/swatch centres/baselines, sibling spacing, edge padding and rounded/layer agreement. Regression checks panel and effect-column bounds, not glyph or shadow paint bounds. No arbitrary-descendant clipping repair is implied.
