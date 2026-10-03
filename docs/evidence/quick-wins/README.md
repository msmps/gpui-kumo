# Meter, Breadcrumbs, InlineCopyText and Select quick-win verification

Pinned Kumo revision `3fd5b648df578cb1ba214dedd30f475009f6a668`, GPUI Kit/Base 0.7.0 and patched GPUI 0.3.7. macOS Metal captures use the project's system font, logical 1040×640 / 520×640 windows and 2× scale. Native font metrics differ from the earlier Linux/browser DejaVu Sans fixture; these are native rendering results, not a new browser pixel-parity claim.

`review.rs` is the exact native capture fixture. To reproduce, temporarily copy it to `apps/gallery/examples/quick_win_review.rs`, then run `cargo run --locked -p kumo-gallery --example quick_win_review -- --capture` from the repo root. It draws through the native renderer and saves `Window::render_to_image()` output, avoiding stale pixels from a fully occluded macOS window. Copy actions use GPUI Kit's actual rendered-selector pointer and Space input paths. After inspection remove the temporary example. The fixture also offers interactive theme/width/reduced-motion controls when run without `--capture`.

| Native matrix | Measured result |
| --- | --- |
| Light/Dark ×1040/520 × selected3/10 | All eight capture cases pass; [raw dimensions](native-results.log) |
| Selected3 popup |61px wide; check appears only on selected row |
| Selected10 popup |68px wide; digits stay on one line, check and text fit |
| Numeric row |33px high in all eight cases |
| Default/styled Meter |8px round track versus caller12px/4px radius track and success fill;25% geometry and semantic value retained |
| Styled Breadcrumbs |56px row,8px horizontal padding,6px radius and themed fill; below640px ancestors collapse and current truncation retains full readable name |
| Copy feedback |Pointer followed by Space leaves localized copied name/check; actual focus remains on copy action; beside taller Save, text/check/control centres agree |

Visual review explicitly inspected text baselines, Meter label/value separation, track edges and 25% fill boundary, breadcrumb chevron/text centres and padding, copy/check centres beside Save, compact numeric check/text spacing and popup padding. Reviewed both themes and widths, focus-ring containment, rounded root/track/popup corners, fill/border/shadow agreement and absence of inner edge fringes. A fractional-width two-digit wrap was found during initial native review and fixed by rounding shaped word widths upward; no failing capture is presented as acceptance evidence.

Pinned Kumo's kumo-binding.css overrides Tailwind's default duration to100ms; Breadcrumbs/InlineCopyText use this duration and cubic-bezier(0.4,0,0.2,1). Breadcrumbs fades the whole copy action; InlineCopyText fades only CopySimple and mounts Check immediately. Rendered regressions exercise reversal and reduced-motion frame cessation. Static captures do not establish temporal motion parity.

Platform speech (#2–#4), catalog browser comparison and OS reduced-motion delivery (#10) remain separate acceptance. InlineCopyText now authors a stable zero-size Label with readable localized value and polite live metadata; this is not measured VoiceOver speech. GPUI cannot report clipboard delivery failure. Arbitrary rounded-descendant clipping remains limited by rectangular GPUI masks (#5); custom styles do not claim to fix it.
