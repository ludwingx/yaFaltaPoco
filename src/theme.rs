use egui::Color32;
use crate::schedule::ColorRgb;

pub const WIDGET_BG: Color32 = Color32::from_rgb(18, 22, 34); // Fondo oscuro nítido
pub const WIDGET_BORDER: Color32 = Color32::from_rgb(38, 45, 66); // Borde recto moderno
pub const CARD_BG: Color32 = Color32::from_rgb(11, 14, 23); // Fondo tarjeta izquierda
pub const TEXT_MUTED: Color32 = Color32::from_rgb(130, 145, 175);

impl From<ColorRgb> for Color32 {
    fn from(c: ColorRgb) -> Self {
        Color32::from_rgb(c.r, c.g, c.b)
    }
}

