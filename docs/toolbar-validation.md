# Toolbar action/link slice contract

Pinned Kumo3fd5b648 Toolbar source, browser tests, documentation and Base0.7.0 Toolbar/Button/Link were inspected. Supported Kumo parts are Root, Button, Link, Input and InputGroup; Root wraps explicit controls. Size customization is deprecated and omitted. Kumo exports neither ToolbarGroup nor Separator; grouping is the joined card and per-item border. Base UI default loopFocus=true and enableHomeAndEndKeys=false are verified in the installed comparison baseline.

This slice implements Button/Link in a retained typed collection. The editor continuation below implements Input/InputGroup. Select/Combobox trigger replacement and richer render composition remain next slices under #35. The caller retains a ToolbarState entity; stable IDs preserve focus through reorder and replacement. Buttons expose quiet base presentation, label/decorative icon, disabled/loading and focusableWhenDisabled (source defaulttrue); unavailable controls reject pointer/key/accessibility activation even when focusable. Links emit application-owned navigation requests with actual modifier/input data; they never launch a browser automatically. Source root disabled propagates to Buttons, while Toolbar.Link's source metadata intentionally ignores group disabled; preserve that distinction.

Toolbar semantics/name and orientation come from Base. Native retained navigation fills verified Base gaps: one roving Tab entry, last-focused reentry, collection changes and skip unavailable controls when configured. HorizontalLeft/Right and verticalUp/Down navigate; looping is configurable. Source Home/End is disabled and left to the host. The Kumo source card stays visually inline even with vertical semantic/navigation orientation; native vertical orientation does not invent a column recipe. RTL remains a separate composition gap for this first slice. Standard modified arrow keys are left to the host. Empty/all-skipped collections leave focus forward without trapping it.

Presentation target:36px controls, source ghost Button geometry,36px icon-only square, rootcontrol surface/8px corners/shadow-xs/outside1pxline ring,1px joined separators, outside keyboard focus ring lifted after sibling paint. Child hover fills round only supported first/last corners. Bounded consumers use a horizontal scroller at narrow widths; arbitrary descendant clipping is not accepted.

Acceptance: actual pointer focus before Space/Enter; disabled-focusable navigation without activation; skipped-disabled/loading behavior; link navigation without duplicate events; Tab exit/reentry, both orientations/looping, owner changes before stale activation, reorder/removal/empty/repeated collections; source browser measurement and native Light/Dark1040/520 input/semantic/visual matrix. Full Rust fmt/tests/lint/locked builds and affected preview must pass before a working-family count advances.

## Measured source and native boundary

The pinned browser fixture records 36px control heights, 14px/21px typography, 12px horizontal padding, 6px icon gaps, 8px end corners and a 36px icon square. The native decorative GearSix SVG uses the source Phosphor regular geometry at 14px (one em), with its MIT license retained. Fractional browser text advances and native integer layout produce small width differences; screenshots and measured bounds are preserved rather than claiming pixel identity.

Root disabled controls activation without changing each Button's original presentation: Refresh stays opaque, originally disabled Paused uses 0.5 opacity, and loading Saving stays opaque. Source ghost hover retains the tint even on focusable unavailable actions. A rendered paint regression checks both tint and original disabled opacity. Standard Button recipes remain unchanged through a private Toolbar hook.

The narrow native viewport reserves 2px around the inline card for focus/ring visibility and reveals the actual retained focused control after layout. This is a bounded native adaptation, not source arbitrary descendant clipping. Disabled links retain the native Link role and leave traversal; source disabled Link renders a Button. That role/composition difference remains explicit acceptance work.

Published Base0.7.0 Link silently replaced a caller-supplied focus handle during render. The [narrow retained-focus patch](gpui-base-link-focus-patch.md) preserves it, so pointer activation, roving navigation and native accessibility report the same node. Versions remain pinned; downstream Cargo roots must retain the documented override.

## Remaining #35 work

The editor continuation below completes plain Input/InputGroup editing and caret-boundary navigation. Popup trigger replacement, direct joined actions, richer descendant composition, RTL, full-gallery interaction, additional platforms and speech acceptance remain open. This action/link milestone counts a working family, not complete Toolbar fidelity. [Reproducible evidence](validation-fixtures.md).

## Parts and parity matrix

| Source capability | Native action/link slice | Acceptance boundary |
| --- | --- | --- |
| Root | Named Base Toolbar, retained collection, joined surface, horizontal/vertical semantics, loop option, group disabled | LTR; arbitrary descendant composition and RTL pending |
| Button | Quiet base, label/icon, icon-only, disabled/loading, source focusableWhenDisabled default | Deprecated size customization omitted; disabled focusable actions remain inert |
| Link | Native Base Link, typed navigation with current destination and real modifiers | Disabled-role adaptation recorded; browser launch is application owned |
| Input | Existing retained editor, source boundary arrows, availability and native semantics | Popup replacement and OS IME/speech remain separate |
| InputGroup | Start/end text/icon/parts, independently tabbable compact actions and suffix over the retained editor | Direct actions, richer field and popup composition pending |
| Group/Separator | N/A as public source parts | Source exports only Root/Button/Link/Input/InputGroup; native per-item border matches joined grouping |
| Entry/exit and collection | One Tab stop, last focus reentry, source arrows/loop, stable IDs and recovery | Home/End not enabled by source; modified arrows left to host |
| Motion | Existing Button loading indicator | No invented Toolbar transition; platform/reduced-motion composition follows hosted controls |

## Retained editor continuation

`ToolbarItem::input(id, &input_state, width)` mounts one existing single-line InputState. `input_group` adds text/icon/parts/action start/end addons and the existing suffix recipe. Width is the whole joined item including its separator; the default base size remains36px/14px. The editor owner keeps name, value, placeholder, read-only/disabled state, selection, history and InputEvent Change/Submit notifications. The Toolbar does not mirror text or create editor entities during render. One editor entity mounts once; stable IDs preserve the actual editor focus handle on reorder, while replacing the editor entity under an ID focuses the new handle. Collection-owned subscriptions are rebuilt on collection changes, not render.

Source composite navigation is inspected and measured: plain horizontal Left at the start and Right at the end (vertical Up/Down at the respective text boundaries) can move out only with a collapsed selection. Shift/modifier operations and existing selections remain in the editor. Native marked composition additionally stays in the editing engine. Home/End keep native caret behavior. Unavailable Inputs remain focusable by source default; explicit false skips them. Root disabled includes Inputs/InputGroup editors while links preserve their separate policy.

The source browser confirms `aria-disabled=true`, DOM disabled=false, opacity1 and default foreground for unavailable Toolbar Inputs. Native behavior therefore uses Base's existing read-only edit guard internally, authors disabled semantics at the facade, and intercepts the source's unavailable keyboard/pointer operations. It does not mutate the owner's `is_disabled`/`is_read_only` props. Editing availability is synchronized immediately for owner/root changes; accessible SetValue/ReplaceSelectedText also read the effective guard. Ordinary Input remount restores its own policy. Source/native actual input and disabled selection rejection are recorded.

GPUI dispatches bound Input actions before raw key handlers. Root action captures handle the four movement actions before Base, yielding to its caret/selection engine inside text; raw keys remain the fallback for other controls. Unavailable editor captures gate selection, clipboard, editing and submit actions while Tab remains host owned. This fixes the observed reversed navigation through a disabled editor and selection mutation that a raw-key-only implementation missed. The ordinary Input action policy remains unchanged.

Shared gallery and focused editor preview cover horizontal/vertical/disabled roots, plain Inputs, icon/text addons, trailing text and suffix, owner read-only/availability updates, removal and reinsertion. [Editor evidence](validation-fixtures.md) distinguishes source focus-on-removal (BODY) from native recovery to an eligible control, retained value after native reinsertion, and frame settling before screenshot capture. Six added rendered regressions cover Unicode selection, owner/clipboard guards, collection/vertical/narrow/remount behavior ordinary caller Tab policy and actual marked composition through the same Base engine. These are native adaptations and bounded composition evidence, not browser-identical focus removal or OS IME/speech acceptance.

Remaining Toolbar work is now popup trigger replacement, direct InputGroup actions, richer field/descendant composition, RTL, full-gallery interaction and additional platform/speech acceptance. Coverage stays29/43 working families,14 unported.


## Compact addon action continuation

`start`/`end` now accept existing InputGroupAddon button factories. Actions retain independent Base focus and Tab stops; they are not Toolbar collection entries. Toolbar root disabled guards only the editor, leaving actions enabled and sensitive. InputState owner disabled gates the whole group and skips its editor during roving navigation. Group semantic disabled state follows that owner, rather than the Toolbar override. Existing InputGroup action disabled-OR policy remains; source explicit-false override is a documented gap.

Arrow navigation from an action uses the remembered composite entry (initially the first eligible entry); caret boundary restrictions apply only when the actual editor owns focus. Hosted-focus containment is used for viewport reveal, reorder retention and whole-group removal recovery. A removed focused group recovers to a remaining entry; Tab then exits. Independent addon removal is now covered by the continuation below.

The shared gallery editor preview now includes compact Clear actions and owner availability toggles (Alt+g). Two new rendered regressions cover pointer focus before Space/Enter, once-only activation, availability distinctions, initial/remembered entry navigation, reorder and focused group removal/Tab exit. [Browser/native evidence and reproduction](validation-fixtures.md).


## Independent addon lifecycle continuation

Compact action factories retain focus handles by caller ID within their start/end addon scope. Passive parts can reorder without moving action identity. Focused removal, disabled/loading availability and caller-focus replacement recover to the existing editor after render, preserving value and selection. Recovery is cancelled if the user moves focus or the same provided handle becomes available again. Caller IDs must be unique within each addon. Stale handles are pruned; factories keep their weak-capture contract.

Toolbar delegates recovery to its current eligible editor/collection entry or host exit. Standalone InputGroup also recovers to the editor when available. Source removal leaves BODY; native editor recovery is deliberate and does not remove or disable supported action functionality. Actions remain visible by default; the gallery's Alt+c/action toggle exercises an owner removing/reinserting a slot.

Three new rendered regressions cover independent action lifecycle, current root availability, cancelled stale recovery, preserved Unicode selection, caller-focus replacement and same-handle reinstatement. [Pinned browser/native evidence](validation-fixtures.md). Next composition work is direct joined actions, then popup/replacement composition.
