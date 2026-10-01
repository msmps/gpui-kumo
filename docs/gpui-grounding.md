# GPUI state and rendering

Read when choosing state ownership, building a root view, wiring updates, or bootstrapping an app. The [source baseline](gpui-sources.md) defines which upstream revision these facts describe.

## State and rendering levels

| Level | Responsibility | Design-system use |
| --- | --- | --- |
| `Entity<T>` | Handle to state owned by `App` | Durable values and behavior shared across callbacks |
| `Render` view | Build UI from a mutable entity | Input, selection, expansion, subscriptions, and task ownership |
| `RenderOnce` component | Consume presentation data to build UI | Button, label, surface, and other reusable compositions |
| `Element` | Request layout, prepaint, and paint | Specialized measurement, hit testing, or rendering |

`Render` receives `&mut self`, `&mut Window`, and `&mut Context<Self>`; `RenderOnce` receives `self`, `&mut Window`, and `&mut App`. Both return `impl IntoElement`. `IntoElement` converts into a concrete element; `AnyElement` erases that concrete type for heterogeneous storage. `#[derive(IntoElement)]` supports `RenderOnce` composition. [Element traits](https://github.com/zed-industries/zed/blob/95cd535a5fad96d649513f96c5784ceefd379e47/crates/gpui/src/element.rs)

## Context and lifetime

`App` is the application context. `Context<T>` dereferences to it and adds entity-specific operations. `Window` is passed separately for window state, focus, geometry, and painting. Use `entity.read(cx)` and `entity.update(cx, |state, cx| ...)` for entity access. `cx.listener(...)` adapts callbacks to receive the owning entity's state. [Context source](https://github.com/zed-industries/zed/blob/95cd535a5fad96d649513f96c5784ceefd379e47/crates/gpui/src/app/context.rs)

Application state survives rendering. Treat constructed elements and callbacks as frame-scoped descriptions; GPUI can also retain identified element state and reuse cached view output. Read [Interaction](gpui-interaction.md) for identity and [Primitives](gpui-primitives.md) for drawing phases.

## State changes and background work

Use `cx.notify()` to notify observers after visible entity state changes. `observe` listens for notifications; `subscribe` listens for typed emitted events from an `EventEmitter<E>`. They express different contracts. Retain returned subscriptions for the owner's lifetime or intentionally detach them. Dropping a subscription cancels it. [Context](https://github.com/zed-industries/zed/blob/95cd535a5fad96d649513f96c5784ceefd379e47/crates/gpui/src/app/context.rs), [subscriptions](https://github.com/zed-industries/zed/blob/95cd535a5fad96d649513f96c5784ceefd379e47/crates/gpui/src/subscription.rs)

`Context::spawn` supplies a weak entity and an async application context. Retain or detach its task. Async entity/window access is fallible because the target can close before completion. Use background execution for expensive work and update UI state through the context afterward; spawning a foreground future alone does not move blocking computation off the UI thread. [Context](https://github.com/zed-industries/zed/blob/95cd535a5fad96d649513f96c5784ceefd379e47/crates/gpui/src/app/context.rs), [executors](https://github.com/zed-industries/zed/blob/95cd535a5fad96d649513f96c5784ceefd379e47/crates/gpui/src/executor.rs)

**Design-system guidance:** create durable handles, subscriptions, and tasks in the state owner's lifecycle. Reconstruct presentation from those retained values. Keep network access, file reads, and expensive transformations outside rendering.

## Startup

The researched upstream starts with `gpui_platform::application().run(...)`, then `cx.open_window(...)` and `cx.new(...)` for the root view. The website's `Application::new()` example differs from this revision. Verify startup against the selected dependency. On macOS this revision uses Metal and requires `gpui_platform`'s `font-kit` feature for visible glyph rasterization. Linux/FreeBSD needs a desktop windowing backend; Windows uses Win32 and DirectWrite. The README documents Xcode setup. [README](https://github.com/zed-industries/zed/blob/95cd535a5fad96d649513f96c5784ceefd379e47/crates/gpui/README.md), [platform features](https://github.com/zed-industries/zed/blob/95cd535a5fad96d649513f96c5784ceefd379e47/crates/gpui_platform/Cargo.toml), [website](https://gpui.rs/)
