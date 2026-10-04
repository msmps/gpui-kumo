//! Embedded assets available to application asset sources.

use std::borrow::Cow;

use gpui_kit::{AssetSource, SharedString};

/// Named Kumo icons for applications using [`crate::Icon`].
///
/// Delegate unknown paths from your own `AssetSource` to this source. Components
/// embed their internal icons directly and do not require this registration.
pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> gpui_kit::Result<Option<Cow<'static, [u8]>>> {
        let data: Option<&'static [u8]> = match path {
            "caret-down.svg" => Some(include_bytes!("../assets/caret-down.svg")),
            "empty-copy.svg" => Some(include_bytes!("../assets/empty-copy.svg")),
            _ => None,
        };
        Ok(data.map(Cow::Borrowed))
    }

    fn list(&self, path: &str) -> gpui_kit::Result<Vec<SharedString>> {
        Ok(if path.is_empty() {
            vec!["caret-down.svg".into(), "empty-copy.svg".into()]
        } else {
            vec![]
        })
    }
}

impl std::fmt::Debug for Assets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_struct("Assets");
        debug.finish_non_exhaustive()
    }
}
