# Loader acceptance contract

Selected after Text checkpoint `e617c31`. Source: [pinned Loader](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/loader/loader.tsx). Live Loader docs confirm default/custom size examples; the pinned source additionally defines the translatable accessible name and motion recipe.

| Branch | Acceptance | Verification |
| --- | --- | --- |
| Sizes | Small 16px, base 24px, large 32px; custom pixel size | Real rendered bounds including Button's 14/16px composition |
| Paint | 24-unit view box; circle center12 radius9.5, stroke2, round caps; current foreground; track opacity0.1 painted after active arc | Installed GPUI paths/quads, native light/dark and pinned browser comparison |
| Motion | Rotation2s; dash 0→42→42 and offset0→−16→−59 over1.5s, linear halves | Actual advancing frame output, cycle samples and reduced-motion frame scheduling |
| Name | Status with default Loading, localized override; no focus target | Rendered role/name; platform exposure separately |
| Button | Uses same paint recipe at14/16px; parent already exposes busy/loading | Existing unavailable-state input regressions plus size/layout checks; decorative child avoids duplicate status |
| Edge cases | Empty localized name, custom zero/negative/nonfinite sizes need explicit native policy | Test selected policy without panics or invalid paths |
| Themes | Inherit foreground, including Button's inverse color | Actual prepaint/paint colors after theme update |
| Interaction | No activation, value, hover/pressed/selected/invalid/open state | Not applicable; loading controls own availability |

Review finding: existing Button's private spinner is a fixed 270-degree, 1-second, 1.5px polyline with no faint track. Reuse its composition boundary, replace its presentation with the source recipe. GPUI AnimationExt respects reduce_motion, but its repeating start state would leave Kumo's active dash empty; select a visible static arc for reduced motion and schedule no repeating animation.

## Implementation notes

Standalone Loader and Button now share one GPUI canvas renderer. Radius/stroke scale with the source's 24-unit coordinate system; two SVG arcs form each path and endpoint discs supply round caps. A 6-second animation combines three rotation cycles and four dash cycles. Reduced motion selects a visible static phase and schedules no animation; zero-size indicators also schedule none. Nonpositive/nonfinite custom values normalize to zero rather than sending invalid geometry to the renderer. Standalone Status names default to Loading and support localization; Button's internal graphic is decorative under its existing busy/unavailable node.

Two actual-render tests verify dimensions, roles/names, invalid custom dimensions, empty names, moving painted endpoints, inherited foreground updates in both themes, and absence of further frame requests after reduced motion. Existing Button input/availability regressions remain part of the workspace gate. The paint tests observe caps and scheduling; they do not independently prove the entire SVG path or dash timing formula.

Native macOS inspection on 2026-10-02 showed the four sizes, faint tracks and rounded active segments. A later rebuilt gallery check successfully switched to dark with cmd-l; light/dark captures show advancing native arcs. Accessibility exposed named containers Loading/Chargement, so spoken Status announcements are not claimed. Quit completed and pgrep confirmed the test process stopped. Pinned browser comparison remains pending.

Review: replaced the fixed Button polyline rather than reusing its divergent recipe; retained its parent availability and metadata behavior. Closed the track path to avoid a seam and prevented zero-size indicators from scheduling frames. No independent state engine or external dependency was added. Still review zero-length dash caps and exact SVG path endpoints against the pinned browser before claiming visual parity.

Status: implemented with automated coverage and native light/dark presentation evidence, awaiting browser comparison and spoken status checks.

Checkpoint gate: 43 workspace tests and one doctest pass; formatting, workspace/all-target/all-feature locked Clippy with warnings denied and gallery/example builds pass. The inherited-color assertions exercise actual painted caps in both themes. The upstream block 0.1.6 future-compatibility notice remains separate from successful current checks.

The [deterministic SVG fixture](../tools/validation/loader/loader-reference.html) reproduces the pinned circle/animation markup and samples chosen times in light/dark contexts. It isolates the recipe rather than mounting the React package. Computer-use attempts reported both Chrome and the in-app browser unavailable, so the fixture has not been executed here. Do not treat its authored markup as comparison evidence.

Timing-test repair during the LayerCard gate: the installed unsynced AnimationElement uses Instant::now/start.elapsed, while repeat_synced uses the executor clock. The test previously advanced only the executor and could compare unchanged paint. It now waits two actual 250ms intervals before checking endpoint movement, preserving the production animation policy. Workspace and isolated regression checks pass.
