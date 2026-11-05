use egui::Color32;
use crate::schedule::ColorRgb;

// Paleta monocromática pura y OLED
pub const PURE_BLACK: Color32 = Color32::from_rgb(0, 0, 0); // #000000 Negro puro OLED
pub const SURFACE_DARK: Color32 = Color32::from_rgb(8, 8, 8); // Superficie casi negra
pub const CARD_BLACK: Color32 = Color32::from_rgb(4, 4, 4); // Tarjeta negra pura
pub const BORDER_COLOR: Color32 = Color32::from_rgb(32, 32, 32); // Borde recto sutil

// Neones de alto contraste sobre fondo negro
pub const NEON_CYAN: Color32 = Color32::from_rgb(0, 245, 255); // #00F5FF Neón cian vibrante
pub const NEON_GREEN: Color32 = Color32::from_rgb(57, 255, 20); // #39FF14 Neón verde
pub const NEON_AMBER: Color32 = Color32::from_rgb(255, 176, 0); // #FFB000 Ámbar neón
pub const NEON_PURPLE: Color32 = Color32::from_rgb(180, 75, 255); // Violeta neón

// Textos monocromáticos
pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(245, 245, 245);
pub const TEXT_MUTED: Color32 = Color32::from_rgb(105, 105, 105);
pub const TEXT_DARK: Color32 = Color32::from_rgb(60, 60, 60);

impl From<ColorRgb> for Color32 {
    fn from(c: ColorRgb) -> Self {
        Color32::from_rgb(c.r, c.g, c.b)
    }
}
