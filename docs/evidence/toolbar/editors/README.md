# Toolbar retained editor evidence — 2026-10-03

Pinned Kumo3fd5b648df578cb1ba214dedd30f475009f6a668, Kit/Base0.7.0, GPUI0.3.7 and Rust1.99.0. [Contract and native adaptations](../../../toolbar-validation.md#retained-editor-continuation--2026-10-03). No dependency/version/vendor changes in this continuation.

## Results

- Source Chromium151 fixture with the previous exact React19.2/Base UI1.8/Tailwind4.3.3/Phosphor2.1.10 lock: [four-case results and measured geometry](browser/results.json), initial/focused/removed screenshots and fixture/build/probe beside them. Light/Dark1040/520 pass caret edges, Unicode selection, disabled focus/skip/edit guard, one Tab exit/reentry, read-only and root availability updates, vertical navigation and removal observation. Source intrinsic root width788.40625, height36, font14, unavailable opacity1; icon18px. This fixture reuses actual pinned Kumo source, not a recreated Toolbar.
- Native focused example using the shared gallery editor section: [four-case results](linux/results.json), [probe](linux/probe.py), [raw output](linux/probe.log), initial/focused/removed screenshots and per-case app logs. Xvfb :100/lavapipe, D-Bus/AT-SPI and xdotool provide actual pointer/key input. Unicode selection, current edit/selection guards, skipped editor, narrow grouped-editor reveal, one Tab exit/reentry, Change/Submit once, read-only updates, root availability, vertical boundaries, focused removal/traversal and retained value on reinsertion pass.
- Four new Toolbar rendered tests plus Input marked-composition and ordinary caller Tab-policy tests. [Nine total Toolbar regressions](rendered-tests.log) include the previous action/link checks. Current-owner clipboard rejection deliberately invokes a retained action before rerender; available read-only copying still works. The full gate also exercises marked composition through the same native Base engine, without establishing OS IME acceptance.
- [Full Rust gate](rust-gate.log):196library/1gallery/9doctests, workspace fmt, warning-denied Clippy, locked all-target/all-feature/default gallery builds; exact adapter fmt/12default/14all-feature tests and lint. [Focused example build](example-build.log). Existing dependency warnings remain visible.

## Review and repaired failures

Inspected source/native initial and focused Light/Dark1040/520 for glyph/baseline alignment,18px GearSix centres, addon/editor spacing,36px heights, separator seams, first/last corners, source surface/shadow and lifted focused InputGroup ring. Native bounded scrolling reserves2px and reveals the actual focused editor; semantic editor bounds exclude facade padding/addons. Fractional browser and native integer text advances differ slightly; screenshots do not establish pixel-identical parity. Trailing addon and suffix recipes are visible in the vertical and disabled rows respectively.

The validation loop caught and repaired two implementation errors: raw key capture missed Base actions dispatched first (including reverse movement through an unavailable editor and selection mutation), and Base disabled rendering dimmed source-opaque editor text. Action captures and the existing Base read-only edit guard address those without a second editing engine. Disabled semantics remain authored on the facade and owner props remain unchanged. Clipboard-before-rerender coverage protects the same boundary.

The native probe now waits200ms after confirmed focused/removed states before capture; earlier screenshot samples could show the prior presented frame. State/value checks still await their actual queried targets. Source removes a focused editor to BODY; native deliberately recovers to an eligible control. The native externally retained InputState also survives reinsertion. Source/native removal behavior is recorded separately, not forced into a false equivalence.

## Remaining acceptance

Embedded/direct InputGroup actions, richer field/descendant and popup replacement composition, RTL, full-gallery interaction, macOS/Windows native rendering, speech/screen-reader and OS IME remain open. The shared gallery section is compiled/integrated and exercised in a focused host; that is not full-gallery interaction acceptance. Raw AT-SPI unknown-signature warnings are retained. Queried roles/focus/value/disabled/editability/orientation are platform metadata evidence, not speech acceptance. Coverage remains29/43 working families,14 unported.

## Reproduce

Activate the managed setup and run `bash scripts/check-rust.sh`. Build `cargo build -p kumo-gallery --example toolbar_editors --locked`; then run `linux/probe.py` in a D-Bus session with DISPLAY=:100 and the setup runtime/lavapipe/Python AT-SPI paths. It writes its matrix beside itself. The example supports `--dark`, `--width=520`, Alt+t collection removal/reinsertion, Alt+d root availability and Alt+r owner read-only.

For the source fixture, `npm ci`, keep the Kumo checkout at the recorded absolute path and revision, run `node build.cjs` and `npx @tailwindcss/cli -i input.css -o app.css`, serve on4180 and run `node probe.cjs`. Its recorded Chromium and Playwright paths are environment paths, not source/dependency substitutions.
