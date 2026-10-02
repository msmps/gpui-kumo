# Text contract and validation

Baseline: Kumo `3fd5b648df578cb1ba214dedd30f475009f6a668`, GPUI Kit/Base 0.7.0, GPUI 0.3.7. The pinned [Text source](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/text/text.tsx) defines the contract. Deprecated heading variants are excluded under the repository's scope rule.

## Parity matrix

| Branch | Native contract | Evidence / remaining work |
| --- | --- | --- |
| Copy | Default, secondary, success, error; 12/13/14/16px; bold means medium 500 | Render probe verifies sizes, weights and theme tones; success uses link blue |
| Heading | 16/24px and 20/28px, semibold | Rendered geometry and inherited-font probe |
| Mono | Default/secondary, 13/14px | Render probe verifies consumer mono family, inherited weight and secondary tone |
| Inheritance | Copy/mono inherit line height and weight; heading inherits family with fixed line height/weight | Parent line-height changes and actual prepaint text styles tested |
| Themes | Current semantic colors; consumer mono family survives appearance change | Render probe in light/dark; native light observed; native dark remains pending |
| Accessibility | Full text name; decorative heading stays Label; explicit level opts into Heading | Rendered role/name tests; installed GPUI supports aria_level; spoken hierarchy not verified |
| Composition | Decorative `StyledText` or other inline element through rich_content | Gallery emphasis and render probe; caller supplies complete accessible text to new |
| Layout | Wrapping by default; optional single-line ellipsis; minimum width zero | Narrow Unicode layout tests, repeated identity, native light ellipsis/wrapping; empty boundary tested |
| Interaction | Read-only presentation, no durable value or focus target | Hover/pressed/focused/disabled/selected/loading/invalid/open, pointer activation and keyboard navigation are not applicable |

Text uses GPUI layout and accessibility directly; no Base state engine is needed for read-only presentation. The owning view observes Theme. The component is consumed per render and retains no independent entity or subscription. Interactive controls belong outside rich_content. Native monospace defaults to Menlo on macOS, Consolas on Windows and monospace elsewhere; applications can override Theme.typography.mono_font_family.

An empty string retains GPUI's inherited line box (21px in the fixture). It renders no characters; callers that need absent content to consume no layout space should omit the component. This native adaptation is tested rather than silently assuming browser empty-span geometry.

## Review and limits

Independent review found a medium fidelity issue: copy/headings forced the theme sans font rather than inheriting the consumer's font. The override was removed; the actual prepaint-style regression now exercises font inheritance, medium copy weight, heading weight, mono inheritance and theme colors. Stable IDs distinguish repeated visible labels. Accessible names retain the full string after visual truncation.

On 2026-10-02, rebuilt macOS gallery inspection showed modern headings, all copy/mono examples, inline emphasis, café/emoji wrapping and ellipsis in light appearance. These observations are not a browser pixel comparison. Repeated dark-button clicks left the appearance unchanged and the native accessibility tree exposed only window chrome; native dark and heading-level platform exposure remain pending. Quit completed and process inspection confirmed no kumo-gallery process remained. Windows/Linux and spoken-reader checks were not run.

Text is implemented with automated coverage, awaiting the native dark/accessibility and pinned-browser comparison checks above. Do not infer complete platform parity from compilation or the render probe.
