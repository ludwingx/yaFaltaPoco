use egui::Color32;
use crate::schedule::ColorRgb;

pub const WIDGET_BG: Color32 = Color32::from_rgb(22, 26, 41);
pub const WIDGET_BORDER: Color32 = Color32::from_rgba_premultiplied(255, 255, 255, 25);
pub const DEFAULT_ACCENT: Color32 = Color32::from_rgb(34, 211, 238);

impl From<ColorRgb> for Color32 {
    fn from(c: ColorRgb) -> Self {
        Color32::from_rgb(c.r, c.g, c.b)
    }
}
