# Kumo tokens for the first native slice

Extracted 2026-10-01 for Button, Input, Popover and their gallery context. The typed native theme uses this reference data; implementation policies and validation are recorded below. Component-specific geometry and state recipes are in [Component recipes](kumo-component-recipes.md).

## Baseline and resolution

Kumo source is pinned to [`3fd5b648df578cb1ba214dedd30f475009f6a668`](https://github.com/cloudflare/kumo/tree/3fd5b648df578cb1ba214dedd30f475009f6a668), whose package manifest declares 2.14.0. These values describe the default `kumo` theme, with a 16px root rem and no consumer palette overrides. FedRAMP is outside this extraction.

Resolve semantic variables from [theme-kumo.css](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/styles/theme-kumo.css), taking referenced primitive variables from [kumo-binding.css](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/styles/kumo-binding.css) and Tailwind's theme before considering each CSS fallback. The [lockfile](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/pnpm-lock.yaml) resolves the Kumo package's `tailwindcss` and Vite integration to **4.3.3**; its CLI remains 4.1.17. Tables below use the 4.3.3 direct/Vite baseline, checked from the official [Tailwind package's theme.css](https://unpkg.com/tailwindcss@4.3.3/theme.css). The standalone CLI build is a separate baseline; it has not been rebuilt here.

Colors are recorded in their source color space, with variables and the dark brand mix resolved. `oklch(L C H / A)` uses L in 0–1, C as chroma, H in degrees, and optional alpha A (default 1). For achromatic colors, hue is immaterial and shown as 0. These are not sRGB hex approximations. The native implementation mixes emphasis colors in OKLCH before converting and gamut mapping, as described below.

## Text colors

Every name below has the full `--text-color-kumo-` prefix. Values come from generated theme declarations and referenced Tailwind primitives; a foreground role is distinct from a same-named general color role.

| Suffix | Light | Dark | Needed for |
| --- | --- | --- | --- |
| default | `oklch(.205 0 0)` | `oklch(.97 0 0)` | Body, Button, Input, Popover |
| strong | `oklch(.145 0 0)` | `oklch(.985 0 0)` | Strong text and outline Button hover |
| subtle | `oklch(.556 0 0)` | `oklch(.708 0 0)` | Descriptions and subdued content |
| inactive | `oklch(.87 0 0)` | `oklch(.439 0 0)` | Available semantic role; Input's disabled recipe does not reference it |
| placeholder | `oklch(.708 0 0)` | `oklch(.556 0 0)` | Input placeholder |
| inverse | `oklch(.97 0 0)` | `oklch(.205 0 0)` | Inverted surfaces; emphasis Button uses literal white instead |
| brand | `#f6821f` | `#f6821f` | Orange foreground identity; separate from blue action fill |
| danger | `oklch(.505 .213 27.518)` | `oklch(.704 .191 22.216)` | Error messages and secondary-destructive Button |

`--text-color-kumo-disabled` is absent from the inspected generated theme and binding definitions, although Input references it. This is an upstream recipe gap. The native policy below explicitly selects `subtle`; it does not invent an upstream token.

## General colors

Every name below has the full `--color-kumo-` prefix. This is the small reusable surface/action palette around the three components, including context surfaces that do not yet have a component recipe.

| Suffix | Light | Dark |
| --- | --- | --- |
| canvas | `oklch(.9875 0 0)` | `oklch(.10 0 0)` |
| base | `#ffffff` | `oklch(.17 0 0)` |
| elevated | `oklch(.98 0 0)` | `oklch(.12 0 0)` |
| recessed | `oklch(.965 0 0)` | `oklch(.15 0 0)` |
| tint | `oklch(.97 0 0)` | `oklch(.269 0 0)` |
| contrast | `oklch(.12 0 0)` | `oklch(.985 0 0)` |
| overlay | `oklch(.9875 0 0)` | `oklch(.269 0 0)` |
| control | `#ffffff` | `oklch(.205 0 0)` |
| interact | `oklch(.87 0 0)` | `oklch(.371 0 0)` |
| fill | `oklch(.922 0 0)` | `oklch(.269 0 0)` |
| fill-hover | `oklch(.965 0 0)` | `oklch(.269 0 0)` |
| brand | `oklch(.5772 .2324 260)` | `oklch(.51948 .20916 260)` |
| brand-hover | `oklch(.488 .243 264.376)` | `oklch(.488 .243 264.376)` |
| danger | `oklch(.637 .237 25.331)` | `oklch(.577 .245 27.325)` |
| line | `oklch(.145 0 0 / .10)` | `oklch(.32 0 0)` |
| hairline | `oklch(.935 0 0)` | `oklch(.269 0 0)` |
| focus | `oklch(.15 0 0)` | `oklch(.935 0 0)` |
| shadow-edge | `oklch(0 0 0 / .12)` | `oklch(1 0 0 / .10)` |
| shadow-drop | `oklch(0 0 0 / .08)` | `oklch(0 0 0 / .30)` |
| arrow-edge | `oklch(.145 0 0 / .10)` | `transparent` |
| arrow-stroke | `transparent` | `oklch(.32 0 0)` |

`brand-hover` is an available semantic token, but emphasis Button hover uses a white mix of `brand`, rather than this token. Likewise Popover's `shadow-md` uses Tailwind's literal black shadows, not `shadow-edge`/`shadow-drop`. Keep the roles separate from their actual recipe usage.

### Fallback differences that affect fidelity

The preceding tables resolve variables rather than copying fallback strings. In particular:

- `neutral-900` resolves to achromatic L=.205, replacing the generated default-text/control fallback L=.21 with C=.006.
- Binding's `kumo-neutral-125` resolves to .965, replacing recessed's .96 fallback.
- Binding's `kumo-neutral-975` resolves to .12, replacing light contrast's .085 fallback.
- Binding's `kumo-neutral-50` resolves to .9875, replacing light overlay's .975 fallback.
- Tailwind's `neutral-800` resolves to .269, replacing dark fill-hover's .371 fallback.
- `kumo-neutral-25` and `kumo-neutral-150` have no primitive definition in the inspected binding/Tailwind theme; their use falls back to the values specified at each semantic declaration. The 25 fallback differs between canvas and contrast.

These differences are observable consequences of the pinned files, not proposed corrections to upstream.

## Typography

Kumo's generated sizes override Tailwind defaults. Line heights below are evaluated from the generated expressions, not inferred from metadata.

| Utility | Font size | Default line-height multiplier | Line height |
| --- | --- | --- | --- |
| text-xs | 12px | 1/.75 | 16px |
| text-sm | 13px | 1/.85 | approximately 15.294118px |
| text-base | 14px | 1.5 | 21px |
| text-lg | 16px | 1.5 | 24px |

Normal body weight is 400; Button and Field label use 500. `leading-snug` is 1.375: Field descriptions/errors at 13px therefore have a 17.875px line height. Popover Title and Description use 14px with explicit `leading-6` = 24px, overriding the text-base line height. These references are in the [component recipes](kumo-component-recipes.md).

The library inherits font choice from its consumer. Tailwind 4.3.3's sans stack starts `-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', 'Noto Sans', Arial`, then generic sans and emoji fallbacks. The [Kumo documentation site](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo-docs-astro/src/styles/global.css) overrides this with Inter followed by system fallbacks, sets letter spacing to -.01em and enables `cv02`, `cv03`, `cv04`, `calt`. Those are site choices, not library tokens. Decide whether the native gallery targets the documentation's Inter appearance or the library's consumer-dependent font before screenshot comparison; the extraction selected no font. The initial native policy below uses the macOS system font.

## Spacing, edges and effects

These values combine Kumo recipes with the locked Tailwind theme. At the assumed root rem, `--spacing=.25rem` is 4px.

| Role | Resolved value |
| --- | --- |
| Spacing units used by the slice | 1=4px; 1.5=6px; 2=8px; 3=12px; 4=16px |
| rounded-sm / md / lg | 4px / 6px / 8px |
| rounded-full | Circular/capsule geometry; derive from bounds |
| Normal Button/Input ring | 1px outside the box; layout border is 0 |
| Input focus ring | 1.5px; focus role at 50% alpha (error recipes have their own priority) |
| Button focus-visible ring | 2px; variant overrides choose the color |
| Popover outline | 1px; default offset 0 in light mode, -1px in dark mode |
| shadow-xs | `(x=0, y=1, blur=2, spread=0, black alpha=.05)` |
| shadow-md, first layer | `(0, 4, 6, -1, black alpha=.10)` |
| shadow-md, second layer | `(0, 2, 4, -2, black alpha=.10)` |
| Emphasis Button inset highlight | `(0, 1, 0, 0, emphasis-bg)`, inset |
| Default transition | 100ms, cubic-bezier(.4, 0, .2, 1) |
| Popover explicit transition | 150ms; instant mode 0ms |

Opacity modifiers multiply the paint's alpha; explicit whole-control opacity affects its composited contents. Preserve that distinction. CSS rings and outlines do not consume layout space. Preserve each shadow layer and the inset highlight rather than treating them as one generic shadow. [Tailwind utilities implementation](https://github.com/tailwindlabs/tailwindcss/blob/v4.3.3/packages/tailwindcss/src/utilities.ts), [Kumo binding](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/styles/kumo-binding.css)

## Derived emphasis colors

[Button source](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/button/button.tsx) uses an opaque action token T. Ring mixes T with black 10%; background/inset highlight mixes T with white 30%; normal gradient start mixes T with white 15%; gradient end is T. Hover changes the gradient start to the background/highlight color. With achromatic black/white contributing no hue, the resulting hue is T's hue.

Each tuple below is `(L, C, H)` in OKLCH, alpha 1. The table is arithmetic derived from the source expressions, before sRGB conversion or gamut mapping.

| Variant / mode | Ring | Background / highlight / hover start | Normal gradient start | Gradient end |
| --- | --- | --- | --- | --- |
| Primary light | (.51948,.20916,260) | (.70404,.16268,260) | (.64062,.19754,260) | (.5772,.2324,260) |
| Primary dark | (.467532,.188244,260) | (.663636,.146412,260) | (.591558,.177786,260) | (.51948,.20916,260) |
| Destructive light | (.5733,.2133,25.331) | (.7459,.1659,25.331) | (.69145,.20145,25.331) | (.637,.237,25.331) |
| Destructive dark | (.5193,.2205,27.325) | (.7039,.1715,27.325) | (.64045,.20825,27.325) | (.577,.245,27.325) |

The gradient runs top to bottom, while foreground is fixed white in both modes. Native gradient interpolation should preserve the source's OKLab gradient interpolation separately from the OKLCH token mixes. [Tailwind gradient defaults](https://tailwindcss.com/docs/background-image)

## Extraction scope

The shared values above plus [component recipes](kumo-component-recipes.md) cover the first three components' authored visual requirements. At extraction time, remaining decisions were font/reference profile, undefined disabled Input foreground, conversion/gamut mapping, native text-selection/caret styling (not authored by these Kumo recipes), and translation of CSS rings, inset paint and layered shadows. Source reading and arithmetic resolution were performed; no browser CSS computation, native theme implementation or visual comparison was performed in this step.

## Initial native implementation

[theme.rs](../crates/gpui-kumo/src/theme.rs) owns the Kumo vocabulary and application-wide `Theme` global. `init` installs the light theme after initializing Base. Consumers read `theme(cx)` during rendering and retain an `observe_global::<Theme>` subscription to redraw. `set_theme` publishes a complete snapshot and its Base projection together; `set_appearance` rebuilds the standard palette while retaining the font family. Custom palettes should switch through complete snapshots. Independent previews may pass snapshots directly; Base behaviors still use the application-wide projection.

The initial gallery targets the library's consumer-dependent font profile, using `.SystemUIFont` on macOS. The theme supports a caller-selected family through `with_font_family`; Segoe UI and generic sans are unverified fallback choices on Windows and other platforms. Inter, the documentation site's tracking and font-feature settings are not applied.

Native policies fill two source gaps in a separate `NativeColors` group: disabled Input foreground uses the subtle text role; selection uses the action brand at 25% alpha. These are project choices, not extracted Kumo tokens. Caret styling remains part of Input implementation.

[color.rs](../crates/gpui-kumo/src/color.rs) converts OKLCH through the [Oklab author's inverse linear-sRGB matrix](https://bottosson.github.io/posts/oklab/#converting-from-linear-srgb-to-oklab) and the sRGB transfer function. Out-of-gamut colors reduce chroma at fixed lightness and hue using binary search. This intentionally differs from [CSS local-MINDE gamut mapping](https://www.w3.org/TR/css-color-4/#binsearch); browser color parity remains unverified. Token mixes happen before conversion. Emphasis gradients use GPUI's OKLab interpolation between mapped sRGB stops; mapping stops before interpolation may differ from a browser mapping individual colors along the original gradient.

Spacing, radii, typography, ring/outline dimensions and shadow geometry use logical pixels at the extracted 16px-root baseline. The theme preserves both Popover shadow layers and exposes the emphasis inset highlight as an inset `BoxShadow`. Focus colors remain semantic colors; components must apply the recipe's alpha, ring geometry and state priority. Motion values are stored as durations and cubic-bezier control points; animation, rings and Popover outlines are not yet implemented components.

The adapter projects canvas/default into Base background/foreground, base/default into surface and secondary, brand/white into primary, recessed/subtle into muted, tint/default into accent, danger/white into destructive, line into border and input (an adapter choice treating input as a border role), focus into ring, and native selection into selection. It projects the matching spacing, radius, four typography levels and two shadow slots. Base's unrelated behavior settings and uncovered scale slots remain intact; Kumo components consume Kumo tokens directly rather than relying on those Base defaults.

Validation on 2026-10-01: conversion tests cover a neutral transfer value, a reference red and out-of-gamut brand lightness preservation; an adapter test covers both appearances and retained unrelated settings. Build, formatting and Clippy pass. Native macOS rendering of both palettes, typography, gradients, inset highlights and layered shadows was inspected; Command-L and the appearance controls redraw the existing gallery. This validates the foundation, not browser visual parity, component behavior, full accessibility or other platforms.
