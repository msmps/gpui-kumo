# Dialog modal-core evidence — 2026-10-03

Pinned Kumo `3fd5b648df578cb1ba214dedd30f475009f6a668`, Kit/Base0.7.0, GPUI0.3.7 and Rust1.99.0. [Contract and remaining gaps](../../dialog-validation.md). These results cover the native modal core; #19 remains open.

## Passing observed checks

`browser/results.json` and `linux/results.json`: Light/Dark ×1040/520 pass. Each directory contains edit and alert screenshots for all four cases. The actual pinned Dialog, Button and Input source files are imported by the browser fixture; the native fixture uses the shared gallery `Dialogs` view.

Both verify editor initial focus, forward/reverse trapping, Enter in editor without implicit confirmation, rejected empty save preserving the open panel and Save focus, successful operation once, Escape/restoration, standard outside-pointer dismissal and alert outside-pointer rejection followed by delete once. Native also verifies Dropdown → edit Dialog → Escape restores the surviving menu trigger. Native standard Dialog exports its name and actual panel bounds; AlertDialog maps to AT-SPI Alert; both export Modal. Speech and announcement order are not asserted.

Panel rectangles match:512×237 at(264,64) wide;488×237 at(16,32) narrow. Caller layout uses32px padding,16px vertical gaps,24/32px title and14/21px supporting text. Input/actions are36px high with8px action gap. AT-SPI editor bounds exclude the12px horizontal input padding, so its inner rectangle is not the full painted input surface. Results retain raw values for comparison.

Visual inspection includes both-theme wide/narrow edit and alert layouts: title/input/description edges, action baselines, sibling gaps, wrapping and edge padding; panel12px corners, outside outline, fill and literal source shadows. The supported simple composition stays within the surface. Arbitrary descendants follow LayerCard's existing clipping limit. The source comparison has the same DejaVu Sans family configured, but this is not other-platform typography acceptance. The background gallery compositions differ; compare the modal and caller content.

The final `rust-gate.log` records a passing full pinned gate:218 library tests,1gallery,9doctests (7+2), adapter12default/14all-feature, workspace/adapter fmt, warnings-denied Clippy and locked all-target/all-feature/default-gallery builds. Rendered regressions additionally cover all four widths, default post-paint initial focus, disabled traversal, empty panel traversal, long scrolling, authoritative close preserving newer focus, removed opener, retained-state unmount/remount and two-Dialog topmost ordering. No version or vendor patch changes were needed. Base FocusTrapContainer drops child accessibility metadata; the core instead supplies its own Tab boundary over Base DialogPopup/Backdrop. Entrance fade is150ms; source scale/exit motion remains open.

## Reproduce

Activate the prepared environment and run `bash scripts/check-rust.sh`. Build the native fixture with `cargo build -p kumo-gallery --example dialog --locked`. Start a visible display; this run used Xvfb:100 and software Vulkan. Run:

```bash
DISPLAY=:100 XDG_RUNTIME_DIR=/workspace/.kumo-setup/runtime \
VK_DRIVER_FILES=/workspace/.kumo-setup/sysroot/usr/share/vulkan/icd.d/lvp_icd.json \
PYTHONPATH=/workspace/.kumo-setup/sysroot/usr/lib/python3/dist-packages \
GI_TYPELIB_PATH=/workspace/.kumo-setup/sysroot/usr/lib/x86_64-linux-gnu/girepository-1.0 \
dbus-run-session -- /usr/bin/python3 docs/evidence/dialog/linux/probe.py
```

The native probe uses actual OS pointer/keyboard input and reads AT-SPI focus/roles/value/bounds. It waits for the expected focus before subsequent keys, settles frames before capture and terminates every fixture process. `linux/raw-probe.log` retains the platform's dbind signature warnings separately from passing application checks; per-case application logs are also retained.

For browser reproduction, check out the pinned Kumo source at `/workspace/.kumo-reference`; install the fixture's exact lock (`npm ci` inside `browser`). Run `node docs/evidence/dialog/browser/build.cjs` and the fixture's Tailwind CLI with `-i docs/evidence/dialog/browser/input.css -o docs/evidence/dialog/browser/app.css`. Serve the repository on4184 with `python3 -m http.server 4184` and run `node docs/evidence/dialog/browser/probe.cjs`. The probe uses system Chromium through the prepared Playwright runtime. Build products and node_modules are ignored; source imports and the lock are committed.

Remaining: embedded Dropdown/Select/Popover composition, custom parts and rich triggers, nested-parent removal, scale/exit transitions, native/browser long-content acceptance, RTL, OS IME, other platforms and spoken screen readers. A passing Rust gate or these Linux/browser matrices does not resolve those gates.
