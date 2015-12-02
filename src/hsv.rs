use {Color, Rgb, Luma, Xyz, Lab, Lch, Hsl, Mix, Shade, GetHue, Hue, RgbHue, clamp};

///Linear HSV color space with an alpha component.
///
///HSV is a cylindrical version of RGB, where the `value` component determines
///the _brightness_ of the color, and not the _lightness_. The difference is
///that, for example, red (100% R, 0% G, 0% B) and white (100% R, 100% G, 100%
///B) has the same brightness (or value), but not the same lightness.
#[derive(Clone, Debug, PartialEq)]
pub struct Hsv {
    pub hue: RgbHue,
    pub saturation: f32,
    pub value: f32,
    pub alpha: f32,
}

impl Hsv {
    ///Linear HSV.
    pub fn hsv(hue: RgbHue, saturation: f32, value: f32) -> Hsv {
        Hsv {
            hue: hue,
            saturation: saturation,
            value: value,
            alpha: 1.0,
        }
    }

    ///Linear HSV and transparency.
    pub fn hsva(hue: RgbHue, saturation: f32, value: f32, alpha: f32) -> Hsv {
        Hsv {
            hue: hue,
            saturation: saturation,
            value: value,
            alpha: alpha,
        }
    }
}

impl Mix for Hsv {
    fn mix(&self, other: &Hsv, factor: f32) -> Hsv {
        let factor = clamp(factor, 0.0, 1.0);
        let hue_diff: f32 = (other.hue - self.hue).into();

        Hsv {
            hue: self.hue + factor * hue_diff,
            saturation: self.saturation + factor * (other.saturation - self.saturation),
            value: self.value + factor * (other.value - self.value),
            alpha: self.alpha + factor * (other.alpha - self.alpha),
        }
    }
}

impl Shade for Hsv {
    fn lighten(&self, amount: f32) -> Hsv {
        Hsv {
            hue: self.hue,
            saturation: self.saturation,
            value: (self.value + amount).max(0.0),
            alpha: self.alpha,
        }
    }
}

impl GetHue for Hsv {
    type Hue = RgbHue;

    fn get_hue(&self) -> Option<RgbHue> {
        if self.saturation <= 0.0 || self.value <= 0.0 {
            None
        } else {
            Some(self.hue)
        }
    }
}

impl Hue for Hsv {
    fn with_hue(&self, hue: RgbHue) -> Hsv {
        Hsv {
            hue: hue,
            saturation: self.saturation,
            value: self.value,
            alpha: self.alpha,
        }
    }

    fn shift_hue(&self, amount: RgbHue) -> Hsv {
        Hsv {
            hue: self.hue + amount,
            saturation: self.saturation,
            value: self.value,
            alpha: self.alpha,
        }
    }
}

impl Default for Hsv {
    fn default() -> Hsv {
        Hsv::hsv(0.0.into(), 0.0, 0.0)
    }
}

from_color!(to Hsv from Rgb, Luma, Xyz, Lab, Lch, Hsl);

impl From<Rgb> for Hsv {
    fn from(rgb: Rgb) -> Hsv {
        enum Channel { Red, Green, Blue };

        let val_min = rgb.red.min(rgb.green).min(rgb.blue);
        let mut val_max = rgb.red;
        let mut chan_max = Channel::Red;

        if rgb.green > val_max {
            chan_max = Channel::Green;
            val_max = rgb.green;
        }

        if rgb.blue > val_max {
            chan_max = Channel::Blue;
            val_max = rgb.blue;
        }

        let diff = val_max - val_min;

        let hue = if diff == 0.0 {
            0.0
        } else {
            60.0 * match chan_max {
                Channel::Red => ((rgb.green - rgb.blue) / diff) % 6.0,
                Channel::Green => ((rgb.blue - rgb.red) / diff + 2.0),
                Channel::Blue => ((rgb.red - rgb.green) / diff + 4.0),
            }
        };

        let saturation = if val_max == 0.0 {
            0.0
        } else {
            diff / val_max
        };

        Hsv {
            hue: hue.into(),
            saturation: saturation,
            value: val_max,
            alpha: rgb.alpha,
        }
    }
}

impl From<Luma> for Hsv {
    fn from(luma: Luma) -> Hsv {
        Rgb::from(luma).into()
    }
}

impl From<Xyz> for Hsv {
    fn from(xyz: Xyz) -> Hsv {
        Rgb::from(xyz).into()
    }
}

impl From<Lab> for Hsv {
    fn from(lab: Lab) -> Hsv {
        Rgb::from(lab).into()
    }
}

impl From<Lch> for Hsv {
    fn from(lch: Lch) -> Hsv {
        Rgb::from(lch).into()
    }
}

impl From<Hsl> for Hsv {
    fn from(hsl: Hsl) -> Hsv {
        let x = hsl.saturation * if hsl.lightness < 0.5 {
            hsl.lightness
        } else {
            1.0 - hsl.lightness
        };

        Hsv {
            hue: hsl.hue,
            saturation: 2.0 * x / (hsl.lightness + x),
            value: hsl.lightness + x,
            alpha: hsl.alpha,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Hsv;
    use ::{Rgb, Hsl};

    #[test]
    fn red() {
        let a = Hsv::from(Rgb::rgb(1.0, 0.0, 0.0));
        let b = Hsv::hsv(0.0.into(), 1.0, 1.0);
        let c = Hsv::from(Hsl::hsl(0.0.into(), 1.0, 0.5));

        assert_eq!(a, b);
        assert_eq!(a, c);
    }

    #[test]
    fn orange() {
        let a = Hsv::from(Rgb::rgb(1.0, 0.5, 0.0));
        let b = Hsv::hsv(30.0.into(), 1.0, 1.0);
        let c = Hsv::from(Hsl::hsl(30.0.into(), 1.0, 0.5));

        assert_eq!(a, b);
        assert_eq!(a, c);
    }

    #[test]
    fn green() {
        let a = Hsv::from(Rgb::rgb(0.0, 1.0, 0.0));
        let b = Hsv::hsv(120.0.into(), 1.0, 1.0);
        let c = Hsv::from(Hsl::hsl(120.0.into(), 1.0, 0.5));

        assert_eq!(a, b);
        assert_eq!(a, c);
    }

    #[test]
    fn blue() {
        let a = Hsv::from(Rgb::rgb(0.0, 0.0, 1.0));
        let b = Hsv::hsv(240.0.into(), 1.0, 1.0);
        let c = Hsv::from(Hsl::hsl(240.0.into(), 1.0, 0.5));

        assert_eq!(a, b);
        assert_eq!(a, c);
    }

    #[test]
    fn purple() {
        let a = Hsv::from(Rgb::rgb(0.5, 0.0, 1.0));
        let b = Hsv::hsv(270.0.into(), 1.0, 1.0);
        let c = Hsv::from(Hsl::hsl(270.0.into(), 1.0, 0.5));

        assert_eq!(a, b);
        assert_eq!(a, c);
    }
}
