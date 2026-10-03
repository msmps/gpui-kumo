# Library-owned Select and Link controls

Select owns its trigger focus, disclosure, confirmation and close policy in `SelectState`. Its GPUI ComboBox root exposes the existing readable value, expansion and availability metadata. Base still supplies positioning and compatible action types. There is no Base Select handler left to move focus after an application confirmation callback.

Link and Toolbar Link share a semantic GPUI root that tracks the retained handle used by Kumo. GPUI supplies pointer and keyboard click dispatch; Kumo invokes the existing navigation and observation callbacks once with the current destination and original event. Disabled controls reject activation and tab participation. Public APIs remain unchanged.

The Base Select and Link vendor source now matches the checksum-verified published 0.7.0 archive. Their retired corrections and regression references remain in [Select patch history](gpui-base-select-focus-patch.md) and [Link patch history](gpui-base-link-focus-patch.md) for upstream submission. These replacements remove Kumo's dependence on those two corrections only: Textarea, GPUI tab/animation and Linux accessibility corrections have separate ownership and removal criteria. The root Base override remains for Textarea.

## Reproduce validation

Run the normal gate against the restored Select/Link source and remaining patches:

```sh
bash scripts/check-rust.sh
```

Direct tests that required the retired Base corrections were removed from the active suite; their historical references remain in the patch documents. Kumo's own focus, callback, input and availability regressions continue to run without exclusions.

Review the focused native example:

```sh
cargo run --locked -p kumo-gallery --example select_links -- --width=520
```

Use Switch theme or start with `--dark`; omit the width argument for 760 pixels. Check Select opening, disabled-row skipping, confirmation, Escape and focus restoration; Link pointer/Enter/Space activation; Toolbar arrows across the link and Tab exit; disabled/re-enabled input. The feedback count observes navigation and Toolbar activation without opening an external destination.

## Evidence and limits

The previous Toolbar implementation fails its real pointer/modifier retained-focus regression against stock Base Link; the replacement passes. The additional Select callback-focus test protects owner-selected outside focus, but also passes with the previous Kumo implementation: it is regression coverage, not a claim that this test reproduced a new failure.

Native macOS review with stock Select/Link source exercised Select confirmation and Escape, Link activation, Toolbar navigation and visible Tab exit. Light/dark layouts at 760 and approximately 520 pixels were inspected for text baselines, caret/check/external-icon centres, label offsets, sibling spacing and edge padding. Popup and joined-toolbar borders, fills, radii and focus surfaces remain consistent in these compositions. This does not establish arbitrary-descendant clipping.

The existing macOS frame-presentation issue (#51) still sometimes leaves pixels behind accessibility state until a resize. Resized frames were used for visual inspection; ordinary live-frame consistency is not claimed. Native speech and Linux platform export are separate gates.
