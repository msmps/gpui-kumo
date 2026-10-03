# Toast source contract — announcement feedback slice

Status: working native core; full family acceptance remains open in #34. Pinned Kumo `3fd5b648df578cb1ba214dedd30f475009f6a668`, Kit/Base0.7.0, GPUI0.3.7 and Rust1.99.0 remain authoritative.

Inspected complete `toast.tsx`, its regressions and demos, installed Base Toast/ToastManager/ToastStack/ToastMotion and browser Base UI1.8.0 provider/store/root/viewport behavior. Actual pinned browser Light/Dark1040/520 probe verifies dimensions, external/in-tree dedupe, functional updates, F6 and reverse-Tab restoration. The native acceptance scope and explicit adaptations are recorded below.

| Contract | Actual source authority |
| --- | --- |
| Provider/manager | Toasty/ToastProvider; optional external manager; add/update/close and promise loading/success/error; stable IDs and ordered stack |
| Viewport | Bottom-right,340px wide with32px right/bottom at≥640px; below640px width viewport−32px,16px edges |
| Surface | Actual `toastVariants`:12px corners,16px padding, shadow-lg, outside line ring1px default or variant ring0.3px; layered tint background. Metadata's300px width/8px radius is not authoritative |
| Text | Actual title15.6/20px medium; description14.8/20px default foreground at70% opacity;4px content gap |
| Variants | Default, success, error, warning, info; source filled Phosphor icon16px at2px top offset and8px row gap; default has no icon; variant title/icon/close colors |
| Close | Source ghost square20×20 at top/right8px,12px X glyph,4px radius; variant-aware hover tint |
| Actions/custom content | Caller-provided button props or rich content; actions do not automatically dismiss |
| Lifecycle | Provider defaults5000ms timeout and limit3; timeout0 means persistent; hover/focus and inactive window pause timers; update preserves current order, conditional timeout resets |
| Dedupe | External duplicate add merges payload through Base UI and resets timeout. After a render, in-tree duplicate add bumps existing content and preserves payload; ending duplicates are suppressed by Kumo's in-tree wrapper. Do not replace those differences with a guessed common policy |
| Semantics | Notifications region: polite live, non-atomic, additions/text. Normal toast is focusable nonmodal Dialog; high-priority toast uses AlertDialog with separate announcement behavior. Base Toast forces Alert, so its default role is not source parity |
| Keyboard | F6 focuses Notifications and pauses timers; reverse Tab from viewport restores previous target; Escape closes the focused toast; stack focus entry/exit guards are source behavior |
| Stack/motion |12px peek/gap,10% scale per rank,500ms transform/opacity,150ms height and250ms content fade; hover/focus expands. Swipe dismissal and duplicate bump400ms are separate contracts |

The measured success title/description example is340×76 at(668,692) in1040×800 and488×76 at(16,708) in520×800, both themes. F6 reaches the named Notifications region; Shift+Tab returns to Save document. An external duplicate changes the title once, then an in-tree duplicate preserves it; functional update reads the current title. [Source fixture/probe](evidence/toast/browser/) imports the pinned implementation directly.

Base's manager provides ordered storage, phases, timeout pause and cleanup, but its `push` replaces and moves an existing item and restarts entry. It has no payload/timeout update method. Base ToastStack can configure Kumo gap/peek/scale and expand on hover/focus, but uses springs rather than source CSS easing and does not itself hide behind content or implement focus guards/swipe. Compose deliberately; document any bounded native adaptation. Do not silently use Sonner default motion or Alert semantics.

## Native core acceptance — 2026-10-03

Application-retained `ToastState` and `ToastViewport` compose Base manager/stack infrastructure with Kumo surfaces. Public typed add requests, title/description, five variants, stable IDs, functional content updates, actions and dismiss events are implemented. The document workflow emits success feedback only after confirmed save/delete. The focused `toast` example exercises variants, update and once-only actions.

| Contract | Native core result / limit |
| --- | --- |
| Lifecycle | Default 5s, zero persistent, 500ms exit retention; hover/focus/inactive-window pause; mounted weak timer cleanup on unmount or window close |
| Identity/update | Duplicate add preserves payload/order without restarting TTL; explicit update reads current content, retains order/handles/TTL; ending/missing updates are ignored. External-manager reset/merge and source duplicate bump remain unported |
| Actions | Typed independently focused actions; activation checks current ID/availability/phase before emitting; no automatic dismissal; removed focused action recovers to its row |
| Focus | F6 enters named Notifications, reverse Tab restores producer, forward Tab enters newest row, Escape closes that row; dismissal selects surviving toast or producer while preserving newer outside focus |
| Semantics | Nonmodal named Dialog rows; polite non-atomic Notifications region. Close is always exposed natively; source hides its accessible control until expansion/focus. High-priority AlertDialog and actual spoken announcement acceptance remain open |
| Geometry | Source 340×76 at(668,692), 488×76 at(16,708), 12px corners/16px padding; five source Phosphor assets; separate text and ring accent tokens |
| Painting | 0.3px ring uses device-pixel coverage compensation because GPUI snaps borders/shadows. Blur kernels, glyph rasterization and fractional coverage differ from browser; not pixel-identical. Close hover currently inherits native ghost tokens rather than source current-color/15% |
| Stack/motion | Base configured gap/peek12, scale10%, limit3, 500ms motion plus native opacity; hidden behind content. Source CSS easing, actual content scaling, 150ms height/250ms content fade, duplicate bump, swipe and full entry/exit guards remain open |
| API/composition | Promise helper, high priority, configurable provider/external manager/limit/placement, rich custom parts and timeout-reset update policy remain open |

Eight rendered/input regressions pass: width/placement and action/close, duplicate/update/order/ending guards, focused timeout, hover/narrow geometry, action availability before repaint, F6/Tab/Escape recovery, repeated mount cleanup and retained state after window close. Actual native and pinned browser Light/Dark1040/520 matrices cover all five variants, stable update, named nonmodal roles, pause/resume and action activation. Source/native evidence and reproducible scripts: [Toast evidence](evidence/toast/README.md).

The full pinned Rust gate passes226 library/1gallery/9doctests and adapter12default/14all-feature tests, formatting, warnings-denied lint and locked builds/default gallery. Remote macOS result will be recorded against the published SHA separately. No pins or vendor patches changed. #34 stays open for the full family/platform/speech/IME contract. Next announcement work is composed-workflow and target-platform polish, then recording; full migration continues beyond the demo.
