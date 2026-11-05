use egui::Color32;
use crate::schedule::ColorRgb;

// Paleta oscura profunda para máximo contraste con textos y dígitos neón:
pub const BG_WIDGET: Color32 = Color32::from_rgb(10, 13, 20); // Negro azulado muy oscuro y limpio
pub const BORDER_WIDGET: Color32 = Color32::from_rgba_premultiplied(255, 255, 255, 30);

// Tarjeta interior izquierda: fondo negro profundo (#05070D) con degradado azul intenso abajo
pub const CARD_BASE_DARK: Color32 = Color32::from_rgb(5, 7, 13);
pub const GLOW_BLUE: Color32 = Color32::from_rgb(30, 95, 255);

// Colores Neón vivos de alto contraste para dígitos y textos:
pub const NEON_CYAN: Color32 = Color32::from_rgb(34, 211, 238);   // Cian neón brillante para el contador
pub const NEON_MINT: Color32 = Color32::from_rgb(52, 211, 153);   // Verde menta neón
pub const TEXT_WHITE: Color32 = Color32::from_rgb(255, 255, 255); // Blanco puro
pub const TEXT_BRIGHT: Color32 = Color32::from_rgb(226, 232, 240);// Blanco níveo de alta legibilidad
pub const TEXT_MUTED: Color32 = Color32::from_rgb(148, 163, 184); // Gris claro legible
pub const TEXT_GRAY: Color32 = Color32::from_rgb(160, 174, 192);  // Gris suave legible

impl From<ColorRgb> for Color32 {
    fn from(c: ColorRgb) -> Self {
        Color32::from_rgb(c.r, c.g, c.b)
    }
}

