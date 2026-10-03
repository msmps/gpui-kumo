use gpui_kit::{Div, Entity, div, prelude::*};
use gpui_kumo::{Button, Toast, ToastAction, ToastState, ToastVariant};

pub fn controls(toasts: &Entity<ToastState>) -> Div {
    let mut root = div().flex().flex_wrap().gap_3();
    for (id, label, title, variant) in [
        (
            "saved",
            "Save document",
            "Document saved",
            ToastVariant::Success,
        ),
        (
            "default",
            "Show default",
            "Document updated",
            ToastVariant::Default,
        ),
        ("error", "Show error", "Save failed", ToastVariant::Error),
        (
            "warning",
            "Show warning",
            "Review your changes",
            ToastVariant::Warning,
        ),
        (
            "info",
            "Show info",
            "New version available",
            ToastVariant::Info,
        ),
    ] {
        let toasts = toasts.downgrade();
        root = root.child(Button::new(id, label).on_click(move |_, window, cx| {
            let _ = toasts.update(cx, |state, cx| {
                state.add(
                    Toast::new(id, title)
                        .description("Your changes are ready to share.")
                        .variant(variant),
                    window,
                    cx,
                );
            });
        }));
    }
    let actions = toasts.downgrade();
    root = root.child(
        Button::new("actions", "Show actions").on_click(move |_, window, cx| {
            let _ = actions.update(cx, |state, cx| {
                state.add(
                    Toast::new("action-toast", "Changes saved")
                        .description("You can undo this demo operation.")
                        .variant(ToastVariant::Success)
                        .action(ToastAction::new("undo", "Undo")),
                    window,
                    cx,
                );
            });
        }),
    );
    let update = toasts.downgrade();
    let long = toasts.downgrade();
    root = root.child(Button::new("long", "Show long toast").on_click(move |_, window, cx| {
        let _ = long.update(cx, |state, cx| {
            state.add(
                Toast::new("long-toast", "Your announcement document has been saved and is ready to share")
                    .description("Review the final copy with your team before publishing. Your changes are retained while you decide what to do next.")
                    .variant(ToastVariant::Success)
                    .timeout(std::time::Duration::ZERO)
                    .action(ToastAction::new("review", "Review changes"))
                    .action(ToastAction::new("later", "Later")),
                window,
                cx,
            );
        });
    }));
    root.child(
        Button::new("update", "Update saved toast").on_click(move |_, window, cx| {
            let _ = update.update(cx, |state, cx| {
                state.update(
                    "saved",
                    |content| content.title = "Document ready to share".into(),
                    window,
                    cx,
                );
            });
        }),
    )
}
