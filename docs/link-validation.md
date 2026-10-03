# Link acceptance contract

Current implementation: [library-owned controls](library-controls.md). The Base discussion and historical acceptance results below describe the original implementation.

Selected as a prerequisite for Banner’s compact inline action, following Badge checkpoint `895432d`. Inspect the [pinned Link source](https://github.com/cloudflare/kumo/blob/3fd5b648df578cb1ba214dedd30f475009f6a668/packages/kumo/src/components/link/link.tsx), [Link examples](https://kumo-ui.com/components/link/) and installed gpui-base 0.7.0 link.rs before implementation. The deprecated to routing alias is excluded; destinations use href semantics.

## Required branches

| Branch | Source contract / native decision | Evidence required |
| --- | --- | --- |
| Variants | Inline: link foreground/underline; Current: inherited foreground/underline; Plain: link foreground with 70% foreground on hover | Actual text/underline paint in light/dark, inherited fonts/sizing, hover entry/exit and consumer overrides |
| Typography | Inherit size/line height; underline thickness 0.0625em; source offset 0.15em; current-color decoration 35% light/65% dark and opaque on hover | Measured decoration and native inspection; document selected GPUI underline-position difference |
| Layout | Inline-flex centered children, gap0.1875em; direct Badge composition rounds the root | Narrow/long Unicode, em scaling, source-sized icon and repeated instances |
| External icon | Decorative two-path 24-unit SVG, 1em size, round caps/joins, stroke1.75 light/2 dark | Inline SVG data avoids consumer asset registration; native light/dark and complete accessible name |
| Navigation | Application-owned href target and injected open strategy; source LinkProvider/render adapt to explicit callback/content | Pointer and keyboard invoke open strategy once with destination/event; no implicit browser launch; updated href after rerender |
| Focus/availability | Kumo retains focus; GPUI provides native activation; disabled policy rejects input and tab participation | Tab traversal, visible keyboard focus, disabled/re-enabled paths, nested/repeated links |
| Accessibility | Link role, complete label and URL target; disabled metadata on actual Kumo node | Rendered role/name/action/URL checks; native tree and spoken behavior recorded separately |
| Badge integration | Caller’s typed badge builder binds hover group; only hovered ancestor badge gains ring | Real hover and pointer/keyboard navigation, repeated group names, callback ordering and no duplicate state |
| Styling | Caller refinements after recipe; application owns content and routing | Label/icon composition, nonempty names, color/layout overrides and focus paint bounds |

## Verified dependency limits and decisions to resolve

Base Link uses a keyed focus handle, exposes Role::Link and injected open_with/on_activate, and never launches a browser by itself. It activates with Enter and Space; preserving this native Base convention is a documented adaptation from browser anchors. It does not export URL or disabled bits in its current accessibility writer; its existing parent-node metadata hook can enrich the same control node without adding a duplicate Link.

GPUI UnderlineStyle exposes thickness/color/wavy but no underline offset. Its default position depends on font descent, rather than Kumo’s explicit 0.15em. Prefer the native text primitive and record this discrepancy until a precise, font-aware alternative is verified; do not substitute an arbitrary container-bottom stroke.

Named GPUI hover groups are scoped through an ancestor hitbox stack, so the source’s shared group/link name can be reused across sibling links without global hover bleed. Verify this through actual input before accepting composition.

## Implementation and measured evidence

Link exposes typed variants, an application-owned NavigationRequest, activation observation, availability, decorative content, typed Badge composition and an optional external icon. Styled refinements follow the recipe. No deprecated to alias, browser router, automatic external launch or second activation engine is introduced. SVG paths are embedded from the pinned source; consumers need no asset registration. Native layout uses self-start in column compositions to preserve content width.

Actual-render coverage exercises pointer/Enter/Space strategy-before-observer ordering, updated destinations and retained focus, disabled rejection, disable-between-key-down/up, traversal and re-enablement; inherited typography, measured underline thickness/alpha and hover entry in both themes; repeated Badge ancestor groups without hover bleed; narrow Unicode wrapping with a fixed one-em icon. Em spacing permits physical-pixel snapping. Final gate results are recorded in the progress checkpoint. The existing upstream block 0.1.6 future-compatibility notice remains.

Separate skeptical review found two correctness defects and repaired them. An extra Base ViewElement namespace initially gave the focus-paint observer a different keyed handle; rendering Base directly within Link's boundary makes the observer share the actual handle, verified by keyboard focus paint. Direct flex text overflowed a constrained root in the native gallery; a shrinkable wrapping label now preserves the one-em icon. Focus custom paint accounts for caller borders and clamps resolved radii to the actual quad. General rich content keeps caller-owned layout.

Native macOS light and dark captures on 2026-10-02 show the variant treatments, external icon, composed badges and keyboard focus. Pointer, Return and Space updated the gallery count to three navigation requests. The native tree exposed complete Link names and app:// destinations without decorative children. The corrected wrapping label was captured in light; a rebuilt dark capture did not resolve reliably during this session, so dark wrapping is currently supported by rendered tests rather than a claimed final native capture. The tree did not display an unavailable qualifier for the disabled Link despite the parent-node disabled hook; operating-system/spoken disabled-state export remains unverified. Exact pinned browser comparison and underline-offset parity remain pending.

Two gallery instances were inadvertently started through raw-binary and registered-app launch paths; subsequent inspection alternated between them. Future native checks should launch only the registered app. Native-menu Quit closed the registered instance; the remaining session-owned raw process was terminated. A final pgrep returned no gallery process.

Status: implemented with the measured coverage above; remaining browser, native capture and accessibility checks are explicit. Banner legacy children/text and legacy enum will be omitted; supported Banner.Action requires accent-aware Button recipes next.
