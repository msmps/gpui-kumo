# KUMO-053: Rich Text highlighted font fidelity

Status: Resolved; native/historical proof and required pinned gate pass

GitHub issue: https://github.com/msmps/gpui-kumo/issues/38

Native review of #37 found highlighted `emphasis` appears serif beside sans-serif plain Text. This predates the metadata fix; before/after geometry is identical. Gallery uses GPUI StyledText with a semibold HighlightStyle. Inspect pinned Kumo Text, installed GPUI shaping/run resolution and available font faces before attributing the cause to inheritance.

Acceptance: reproduce highlighted/plain semibold and inherited custom font resolution; repair the smallest verified cause; review painted glyphs/weights in both themes1040/520; meaningful rendered regression, required Rust gates and independent review. Preserve caller-owned rich content and document environment/fallback limits. Read-only GPUI presentation applies; no Base state primitive needed.

Evidence: [native Text capture](../evidence/readable-labels/text-light-1040.png). Next order remains #36, #26 Pagination parts, then dependency-aware backlog including this finding.

2026-10-03 managed Linux reproduction: Light/Dark1040/520 and three inherited font families pass painted review and native face/glyph/advance assertions. Retained historical pixels exactly match the current explicit Sans sample; independent font-file advances identify Sans Bold. No production font patch is justified. [Current probe, captures and limits](../evidence/text-fonts/README.md). The bounded reported discrepancy is resolved by evidence; other Text/platform acceptance remains separate.
