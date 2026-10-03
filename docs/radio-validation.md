# Radio acceptance

Pinned Kumo `3fd5b648df578cb1ba214dedd30f475009f6a668`: Radio source, docs, all demos, tests and type specification inspected. Base 0.7.0 Radio owns controlled activation/focus and actual RadioButton metadata; its RadioGroup supplies grouping/orientation only. No Base RadioIndicator or roving navigation API exists in this version. Presentation and composed keyboard navigation remain library-owned.

| Area | Acceptance |
| --- | --- |
| API | Generic Clone+Eq values with stable independent item IDs; controlled optional selection; default/error, default/card; group/item appearance override; start/end control; horizontal/vertical; rich decorative labels, card descriptions, custom/hidden legend, simultaneous error/helper |
| Value/callback | Single owner, activation of selected item no-op, pointer and keyboard report native event details once; arrow/Home/End proposal with focus transfer, disabled candidates skipped, unknown selection uses first enabled focus entry |
| Focus | One selected/first enabled tab stop; arrows wrap over enabled items; Tab exits group; repeat instances and reordering retain item identity; no focus entity recreated per render |
| Paint | 16px circle, 8px base dot, 2px top offset. Inline 8px gap/normal label, 1px line/error ring. Card 12px padding/gap, 8px radius, fill/border selection/error precedence, medium label and 2px description gap, 2px indicator ring. Group16px gaps; items8px/12px; horizontal cards two equal columns |
| Alignment | Actual indicator/dot centres, control/text top offset, start/end ordering, card padding and narrow wrapping in both themes; control wrapper must preserve parent alignment |
| Themes/states | Default, hover, pressed activation, focus/keyboard focus, disabled, selected, invalid. Loading/open/trapping/dismissal/clipboard/IME N/A |
| Semantics | Actual Base RadioButton checked/selected/name/set position, Base RadioGroup name/orientation. Disabled metadata enriched through GPUI synthetic subtree; native spoken grouping remains separate |
| Verification | Rendered input/focus/callback and geometry regressions, full locked Rust gates, native both-theme gallery. Browser combined ring precedence, web expanded inline hit target and OS speech remain awaiting validation |

Native adaptations: consumer owns initial/uncontrolled value; legend content is separate from complete readable group name. Checkbox/Radio groups render both messages following source. Navigation composes Base's retained focus handles because this installed Base group is semantic-only; it must not duplicate persistent selection state.

User review requirement: inspect rounded card corners for child fill leaks, border/ring clipping and layered contrast in both themes and narrow/resized layouts. The native card indicator ring must remain circular and its paint must not clip at the card edge; rich decorative labels must not introduce unjustified overflow masking. LayerCard arbitrary-descendant clipping remains separately tracked.

## Evidence and review

81 workspace tests and eight doctests pass; locked all-target/all-feature builds, formatting and Clippy with warnings denied pass. Four rendered regressions cover typed controlled selection/callbacks, disabled navigation, retained focus/reordering, unknown values, tab entry/exit, card/start/end geometry, whole-label hover, narrow layouts and both-theme border/fill paint.

Linux native captures show pointer selection followed by Right skipping the disabled option and Space on the checked option leaves exactly two changes. Actual Tab moves from page-size selection to the selected Pro card, then exits to Checkbox, with visible focus rings (entry (capture removed), exit (capture removed)). Dark interaction (capture removed) preserves state across appearance changes. Narrow light (capture removed) and narrow dark (capture removed) show wrapped descriptions and coherent card borders/fills/corners. Gallery header controls now wrap rather than overlap the title.

Independent review found no remaining correctness blocker. Findings repaired: whole-item hover instead of indicator-only hover; card supporting text authored as aria_description on the actual Base Radio; gallery narrow header overlap. Geometry assertions inspect border and fill as separate paint quads. All native testing apps were terminated after capture. Browser pixel/ring parity and spoken platform metadata remain pending; authored accessibility metadata is not a speech validation claim.
