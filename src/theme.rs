use egui::{Color32, Rounding, Stroke};
use crate::schedule::ColorRgb;

pub const DEFAULT_ACCENT: Color32 = Color32::from_rgb(34, 211, 238); // Cyan neón
pub const BG_GLASS: Color32 = Color32::from_rgba_premultiplied(26, 31, 46, 230); // Gris azulado 90%
#[allow(dead_code)]
pub const BG_CARD: Color32 = Color32::from_rgb(15, 20, 32);
pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(255, 255, 255);
pub const TEXT_MUTED: Color32 = Color32::from_rgb(156, 163, 175);
pub const BORDER_COLOR: Color32 = Color32::from_rgba_premultiplied(255, 255, 255, 30);

impl From<ColorRgb> for Color32 {
    fn from(c: ColorRgb) -> Self {
        Color32::from_rgb(c.r, c.g, c.b)
    }
}

pub fn rounded_frame() -> egui::Frame {
    egui::Frame {
        inner_margin: egui::Margin::same(16.0),
        outer_margin: egui::Margin::ZERO,
        rounding: Rounding::same(24.0),
        shadow: egui::epaint::Shadow {
            offset: [0.0, 8.0].into(),
            blur: 16.0,
            spread: 0.0,
            color: Color32::from_black_alpha(100),
        },
        fill: BG_GLASS,
        stroke: Stroke::new(1.0_f32, BORDER_COLOR),
    }
}
