# Badge acceptance contract

Selected after LayerCard checkpoint `ea86f71`. Badge supplies common status/category presentation for Banner/Empty compositions, using Theme and GPUI layout without a behavior engine. Inspect the [pinned Badge source](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/badge/badge.tsx), [semantic theme](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/styles/theme-kumo.css), [primitive bindings](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/styles/kumo-binding.css) and [examples](https://kumo-ui.com/components/badge/). Resolve referenced Tailwind primitives against the existing 4.3.3 baseline before CSS fallbacks.

## Required branches

| Branch | Source contract | Verification |
| --- | --- | --- |
| Supported variants | Primary, Secondary, Error, Warning, Success, Info, Beta, Outline, Red, Green, Neutral, Orange, Purple, Teal, TealSubtle, Blue | Gallery matrix and actual paint/text tokens in both themes; destructive and legacy type alias excluded |
| Base layout | Content width; no flex shrinking; horizontal centered items; gap4; pill radius; padding8×2; 12/16px medium; no wrapping | Real column/narrow-row layout, long Unicode/empty labels, measured height and overrides |
| Filled icon | Decorative content in 12×16px centered wrapper; leading padding6 | Measured icon/label positions, complete accessible label, no duplicate icon announcement |
| Dot | Transparent/default text; outside hairline ring; gap6; 7px decorative dot only for Success/Warning/Error/Neutral | Real paint/geometry; remaining variants retain dot appearance without a dot; icons unavailable on typed dot API |
| Beta/Outline | Beta has 1px dashed brand border, transparent fill and link text; Outline has 1px fill border/base background/default text | Painted border style and measured layout, native inspection |
| Link composition | Ancestor link hover adds current-color ring | Explicit named GPUI hover group adaptation; pointer and keyboard belong to ancestor; actual input hover test |
| Content/identity | Stable scoped names, arbitrary decorative rich label composition | Full accessible name, repeated IDs, inherited consumer font, empty label, theme updates |
| Interaction | Read-only presentation; no inherent focus/activation/value state | Hover only through link composition; other interaction states belong to composed ancestor |
| Consumer styling | Recipe first, caller refinements last | Padding/font/background overrides; outside ring follows effective caller text color |

## Source findings and adaptations

The source map controls variants: prose mentions neutral-subtle/inverted as variant keys, but they do not exist. Primary uses the inverted badge token; Secondary uses its neutral-subtle foreground.

TealSubtle references bg-kumo-badge-teal-subtle, but neither the pinned generated theme nor primitive bindings define that background token. Preserve a transparent native background and the defined subtle teal foreground until a pinned browser build establishes the effective utility result. This is an upstream recipe gap, not permission to invent a tint.

Filled and dot are separate Rust construction types so dot badges cannot accept an icon. Dot mode can use any supported variant, matching source behavior; only four supply a dot. Decorative rich content keeps the caller's complete label as the accessible text. Link hover requires an explicit named ancestor group, because GPUI has no DOM a:hover ancestor selector. GPUI Length has no fit-content variant; the native badge uses self_start to preserve content width in stretched columns. Row consumers can apply self_center, as demonstrated in the gallery matrix, to inherit a centered row treatment. This adaptation needs browser comparison when mixed-height baseline alignment matters.

## Implementation, review and measured results

Badge<Filled> and Badge<Dot> expose supported variants, complete labels, optional decorative rich content, caller Styled refinements and explicit named ancestor-hover composition. Only the filled type has icon(). The compile-fail doctest verifies that constraint. The owner observes Theme; Badge creates no independent entity, focus target or activation/value state.

Theme now separates badge solid/inverted/subtle roles, semantic status foregrounds and translucent tints. Badge success uses the success foreground, while Text’s authored success recipe continues using link. Icon content lives in a 12×16px wrapper with a centered 12px child slot; callers size supplied elements to that slot, as the gallery SVG does. Escape-hatch styling applies after the recipe. The transparent outside ring remains component-owned and is distinct from Styled shadow lists.

Skeptical review and native paint exposed two medium defects: a zero-blur shadow filled dot interiors, and custom quad painting left pill radii unclamped. Rings now use transparent border-only quads with clamped radii, preserving outside geometry without consuming layout space. The new rendered regression fails with the former shadow implementation and passes after correction. Root-border offsets are included for hovered Outline/Beta geometry. Hover ring color resolves after caller text-color refinements. LayerCard’s custom background path also gained the required radius clamp, with its existing caller-radius test expanded to oversized radii in both themes.

Three rendered tests pass: all 16 filled variants’ dimensions/typography/identity with light/dark recipes and inherited fonts; dot/icon spacing, unsupported-dot fallback, empty/long Unicode names and caller padding/background; transparent border-only dot paint plus actual ancestor-hover entry/exit, outside bounds and caller current color. The gallery exercises the full filled map, four status dots, the no-dot fallback, icon content and hover group.

Native macOS light/dark screenshots on 2026-10-02 show the filled palette, dashed Beta, outlined badges, transparent dot interiors and correctly bounded pill outlines. Pointer input on the hover example visibly added the current-color ring in dark. The native tree exposed every Badge label as text and did not expose separate decorative dots/icons. Appearance input updated on a later observation, preserving the existing native input/frame consistency caveat. Native-menu Quit completed and pgrep confirmed no gallery process remained.

Gate: 52 workspace tests and four doctests (including compile-fail) pass; formatting, all-target/all-feature locked Clippy with warnings denied and gallery/example builds pass. The upstream block 0.1.6 future-compatibility notice remains separate.

Status: implemented with automated and native light/dark evidence; pinned browser pixel comparison, exact mixed-height baseline alignment and effective TealSubtle background utility remain pending. Link navigation/keyboard integration belongs to the subsequent Link contract; this milestone verifies the explicit ancestor-hover mechanism, not a native navigation component.
