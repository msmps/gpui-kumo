//! Application-owned Kumo tokens. Source values and native policies are recorded
//! in `docs/kumo-tokens.md`.

use std::time::Duration;

use gpui_kit::{
    App, Background, BoxShadow, ColorSpace, FontWeight, Global, Hsla, Pixels, SharedString, base,
    linear_color_stop, linear_gradient, px, rgb,
};

use crate::color::Oklch;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Appearance {
    #[default]
    Light,
    Dark,
}

impl Appearance {
    pub fn opposite(self) -> Self {
        match self {
            Self::Light => Self::Dark,
            Self::Dark => Self::Light,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TextColors {
    pub default: Hsla,
    pub strong: Hsla,
    pub subtle: Hsla,
    pub inactive: Hsla,
    pub placeholder: Hsla,
    pub inverse: Hsla,
    pub brand: Hsla,
    /// Kumo's link tone, also used by its success Text variant.
    pub link: Hsla,
    pub danger: Hsla,
    pub success: Hsla,
    pub warning: Hsla,
    /// Information-message foreground, distinct from the general icon accent.
    pub info: Hsla,
}

#[derive(Clone, Debug)]
pub struct Colors {
    pub canvas: Hsla,
    pub base: Hsla,
    pub elevated: Hsla,
    pub recessed: Hsla,
    pub tint: Hsla,
    pub contrast: Hsla,
    pub overlay: Hsla,
    pub control: Hsla,
    pub interact: Hsla,
    pub fill: Hsla,
    pub fill_hover: Hsla,
    pub brand: Hsla,
    pub brand_hover: Hsla,
    pub danger: Hsla,
    pub danger_tint: Hsla,
    pub success: Hsla,
    pub success_tint: Hsla,
    pub warning: Hsla,
    pub warning_tint: Hsla,
    /// General information accent for icon fills and Banner actions.
    pub info: Hsla,
    pub info_tint: Hsla,
    pub line: Hsla,
    pub hairline: Hsla,
    pub focus: Hsla,
    pub shadow_edge: Hsla,
    pub shadow_drop: Hsla,
    pub arrow_edge: Hsla,
    pub arrow_stroke: Hsla,
}

/// Semantic badge color roles from the pinned Kumo theme.
#[derive(Clone, Debug)]
pub struct BadgeColors {
    pub inverted: Hsla,
    pub inverted_foreground: Hsla,
    pub neutral_subtle_foreground: Hsla,
    pub teal_subtle_foreground: Hsla,
    pub red: Hsla,
    pub green: Hsla,
    pub neutral: Hsla,
    pub orange: Hsla,
    pub purple: Hsla,
    pub teal: Hsla,
    pub blue: Hsla,
    pub solid_foreground: Hsla,
    pub orange_foreground: Hsla,
}

/// Appearance-specific colors for Kumo Switch tracks and thumbs.
#[derive(Clone, Debug)]
pub struct SwitchColors {
    pub off_track: Hsla,
    pub off_ring: Hsla,
    pub on_track: Hsla,
    pub on_ring: Hsla,
    pub off_thumb: Hsla,
    pub on_thumb: Hsla,
    pub neutral_off_track: Hsla,
    pub neutral_on_track: Hsla,
    pub neutral_on_ring: Hsla,
    pub neutral_on_thumb: Hsla,
}

/// Native policies filling gaps in the web recipes, kept distinct from upstream.
#[derive(Clone, Debug)]
pub struct NativeColors {
    pub disabled_input_foreground: Hsla,
    pub selection: Hsla,
}

/// Accent gradients for Banner actions, mixed in source OKLCH before mapping.
#[derive(Clone, Debug)]
pub struct BannerAccents {
    /// General information accent, using the defined Tailwind blue-500.
    pub info: Emphasis,
    /// Appearance-specific warning accent.
    pub warning: Emphasis,
    /// Neutral-700 accent, shared across appearances.
    pub secondary: Emphasis,
}

#[derive(Clone, Copy, Debug)]
pub struct TextStyle {
    pub size: Pixels,
    pub line_height: Pixels,
    pub weight: FontWeight,
}

impl TextStyle {
    fn new(size: f32, line_height: f32) -> Self {
        Self {
            size: px(size),
            line_height: px(line_height),
            weight: FontWeight::NORMAL,
        }
    }

    fn base_token(self) -> base::TextStyleToken {
        base::TextStyleToken {
            size: self.size,
            line_height: self.line_height,
            weight: self.weight,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Typography {
    pub font_family: SharedString,
    /// Native monospace counterpart of Kumo's consumer-provided mono stack.
    pub mono_font_family: SharedString,
    pub xs: TextStyle,
    pub sm: TextStyle,
    pub base: TextStyle,
    pub lg: TextStyle,
    pub heading: TextStyle,
    pub heading_lg: TextStyle,
    pub field_description: TextStyle,
    pub popover_title: TextStyle,
    pub popover_description: TextStyle,
}

#[derive(Clone, Copy, Debug)]
pub struct Spacing {
    pub four: Pixels,
    pub six: Pixels,
    pub eight: Pixels,
    pub twelve: Pixels,
    pub sixteen: Pixels,
}

#[derive(Clone, Copy, Debug)]
pub struct Radii {
    pub sm: Pixels,
    pub md: Pixels,
    pub lg: Pixels,
}

#[derive(Clone, Debug)]
pub struct Effects {
    pub control_ring_width: Pixels,
    pub input_focus_ring_width: Pixels,
    pub keyboard_focus_ring_width: Pixels,
    pub popover_outline_width: Pixels,
    pub popover_outline_offset: Pixels,
    pub shadow_xs: Vec<BoxShadow>,
    pub shadow_md: Vec<BoxShadow>,
    pub transition: Duration,
    pub popover_transition: Duration,
    pub easing: [f32; 4],
}

#[derive(Clone, Debug)]
pub struct Emphasis {
    pub ring: Hsla,
    pub background: Hsla,
    pub gradient_start: Hsla,
    pub gradient_end: Hsla,
}

impl Emphasis {
    fn new(token: Oklch) -> Self {
        Self {
            ring: token.mix_black(0.1).paint(),
            background: token.mix_white(0.3).paint(),
            gradient_start: token.mix_white(0.15).paint(),
            gradient_end: token.paint(),
        }
    }

    /// Top-to-bottom gradient, with OKLab interpolation between mapped stops.
    pub fn gradient(&self, hovered: bool) -> Background {
        linear_gradient(
            180.,
            linear_color_stop(
                if hovered {
                    self.background
                } else {
                    self.gradient_start
                },
                0.,
            ),
            linear_color_stop(self.gradient_end, 1.),
        )
        .color_space(ColorSpace::Oklab)
    }

    pub fn inset_highlight(&self) -> BoxShadow {
        BoxShadow::new(px(0.), px(1.), self.background).inset()
    }
}

/// One application-wide theme. Independent previews may pass snapshots directly.
#[derive(Clone, Debug)]
pub struct Theme {
    pub appearance: Appearance,
    pub text: TextColors,
    pub colors: Colors,
    pub native: NativeColors,
    pub badge: BadgeColors,
    pub switch: SwitchColors,
    pub banner: BannerAccents,
    pub typography: Typography,
    pub spacing: Spacing,
    pub radii: Radii,
    pub effects: Effects,
    pub primary: Emphasis,
    pub destructive: Emphasis,
}

impl Global for Theme {}

impl Theme {
    // The pinned brand L=.5772 happens to approximate Euler's constant.
    #[allow(clippy::approx_constant)]
    pub fn new(appearance: Appearance) -> Self {
        let dark = appearance == Appearance::Dark;
        let neutral =
            |light, dark_value| Oklch::gray(if dark { dark_value } else { light }).paint();
        let action = Oklch::new(0.5772, 0.2324, 260.);
        let action = if dark { action.mix_black(0.1) } else { action };
        let danger = if dark {
            Oklch::new(0.577, 0.245, 27.325)
        } else {
            Oklch::new(0.637, 0.237, 25.331)
        };
        // Tailwind 4.3.3's defined blue-500 wins over Kumo's sky-colored fallback.
        let info = Oklch::new(0.623, 0.214, 259.815);
        let warning = if dark {
            Oklch::new(0.645, 0.168, 50.)
        } else {
            Oklch::new(0.739, 0.177, 58.2)
        };
        let white = Hsla::from(rgb(0xffffff));
        let black = Hsla::from(rgb(0x000000));
        let line = if dark {
            neutral(0., 0.32)
        } else {
            neutral(0.145, 0.).alpha(0.1)
        };
        let choose = |light, dark_value| if dark { dark_value } else { light };
        let text = TextColors {
            default: neutral(0.205, 0.97),
            strong: neutral(0.145, 0.985),
            subtle: neutral(0.556, 0.708),
            inactive: neutral(0.87, 0.439),
            placeholder: neutral(0.708, 0.556),
            inverse: neutral(0.97, 0.205),
            brand: rgb(0xf6821f).into(),
            link: if dark {
                Oklch::new(0.707, 0.165, 254.624)
            } else {
                Oklch::new(0.424, 0.199, 265.638)
            }
            .paint(),
            success: choose(
                Oklch::new(0.432, 0.095, 166.913),
                Oklch::new(0.905, 0.093, 164.15),
            )
            .paint(),
            warning: choose(
                Oklch::new(0.597, 0.144, 57.5),
                Oklch::new(0.75, 0.183, 55.934),
            )
            .paint(),
            info: choose(
                Oklch::new(0.424, 0.199, 265.638),
                Oklch::new(0.707, 0.165, 254.624),
            )
            .paint(),
            danger: if dark {
                Oklch::new(0.704, 0.191, 22.216)
            } else {
                Oklch::new(0.505, 0.213, 27.518)
            }
            .paint(),
        };
        let colors = Colors {
            canvas: neutral(0.9875, 0.10),
            base: if dark { neutral(0., 0.17) } else { white },
            elevated: neutral(0.98, 0.12),
            recessed: neutral(0.965, 0.15),
            tint: neutral(0.97, 0.269),
            contrast: neutral(0.12, 0.985),
            overlay: neutral(0.9875, 0.269),
            control: if dark { neutral(0., 0.205) } else { white },
            interact: neutral(0.87, 0.371),
            fill: neutral(0.922, 0.269),
            fill_hover: neutral(0.965, 0.269),
            brand: action.paint(),
            brand_hover: Oklch::new(0.488, 0.243, 264.376).paint(),
            danger: danger.paint(),
            danger_tint: choose(
                Oklch::new(0.936, 0.032, 17.7),
                Oklch::new(0.429, 0.176, 28.7),
            )
            .paint()
            .alpha(if dark { 0.17 } else { 0.42 }),
            success: choose(
                Oklch::new(0.596, 0.145, 163.225),
                Oklch::new(0.765, 0.177, 163.223),
            )
            .paint(),
            success_tint: choose(
                Oklch::new(0.962, 0.043, 156.7),
                Oklch::new(0.393, 0.096, 152.3),
            )
            .paint()
            .alpha(if dark { 0.2 } else { 0.57 }),
            warning: warning.paint(),
            info: info.paint(),
            warning_tint: choose(
                Oklch::new(0.931, 0.107, 94.6),
                Oklch::new(0.353, 0.079, 65.),
            )
            .paint()
            .alpha(if dark { 0.37 } else { 0.2 }),
            info_tint: choose(
                Oklch::new(0.932, 0.032, 255.6),
                Oklch::new(0.38, 0.145, 265.5),
            )
            .paint()
            .alpha(if dark { 0.22 } else { 0.45 }),
            line,
            hairline: neutral(0.935, 0.269),
            focus: neutral(0.15, 0.935),
            shadow_edge: if dark {
                white.alpha(0.1)
            } else {
                black.alpha(0.12)
            },
            shadow_drop: black.alpha(if dark { 0.3 } else { 0.08 }),
            arrow_edge: if dark { black.alpha(0.) } else { line },
            arrow_stroke: if dark { line } else { black.alpha(0.) },
        };
        Self {
            appearance,
            switch: SwitchColors {
                off_track: neutral(0.922, 0.371),
                off_ring: neutral(0.87, 0.439),
                on_track: choose(info, Oklch::new(0.546, 0.245, 262.881)).paint(),
                on_ring: choose(Oklch::new(0.546, 0.245, 262.881), info).paint(),
                off_thumb: if dark {
                    Oklch::gray(0.24).paint()
                } else {
                    colors.base
                },
                on_thumb: if dark {
                    Oklch::new(0.809, 0.105, 251.813).paint()
                } else {
                    colors.base
                },
                neutral_off_track: if dark {
                    colors.base
                } else {
                    Oklch::gray(0.935).paint()
                },
                neutral_on_track: if dark {
                    colors.base
                } else {
                    Oklch::gray(0.556).paint()
                },
                neutral_on_ring: neutral(0.439, 0.371),
                neutral_on_thumb: if dark {
                    Oklch::gray(0.708).paint()
                } else {
                    colors.base
                },
            },
            badge: BadgeColors {
                inverted: if dark {
                    white
                } else {
                    Oklch::gray(0.145).paint()
                },
                inverted_foreground: if dark { black } else { white },
                neutral_subtle_foreground: neutral(0.269, 0.922),
                teal_subtle_foreground: choose(
                    Oklch::new(0.437, 0.078, 188.216),
                    Oklch::new(0.91, 0.096, 180.426),
                )
                .paint(),
                red: choose(
                    Oklch::new(0.577, 0.245, 27.325),
                    Oklch::new(0.505, 0.213, 27.518),
                )
                .paint(),
                green: choose(
                    Oklch::new(0.596, 0.145, 163.225),
                    Oklch::new(0.508, 0.118, 165.612),
                )
                .paint(),
                neutral: neutral(0.556, 0.439),
                orange: Oklch::new(0.815, 0.197, 76.).paint(),
                purple: choose(
                    Oklch::new(0.558, 0.288, 302.321),
                    Oklch::new(0.496, 0.265, 301.924),
                )
                .paint(),
                teal: choose(
                    Oklch::new(0.549, 0.096, 184.565),
                    Oklch::new(0.511, 0.096, 186.391),
                )
                .paint(),
                blue: choose(
                    Oklch::new(0.546, 0.245, 262.881),
                    Oklch::new(0.488, 0.243, 264.376),
                )
                .paint(),
                solid_foreground: white,
                orange_foreground: black,
            },
            native: NativeColors {
                disabled_input_foreground: text.subtle,
                selection: colors.brand.alpha(0.25),
            },
            text,
            colors,
            typography: Typography {
                font_family: system_font().into(),
                mono_font_family: system_mono_font().into(),
                xs: TextStyle::new(12., 16.),
                sm: TextStyle::new(13., 13. / 0.85),
                base: TextStyle::new(14., 21.),
                lg: TextStyle::new(16., 24.),
                heading: TextStyle {
                    weight: FontWeight::SEMIBOLD,
                    ..TextStyle::new(16., 24.)
                },
                heading_lg: TextStyle {
                    weight: FontWeight::SEMIBOLD,
                    ..TextStyle::new(20., 28.)
                },
                field_description: TextStyle::new(13., 17.875),
                popover_title: TextStyle {
                    weight: FontWeight::MEDIUM,
                    ..TextStyle::new(14., 24.)
                },
                popover_description: TextStyle::new(14., 24.),
            },
            spacing: Spacing {
                four: px(4.),
                six: px(6.),
                eight: px(8.),
                twelve: px(12.),
                sixteen: px(16.),
            },
            radii: Radii {
                sm: px(4.),
                md: px(6.),
                lg: px(8.),
            },
            effects: Effects {
                control_ring_width: px(1.),
                input_focus_ring_width: px(1.5),
                keyboard_focus_ring_width: px(2.),
                popover_outline_width: px(1.),
                popover_outline_offset: px(if dark { -1. } else { 0. }),
                shadow_xs: vec![
                    BoxShadow::new(px(0.), px(1.), black.alpha(0.05)).blur_radius(px(2.)),
                ],
                shadow_md: vec![
                    BoxShadow::new(px(0.), px(4.), black.alpha(0.1))
                        .blur_radius(px(6.))
                        .spread_radius(px(-1.)),
                    BoxShadow::new(px(0.), px(2.), black.alpha(0.1))
                        .blur_radius(px(4.))
                        .spread_radius(px(-2.)),
                ],
                transition: Duration::from_millis(100),
                popover_transition: Duration::from_millis(150),
                easing: [0.4, 0., 0.2, 1.],
            },
            primary: Emphasis::new(action),
            destructive: Emphasis::new(danger),
            banner: BannerAccents {
                info: Emphasis::new(info),
                warning: Emphasis::new(warning),
                secondary: Emphasis::new(Oklch::gray(0.371)),
            },
        }
    }

    pub fn with_font_family(mut self, family: impl Into<SharedString>) -> Self {
        self.typography.font_family = family.into();
        self
    }
}

fn system_font() -> &'static str {
    if cfg!(target_os = "macos") {
        ".SystemUIFont"
    } else if cfg!(target_os = "windows") {
        "Segoe UI"
    } else {
        // GPUI0.3.7 matches concrete family names; CSS generic aliases are
        // not resolved by its CosmicText backend. Use a concrete sans family.
        "DejaVu Sans"
    }
}

fn system_mono_font() -> &'static str {
    if cfg!(target_os = "macos") {
        "Menlo"
    } else if cfg!(target_os = "windows") {
        "Consolas"
    } else {
        "monospace"
    }
}

/// Read during render; consumers retain `observe_global::<Theme>` subscriptions.
pub fn theme(cx: &App) -> &Theme {
    cx.global::<Theme>()
}

/// Publish Kumo and its Base projection together before the next render.
pub fn set_theme(theme: Theme, cx: &mut App) {
    let mut base_theme = base::Theme::global(cx);
    project_to_base(&theme, &mut base_theme);
    cx.set_global(base_theme);
    cx.set_global(theme);
}

/// Change the standard palette while retaining the consumer's font family.
/// Custom palette overrides should instead publish a complete `Theme` snapshot.
pub fn set_appearance(appearance: Appearance, cx: &mut App) {
    let family = theme(cx).typography.font_family.clone();
    let mono_family = theme(cx).typography.mono_font_family.clone();
    let mut next = Theme::new(appearance).with_font_family(family);
    next.typography.mono_font_family = mono_family;
    set_theme(next, cx);
}

// This is the only boundary that knows Base's token vocabulary. Keep unrelated
// behavior settings and slots not covered by our first slice as they were.
fn project_to_base(kumo: &Theme, base: &mut base::Theme) {
    base.appearance = match kumo.appearance {
        Appearance::Light => base::ThemeAppearance::Light,
        Appearance::Dark => base::ThemeAppearance::Dark,
    };
    let c = &kumo.colors;
    let t = &kumo.text;
    let white = rgb(0xffffff).into();
    base.tokens.colors = base::ColorTokens {
        background: c.canvas,
        foreground: t.default,
        surface: c.base,
        surface_foreground: t.default,
        primary: c.brand,
        primary_foreground: white,
        secondary: c.base,
        secondary_foreground: t.default,
        muted: c.recessed,
        muted_foreground: t.subtle,
        accent: c.tint,
        accent_foreground: t.default,
        destructive: c.danger,
        destructive_foreground: white,
        border: c.line,
        input: c.line,
        ring: c.focus,
        selection: kumo.native.selection,
    };
    base.tokens.radius.sm = kumo.radii.sm;
    base.tokens.radius.md = kumo.radii.md;
    base.tokens.radius.lg = kumo.radii.lg;
    base.tokens.spacing.xs = kumo.spacing.four;
    base.tokens.spacing.sm = kumo.spacing.eight;
    base.tokens.spacing.md = kumo.spacing.twelve;
    base.tokens.spacing.lg = kumo.spacing.sixteen;
    let typography = &mut base.tokens.typography;
    typography.sans = kumo.typography.font_family.clone();
    typography.xs = kumo.typography.xs.base_token();
    typography.sm = kumo.typography.sm.base_token();
    typography.md = kumo.typography.base.base_token();
    typography.lg = kumo.typography.lg.base_token();
    base.tokens.shadow.sm = kumo.effects.shadow_xs.clone();
    base.tokens.shadow.md = kumo.effects.shadow_md.clone();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_projection_preserves_unrelated_settings_and_replaces_appearance_tokens() {
        let mut base = base::Theme::default();
        let handle = Hsla::from(rgb(0x123456));
        base.resizable.handle = Some(handle);
        let mono = base.tokens.typography.mono.clone();
        for appearance in [Appearance::Dark, Appearance::Light] {
            let kumo = Theme::new(appearance).with_font_family("Custom Font");
            project_to_base(&kumo, &mut base);
            assert_eq!(base.tokens.colors.background, kumo.colors.canvas);
            assert_eq!(base.tokens.colors.foreground, kumo.text.default);
            assert_eq!(base.tokens.colors.selection, kumo.native.selection);
            assert_eq!(
                base.appearance == base::ThemeAppearance::Dark,
                appearance == Appearance::Dark
            );
            assert_eq!(base.tokens.typography.md.size, px(14.));
            assert_eq!(base.tokens.typography.sans.as_ref(), "Custom Font");
            assert_eq!(base.tokens.typography.mono, mono);
            assert_eq!(base.resizable.handle, Some(handle));
        }
    }
}
