# Validation fixtures

Authored browser/native probes live in `tools/validation/`. Active work and acceptance outcomes belong in [GitHub issues](https://github.com/msmps/gpui-kumo/issues). The full Rust gate is `bash scripts/check-rust.sh`; passing it does not establish browser, OS IME or spoken screen-reader acceptance.

## Browser setup

Fixtures import Kumo revision `3fd5b648df578cb1ba214dedd30f475009f6a668` from `/workspace/.kumo-reference`. Each browser folder retains `package.json` and its lockfile. In that folder, run `npm ci`, `node build.cjs`, and `npx tailwindcss -i input.css -o app.css`. Serve the resulting folder on the port expected by its probe. Inspect the probe for its Chromium/Playwright executable and absolute managed-runtime paths; adapt them for another machine. Keep the reference revision, generated CSS, font, viewport and device scale explicit in comparisons.

## Native setup

Linux probes require X11/Xvfb, a private D-Bus session, AT-SPI/PyAT-SPI, GTK introspection, xdotool and a working Vulkan renderer. Build the matching fixture under `apps/gallery/examples/` before running its probe under `dbus-run-session`. The announcement probe uses the `announcement` example; Toolbar addon probes use `toolbar_editors`; other example names match their component folder. Foundation capture scripts distinguish full gallery from the focused `foundations` example.

Scripts retain managed-workspace/runtime assumptions and are manual validation tools, not portable CI tests. Check display, renderer, dependency and Python paths before running. Generated logs, results and captures are disposable. Attach relevant failures or acceptance summaries to issues; do not commit a new chronology of successful runs.

The quick-win fixture uses macOS Metal and `Window::render_to_image`, with actual rendered pointer/Space paths. Copy it temporarily into the gallery examples, run the capture command below, inspect both themes at1040/520, then remove the temporary example. It supports interactive controls without `--capture`. Ensure foreground visibility when reviewing real presentation: an occluded macOS window can update state while suspending displayed frames.

The historical font verifier intentionally reads the issue-referenced PNGs under `docs/evidence/readable-labels/` and `docs/evidence/text-fonts/linux/`, plus the metrics-only `tools/validation/text-fonts/font-metrics.json` fixture. It requires Pillow/FreeType and fontconfig. The fixture contains only the font size, sample text and measured advances; raw logs are not retained. The capture script writes new artifacts under `tools/validation/text-fonts/linux/`.

## Fixture inventory

Run commands from the listed directory unless the command specifies the repository root. Browser build commands precede browser checks; native probes require an already built example and display/session setup.

| Directory | Entry points |
| --- | --- |
| `tools/validation/announcement` | `python3 probe.py` |
| `tools/validation/dialog/browser` | `node build.cjs`; `node probe.cjs` |
| `tools/validation/public-api/linux` | From repository root: `cargo build -p kumo-gallery --example public_api --locked`; `dbus-run-session -- python3 tools/validation/public-api/linux/probe.py --binary target/debug/examples/public_api --output /tmp/kumo-public-api` |
| `tools/validation/dialog/linux` | `python3 probe.py` |
| `tools/validation/dropdown/browser` | `node build.cjs`; `node probe.cjs` |
| `tools/validation/dropdown/linux` | `python3 probe.py` |
| `tools/validation/foundation-layout/linux` | `python3 capture-gallery.py`; `python3 capture-preview.py` |
| `tools/validation/pagination/native-browser/browser` | `node build.cjs`; `node check.cjs` |
| `tools/validation/quick-wins` | copy `review.rs` to `apps/gallery/examples/quick_win_review.rs`, then `cargo run --locked -p kumo-gallery --example quick_win_review -- --capture` |
| `tools/validation/tabs/edge-opacity/linux` | `python3 probe.py` |
| `tools/validation/tabs/linux` | `python3 probe.py` |
| `tools/validation/tabs/overflow-motion/browser` | `node build.cjs`; `node probe.cjs` |
| `tools/validation/tabs/overflow-motion/linux` | `python3 probe.py` |
| `tools/validation/text-fonts` | `python3 verify.py` |
| `tools/validation/text-fonts/linux` | `python3 capture.py` |
| `tools/validation/toast/browser` | `node acceptance.cjs`; `node alignment.cjs`; `node build.cjs`; `node extract-icons.cjs`; `node probe.cjs` |
| `tools/validation/toast/linux` | `python3 alignment.py`; `python3 probe.py`; `python3 workflow.py` |
| `tools/validation/toolbar/addon-actions/browser` | `node build.cjs`; `node probe.cjs` |
| `tools/validation/toolbar/addon-actions/linux` | `python3 probe.py` |
| `tools/validation/toolbar/addon-removal/browser` | `node build.cjs`; `node probe.cjs` |
| `tools/validation/toolbar/addon-removal/linux` | `python3 probe.py` |
| `tools/validation/toolbar/browser` | `node build.cjs`; `node probe.cjs` |
| `tools/validation/toolbar/editors/browser` | `node build.cjs`; `node probe.cjs` |
| `tools/validation/toolbar/editors/linux` | `python3 probe.py` |
| `tools/validation/toolbar/linux` | `python3 probe.py` |

## Review requirements

Inspect Light/Dark and wide/narrow layouts, pointer/keyboard/focus/owner-update paths, glyph/control centres, text baselines, sibling gaps and edge padding. Review rounded corners and nested fill/border/focus/shadow agreement. Follow [verification](gpui-verification.md); static captures alone do not establish motion, arbitrary-descendant rounded clipping or platform speech. Preserve explicit limitations and meaningful negative controls.

## Maintained Linux regression probes

Run from the repository root under `dbus-run-session` with DISPLAY and native dependencies available. Pass `--binary target/debug/kumo-gallery`; PageSize/Dropdown also accept `--output /tmp`. Inspect each probe's `--help` for supported arguments.

| Probe | Active scope |
| --- | --- |
| `tools/validation/linux/validate-input-accessibility.py` | [#49](https://github.com/msmps/gpui-kumo/issues/49): live Unicode text reads, selection/caret, editing gates and clipboard behavior |
| `tools/validation/linux/validate-pagination-dropdown.py` | [#26](https://github.com/msmps/gpui-kumo/issues/26): Dropdown owner proposals, focus/modes, collision and native paint; use `--require-expansion-state` for authored disclosure export |
| `tools/validation/linux/validate-pagination-page-size.py` | [#26](https://github.com/msmps/gpui-kumo/issues/26): PageSize owner proposals, focus, option collection and popup paint; use `--require-expansion-state` for authored disclosure export |

These are manual acceptance tools. They establish exported metadata and actual input/focus behavior, not screen-reader speech or OS IME acceptance. The one-off Busy, Pagination baseline and readable-label before/after scripts are retired; completed repairs retain their Rust regressions and immutable issue evidence.

The static Loader browser reference is `tools/validation/loader/loader-reference.html`; use it for fresh arc/track/reduced-motion comparisons.

The `public_api` fixture also exercises Field metadata transfer and feedback: F7 toggles an outer Field error, F10 changes retained control availability, and F11 removes the focused endpoint action. The endpoint optional indicator, label tooltip and helper originate on InputGroup and transfer into Field. Verify native names/descriptions plus visible borders/messages; changing accessibility alone is insufficient when presentation is occluded.
