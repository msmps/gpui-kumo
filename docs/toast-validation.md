# Toast source contract — announcement feedback slice

Status: unported. This is source/browser evidence and the next native contract; it does not increment coverage. Pinned Kumo `3fd5b648df578cb1ba214dedd30f475009f6a668`, Kit/Base0.7.0, GPUI0.3.7 and Rust1.99.0 remain authoritative.

Inspected complete `toast.tsx`, its regressions and demos, installed Base Toast/ToastManager/ToastStack/ToastMotion and browser Base UI1.8.0 provider/store/root/viewport behavior. Actual pinned browser Light/Dark1040/520 probe verifies dimensions, external/in-tree dedupe, functional updates, F6 and reverse-Tab restoration. No native Toast acceptance is claimed.

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

Next native slice connects confirmed document operations to retained feedback, then verifies add/update/dismiss ordering, timeout/pause, focus-safe removal, repeated mounts, real activation and both-theme/narrow visual alignment. Full acceptance stays in [KUMO-049/#34](issues/049-toast-port.md); source audit does not resolve motion, announcements, OS IME or other-platform checks.
