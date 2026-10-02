# Rounded ring edge repair

User review of the neutral-off Switch exposed visibly dark corner notches, especially in light appearance. Existing geometry tests and normal-size captures had missed the pixel defect. Enlarged review is now required for inner and outer rounded edges.

The installed GPUI 0.3.7 WGPU `fs_quad` shader blends the border over its background, then interpolates those straight RGB/alpha values at the inner boundary and finally premultiplies. A transparent-black background contributes black RGB during interpolation, darkening partially covered border pixels. The installed Metal shader has the same relevant interpolation. This is a source diagnosis; native Metal pixels remain unverified.

Own border-only quads now use the border colour with zero alpha for their transparent background. This retains RGB during interpolation and adds no opaque interior fill, including translucent focus rings. Button, Checkbox, Radio, Switch, Input, Popover, Link, Empty command and Badge link-hover ring are repaired. LayerCard/Loader fill-only quads and intentionally transparent Button surfaces are separate and unchanged. Base's interaction, geometry, focus and callback paths remain authoritative.

| Native Linux evidence | Before | After |
| --- | --- | --- |
| Light neutral-off corner minimum RGB channel | 175 | 231 |
| Light pixel (58,288) | (176,176,176) | (234,234,234) |
| Dark corner channel range | 13–38 | 15–38 |
| Dark pixel (58,288) | (26,26,26) | (35,35,35) |

Measurements use the same 1040×800 gallery placement: four 5px corner regions around the base neutral-off track. [Before light](evidence/switch-light.png), [repaired light](evidence/switch-edge-light.png), [before dark](evidence/switch-dark.png), [repaired dark](evidence/switch-edge-dark.png). Thumb shadows are preserved; palette values were not changed to conceal the defect.

The native pixel regression [check-switch-edge-capture.py](../scripts/check-switch-edge-capture.py) passes on the corrected capture (minimum231) and fails on the original capture (minimum175). Its220 floor is a conservative sentinel for this fixed light neutral recipe and softened thumb shadow, not a universal contrast rule. Requires Pillow in the capture environment. It accepts explicit origin coordinates when gallery order changes; do not run it on dark/theme-scaled captures or silently assume its default coordinates still identify the control.

```sh
python3 scripts/check-switch-edge-capture.py docs/evidence/switch-edge-light.png
# Negative control: expected failure on the pre-repair capture.
python3 scripts/check-switch-edge-capture.py docs/evidence/switch-light.png
```

Badge normal filled borders avoid this exact transparent-background mechanism. Its optional link-hover ring did share it and is repaired. [Light hover](evidence/badge-edge-hover-light.png) and [dark hover](evidence/badge-edge-hover-dark.png) were inspected enlarged: continuous pill edge, correct fill/transparent interior, no dark notches. [Other controls light](evidence/control-edges-light.png), [dark](evidence/control-edges-dark.png) and [Button dark](evidence/button-edges-dark.png) were inspected. This does not resolve KUMO-019 arbitrary-descendant clipping.

Independent review confirmed the shader diagnosis and matching-RGB zero-alpha repair, including translucent rings, and identified the additional Input/Popover/Link/Empty/Badge sites. LayerCard's zero-width transparent border was excluded after checking actual paint arguments.

Final gate: 86 workspace tests and eight doctests pass; formatting, locked all-target/all-feature Clippy with warnings denied and builds pass. Pixel positive and negative controls behave as recorded. No live native test applications remain.
