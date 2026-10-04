//! Validation for names supplied by external data.
use gpui_kit::SharedString;

/// A nonblank accessible name. Whitespace within a meaningful name is preserved.
///
/// Validate user or service data before passing it to infallible component builders:
/// ```
/// use gpui_kumo::{AccessibleName, Button};
/// let name = AccessibleName::try_from("Delete project Foo")?;
/// let button = Button::new("delete", "Delete").accessibility_label(name);
/// # Ok::<(), gpui_kumo::BlankName>(())
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccessibleName(SharedString);

/// A name contained only whitespace or was empty.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlankName;
impl std::fmt::Display for BlankName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("an accessible name must contain non-whitespace text")
    }
}
impl std::error::Error for BlankName {}
impl TryFrom<SharedString> for AccessibleName {
    type Error = BlankName;
    fn try_from(value: SharedString) -> Result<Self, Self::Error> {
        if value.trim().is_empty() {
            Err(BlankName)
        } else {
            Ok(Self(value))
        }
    }
}
impl TryFrom<&str> for AccessibleName {
    type Error = BlankName;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(SharedString::from(value.to_owned()))
    }
}
impl TryFrom<String> for AccessibleName {
    type Error = BlankName;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(SharedString::from(value))
    }
}
impl From<AccessibleName> for SharedString {
    fn from(value: AccessibleName) -> Self {
        value.0
    }
}
pub(crate) fn nonblank(value: impl Into<SharedString>) -> SharedString {
    AccessibleName::try_from(value.into())
        .expect("a control requires a nonblank accessible name")
        .into()
}
