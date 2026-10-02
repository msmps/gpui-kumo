# InputGroup parity and continuation

Pinned Kumo source/docs, all part files, context spacing recipes, demos and tests inspected; installed Base InputBase/Input source verified. No Base InputGroup exists. Reuse the existing retained InputState and its Base editor, selection/clipboard/IME and accessibility bridge. No editing entities or subscriptions are recreated for addons.

| Area | Shared-container acceptance / status |
| --- | --- |
| API | InputGroup over retained state, source xs/sm/base/lg, passive start/end text or icon addons, existing label/helper/error presentation implemented. Rich addon content, compact buttons, suffix and individual/hybrid focus modes pending |
| Geometry | Source actual heights24/28/36/44; addon outer6/6/8/10, input seam4/6/8/10 and icon10/13/18/20; existing source radius/text recipes reused. Group fills parent; source standalone heights remain unchanged |
| Editing | Unicode input/backspace, existing state across sizes/themes, pointer addon focus and disabled edit rejection tested. Existing selection/clipboard/accessibility regressions still pass. Group-specific native IME/platform speech not run |
| State | InputState owns availability/value/focus; group has presentation only. Error state supplied by owner. Hover/pressed/selected/loading/open N/A for passive adornments |
| Layers | Content row clips long passive addon text; ring paints outside that mask. Arbitrary rounded descendant clipping remains GPUI limitation; passive text/icons have no corner-covering background. Do not extend this claim to rich children |
| Edge cases | Narrow52px rendered bounds stay constrained; an oversized nonshrinking addon can consume all editor room, as the source recipe does. Native128px long-addon capture shows no bleed into adjacent content |
| Themes | Native light/dark and520px dark inspected: baseline/control centres, padding, seams, rounded surfaces, retained typed value |
| Semantics | Existing Base text-input accessible name/value/invalid/disabled bridge retained. Documentation claims automatic group role, but pinned Root source does not set it; no invented duplicate input/group semantics |

Review found missing content clipping and repaired it with a separate masked row, preserving ring paint.89 workspace tests/eight doctests, formatting, warning-denied all-target/all-feature Clippy and all-target/all-feature locked build pass. Evidence: [light](evidence/input-group-light.png), [dark](evidence/input-group-dark.png), [narrow dark](evidence/input-group-narrow-dark.png). Native typed users survives theme/resize. Use window-relative xdotool coordinates: window placement varies; absolute desktop coordinates can silently miss the target. Screenshot labels alone do not establish theme or callback state—inspect pixels/header/counters.

Next coherent milestone: inline Suffix, measuring current Base value/placeholder with the matching text style and constraining editor width while preserving selection/IME. Then compact addon Buttons and shared focus-within, individual/hybrid border zones, rich addon composition, Field required/optional parity and Tooltip dependency. Keep this component marked partial until these contracts and validation are delivered.
