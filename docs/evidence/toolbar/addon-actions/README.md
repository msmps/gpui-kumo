# Toolbar compact addon actions

Pinned source Kumo3fd5b648df578cb1ba214dedd30f475009f6a668; Base0.7.0, GPUI0.3.7, Rust1.99.0 unchanged.

`browser/` imports the real pinned Toolbar/InputGroup, with locked dependencies. Build with `node build.cjs` and Tailwind CLI (`-i input.css -o app.css`), serve port4181, run `node probe.cjs`. Linux uses the gallery's actual shared ToolbarEditors section and focused preview; activate `/workspace/.kumo-setup/activate.sh`, build `cargo build -p kumo-gallery --example toolbar_editors --locked`, then run `linux/probe.py` under dbus-run-session on DISPLAY=:100 with the setup runtime/Vulkan/Python/GI paths used by preceding editor evidence.

Browser Light/Dark1040/520 passes independent action Tab focus, pointer focus before Space/Enter, root-disabled editor metadata with enabled action, owner-disabled editor/action and traversal skipping, remembered-entry arrow navigation, and focused group removal. Removal leaves browser BODY; subsequent Tab reaches After. Native intentionally restores a remaining Toolbar entry and exits to the next group on Tab.

Compact Clear is26px tall with12px text, horizontally inset4px and vertically centred in the36px group. Existing addon factories retain caller IDs and weak ownership. Root availability applies only to the editor; group-owner availability applies to both editor and actions. No second editor value or engine is introduced.

Remaining composition includes direct joined actions, independent addon removal while retaining its editor, popup/replacement composition, explicit disabled=false overrides inside a disabled group, RTL, full-gallery/platform/speech acceptance. The existing native InputGroup disabled OR policy is preserved. Toolbar#35 stays open; catalog coverage remains29/43 working families,14 unported.

Native pointer validation exposed an accessibility notification made during paint: a programmatic clear changed the retained model but the synthetic text run remained stale until another input event. Input now defers that weak-owner notification until after paint. The native probe confirms empty text before any subsequent key, then checks the visible operation count for once-only pointer/Space/Enter, including root-disabled editor composition. No vendor patch is added.

Visual review compares actual Clear text/control centre lines,18px Gear glyph,14px editor baseline,12px action baseline,4px end inset,6px addon spacing, narrow reveal, group separators and outer corner/ring/shadow layers in both themes. Intrinsic font advances remain host dependent. Source focus-visible persistence after pointer/keyboard modality changes and native last-input focus-ring policy remain a fidelity difference; complete focus-visible parity is not claimed. Raw dbind unknown-signature warnings are retained; native roles/focus/sensitive/text observations are evidence, not speech acceptance.

Final Linux matrix Light/Dark1040/520: PASS. Full locked Rust gate:198 library,1 gallery,9 doctests; workspace formatting, warning-denied Clippy, all-target/all-feature and default gallery builds; standalone adapter12 default/14 all-feature tests/format/lint: PASS. See `rust-gate.log` and `example-build.log`.
