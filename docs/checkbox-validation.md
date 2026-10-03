# Checkbox acceptance

Pinned Kumo revision and Base versions follow port-progress. Source, docs, tests and all Checkbox demos inspected. This milestone implements the single Checkbox; Group/Item/Legend and select-all aggregation follow separately.

| Area | Acceptance |
| --- | --- |
| API/state | Explicit controlled Unchecked/Checked/Indeterminate, default/error variant, disabled, required optional label, label-first/bare forms, named callbacks with native event modifiers. Caller commits state; no mirrored value entity |
| Behavior | Base Checkbox and Indicator; pointer over label/control and Space/Return each propose once; mixed activates checked; disabled blocks both input paths and traversal; stable supplied or keyed focus |
| Paint | 16px square, 4px radius, base/contrast fill and hairline/contrast ring; inverse 12px Phosphor bold marks; 8px label gap, 2px top alignment; normal/error/focus ring precedence; disabled square opacity .5, readable label unchanged |
| Themes/edges | Light/dark semantic roles, long wrapping label/narrow layout, repeated instances, external state updates |
| Semantics | Base actual CheckBox role, readable name, false/true/mixed state, disabled metadata; OS speech separately unverified. Required is label presentation, native form submission N/A |
| N/A | Loading/open/focus trap/dismissal/clipboard/IME |
| Deferred | Tooltip, Group/Item/Legend aggregation, browser pixels, CSS pseudo-element expanded bare hit target, spoken platform semantics |

Native adaptation: the Base semantic root encloses the decorative square and visible label. Label activation therefore uses the same Base callback and focus path, avoiding a second listener and duplicate activation. Single Checkbox requires a complete accessible string at construction, even when rendered bare.

Review repairs: constructor rejects blank names; Label content mode inherits normal Checkbox typography; hover changes the unselected ring to hairline while selected contrast takes precedence. Combined error/hover/focus browser precedence remains awaiting generated-CSS comparison. Disabled parent metadata is authored through GPUI’s synthetic-subtree hook, which Base test snapshots do not capture; input gating is tested separately, and native spoken exposure is not claimed. Bold marks are extracted from Phosphor React v2.1.10 defs/Check.tsx and defs/Minus.tsx; existing MIT notice applies.

Group milestone acceptance: Group/Item/legend slot and hidden legend; explicit item values/duplicate rejection; controlled Vec selection preserves unrelated values and emits once; individual/group disabling; aggregate checked/mixed/empty selection from explicit participation values; empty participation inert; source group gaps16/item gaps8. Pointer/Space, externally changed values and both-theme composition must be exercised. Native role Group with complete name replaces fieldset/legend linkage; spoken grouping remains unverified. Group items reuse single Checkbox presentation and are not yet independently usable uncontrolled items. No UI entity mirrors application selection. Error and description both render because pinned source does so, despite its docs claiming replacement. Aggregate participation is explicit, including disabled values only when the caller includes them.

Group gate: 74 tests/six doctests, format, warning-denied all-target/all-feature Clippy and builds pass. Reviewer caught Field line-height reuse; group now uses small-text metrics, with half-device-pixel layout quantization in the geometry assertion. Repeated groups, prefixed aggregate/item identities, custom/hidden legend, empty aggregate and source simultaneous help/error are covered. Mixed light (capture removed), pointer-selected all (capture removed), and Space-cleared dark (capture removed) were inspected. The live gallery was terminated. Native API adaptation: Item is a valued configuration in Group; standalone controlled uses Checkbox. Legend is a visible-content slot with a separate complete group name; default/uncontrolled state is owned by the consumer. Per-item override callbacks and rich item-label slots remain gaps rather than silently supported capabilities. Disabled items dim their complete label/control, unlike single Checkbox which dims only its square.


## Implementation notes

Pinned Kumo Item type declares a string label; the earlier rich-item-label backlog entry was incorrect. Rich single Checkbox labels remain a separate source capability. Matching Base UI1.8.0 CheckboxRoot derives checked from group.value for valued items, so its item checkedProp is superseded; native selected values likewise remain the sole checked owner. Explicit Item indeterminate presentation remains pending.

`CheckboxItem::on_change` observes the Base-proposed State and native ClickEvent, returning `std::ops::ControlFlow<()>`. Continue permits the group proposal; Break cancels it. The item observer runs before the group callback, matching onCheckedChange/details.cancel ordering. The group proposal uses the selection supplied at the completed render, preserving unrelated values. A callback that owns a different selection update should cancel this proposal explicitly. Neither callback mutates internal checked state. Aggregate/group rerenders and external owner changes do not fire item callbacks; observers also work without a group handler.

| Acceptance | Evidence |
| --- | --- |
| Pointer/Space/Return | Ordered item then group events exactly once per input; both-theme rendered event log assertions |
| Cancellation/ignored proposal | Item fires alone on cancellation, visual checked and group values retained. Ignored group proposal and observer-only item likewise remain controlled |
| Disabled | Focus an enabled item, disable it, then pointer/Space/Return are rejected; disabled group blocks child and aggregate callbacks. Existing Base implementation retained |
| Aggregate/external ownership | Select all emits only group callback; external selection changes emit neither callback. Unknown selected values preserved |
| Geometry/themes | Existing variants/radii/sizes retained; native light/dark wide/520px verify long string wrapping, first-line control alignment, label-first optional layout, rings/disabled contrast and corner edges |
| Semantics | Existing Group/Checkbox readable names, checked states and availability untouched; native platform speech/browser comparison remain unverified |
| N/A/gaps | No new loading/open/invalid/selected state beyond existing Checkbox; clipboard/IME/trapping N/A. Item indeterminate and single rich/tooltip labels remain gaps |

103 workspace tests/nine doctests, formatting, warning-denied workspace all-target/all-feature Clippy and locked builds passed; existing upstream profiler warning remains visible. Independent review found no blocking ownership, state snapshot, callback or disabled-path defect. Native default light (capture removed), three Email events/three SMS cancellations (capture removed), aggregate dark (capture removed), narrow dark (capture removed) and narrow light (capture removed) reviewed. The native counters show3 each after separate pointer/Space/Return. Select all changes the group without firing item callbacks, matching the explicit source contract. Long string/help wrapping, first-line control offsets, optional label-first layout, off/focus/disabled edges and both themes inspected. No new persistent entity, subscription or dependency introduced.


## Implementation notes

Pinned single Checkbox label is ReactNode; Item label remains string-only. `Checkbox::content` consumes a decorative element while the constructor string remains the complete accessible name. Existing Label content typography, optional decoration and Base Checkbox activation are reused; interactive accessories are outside this slot. No new persistent state, entity or subscription.

| Acceptance | Evidence |
| --- | --- |
| Rich API/name | StyledText bold/italic example; rendered CheckBox name preserves complete Unicode constructor string |
| Pointer/Space/Return | Rich-label click and each key produce exactly one controlled proposal; mixed activates checked |
| Disabled/state ownership | Focus then disable blocks pointer and both keys; caller owns all checked state |
| Geometry/themes | Both orders at180px wrap inside root; native1040/520px light/dark shows first-line control alignment and optional label-first decoration |
| States/N/A | Existing default/error/checked/mixed/focus/disabled retained; no new loading/open state. Trapping/clipboard/IME N/A |
| Gaps | Contextual help, item indeterminate, browser pixel comparison and spoken platform semantics remain pending |

104 workspace tests/nine doctests, formatting, warning-denied workspace all-target/all-feature Clippy and locked builds passed. Existing dependency profiler warning remains visible. Independent reviewer found no blockers in consumed-slot ownership, hidden-label disposal, accessible names or Base activation. Native light (capture removed), three pointer/Space/Return proposals (capture removed), dark (capture removed), narrow dark (capture removed) and narrow light (capture removed) inspected: bold/italic glyphs actually paint; Unicode survives, wrapped lines align under text, control tracks first line, optional suffix and label-first control remain aligned. Corners, borders and focus layering reviewed in both themes. No new review defect found; this does not establish browser or OS speech parity. Gallery terminated after capture.
