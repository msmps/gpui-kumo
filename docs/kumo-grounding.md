# Kumo reference for GPUI

## Baseline and scope

Reviewed 2026-10-01. This records upstream facts and separate **GPUI port guidance**; it does not select this project's dependency, component API, palette, or implementation scope. Use the [canonical glossary](../CONTEXT.md) for local terminology.

[Kumo](https://kumo-ui.com/) is Cloudflare's React library for product interfaces. Its [llms.txt](https://kumo-ui.com/llms.txt) is a curated index linking Markdown documentation, not an embedded manual. The deployed [installation page](https://kumo-ui.com/installation.md) reports package version **2.14.0**. The source inspected here is upstream `main` at **`3fd5b648df578cb1ba214dedd30f475009f6a668`** ([snapshot](https://github.com/cloudflare/kumo/tree/3fd5b648df578cb1ba214dedd30f475009f6a668)). These are distinct baselines; no equivalence between that commit and the published package was established.

## Architecture and catalog

Kumo combines styled components with unstyled Base UI behavior. It exposes package imports, granular imports, and Base UI primitive re-exports. Styles are distributed for Tailwind v4 or as compiled standalone CSS. Floating components use DOM portals; application-root isolation addresses their stacking context. These mechanisms are web-specific. [Installation](https://kumo-ui.com/installation.md)

Upstream **components** are versioned, reusable library units; **blocks** are compositions copied into the consuming project through its CLI and owned there. PageHeader, ResourceList, and DeleteResource are block references. In this repository, reserve **primitive** for GPUI building blocks even where Kumo describes a library component as a primitive. [Components versus blocks](https://kumo-ui.com/components-vs-blocks.md)

Useful catalog groups for research are:

| Area | Representative references |
| --- | --- |
| Actions and navigation | Button, Link, ButtonGroup, Tabs, Breadcrumbs, Sidebar, Toolbar |
| Forms and selection | Input, InputArea, InputGroup, Checkbox, Radio, Switch, Select, Combobox, Autocomplete, DatePicker, TagInput |
| Surfaces and overlays | LayerCard, LayerDialog, Dialog, Popover, Dropdown, Tooltip, CommandPalette |
| Content and feedback | Text, Badge, Banner, Table, Empty, Loader, Meter, SkeletonLine, Toast |
| Layout and composed patterns | Flow, Grid, blocks, chart examples |

This is a research grouping, not an upstream taxonomy or promised port coverage. Discover individual contracts through the [documentation index](https://kumo-ui.com/llms.txt) and [registry](https://kumo-ui.com/registry.md).

## Styling, tokens, and themes

Kumo styles use semantic roles rather than raw utility palette names. `data-mode` selects light/dark; CSS `light-dark()` resolves token pairs. `data-theme` selects `kumo` or `fedramp`; theme overrides preserve names and inherit unspecified values. A centralized config generates theme CSS. Surface roles include canvas, base, elevated, recessed, tint, and contrast; text roles include default, strong, subtle, inactive, placeholder, inverse, and link. Status roles have indicator and tinted-background counterparts. Hairline and line distinguish surface edges. [Colors](https://kumo-ui.com/colors.md)

Token identity includes its category: `--text-color-kumo-brand` is orange `#f6821f`, while `--color-kumo-brand` is blue. Values can contain OKLCH, alpha, variable fallbacks, and color mixing. Consult full token definitions instead of treating identical suffixes as interchangeable. [Token reference](https://kumo-ui.com/colors/#token-reference)

Generated theme CSS confirms the typography scale at this snapshot: xs 12px, sm 13px, base 14px, and lg 16px. Component-specific line-height classes can preserve inherited line height. Capture the resolved font size and line height separately when comparing with GPUI. [Generated theme](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/styles/theme-kumo.css)

**GPUI port guidance:** represent semantic roles with Rust theme values, retain separate color categories, and resolve mode/theme explicitly. Translate evaluated color, size, font, spacing, border, shadow, and state values into GPUI styles. Matching Tailwind-like method names does not establish matching visuals. See [GPUI styling](gpui-styling.md).

## Representative component contracts

**Button:** variants are primary, secondary, ghost, destructive, secondary-destructive, and outline; default secondary. Sizes are xs/sm/base/lg; default base. Shapes are base/square/circle. Leading icons and loading are supported; loading disables activation. Icon-only controls need an accessible name. `title` provides a tooltip, including for unavailable controls. LinkButton separates navigation from in-place activation. [Button docs](https://kumo-ui.com/components/button.md), [pinned implementation](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/button/button.tsx)

At the pinned revision, primary/destructive emphasis uses a vertical gradient, OKLCH color mixtures, a ring, and an inset highlight. It is richer than a flat brand/danger fill. Rectangular size classes specify heights of 20/26/36/40px at a 16px rem; square/circle xs uses a separate compact size. **GPUI port guidance:** extract each recipe's resolved geometry and effects, including shape-specific overrides, rather than copying its size name. [Button recipes](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/button/button.tsx)

**Input:** a label enables a composed Field wrapper with description and error support; bare controls require an accessible name. Sizes are xs/sm/base/lg, default base. Errors accept a message or a validity-matching object. Disabled, optional-label indication, and controlled value examples are documented. Browser validity behavior requires a native contract rather than a mechanical prop copy. [Input docs](https://kumo-ui.com/components/input.md)

**Text:** current body sizes are 12/13/14/16 px for xs/sm/base/lg. Heading is semibold at 16 px, or 20 px with lg. Monospace is 13 px, or 14 px with lg. Visual variant and semantic element are independent. Heading defaults to span; deprecated heading1/2/3 require `as`. Body supports bold; mono/heading do not. Truncation combines ellipsis with allowance to shrink. [Text docs](https://kumo-ui.com/components/text.md), [pinned implementation](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/text/text.tsx), [rendered scale](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/styles/theme-kumo.css)

**LayerCard:** Primary and Secondary compose layered content; direct children also support a simple surface. This is useful visual-depth reference alongside the surface tokens. [LayerCard docs](https://kumo-ui.com/components/layer-card.md)

## Behavior, fidelity, and unresolved details

Kumo relies on Base UI for role semantics, keyboard navigation, pointer handling, and focus management. Application authors still supply meaningful names, visible focus, and appropriate contrast. This is an upstream web accessibility contract, not evidence that a native recreation inherits accessibility support. [Accessibility](https://kumo-ui.com/accessibility.md)

**GPUI port guidance:** define activation, availability, value ownership, focus movement/restoration, overlay dismissal, and accessible meaning per component. Validate native text editing, selection, IME, and platform assistive-technology behavior independently. Use [GPUI interaction](gpui-interaction.md) and [verification](gpui-verification.md).

Upstream documentation and metadata drift: Text prose still describes older heading restrictions and a `base` variant, while current API/source use `body` and the new heading contract. Pinned `KUMO_TEXT_STYLING.fontSizes` lists 14/16/18 for sm/base/lg despite current rendered-size documentation listing 13/14/16; `success` currently selects `text-kumo-link`. Check actual component classes and generated theme CSS before porting; explicitly decide whether to preserve observed behavior or intended semantics. [Pinned Text source](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/text/text.tsx)

No native components, exhaustive token extraction, screenshot comparisons, or runtime accessibility tests were performed during this grounding pass.
