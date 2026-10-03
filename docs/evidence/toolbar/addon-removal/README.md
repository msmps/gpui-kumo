# Independent Toolbar addon removal

Pinned Kumo3fd5b648df578cb1ba214dedd30f475009f6a668; Base0.7.0, GPUI0.3.7, Rust1.99.0 unchanged. No vendor changes.

The native retained InputState now retains compact addon action handles by caller ID within start/end scopes. Rendered factories remain weak and execute once per render. Caller-provided handles win; removed handles are pruned. Removal, disabled/loading availability or replacement of a focused handle schedules recovery after rendering; a newer focus change cancels it. Toolbar recovers to the editor when eligible, otherwise to an eligible collection entry or host exit. Standalone InputGroup recovers to an available editor or host traversal. No value/selection/history mirror, new entity, or per-render subscription is added.

`browser/` imports the actual pinned Toolbar/InputGroup. Install locked fixture packages, `node build.cjs`, run Tailwind `-i input.css -o app.css`, serve port4182, then `node probe.cjs`. The fixture mounts the same220px InputGroup with passive18px Gear/Filter and compact26px Clear. Alt+c removes/reinserts only the primary trailing action addon; Alt+d controls Toolbar availability.

Source Light/Dark1040/520 preserves the retained editor value and Unicode selection while removal leaves browser BODY. The subsequent Tab target is recorded per case rather than assumed. Native recovery is a deliberate adaptation: focus the existing editor and preserve its selected 🦀, then Tab exits to the next Toolbar; reverse Tab reenters the remembered editor. Reinserted actions activate normally; root-disabled editor recovery still rejects edits.

Build `cargo build -p kumo-gallery --example toolbar_editors --locked` with setup activation. Run `linux/probe.py` under dbus-run-session on DISPLAY=:100 and the setup runtime/Vulkan/Python/GI paths from preceding evidence. The shared gallery preview exposes Alt+c and a visible action toggle. Native roles/focus/text/Sensitive queries and raw dbind warnings remain distinct from speech/OS IME acceptance.

Three new rendered regressions cover independent removal/editor retention/Tab exit, unavailable action recovery/current root policy/cancelled stale recovery, and standalone Unicode selection/caller-focus replacement. Existing passive reorder and weak-owner unmount regressions still execute.

Direct joined Toolbar actions, popup/replacement/richer composition, disabled=false group overrides, focus-visible modality parity, RTL and full-gallery/platform/speech acceptance remain open. Coverage remains29/43 working families,14 unported; #35 stays open.

Published implementation **6fccb60409a2675bae27a2398c9de51a3c109d8f** passes the [remote macOS Rust gate37134143787](https://github.com/msmps/gpui-kumo/actions/runs/37134143787).

Final source/native Light/Dark1040/520: PASS. Full locked Rust gate:201 library/1gallery/9doctests; fmt/warnings-denied Clippy/locked builds/default gallery; exact adapter12 default/14 all-feature tests/format/lint: PASS.

Alignment/layer review compares painted Gear/icon/control centres,14px editor and12px action baselines,6px addon gaps/4px end inset before removal, expanded editor edge padding after removal,220px group width, narrow reveal, joined separators/corners/surface/shadow and whole-group focus ring in both themes. The native focus indication after removal is deliberately different from source BODY. Existing source/native focus-visible modality differences remain documented by preceding evidence. No arbitrary-descendant clipping or full fidelity claim is added.

Native Unicode entry uses xclip0.13-4 from the setup sysroot and the actual Ctrl+V editing path. AT-SPI exposes readable text/selection but not EditableText in this adapter; direct xdotool supplementary Unicode synthesis was unreliable. Neither is claimed as native acceptance. Clipboard-backed entry, keyboard selection and queried retained text/selection are the passing evidence.
