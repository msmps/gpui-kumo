# Pagination dropdown continuation (#26 / KUMO-041)

Recreated over verified `work`6c316ab533e2ab7cb5d54e694d2c9c3f10b3686e. Only main/work are available; no recovered candidate. Earlier174-test/native/browser claims describe missing work and are not acceptance evidence for this recreation. Family remains open; coverage stays27/43 working,16 unported.

## Contract

`Pagination::page_selector(pagination::PageSelector::Dropdown)` selects the recommended source page dropdown; Input remains default. Full controls and Known totals allocate all page options with stable numeric IDs. Input, Simple and Unknown modes keep the retained Select's option list empty, including usize-max totals. Dropdown intentionally enumerates every page; use Input for large datasets. Known empty totals preserve the existing normalized page1 behavior.

One retained Base PaginationState owns bounds/page/availability. A retained controlled Kumo Select projects the page and proposes through Base's guarded request; it never accepts independently. The native Input still owns its draft and selection. Programmatic owner setters remain silent. Weak handlers stamp the owner revision synchronously at Select activation and defer delivery until the Select borrow ends. Page, size, total, availability, mode and size-option updates invalidate pending proposals; removed Select mounts also reject pending delivery. Public Select events remain unchanged. No render-time entities/subscriptions or second page/draft authority.

| Branch | Required acceptance |
| --- | --- |
| Proposals | Accepted/rejected once; pointer, Space, Enter and same-value no-op; page and PageSize stale-owner cancellation |
| Collection | Full/Known/Dropdown only; stable option IDs; changed totals/per-page/owner values while open; Input/Simple/Unknown huge counts never enumerate |
| Focus/lifetime | Pointer trigger/content focus, Escape and Tab restoration/exit, disabled/restored, focused part removal, unmount/remount and repeated instances |
| Source paint | Both themes; standard-shaped42×36 navigation,214px input/83px simple, three dropdown overlaps with no leading Select overlap; flat hairline middle trigger and opaque disabled joined borders |
| Popup | Source Base trigger gap6; source14px/21px base text yields33px compact option rows; selected-only checks; placement/collision/scrolling and hovered/open paint |
| Native | Light/Dark1040/520; actual glyphs, centres/baselines/gaps/padding, joins/corners/fills/focus/shadows, owner state and PageSize regression |
| Platform | Browser#10, IME#1, spoken readers#2–#4 remain separate; authored semantics are not exported/speech proof |

Pinned source inspected anew: Pagination source/tests, Select source, Button source, InputGroup root/button/input; installed Base Pagination and Select disclosure/positioning implementation. Existing acceptance document records the complete original demos/docs inspection. No version changes or new vendor patch. Source links use Kumo3fd5b648df578cb1ba214dedd30f475009f6a668.

## Implementation and review

Typed selector, retained page Select, fresh compound controls and gallery accepted/rejected/dropdown-with-PageSize examples added. Existing Input and PageSize examples remain available. Skeptical diff review identified and repaired proposal timing, repeat-render owner notifications, absent middle border participation, stale unmounted delivery, and disabled border opacity. New five rendered/input regressions exercise the contracts above; existing input geometry and numeric-row assertions now use the source-derived dimensions. Shared Select uses its actual base typography for option rows and Button size gaps for trigger spacing; compact PageSize measures the same recipe. The native PageSize probe now expects33px rows; historical32px evidence remains historical.

## Measured validation and limits

Local workspace and standalone adapter formatting pass with pinned Rust1.99. Python PageSize probe syntax passes. The locked Linux script stops before test execution at missing `hdrhistogram` index metadata. Fresh HTTPS probe fails to connect to configured proxy:8080 despite enforced network policy. Local tests/lint/build/native/browser are not claimed. Remote macOS CI pending at the implementation checkpoint; record its exact SHA/run/results before acceptance.

Both-theme painted geometry regressions are headless evidence only when executed. No new native screenshot, glyph/font, OS input, accessible export or browser acceptance is claimed. #42 remains open for its native foundation matrix; #38 requires native glyph reproduction. Restore managed connectivity, run the Linux gate, then complete #42 and this dropdown/native PageSize matrix before full family acceptance.

## First measured remote run and repair

[Run37117276643](https://github.com/msmps/gpui-kumo/actions/runs/37117276643), job111186393342, at46f919e compiled the workspace and executed175 library regressions:173 passed, two new tests failed. The focus-removal host did not register the gallery's global Tab actions after leaving Select; the painted-border assertion expanded the Button's outer bounds a second time (its child canvas already excludes the1px CSS border). Test-only repairdfbbdc06 registers actual gallery traversal actions and compares the exact outer Button rectangle, retaining full-opacity/Select hairline assertions. Further lifecycle coverage rejects an activated page proposal when the root is removed before deferred delivery. No production guard or assertion was weakened. Full repaired gate is pending; initial run did not reach doctests/lint/build/adapter stages. GitHub has not yet scheduled the repair push; check actual latest head rather than treating the initial run as current validation.

Final source review also preserves Select Trigger shadow-xs: Pagination only overrides rounded corners and hairline ring on this standalone Select; InputGroup.Button shadow-none does not apply to it. The trigger therefore retains its existing source shadow in the joined composition.
