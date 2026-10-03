# Readable Label values

Tracker#37/KUMO-052. Baseline published `d195dedf0ae27f37ac62a18414be25e996158c64`,165 tests/nine doctests; Kumo3fd5b648, Kit/Base0.7.0, GPUI0.3.7. Installed consumer0.38.0 and atspi_common0.19.1 inspected: Role::Label takes its exported name from value. Pagination's actual native probe exposed this mismatch; existing headless `.label()` checks passed with an empty platform name.

Highest-value selected foundation: expose readable values on existing Text, Label/Field, Badge, Banner title/plain description and BreadcrumbCurrent nodes. No public API/dependency/presentation change. Existing Label names remain authoritative; clone the same SharedString into value. Retain all entities, focus/activation contracts and caller-owned rich-content semantics.

| Area | Acceptance |
| --- | --- |
| Names | Actual Label node name and full readable value agree, including Unicode, truncated/long content, repeated instances and optional suffix; native exported sample names match complete text |
| Updates | Owner text/optional/theme/width changes produce current values; no recreated input or durable entities |
| Roles | Explicit Heading remains Heading with its existing name/level and no added value. Label as_content and loading BreadcrumbCurrent remain without standalone Label/value; caller-owned rich Banner description unchanged |
| Presentation | Existing typography/tokens, dark/light, narrow geometry, icon/control centres, baselines/spacing, borders/corners/rings/layers remain source-owned; metadata must not change layout |
| Input | Label focus association and disabled guard remain correct; other labels are read-only. No new activation, edit action or live-region semantics |
| N/A | New variants/sizes/controls, selected/invalid/open states, focus trap, IME and persistent lifecycle machinery; existing components retain those contracts |
| Gate | Observable rendered value/name/role/update regressions, existing geometry/activation tests, actual Linux X11 names before/after in four theme/width combinations, independent review and required Rust gates; speech/other platforms remain separate |

Baseline native probe (with the standalone optional Label gallery sample): all four light/dark1040/520 combinations contain143 Label nodes,139 unnamed. All nine selected Text/Badge/Banner/Breadcrumb/Label names are missing, while the explicit Heading name remains correct.

After repair: all143 Label nodes have names, all nine complete sampled names are present and the explicit Heading remains preserved in every combination. Every ordered Label bound is identical before/after (zero changed bounds). [Raw before](evidence/readable-labels/before.json), [after](evidence/readable-labels/after.json) and [geometry comparison](evidence/readable-labels/geometry-comparison.json) preserve the measured evidence. The public probe is `scripts/validate-readable-labels.py --fixed`; its CLI works without importing optional native dependencies before argument parsing. The private equivalent produced baseline/after evidence; the committed public probe subsequently ran successfully and produced identical after results.

Passed: formatting,166 workspace tests/nine doctests, warning-denied all-target/all-feature Clippy/build and default-feature native gallery build (`/tmp/label-values-{fmt,test,clippy,build,native-build}.log`). One cross-component actual-render regression exercises owner text/optional/theme/width changes, long Unicode/truncated content and preserved heading/as-content/loading policies. Existing Badge/Field/Banner/Text assertions now also check values; existing focus/disabled and geometry tests pass. Independent diff review found no blocking issue; no new state, subscription or activation machinery.

OS speech, VoiceOver and Windows are not established by Linux metadata; #2/#3/#4 retain those separate workflows. Next selected: #36 pinned Linux disabled-state mapping, then #26 Pagination dropdown/PageSize.

Review findings: initial offscreen theme activation left captures light despite dark filenames; the capture procedure now returns to the visible theme controls before switching and checks painted colours. This was an evidence defect, not a component change. Native rich Text highlighted glyphs also appear serif; the pre-existing font-resolution question is tracked in [#38/KUMO-053](issues/053-rich-text-highlight-font.md), with no claim that this metadata fix resolves it. Future native captures must verify actual theme colours, not filenames.

Final visual review: all20 captures across both themes/widths reviewed. No new icon/control centre, text baseline, label/helper offset, sibling spacing, padding, rounded border/fill/ring/layer discrepancy found. Narrow Banner text wraps alongside actions; Badge pills wrap cleanly; BreadcrumbCurrent truncation keeps row centres. These captures show static visible states; existing rendered tests establish focus/disabled guards. Independent final reviewer agreed.
