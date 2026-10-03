# Toast core evidence — 2026-10-03

Pinned Kumo `3fd5b648df578cb1ba214dedd30f475009f6a668`, Kit/Base0.7.0, GPUI0.3.7, Rust1.99.0. [Contract and remaining acceptance](../../toast-validation.md). These are bounded core results, not full-family or spoken announcement acceptance.

## Results

- Pinned browser audit `browser/results.json`: Light/Dark1040/520 pass stable manager/in-tree dedupe differences, functional updates, source geometry, F6/reverse Tab.
- Browser `acceptance-results.json`: four cases pass five variants, close geometry, once-only action without dismissal, hover pause and expiry after exit.
- Native Linux `linux/results.json`: four cases pass nonmodal Dialog, measured geometry, F6/producer restoration, focus/hover pause past5s, stable update, five variants, action once and expiry after pointer exit.
- Eight native rendered/input tests pass, including current availability before repaint, focused action removal/order, repeated mount cleanup and host window close with retained application state.
- Full locked Rust gate `rust-gate.log`:226library/1gallery/9doctests and adapter12/14 pass, workspace/exact adapter formatting and warnings-denied Clippy, all-target/all-feature and default gallery builds. Existing upstream profiler deprecation warning remains visible; no vendor changes.

Normal rows export AT-SPI Dialog without Modal; Notifications exports landmark. Authored polite live-region metadata is present in GPUI; AT-SPI attributes are empty, and actual speech has not been validated. Source hides Close from accessibility until expanded/focused; native keeps Close exposed.

## Visual review

Compare the five variant PNGs in each directory in both themes and1040/520 widths. Source/native root is340×76 at(668,692) wide and488×76 at(16,708) narrow. Icons16px align at title top+2 with8px text gap; title/description baselines have20px line height and4px gap, surface padding16. Default has no icon. Close20px sits8px from top/right with12px centered X. Rounded tint children match12px surface corners; outside ring/shadow follow the same curve. Source text accents and ring accents are separate token sets.

GPUI's physical-pixel snapping requires coverage compensation for variant0.3px rings. Source/native blur and glyph rasterization still differ; these screenshots establish geometry/color/alignment rather than pixel identity. Source current-color/15% Close hover, source CSS/swipe/bump motion, full stack guards/content scaling and arbitrary custom composition remain open.

## Reproduce

Browser dependencies are pinned in `browser/package.json`. Build the pinned-source fixture with `node build.cjs` and its Tailwind CLI4.3.3 `-i input.css -o app.css`, then serve the directory on4185. `node probe.cjs` and `node acceptance.cjs` use Playwright/Chromium (paths documented in those scripts). Original source audit imported the pinned implementation directly; build products/node_modules remain ignored. Icon export provenance and MIT license are in `crates/gpui-kumo/assets/toast-icons.md` and `toast-icons.LICENSE`.

Native: source `/workspace/.kumo-setup/activate.sh`, build `cargo build -p kumo-gallery --example toast --example dialog --locked`, and run `linux/probe.py` in an accessibility-enabled D-Bus session on Xvfb100 with the configured Lavapipe driver. The script records real pointer/key activation and AT-SPI state. `linux/workflow.py` separately checks the composed document flow. Raw runtime/adapter diagnostics are preserved rather than hidden. Run the full Rust gate with `bash scripts/check-rust.sh`.

## Dense-content / composed-workflow continuation

Source `browser/alignment-results.json` and native `linux/alignment/results.json` pass Light/Dark1040/520. Action surfaces126px high and long two-action surfaces206px wide /186px narrow agree. The PNGs show consistent title/description baselines, source icon offset,8px action gap and16px surface padding; buttons stay within surface bounds. Native rounded tint/ring layers remain aligned. Arbitrary content and overflowing action sets are separate gaps.

`linux/workflow/results.json` passes four cases for actual Dropdown → Dialog → validated application operation → Toast. Screenshot/logs retain the authored diagnostic host at that run; the focused announcement host is recorded in [announcement evidence](../announcement/README.md). Published Toast core94351f4 passes [macOS37138801443](https://github.com/msmps/gpui-kumo/actions/runs/37138801443).
