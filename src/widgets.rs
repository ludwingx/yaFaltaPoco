use egui::{pos2, vec2, Color32, FontId, Pos2, Rect, Response, Rounding, Sense, Stroke, Ui, Vec2};
use crate::theme::{CARD_BG, TEXT_MUTED};

/// Dibuja la tarjeta izquierda con bordes RECTOS (sin redondear),
/// mostrando la fecha, hora actual del sistema, y el temporizador protagonista
pub fn draw_timer_card(
    ui: &mut Ui,
    rect: Rect,
    date_text: &str,
    current_time_str: &str,
    countdown_str: &str,
    countdown_label: &str,
    bottom_glow_color: Color32,
    progress: f32,
) {
    let painter = ui.painter();

    // 1. Fondo base recto de la tarjeta
    painter.rect_filled(rect, Rounding::ZERO, CARD_BG);

    // 2. Degradado vertical inferior naciendo desde la base hacia arriba
    let glow_height = rect.height() * 0.45;
    let steps = 20;
    for i in 0..steps {
        let t = i as f32 / steps as f32;
        let y_start = rect.max.y - glow_height * (1.0 - t);
        let y_end = rect.max.y - glow_height * (1.0 - (t + 1.0 / steps as f32));
        
        let alpha = (t * t * 230.0) as u8;
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
        painter.rect_filled(strip_rect, Rounding::ZERO, step_color);
    }

    // 3. Borde recto fino
    painter.rect_stroke(
        rect,
        Rounding::ZERO,
        Stroke::new(1.0, Color32::from_rgba_premultiplied(255, 255, 255, 25)),
    );

    // 4. Arriba: Fecha ("Nov 05") y Hora actual ("17:25:40")
    painter.text(
        pos2(rect.min.x + 18.0, rect.min.y + 16.0),
        egui::Align2::LEFT_TOP,
        date_text,
        FontId::proportional(22.0),
        Color32::WHITE,
    );
    painter.text(
        pos2(rect.min.x + 18.0, rect.min.y + 45.0),
        egui::Align2::LEFT_TOP,
        format!("Hora: {}", current_time_str),
        FontId::monospace(13.0),
        Color32::from_rgb(160, 180, 215),
    );

    // 5. Centro: Contador regresivo descontando en tiempo real
    painter.text(
        pos2(rect.center().x, rect.min.y + 105.0),
        egui::Align2::CENTER_CENTER,
        countdown_str,
        FontId::monospace(26.0),
        Color32::WHITE,
    );

    painter.text(
        pos2(rect.center().x, rect.min.y + 130.0),
        egui::Align2::CENTER_CENTER,
        countdown_label,
        FontId::proportional(12.5),
        Color32::from_rgb(200, 220, 255),
    );

    // Barra de progreso del bloque
    let bar_rect = Rect::from_center_size(
        pos2(rect.center().x, rect.min.y + 152.0),
        vec2(rect.width() - 36.0, 4.0),
    );
    painter.rect_filled(
        bar_rect,
        Rounding::ZERO,
        Color32::from_rgba_premultiplied(255, 255, 255, 40),
    );
    let active_w = (rect.width() - 36.0) * progress.clamp(0.0, 1.0);
    if active_w > 0.0 {
        let active_fill = Rect::from_min_size(bar_rect.min, vec2(active_w, 4.0));
        painter.rect_filled(active_fill, Rounding::ZERO, bottom_glow_color);
    }

    // 6. Abajo: "Ya falta poco…"
    painter.text(
        pos2(rect.min.x + 18.0, rect.max.y - 18.0),
        egui::Align2::LEFT_BOTTOM,
        "Ya falta poco…",
        FontId::proportional(12.0),
        Color32::from_rgb(180, 205, 255),
    );
}

/// Dibuja un item de turno de la lista derecha.
/// Si está completado (`is_completed == true`), se muestra tachado/subrayado y atenuado.
pub fn draw_turn_item(
    ui: &mut Ui,
    title: &str,
    time_range: &str,
    status_text: &str,
    color: Color32,
    is_active: bool,
    is_completed: bool,
) -> Response {
    let desired_size = vec2(ui.available_width(), 46.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());
    let painter = ui.painter();

    // Fondo recto si hover o activo
    if response.hovered() || is_active {
        let hover_bg = if is_active {
            Color32::from_rgba_premultiplied(color.r(), color.g(), color.b(), 35)
        } else {
            Color32::from_rgba_premultiplied(255, 255, 255, 12)
        };
        painter.rect_filled(rect, Rounding::ZERO, hover_bg);
    }

    // Barra vertical de color recta
    let bar_rect = Rect::from_min_size(
        pos2(rect.min.x + 4.0, rect.min.y + 6.0),
        vec2(4.0, 34.0),
    );
    let bar_color = if is_completed {
        Color32::from_rgb(80, 90, 110)
    } else {
        color
    };
    painter.rect_filled(bar_rect, Rounding::ZERO, bar_color);

    // Título del turno
    let title_color = if is_completed {
        Color32::from_rgb(110, 120, 140)
    } else if is_active {
        Color32::WHITE
    } else {
        Color32::from_rgb(220, 230, 245)
    };

    let title_pos = pos2(rect.min.x + 18.0, rect.min.y + 7.0);
    painter.text(
        title_pos,
        egui::Align2::LEFT_TOP,
        title,
        FontId::proportional(14.5),
        title_color,
    );

    // Si está completado: tachado/subrayado horizontal
    if is_completed {
        let text_len = title.len() as f32 * 8.2;
        painter.line_segment(
            [
                pos2(title_pos.x, title_pos.y + 9.0),
                pos2(title_pos.x + text_len, title_pos.y + 9.0),
            ],
            Stroke::new(1.4, Color32::from_rgb(140, 150, 170)),
        );
    }

    // Horario abajo (ej: 08:00 - 12:00)
    painter.text(
        pos2(rect.min.x + 18.0, rect.min.y + 26.0),
        egui::Align2::LEFT_TOP,
        time_range,
        FontId::proportional(12.0),
        TEXT_MUTED,
    );

    // Estado a la derecha (ej: "En curso", "Faltan 2h 10m", "Terminado ✓")
    let status_color = if is_completed {
        Color32::from_rgb(100, 116, 139)
    } else if is_active {
        Color32::from_rgb(52, 211, 153)
    } else {
        Color32::from_rgb(160, 175, 205)
    };
    painter.text(
        pos2(rect.max.x - 8.0, rect.center().y),
        egui::Align2::RIGHT_CENTER,
        status_text,
        FontId::proportional(12.0),
        status_color,
    );

    response
}

/// Botón circular "+" flotante en esquina inferior derecha
pub fn draw_add_button(ui: &mut Ui, center: Pos2, radius: f32) -> Response {
    let size = Vec2::splat(radius * 2.0);
    let rect = Rect::from_center_size(center, size);
    let response = ui.allocate_rect(rect, Sense::click());
    let painter = ui.painter();

    let is_hovered = response.hovered();

    let glow_alpha: u8 = if is_hovered { 120 } else { 70 };
    for r in 1..=4 {
        let rad = radius + (r as f32) * 1.5;
        let a = (glow_alpha as f32 * (1.0 - (r as f32 / 5.0))) as u8;
        painter.circle_stroke(
            center,
            rad,
            Stroke::new(1.5, Color32::from_rgba_premultiplied(37, 99, 235, a)),
        );
    }

    painter.circle_stroke(
        center,
        radius + 1.0,
        Stroke::new(2.0, Color32::from_rgb(37, 99, 235)),
    );

    let core_color = if is_hovered {
        Color32::from_rgb(30, 42, 65)
    } else {
        Color32::from_rgb(16, 22, 35)
    };
    painter.circle_filled(center, radius, core_color);

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
