# Kumo recipes for the first native components

Extracted 2026-10-01 from Cloudflare Kumo commit `3fd5b648df578cb1ba214dedd30f475009f6a668`. This is a source recipe reference for Button, Input, Field and Popover, not a native implementation or a measured screenshot comparison. Use [Kumo tokens](kumo-tokens.md) for light/dark values and shared utility defaults. Source facts below are separate from **Port guidance**.

## Units and evidence

Geometry in the tables evaluates Tailwind spacing at a 16px root and unmodified `--spacing: 0.25rem`; radius utilities depend on the consumer's Tailwind theme. Kumo overrides `text-xs` to 12px, `text-sm` to 13px and `text-base` to 14px. Their line heights are respectively `12 × (1 / 0.75) = 16px`, `13 × (1 / 0.85) ≈ 15.2941px`, and `14 × 1.5 = 21px`. These are source evaluations, not measurements of arbitrary consumers. The pinned lockfile resolves Kumo's direct Tailwind dependency to 4.3.3, while its CLI uses a separate 4.1.17 installation. [Generated Kumo theme](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/styles/theme-kumo.css), [Tailwind default utilities](https://github.com/tailwindlabs/tailwindcss/blob/v4.3.3/packages/tailwindcss/theme.css), [Lockfile](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/pnpm-lock.yaml)

Kumo's `cn` utility re-exports external package `cn` version 0.4.0. Component callers supply `className` after standard recipe classes and can supply inline styles; this document describes default recipes without caller overrides. Exact simultaneous-state cascade and utility merging still require inspecting generated CSS or rendering a browser reference. Class-string order alone does not prove CSS precedence. [Utility](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/utils/cn.ts), [Manifest](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/package.json)

## Button

Default choices are `variant=secondary`, `size=base`, `shape=base`. Shared classes: `group flex w-max shrink-0 items-center font-medium select-none border-0 shadow-xs focus:ring-kumo-focus/50 focus:outline-none focus-visible:ring-2 focus-visible:ring-kumo-brand cursor-pointer disabled:cursor-not-allowed disabled:text-kumo-subtle`. This is content-sized width, medium weight (500), zero border, and an external shadow/ring rather than a layout-consuming border. [Owning Button source](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/button/button.tsx)

| Size | Exact size classes | Height | Horizontal padding | Outer gap | Radius | Text size / line height |
| --- | --- | --- | --- | --- | --- | --- |
| xs | `h-5 gap-1 rounded-sm px-1.5 text-xs` | 20px | 6px | 4px | 4px | 12 / 16px |
| sm | `h-6.5 gap-1 rounded-md px-2 text-xs` | 26px | 8px | 4px | 6px | 12 / 16px |
| base | `h-9 gap-1.5 rounded-lg px-3 text-base` | 36px | 12px | 6px | 8px | 14 / 21px |
| lg | `h-10 gap-2 rounded-lg px-4 text-base` | 40px | 16px | 8px | 8px | 14 / 21px |

Square uses `items-center justify-center p-0`; circle adds `rounded-full`. Both add `size-3.5`, `size-6.5`, `size-9`, or `size-10`: nominal square dimensions 14/26/36/40px, with zero padding. The xs compact override differs from its rectangular 20px height. Primary/destructive put icon and text inside a separate `relative flex items-center gap-1.5` wrapper: their actual content gap is always 6px, even when the outer size recipe says 4 or 8px. Other variants use the outer gap. Icons passed by callers have no universally imposed size; loading uses Loader size 14, except lg size 16. [Button source](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/button/button.tsx)

| Variant | Exact variant classes |
| --- | --- |
| primary | `relative overflow-hidden bg-(--kumo-button-emphasis-bg) !text-white ring ring-(--kumo-button-emphasis-ring) focus:ring-(--kumo-button-emphasis-ring) focus-visible:ring-(--kumo-button-emphasis-ring) active:ring-(--kumo-button-emphasis-ring) disabled:opacity-50` |
| secondary | `bg-kumo-base !text-kumo-default ring not-disabled:hover:bg-kumo-tint disabled:bg-kumo-base/50 disabled:!text-kumo-default/70 ring-kumo-line data-[state=open]:bg-kumo-base` |
| ghost | `text-kumo-default hover:bg-kumo-tint shadow-none bg-inherit` |
| destructive | Same classes as primary; emphasis token changes from brand to danger |
| secondary-destructive | `bg-kumo-base !text-kumo-danger ring not-disabled:hover:!text-kumo-danger not-disabled:hover:ring-kumo-danger/30 disabled:bg-kumo-base/50 disabled:!text-kumo-danger/70 ring-kumo-line data-[state=open]:bg-kumo-base` |
| outline | `bg-transparent text-kumo-default ring ring-kumo-line transition-colors not-disabled:hover:text-kumo-strong not-disabled:hover:ring-kumo-focus/25` |

All six recipes are defined in [Button source](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/button/button.tsx). These `text-kumo-*` classes use text-color tokens, while `bg`, `ring`, and emphasis expressions use general color tokens. The primary emphasis token is `var(--color-kumo-brand)`; destructive uses `var(--color-kumo-danger)`.

For emphasis token `T`, preserve these expressions in OKLCH:

| Derived value | Source expression | Use |
| --- | --- | --- |
| Ring | `color-mix(in oklch, T, black 10%)` | 1px default outer ring; retained ring color during focus/focus-visible/active |
| Base/highlight | `color-mix(in oklch, T, white 30%)` | Parent background, 1px inset top highlight, hover gradient start |
| Rest gradient start | `color-mix(in oklch, T, white 15%)` | Top stop |
| Gradient end | `T` | Bottom stop |

The decorative layer has `absolute inset-0 rounded-[inherit] bg-linear-to-b from-(--kumo-button-emphasis-gradient-start) to-(--kumo-button-emphasis-gradient-end) shadow-[inset_0_1px_0_0_var(--kumo-button-emphasis-bg)] group-hover:from-(--kumo-button-emphasis-bg)`. It is hidden from accessibility. The ring-width base is the 1px `ring` utility; focus-visible adds 2px from the shared recipe. Resolve mixtures and gradient interpolation before converting to GPUI colors; RGB mixing does not reproduce this source expression. [Button source](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/button/button.tsx)

Availability details matter: `disabled={loading || disabled}`, but explicit `disabled` additionally adds `cursor-not-allowed opacity-50` to every variant. Loading alone gets 50% opacity only from the primary/destructive `disabled:opacity-50` recipe; secondary/secondary-destructive instead use their alpha color recipes. Ghost hover and emphasis `group-hover` are not gated by `not-disabled`. Disabled opacity is a group compositing effect, not just alpha on text/fill. Error/focus state combinations and open+hover priority need generated-CSS verification. Icon-only shapes require a name through aria attributes or string/number title; title can supply tooltip and fallback name. [Button source](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/button/button.tsx)

### Native Button contract

[button.rs](../crates/gpui-kumo/src/button.rs) implements a consumed `RenderOnce` component over Base Button. `button::Variant`, `Size` and `Shape` are closed enums with secondary/base/standard defaults. `Button::new(id, label)` uses the label as its accessible name; `Button::icon(id, name, icon)` requires a name and defaults to square. Names must be nonempty. `accessibility_label` can override the name without changing visible text. Leading/trailing icons accept decorative, noninteractive elements; callers own their dimensions, and loading replaces the leading icon. Use the `Icon` SVG wrapper for inherited foreground tint; raw GPUI SVGs require an explicit tint. The component exposes semantic builders, not a general `Styled` override.

The owner supplies loading, disabled and open presentation. `open` retains the secondary surface during hover and does not turn Button into a toggle or an accessible Popover trigger; overlay composition will supply that contract. Availability is `disabled || loading`. Base gates pointer, Enter, Space and accessible click through one callback and excludes unavailable buttons from traversal. Focus is keyed by the caller's stable ID unless `track_focus` supplies an existing handle. Disabling does not force focus elsewhere; unavailable buttons stop accepting activation and the owner may move focus explicitly.

Standard buttons size to their content in a flex wrapper; square/circle use fixed dimensions. Labels stay on one line, with no automatic truncation or wrapping. Compact xs remains 14px, matching source geometry rather than introducing a larger target. Callers should choose a larger size where appropriate. Internal decorative content has no accessible text node duplicating the parent's name.

Native paint follows the source colors, geometry, emphasis mixes and inset highlight, with these explicit policies:

- Unavailable controls have no hover feedback, including ghost and emphasis variants. Loading and explicit disabled retain their distinct source opacity/color recipes. GPUI opacity is applied to rendered primitives; exact CSS group compositing is not established.
- Focus wins over hover ring color. Pointer focus uses a 1px focus/50 ring, keyboard focus a 2px brand ring, and emphasis variants retain their emphasis-ring color. A canvas paints an outside rounded outline without adding layout borders; a spread shadow would incorrectly fill transparent interiors in GPUI.
- Outline keeps a transparent surface and omits `shadow-xs` because GPUI's drop shadow paints inside transparent boxes. Other authored drop shadows and emphasis highlights are retained. Outline color changes are immediate in this initial slice; its source 100ms transition remains follow-up work.
- Pressing adds no invented darkening or scale; Kumo has no separate authored pressed fill. Loading uses a native 270-degree stroked arc at 14px (16px for lg), rotating once per second. GPUI's animation wrapper renders it statically and stops frame requests under reduced motion.
- Base/GPUI cannot expose disabled/busy flags through the current fluent API. The component supplies `Unavailable`/`Loading` descriptions and removes activation while unavailable; descriptions do not replace those missing metadata flags.

[Headless tests](../crates/gpui-kumo/src/button_tests.rs) verify operation counts for pointer, Enter and Space; role/name exposure; disabled/loading rejection, traversal exclusion and re-enabling; cancellation when disabled between key down/up; all size/shape dimensions; and content-sized width inside a wider block container. GPUI test support is a dev dependency with default features disabled. The regenerated lockfile includes optional upstream packages, but the native gallery's normal dependency graph still excludes the styled Component layer and bundled assets. Tokio remains at its original locked version.

Native macOS gallery checks cover both appearances, keyboard activation, loading rejection and size/slot previews. Browser pixel parity, assistive-technology activation and missing state flags, narrow-container behavior and other platforms remain unverified. Use the gallery to compare light/dark variants, sizes, slots, available/loading/disabled states and the controlled action counter.

## Input and Field

Input uses the same four size-class strings and evaluated height/padding/radius/text values as rectangular Button above. Actual rendering supplies no fixed width class; native width should be an explicit component/layout decision. The exported `KUMO_INPUT_STYLING` object is stale: it lists base/lg 16px text, xs radius 2px and fixed widths, whereas the actual classes render 14px text, default xs radius 4px and no specified width. Do not port those metadata values. [Input source](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/input/input.tsx)

| Recipe | Exact classes / condition |
| --- | --- |
| Shared frame | `border-0 bg-kumo-control text-kumo-default ring ring-kumo-line outline-none focus:outline-none kumo-input-placeholder disabled:text-kumo-disabled` |
| Default | `focus:ring-kumo-focus/50 focus:ring-[1.5px]` |
| Error | `!ring-kumo-danger focus:ring-kumo-danger/50 focus:ring-[1.5px]` |
| Optional parent-focus recipe | `focus-within:ring-[1.5px] focus-within:ring-kumo-focus/50`, or danger/50 for error |

The exported Input calls `inputVariants({size, variant, focusIndicator:true})`; the parent-focus recipe is available to compositions but not enabled by this Input. Variant selection is `variantProp ?? (error ? "error" : "default")`: explicit `default` can override a truthy error. In the error recipe, `!ring-kumo-danger` is important while `focus:ring-kumo-danger/50` is not; therefore the intended half-alpha focus color cannot be assumed to win. The focus ring width still changes to 1.5px. [Input source](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/input/input.tsx)

Placeholder color is `var(--text-color-kumo-placeholder)` in a dedicated vanilla `::placeholder` rule. `text-kumo-disabled` refers to a token not defined in the inspected Kumo theme; do not invent a resolved disabled text value or infer 50% opacity from stale metadata. There is no explicit input opacity or disabled background class in the actual recipe. Selection, caret, text baseline, browser default width and some native input-type affordances remain browser/consumer dependent. [Binding CSS](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/styles/kumo-binding.css), [Theme](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/styles/theme-kumo.css)

Label, error, or description creates a Field wrapper. It uses `grid gap-2` (8px vertical gaps). Label is `m-0 text-base font-medium text-kumo-default select-none`: 14px / 21px, weight 500. Label content uses `inline-flex items-center gap-1` (4px); explicit `required=false` adds `(optional)` in `font-normal text-kumo-subtle` (weight 400). Optional label tooltip is a ghost xs square Button containing a `size-4` (16px) Info icon, larger than that button's nominal 14px square. [Field source](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/field/field.tsx), [Label source](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/label/label.tsx)

Description uses `text-sm leading-snug text-kumo-subtle col-span-full`; error uses the same recipe with `text-kumo-danger`: 13px text, 17.875px line height. Providing error removes the description branch, even when a validity-matching error may not currently be displayed. A string error normalizes to `{message: error, match: true}`; structured errors use browser ValidityState matches. **Port guidance:** choose explicit native validation/display rules instead of importing browser validity semantics. [Field source](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/field/field.tsx)

## Popover

Popup base is `flex flex-col rounded-lg bg-kumo-base px-4 py-3 text-sm text-kumo-default shadow-md outline outline-kumo-line kumo-popover-popup`: 8px radius, 16px horizontal and 12px vertical padding, 13px body text with approximately 15.2941px line height, 1px outline and default Tailwind medium shadow. No default panel width, inter-child gap or backdrop fill is specified. Dark mode moves the outline inside with `outline-offset:-1px`; light uses the default offset. [Popover source](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/popover/popover.tsx), [Binding CSS](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/styles/kumo-binding.css)

Modern `Popover.Popup` adds `origin-(--transform-origin) transition-[transform,scale,opacity] duration-150 data-starting-style:scale-90 data-starting-style:opacity-0 data-ending-style:scale-90 data-ending-style:opacity-0 data-instant:duration-0`: 150ms transition between scale .9/opacity 0 and rest values, with origin supplied by Base UI positioning. Deprecated `Popover.Content` instead supplies opacity-only transitions, adds Arrow automatically, and defaults align center/alignOffset 0/positionMethod absolute. Do not confuse these two recipes. The Positioner wrapper defaults bottom and sideOffset 8px; other positioning defaults/collision behavior are inherited from Base UI and require an explicit native contract. No reduced-motion class is present in these recipes. [Popover source](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/popover/popover.tsx)

Title is `m-0 text-base leading-6 font-medium`: 14px / 24px, weight 500. Description is `m-0 text-base leading-6 text-kumo-subtle`: 14px / 24px. Trigger and Close add no default visual recipe; composition with Button supplies it. Backdrop and Viewport add no styling. Arrow uses a 20×10px SVG with fill `--color-kumo-base`, outer edge `--color-kumo-arrow-edge`, inner stroke `--color-kumo-arrow-stroke`. Top/bottom positions offset -8px with top rotated 180°; left/right use -13px side offsets and ±90° rotation. Use the SVG's separate light/dark edge paths rather than guessing a generic triangle stroke. [Popover source](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/popover/popover.tsx)

## Minimum token dependencies and native checks

General color tokens needed: `--color-kumo-base`, `--color-kumo-control`, `--color-kumo-tint`, `--color-kumo-line`, `--color-kumo-focus`, `--color-kumo-brand`, `--color-kumo-danger`, `--color-kumo-arrow-edge`, `--color-kumo-arrow-stroke`. Text tokens needed: `--text-color-kumo-default`, `--text-color-kumo-strong`, `--text-color-kumo-subtle`, `--text-color-kumo-danger`, `--text-color-kumo-placeholder`; unresolved reference `--text-color-kumo-disabled` requires a documented decision. White/black and transparency are also used by recipes. Typography, medium/normal weight, spacing, radius, ring/outline widths, `shadow-xs`, `shadow-md`, emphasis inset highlight and transition values are separate dimensions. [Button](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/button/button.tsx), [Input](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/input/input.tsx), [Popover](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/popover/popover.tsx)

**Port guidance:** when implementing, compare combined states (disabled+hover, loading, error+focus, open+hover, keyboard focus-visible) against generated web CSS; decide which upstream quirks to preserve or correct. Native paint must distinguish opacity composition, out-of-layout rings/outlines and regular borders. Verify font metrics, gradient interpolation/gamut conversion, shadows, arrow seams, animation origin and focus restoration in the gallery. This extraction does not establish runtime visual fidelity or native accessibility.
