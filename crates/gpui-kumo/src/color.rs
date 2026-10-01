//! Conversion of the pinned OKLCH tokens into GPUI's sRGB paint values.
//! Out-of-gamut colors reduce chroma at fixed lightness/hue. This is a deliberate
//! native policy, not a claim to implement browser local-MINDE gamut mapping.

use gpui_kit::{Hsla, Rgba};

#[derive(Clone, Copy)]
pub(crate) struct Oklch {
    pub l: f64,
    pub c: f64,
    pub h: f64,
}

impl Oklch {
    pub const fn new(l: f64, c: f64, h: f64) -> Self {
        Self { l, c, h }
    }

    pub fn gray(l: f64) -> Self {
        Self::new(l, 0., 0.)
    }

    // Black/white have powerless hue. These operations apply only to opaque
    // extracted action tokens; mix before converting or gamut mapping.
    pub fn mix_white(self, fraction: f64) -> Self {
        Self::new(
            self.l * (1. - fraction) + fraction,
            self.c * (1. - fraction),
            self.h,
        )
    }

    pub fn mix_black(self, fraction: f64) -> Self {
        Self::new(self.l * (1. - fraction), self.c * (1. - fraction), self.h)
    }

    pub fn paint(self) -> Hsla {
        let mut linear = self.linear_srgb();
        if !in_gamut(linear) {
            let (mut low, mut high) = (0., self.c);
            for _ in 0..24 {
                let c = (low + high) / 2.;
                let candidate = Self { c, ..self }.linear_srgb();
                if in_gamut(candidate) {
                    low = c;
                    linear = candidate;
                } else {
                    high = c;
                }
            }
        }
        Hsla::from(Rgba {
            r: encode_srgb(linear[0]),
            g: encode_srgb(linear[1]),
            b: encode_srgb(linear[2]),
            a: 1.,
        })
    }

    fn linear_srgb(self) -> [f64; 3] {
        // Björn Ottosson's public-domain inverse Oklab matrix (2021 revision):
        // https://bottosson.github.io/posts/oklab/#converting-from-linear-srgb-to-oklab
        let a = self.c * self.h.to_radians().cos();
        let b = self.c * self.h.to_radians().sin();
        let l = (self.l + 0.3963377774 * a + 0.2158037573 * b).powi(3);
        let m = (self.l - 0.1055613458 * a - 0.0638541728 * b).powi(3);
        let s = (self.l - 0.0894841775 * a - 1.2914855480 * b).powi(3);
        [
            4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
            -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
            -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s,
        ]
    }
}

fn in_gamut(rgb: [f64; 3]) -> bool {
    rgb.into_iter().all(|channel| (0. ..=1.).contains(&channel))
}

fn encode_srgb(linear: f64) -> f32 {
    let linear = linear.clamp(0., 1.);
    if linear <= 0.0031308 {
        (12.92 * linear) as f32
    } else {
        (1.055 * linear.powf(1. / 2.4) - 0.055) as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neutral_conversion_uses_srgb_transfer_function() {
        let rgb = Rgba::from(Oklch::gray(0.5).paint());
        // L=.5 gives linear RGB=.125, encoded sRGB=.38857286.
        for channel in [rgb.r, rgb.g, rgb.b] {
            assert!((channel - 0.38857286).abs() < 0.000001);
        }
    }

    #[test]
    fn published_srgb_red_oklab_coordinates_convert_back_to_red() {
        let rgb = Rgba::from(Oklch::new(0.62795536, 0.25768331, 29.233885).paint());
        assert!((rgb.r - 1.).abs() < 0.0001);
        assert!(rgb.g < 0.0001 && rgb.b < 0.0001);
    }

    #[test]
    // Preserve the pinned Kumo brand value, not Euler's constant.
    #[allow(clippy::approx_constant)]
    fn out_of_gamut_action_is_bounded_and_keeps_lightness() {
        let rgb = Rgba::from(Oklch::new(0.5772, 0.2324, 260.).paint());
        assert!(
            [rgb.r, rgb.g, rgb.b]
                .into_iter()
                .all(|v| v.is_finite() && (0. ..=1.).contains(&v))
        );
        let decode = |v: f32| {
            let v = f64::from(v);
            if v <= 0.04045 {
                v / 12.92
            } else {
                ((v + 0.055) / 1.055).powf(2.4)
            }
        };
        let (r, g, b) = (decode(rgb.r), decode(rgb.g), decode(rgb.b));
        let l = (0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b).cbrt();
        let m = (0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b).cbrt();
        let s = (0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b).cbrt();
        assert!((0.2104542553 * l + 0.793617785 * m - 0.0040720468 * s - 0.5772).abs() < 0.00001);
    }
}
