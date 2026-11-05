use egui::Color32;
use crate::schedule::ColorRgb;

// Paleta oscura limpia basada en egui-ui-design y la referencia visual:
pub const BG_WIDGET: Color32 = Color32::from_rgb(18, 22, 33); // Tono pizarra oscuro #121621
pub const BORDER_WIDGET: Color32 = Color32::from_rgba_premultiplied(255, 255, 255, 20);

// Tarjeta interior izquierda: negro suave (#0A0C14) con brillo azul degradado en la base
pub const CARD_BASE_DARK: Color32 = Color32::from_rgb(10, 12, 20);
pub const GLOW_BLUE: Color32 = Color32::from_rgb(37, 99, 235); // Azul zafiro vibrante (#2563EB)

// Acentos y Tipografía
pub const NEON_CYAN: Color32 = Color32::from_rgb(34, 211, 238);   // Cian neón suave para el contador
pub const NEON_MINT: Color32 = Color32::from_rgb(52, 211, 153);   // Menta para el botón circular
pub const TEXT_WHITE: Color32 = Color32::from_rgb(255, 255, 255); // Blanco
pub const TEXT_BRIGHT: Color32 = Color32::from_rgb(226, 232, 240);// Texto primario
pub const TEXT_MUTED: Color32 = Color32::from_rgb(148, 163, 184); // Texto secundario
pub const TEXT_GRAY: Color32 = Color32::from_rgb(120, 135, 160);  // Controles y detalles

impl From<ColorRgb> for Color32 {
    fn from(c: ColorRgb) -> Self {
        Color32::from_rgb(c.r, c.g, c.b)
    }
}


