# Popover validation · 2026-10-01

The native Popover now flips at window edges, offers Kumo's optional arrow, and fades in over 150ms with reduced-motion support. Its lifecycle, rendering and accessibility were checked against the pinned browser reference and the native Linux application.

## Reference and measurements

Reference source: Cloudflare Kumo commit `3fd5b648df578cb1ba214dedd30f475009f6a668`, modern `Popover.Positioner` / `Popover.Popup` / `Popover.Arrow` composition. A separate fixture imports that exact source and its CSS, rather than a hand-written approximation. The reference repository's root lockfile only pins its package manager, not component dependencies. The fixture explicitly pins Base UI 1.8.0, React/React DOM 19.2.4, `cn` 0.4.0, Tailwind/CLI 4.1.17 and esbuild 0.27.2. Playwright drives Chromium 151.0.7922.173 at a 1040×800 viewport. Browser defaults and native font choices remain distinct.

| Property | Browser measurement | Native implementation/check |
| --- | --- | --- |
| Explicit surface width | 280px | 280px; rendered geometry tests |
| Radius | 8px | Theme `lg`, 8px |
| Padding | 12px vertical, 16px horizontal | Same tokens |
| Body size / line height | 13px / 15.2941px | Same theme metrics |
| Default gap | 8px below trigger | Rendered geometry tests |
| Outline | 1px, offset 0 light / −1px dark | Outside/inside outline canvas; screenshots |
| Surface | White light / neutral OKLCH 17% dark | Kumo base colors |
| Collision | Bottom flips to top near bottom edge | All four opposite-side flips tested; moving trigger tested |
| Shadow | Two CSS layers: 4/6/−1 and 2/4/−2px, 10% black | Authored shadow; GPUI lacks CSS spread, a documented paint difference |
| Motion | 150ms scale 90% + opacity on enter/exit | 150ms linear opening opacity; immediate dismissal; reduced-motion tests |

Browser assertions pass for width, padding, radius, body size, gap, outline offsets, bottom-to-top flipping, Escape returning focus to the trigger, and outside dismissal preserving the clicked control's focus. Native rendered-input tests establish the corresponding lifecycle and positioning behavior. This is a measured geometry/behavior comparison, not a whole-image pixel-equivalence claim. Fonts, shadow rasterization, CSS spread, motion and collision strategies can differ.

## Native checks

Rust tests run through actual element layout, paint and input dispatch. The collision test puts the trigger near all four edges, asserts the resolved side and exact 8px separation, and moves an already-open trigger. The motion test samples painted background opacity at 0/75/150ms and verifies an immediately opaque reduced-motion opening. The nested dismissal test also asserts independent child fade progress and fully opaque settling, preventing a panel from remaining invisibly mounted in a deferred tree. Existing tests cover nested dismissal, editing retention, weak-owner cleanup, outside-focus preservation and single activation.

The gallery is also exercised on Debian 13 using Xvfb and software Vulkan. A private D-Bus session enables `org.a11y.Status.ScreenReaderEnabled`; Python AT-SPI bindings inspect the actual AccessKit-exported tree and invoke its accessible click action. These checks validate accessible activation, the named dialog, focused editable entry, updated trigger descriptions, nested and parent Escape dismissal, trigger focus restoration, retained edits, and light/dark rendering. Native screenshots inspect retained edits and the optional arrow and its surface seam, including a right-side nested panel. AT-SPI cache-signature warnings require clearing the client's cache before each tree read; assertions use live queries.

Validation commands for the Rust workspace:

```sh
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo build -p kumo-gallery
```

## Visible side-flipping evidence

The runnable native example places the trigger at the selected window edge. These are unedited captures of its 640×480 X11 window on 2026-10-01. Each popup was opened through the trigger's actual accessible click action. Live AT-SPI component bounds independently verify the observed side and gap:

| Requested side | Observed side | Measured gap | Native capture |
| --- | --- | --- | --- |
| Bottom | Top | 8px | [Bottom → top](evidence/popover-flip-bottom-to-top.png) |
| Top | Bottom | 8px | [Top → bottom](evidence/popover-flip-top-to-bottom.png) |
| Left | Right | 8px | [Left → right](evidence/popover-flip-left-to-right.png) |
| Right | Left | 8px | [Right → left](evidence/popover-flip-right-to-left.png) |

[Raw measurements](evidence/popover-flips.json) include trigger/popup bounds and capture environment. For example, the bottom-requested trigger spans y=428–464 while its popup spans y=316–420: the popup is entirely above the trigger with an 8px gap. The right-requested trigger starts at x=490 while its popup ends at x=482, proving a left-side placement with the same gap.

Reproduce using [the edge example](../apps/gallery/examples/popover_edges.rs):

```sh
cargo run -p kumo-gallery --example popover_edges -- bottom
```

Click the edge trigger. Replace `bottom` with `top`, `left` or `right` to inspect the other cases. Resize the window while open to exercise remeasurement and collision fitting.

## Concrete limits

The selected `accesskit_atspi_common` 0.19.1 `NodeWrapper::state` omits expanded/collapsed state mapping. GPUI exposes `aria_expanded`, and tests verify that metadata, but the Linux bus does not receive the corresponding state flag. The trigger's updated “Expanded nonmodal dialog” / “Collapsed nonmodal dialog” description provides a verified descriptive fallback. Fixing the actual flag requires an upstream adapter change or a maintained dependency patch.

The Input entry exposes its role, name and focus through AT-SPI, but the current Base editing element supplies no accessible text-run subtree, so the adapter does not expose the AT-SPI Text interface. Its `aria_value` metadata alone does not establish screen-reader text editing. Restoring accessible text ranges and editing actions requires an Input-specific accessibility bridge; this Popover validation does not claim that capability.

GPUI 0.3.7 exposes transformations for SVG sprites, not a general affine transform for a complete interactive subtree. The native motion contract uses a fade and removes controls immediately on dismissal, avoiding retained invisible focus targets. It does not reproduce the browser's scale animation or exit fade.

AT-SPI tree/actions/focus checks do not establish spoken announcements or screen-reader usability. VoiceOver requires macOS, Windows screen readers require Windows, and neither platform is available in this Debian environment. Manual spoken-reader navigation and additional platform validation remain explicit acceptance checks. Button/Input browser pixel comparisons are outside this Popover comparison.
