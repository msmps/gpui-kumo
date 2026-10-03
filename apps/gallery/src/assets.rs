use gpui_kit::{AssetSource, SharedString};
use std::borrow::Cow;

/// Assets used by the gallery and its focused examples.
pub struct GalleryAssets;

impl AssetSource for GalleryAssets {
    fn load(&self, path: &str) -> gpui_kit::Result<Option<Cow<'static, [u8]>>> {
        Ok(match path {
            "toolbar-settings.svg" => Some(Cow::Borrowed(include_bytes!(
                "../assets/toolbar-settings.svg"
            ))),
            "workspace.svg" => Some(Cow::Borrowed(include_bytes!("../assets/workspace.svg"))),
            _ => return gpui_kumo::assets::Assets.load(path),
        })
    }

    fn list(&self, path: &str) -> gpui_kit::Result<Vec<SharedString>> {
        let mut assets = gpui_kumo::assets::Assets.list(path)?;
        if path.is_empty() {
            assets.extend(["workspace.svg".into(), "toolbar-settings.svg".into()]);
        }
        Ok(assets)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gallery_asset_source_resolves_application_and_kumo_icons() {
        let assets = GalleryAssets;
        let mut paths = assets.list("").unwrap();
        paths.sort();
        assert_eq!(
            paths,
            [
                "caret-down.svg",
                "empty-copy.svg",
                "toolbar-settings.svg",
                "workspace.svg"
            ]
            .map(SharedString::from)
        );
        for path in paths {
            let data = assets.load(&path).unwrap().unwrap();
            assert!(std::str::from_utf8(&data).unwrap().contains("<svg"));
        }
        assert!(assets.load("missing.svg").unwrap().is_none());
        assert!(assets.list("missing/").unwrap().is_empty());
    }
}
