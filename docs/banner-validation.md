# Banner acceptance contract

Selected after Link checkpoint `62f5ab4`. The [pinned root](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/banner/banner.tsx), [action recipe](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/banner/banner-action.tsx) and [official examples](https://kumo-ui.com/components/banner/) control this milestone. Supported structured title/description/actions are included. Deprecated children/text and legacy enum are excluded.

| Branch | Contract | Verification |
| --- | --- | --- |
| Variants | Info/default, alert, error and secondary; tint backgrounds and text roles; icon uses distinct general fill token | Actual light/dark paint and inherited content styles |
| Sizes | Base: padding16×12, gap12, radius8, text14/21, items-start; compact: padding12×8, gap8, radius6, text13/15.29, items-center | Rendered dimensions, narrow/empty/long Unicode and caller refinements |
| Structured parts | Title medium/snug; description13px/snug; base stack gap2; compact baseline wrap gap6; no-title content row has top1px | Layout and text style probes; multiple cards and repeated part IDs |
| Icon | Caller-owned decorative icon; fixed wrapper height1.375em base/1.25em compact; semantic fill distinct from text foreground | Measured wrapper and inherited icon foreground |
| Actions | Trailing no-shrink row gap8; a sole typed Link becomes inline content in compact mode | Narrow composition, wrapping and actual Link activation |
| Accent action | Primary gradient/emphasis, secondary transparent outline, ghost transparent; accent hover; sizeSm/base or Xs/compact | All variants/treatments, loading/disabled and icon-only sizing; source OKLCH mixing before gamut mapping |
| Interaction/state | Root has no focus/activation; Button and Link keep Base behavior; caller owns dismiss/value state | Pointer/Enter/Space once, traversal, retained theme/state and nested interaction |
| Accessibility | Static message content and complete action names; no invented live-region role | Actual control metadata and native tree; spoken semantics separately pending |
| Styling/composition | Root Styled after recipe; typed actions resolve context before erasure; custom action/description keep ownership | Caller sizing/padding/color and rich content examples |

Native inline links remain real focused elements in a wrapping flex composition rather than browser inline span layout. GPUI has no inline native interactive text run; exact baseline/wrapping differences must be measured and documented. The Button private accent recipe must reuse existing focus/activation/loading machinery, without creating another engine. Context translates to typed action resolution at the Banner boundary, not global state.

## Implementation and review

Banner implements four supported variants, base/compact sizes, structured text/rich description, decorative icon and typed/custom actions. Empty strings count as absent text. React context becomes explicit typed action resolution at the Banner boundary; only a sole typed Link is promoted to compact content flow. A native wrapping flex item keeps the Link interactive, but cannot reproduce browser text-run placement on the last description line. Banner adds no inherent focus target or live-region role.

Banner.Action resolves to the existing Button with a private accent recipe. It reuses Base activation, retained focus, Loader, disabled/busy metadata and existing Button opacity/focus policies. These native policies retain the first slice's documented differences. Caller Styled refinements apply to the Button surface; the native intrinsic-width wrapper still owns placement. Decoration and custom content retain caller-owned semantics.

Theme adds the missing general info accent and information/warning/neutral emphasis gradients. Defined Tailwind 4.3.3 blue-500 (L0.623/C0.214/h259.815) wins over Kumo's sky-colored fallback; warning follows appearance, neutral uses neutral-700, and error reuses destructive emphasis. Mixing occurs in OKLCH before native gamut mapping. Text roles and decorative icon fills remain distinct.

Separate skeptical review fixed a high-severity layout defect: a zero percentage flex basis collapsed the content column inside the scrolling gallery, causing character-per-line text. A gallery-shaped rendered regression failed before correction and passes with automatic flex basis, including resize down/up. Additional review preserved Loader stroke inheritance instead of applying decorative icon fill tint, and made Button's ring follow caller borders/radii with explicit canvas insets and clamping. A bordered pill-shaped action is covered by actual ring paint.

Five actual-render tests cover palette/geometry/typography/parts in both themes, all action treatments and accent gradients/outline/hover fills, loading/disabled rejection, pointer/Enter/Space exactly once, compact inline Link activation, repeated IDs, caller styles, rich description, empty content and realistic scrolling/resize composition. Source-sized dimensions allow physical-pixel snapping. Formatting, full workspace tests, warning-denied all-target/all-feature locked Clippy and gallery/example builds are the final checkpoint gate; measured totals appear in port progress. The upstream block 0.1.6 future-compatibility notice is separate.

Native rebuilt light output shows the repaired column and all source treatments, compact Link and caller-owned dismissal affordance. Accessible paragraphs and named Button/Link actions are exposed; pointer/Return/Space produced three activations in the native tree. Subsequent pixels/theme/availability snapshots were inconsistent with exposed state, so native dark, final disabled/loading/dismissal and spoken semantics remain pending under [GitHub #51](https://github.com/msmps/gpui-kumo/issues/51). Exact pinned browser comparisons remain pending. The registered app was quit; process absence is checked before committing.

Status: implemented with automated coverage and corrected native default layout; comparison/platform checks above remain open.
