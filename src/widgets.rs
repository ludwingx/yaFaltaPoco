use egui::{pos2, vec2, Color32, FontId, Pos2, Rect, Response, Rounding, Sense, Stroke, Ui, Vec2};

/// Colores idénticos a la imagen de referencia:
/// Fondo general: #161a29 con borde sutil translúcido
pub const WIDGET_BG: Color32 = Color32::from_rgb(22, 26, 41);
pub const WIDGET_BORDER: Color32 = Color32::from_rgba_premultiplied(255, 255, 255, 25);

/// Tarjeta izquierda: Fondo oscuro casi negro (#0d111a) con un degradado azul (#2563eb / #3b82f6)
/// emergiendo intensamente desde la base inferior hacia arriba
pub fn draw_reference_card(
    ui: &mut Ui,
    rect: Rect,
    headline: &str,
    subhead: &str,
    count_text: &str,
    bottom_glow_color: Color32,
) {
    let painter = ui.painter();
    let card_rounding = Rounding::same(24.0);

    // 1. Fondo base de la tarjeta (Negro grafito profundo)
    painter.rect_filled(rect, card_rounding, Color32::from_rgb(13, 16, 26));

    // 2. Degradado vertical inferior (como en la foto de referencia)
    let glow_height = rect.height() * 0.48;
    let steps = 24;
    for i in 0..steps {
        let t = i as f32 / steps as f32; // 0.0 arriba a 1.0 abajo
        let y_start = rect.max.y - glow_height * (1.0 - t);
        let y_end = rect.max.y - glow_height * (1.0 - (t + 1.0 / steps as f32));
        
        let alpha = (t * t * 240.0) as u8;
        let step_color = Color32::from_rgba_premultiplied(
            ((bottom_glow_color.r() as f32) * t) as u8,
            ((bottom_glow_color.g() as f32) * t) as u8,
            ((bottom_glow_color.b() as f32) * t) as u8,
            alpha,
        );

        let strip_rect = Rect::from_min_max(
            pos2(rect.min.x, y_start),
            pos2(rect.max.x, y_end.min(rect.max.y)),
        );
        let round = if i == steps - 1 {
            Rounding { nw: 0.0, ne: 0.0, sw: 24.0, se: 24.0 }
        } else {
            Rounding::ZERO
        };
        painter.rect_filled(strip_rect, round, step_color);
    }

    // 3. Borde sutil superior/lateral
    painter.rect_stroke(
        rect,
        card_rounding,
        Stroke::new(1.0, Color32::from_rgba_premultiplied(255, 255, 255, 18)),
    );

    // 4. Tipografía en la tarjeta:
    // Arriba: "Oct 30" (o tiempo restante / fecha) en blanco grande nítido y limpio
    painter.text(
        pos2(rect.min.x + 22.0, rect.min.y + 26.0),
        egui::Align2::LEFT_TOP,
        headline,
        FontId::proportional(28.0),
        Color32::WHITE,
    );

    // Abajo: "Thursday" / "6 events" (Día y conteo)
    painter.text(
        pos2(rect.min.x + 22.0, rect.max.y - 48.0),
        egui::Align2::LEFT_BOTTOM,
        subhead,
        FontId::proportional(15.0),
        Color32::WHITE,
    );
    painter.text(
        pos2(rect.min.x + 22.0, rect.max.y - 24.0),
        egui::Align2::LEFT_BOTTOM,
        count_text,
        FontId::proportional(14.0),
        Color32::from_rgb(180, 205, 255),
    );
}

/// Fila limpia de la lista "UPCOMING" (igual al diseño de la imagen)
pub fn draw_upcoming_item(
    ui: &mut Ui,
    title: &str,
    time_range: &str,
    color: Color32,
    is_active: bool,
) -> Response {
    let desired_size = vec2(ui.available_width(), 44.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());
    let painter = ui.painter();

    if response.hovered() || is_active {
        let hover_bg = if is_active {
            Color32::from_rgba_premultiplied(color.r(), color.g(), color.b(), 25)
        } else {
            Color32::from_rgba_premultiplied(255, 255, 255, 10)
        };
        painter.rect_filled(rect, Rounding::same(10.0), hover_bg);
    }

    // Píldora/Barra vertical de color (exacta a la imagen)
    let bar_rect = Rect::from_min_size(
        pos2(rect.min.x + 6.0, rect.min.y + 6.0),
        vec2(3.5, 32.0),
    );
    painter.rect_filled(bar_rect, Rounding::same(2.0), color);

    // Texto: Título (ej. "Daily Meeting")
    painter.text(
        pos2(rect.min.x + 20.0, rect.min.y + 7.0),
        egui::Align2::LEFT_TOP,
        title,
        FontId::proportional(15.0),
        Color32::from_rgb(235, 240, 255),
    );

    // Subtítulo: Rango horario (ej. "10:00 - 10:30")
    painter.text(
        pos2(rect.min.x + 20.0, rect.min.y + 25.0),
        egui::Align2::LEFT_TOP,
        time_range,
        FontId::proportional(12.5),
        Color32::from_rgb(120, 135, 165),
    );

    response
}

/// Botón circular "+" con anillo de resplandor degradado idéntico a la imagen de referencia
pub fn draw_glowing_add_button(ui: &mut Ui, center: Pos2, radius: f32) -> Response {
    let size = Vec2::splat(radius * 2.0);
    let rect = Rect::from_center_size(center, size);
    let response = ui.allocate_rect(rect, Sense::click());
    let painter = ui.painter();

    let is_hovered = response.hovered();

    // 1. Resplandor difuso exterior (multi-capas)
    let glow_alpha: u8 = if is_hovered { 110 } else { 65 };
    for r in 1..=6 {
        let rad = radius + (r as f32) * 1.5;
        let a = (glow_alpha as f32 * (1.0 - (r as f32 / 7.0))) as u8;
        painter.circle_stroke(
            center,
            rad,
            Stroke::new(1.8, Color32::from_rgba_premultiplied(34, 197, 94, a)),
        );
    }

    // 2. Anillo fino degradado (verde menta / cian)
    painter.circle_stroke(
        center,
        radius + 1.0,
        Stroke::new(2.2, Color32::from_rgb(52, 211, 153)),
    );

    // 3. Núcleo oscuro del botón
    let core_color = if is_hovered {
        Color32::from_rgb(28, 35, 52)
    } else {
        Color32::from_rgb(17, 22, 35)
    };
    painter.circle_filled(center, radius, core_color);

    // 4. Signo "+" blanco fino y nítido
    let plus_len = radius * 0.42;
    let plus_stroke = Stroke::new(2.0, Color32::WHITE);
    painter.line_segment(
        [
            pos2(center.x - plus_len, center.y),
            pos2(center.x + plus_len, center.y),
        ],
        plus_stroke,
    );
    painter.line_segment(
        [
            pos2(center.x, center.y - plus_len),
            pos2(center.x, center.y + plus_len),
        ],
        plus_stroke,
    );

    response
}
