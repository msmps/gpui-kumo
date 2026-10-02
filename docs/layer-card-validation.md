# LayerCard acceptance contract

Selected after Loader checkpoint `20f01e4`. Inspect the [pinned source](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/layer-card/layer-card.tsx) before implementing; [live examples](https://kumo-ui.com/components/layer-card) include simple surfaces, layered headers, interactive header accessories, multiple cards and Input/filter compositions. Examples inform consumer usage; the pinned source controls the recipe.

Surface is explicitly deprecated in the [pinned compatibility wrapper](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/surface/surface.tsx). It is excluded. LayerCard provides the supported simple surface and layered treatments.

## Required branches

| Branch | Source contract | Acceptance evidence |
| --- | --- | --- |
| Simple root | overflow hidden; 8px radius; base background; shadow-xs; outside 1px line ring; no default padding | Painted bounds/ring/shadow, clipped content and consumer padding override |
| Layered root | full-width flex column; overflow hidden; 8px radius; elevated background; 14px text; outside hairline ring; no simple shadow | Root selected when structured parts exist; actual paint in both themes |
| Secondary | vertical margins −8px; horizontal flex aligned center; gap8px; elevated background; padding16px; 14px medium subtle text | Measured spacing/overlap, text inheritance, long content and nested Button |
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

Status: source/docs inspection and acceptance defined; implementation not started. Badge is the next companion presentation milestone: omit its deprecated destructive variant and legacy compatibility type alias; derive supported variants from the actual map rather than contradictory prose that mentions nonexistent map keys.
