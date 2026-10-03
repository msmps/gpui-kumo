# Toolbar action/link evidence — 2026-10-03

Pinned Kumo3fd5b648df578cb1ba214dedd30f475009f6a668, Kit/Base0.7.0, GPUI0.3.7, Rust1.99.0. [Scope and native adaptations](../../toolbar-validation.md); [retained Link focus correction](../../gpui-base-link-focus-patch.md).

## Passed

[Remote macOS Rust gate37128708107](https://github.com/msmps/gpui-kumo/actions/runs/37128708107) succeeds for implementation da6ff47. The implementation SHA and local/source/native results below identify the tested tree.

- Pinned source browser fixture: Chromium151, React19.2, Base UI1.8, Tailwind4.3.3 and Phosphor2.1.10. Light/Dark1040/520 functional/geometry matrix: [results](browser/results.json), [measured source](browser/source-measurements.json), [availability presentation](browser/source-states.json), screenshots and runnable fixture/probe/lock beside them.
- Real native focused preview using the actual gallery Toolbar section: Xvfb :100, Vulkan lavapipe, D-Bus/AT-SPI and xdotool. Light/Dark1040/520: [results](linux/results.json), [probe](linux/probe.py), [raw probe output](linux/probe.log), initial/focused/removed screenshots and per-case application logs. Pointer-before-Space, disabled/loading gating, native Link focus/navigation, entry/exit/reentry, vertical/nonloop, source Home/End policy, group availability, focused removal and narrow reveal pass. After visiting the vertical Paused action, subsequent Tab entry correctly restores Paused; the probe preserves that expectation.
- Five rendered regressions cover stale owner actions/current destination/modifiers, retained actual link pointer focus, collection reorder/removal/empty focus, orientation/looping/reveal and source ghost hover/disabled opacity. [Test output](toolbar-tests.log).
- Full [Rust gate](rust-gate.log):190 library,1 gallery,9 doctests, workspace fmt/warning-denied Clippy, locked all-target/all-feature and default gallery builds; exact adapter fmt/12default/14all-feature tests/lint. [Focused example build](example-build.log). Existing dependency warnings remain visible.

## Visual review and limits

Inspected native/source wide and narrow Light/Dark screenshots for joined borders, root fill/shadow, 8px first/last corners, keyboard ring layering, icon centres, typography and padding. GearSix is the source regular14px glyph, with its license retained. Root-disabled buttons retain original source opacity and foreground. Native text/layout advances give a roughly574px intrinsic card versus568.484px browser card; this bounded difference is recorded, not accepted as exact pixel parity. The native 2px viewport reserve keeps focused Deploy fully visible at520; offscreen controls are revealed by navigation.

This is focused shared-gallery-section acceptance. Full-gallery interaction, macOS/Windows, speech/screen-reader, RTL, Input/InputGroup and popup replacement composition remain not run or unimplemented. Raw AT-SPI unknown-signature warnings are retained; queried native roles, disabled/Busy states, orientation and focus are evidence, not speech acceptance. Disabled source links become buttons; the native retained Link-role adaptation remains explicit.

## Reproduce

Activate the managed Rust/native setup, run `bash scripts/check-rust.sh`, build `cargo build -p kumo-gallery --example toolbar --locked`, then run `linux/probe.py` inside a D-Bus session with DISPLAY=:100 and the setup runtime/lavapipe/Python AT-SPI paths. The probe uses the repository path and saves its matrix beside itself.

For the browser fixture, use its committed package lock (`npm ci`), make the pinned source checkout available at the fixture's recorded reference path, run `node build.cjs` and `npx @tailwindcss/cli -i input.css -o app.css`, serve the directory on4176, and run `node probe.cjs`. The source checkout must remain at the recorded revision. This managed fixture uses the recorded system Chromium and Playwright module path; adjust only environment paths elsewhere.
