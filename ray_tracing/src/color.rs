use std::fmt;
use std::ops::{Add, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color {
            r: r.clamp(0, 255),
            g: g.clamp(0, 255),
            b: b.clamp(0, 255),
        }
    }

    pub fn from_hex(hex: u32) -> Color {
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;
        Color { r, g, b }
    }

    pub fn to_hex(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, color2: Color) -> Color {
        let r = self.r.saturating_add(color2.r);
        let g = self.g.saturating_add(color2.g);
        let b = self.b.saturating_add(color2.b);

        Color { r, g, b }
    }
}

impl Mul<i32> for Color {
    type Output = Color;

    fn mul(self, factor: i32) -> Color {
        let r = (self.r as i32 * factor).clamp(0, 255) as u8;
        let g = (self.g as i32 * factor).clamp(0, 255) as u8;
        let b = (self.b as i32 * factor).clamp(0, 255) as u8;
        Color { r, g, b }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Color(r: {}, g: {}, b: {})", self.r, self.g, self.b)
    }
}


