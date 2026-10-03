# LayerCard acceptance contract

Selected after Loader checkpoint `20f01e4`. Inspect the [pinned source](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/layer-card/layer-card.tsx) before implementing; [live examples](https://kumo-ui.com/components/layer-card) include simple surfaces, layered headers, interactive header accessories, multiple cards and Input/filter compositions. Examples inform consumer usage; the pinned source controls the recipe.

Surface is explicitly deprecated in the [pinned compatibility wrapper](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/surface/surface.tsx). It is excluded. LayerCard provides the supported simple surface and layered treatments.

## Required branches

| Branch | Source contract | Acceptance evidence |
| --- | --- | --- |
| Simple root | overflow hidden; 8px radius; base background; shadow-xs; outside 1px line ring; no default padding | Painted bounds/ring/shadow, clipped content and consumer padding override |
| Layered root | full-width flex column; overflow hidden; 8px radius; elevated background; 14/21px text; outside hairline ring; no simple shadow | Root selected when structured parts exist; actual paint in both themes |
| Secondary | vertical margins −8px; horizontal flex aligned center; gap8px; elevated background; padding16px; 14/21px medium subtle text | Measured spacing/overlap, text inheritance, long content and nested Button |
| Primary | relative flex column; gap8px; hidden overflow; 8px radius; base background; padding16px except right12px; inherited text; outside fill ring | Exact geometry, clipping and content inheritance |
| Composition | Simple content and ordered primary/secondary sections; sections can contain arbitrary elements | Native typed construction preserves mode/part information before AnyElement erasure; document adaptation of React fragment/type inspection |
| Styling | Root/sections accept caller refinements after recipe | Demonstrate width/padding and section layout overrides; documented precedence |
| Identity | Multiple cards and sections retain distinct scoped IDs | Actual rendered selectors, reordered or repeated content |
| Interaction | Card container has no inherent activation or focus; composed controls keep their own behavior | Input editing and Button pointer/keyboard activation still work inside parts without duplicate listeners/state |
| Themes | Current base/elevated/line/hairline/fill/subtle tokens | Retained owner observes Theme; changes update paint without remounting controls |
| Edges | Empty card/section, narrow width, Unicode, oversized content and nested interactive content | Real layout and input; explicit native differences |
| Accessibility | Container grouping must preserve descendants; card presentation does not imply button/link | Rendered/native tree; caller-controlled semantic adaptation documented |

Hover/pressed/disabled/selected/loading/invalid/open states and focus trapping/dismissal are not inherent LayerCard states. Relevant states of nested controls still require input verification. Native dark and browser comparisons remain separate from headless paint/layout checks.

Existing Theme already contains all LayerCard colors, 4/8/12/16px spacing, 8px radius and shadow-xs. Implement over GPUI composition rather than inventing Base interaction state. Select a small typed API for simple versus section content; retain recipe-specific parts within the component and apply consumer refinements deliberately. Do not reintroduce deprecated Surface aliases.

## Implementation and review

LayerCard and typed Section::primary/secondary implement the source treatments. The section builder preserves structured mode before AnyElement erasure; ordinary child content remains unstructured. Styled refinements apply after the root/section recipe, including replacement of the native shadow list. React element replacement/fragment type inspection translates to explicit section construction and caller-owned native content/semantic wrappers. The card itself adds no activation, focus target or accessible role; descendants retain their existing contracts.

Outside rings use zero-blur shadows with 1px spread; installed GPUI Style::paint draws drop shadows before Div enters the overflow mask, so its own ring is not clipped by its child overflow policy. This source evidence does not establish pixel parity. GPUI's overflow mask is rectangular and has no corner radii: see [GitHub #5](https://github.com/msmps/gpui-kumo/issues/5) for the concrete fidelity limit and remaining acceptance.

Skeptical review found a medium typography mismatch: layered roots/secondary sections set only text size, inheriting a custom parent's line height instead of Kumo's text-base line height. Both now set the base line-height token, with a rendered regression comparing simple inheritance to layered recipes. The component retains no redundant entities, listeners or state. The gallery owns its Input once and retains input/theme subscriptions; its Button callback updates the owner once.

Six actual-render tests pass: default secondary corner paint and header/footer clipping with caller radius/background overrides; geometry and caller overrides in both themes; simple versus layered line height; nested pointer/Enter/Space single activation, focus traversal, Unicode editing after narrowing/theme changes and disabled rejection; empty content and oversized rectangular paint masks. The gallery contains simple, interactive layered and narrow Unicode examples with repeated scoped header/body IDs. Current gate: 49 workspace tests and two doctests pass; formatting, all-target/all-feature locked Clippy with warnings denied and gallery/example builds pass. The upstream block 0.1.6 future-compatibility notice is separate from these passing checks.

Native macOS inspection on 2026-10-02 reproduced square secondary fills covering the rounded root. Rebuilt light and dark captures after the private paint correction show smooth top corners on the interactive and narrow cards. The cmd-l shortcut changed appearance; Save activation eventually updated the native counter once. Accessibility preserved Text names, Save and the named Card project name Input. Accessible value assignment followed by pointer focus/paste changed the exposed Unicode value, but captured Input pixels lagged that value; native input/frame consistency remains unverified. Earlier unchanged input captures are retained as a session limitation rather than declared a dependency defect. Quit completed and pgrep confirmed no gallery process remained. Pinned browser comparison remains pending.

Status: implemented with automated coverage; the reported default top-corner fill defect is corrected; rounded subtree clipping, native input/frame consistency and pinned browser pixel checks remain open. Badge is the next companion presentation milestone: omit its deprecated destructive variant and legacy compatibility type alias; derive supported variants from the actual map rather than contradictory prose that mentions nonexistent map keys.
