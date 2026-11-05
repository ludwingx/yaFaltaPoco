use egui::Color32;
use crate::schedule::ColorRgb;

// Paleta visual idéntica a la imagen de referencia:
// Fondo oscuro gris-azulado semitransparente con esquinas redondeadas
pub const BG_WIDGET: Color32 = Color32::from_rgb(26, 31, 46);
pub const BORDER_WIDGET: Color32 = Color32::from_rgba_premultiplied(255, 255, 255, 22);

// Tarjeta interior izquierda: negro puro arriba y azul luminoso abajo
pub const CARD_BASE_DARK: Color32 = Color32::from_rgb(12, 16, 26);
pub const GLOW_BLUE: Color32 = Color32::from_rgb(45, 115, 255);

// Textos
pub const TEXT_WHITE: Color32 = Color32::from_rgb(255, 255, 255);
pub const TEXT_GRAY: Color32 = Color32::from_rgb(148, 163, 184);

impl From<ColorRgb> for Color32 {
    fn from(c: ColorRgb) -> Self {
        Color32::from_rgb(c.r, c.g, c.b)
    }
}
