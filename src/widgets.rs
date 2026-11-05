use egui::{pos2, vec2, Color32, FontId, Pos2, Rect, Response, Rounding, Sense, Stroke, Ui, Vec2};
use crate::theme::{CARD_BLACK, NEON_CYAN, TEXT_DARK, TEXT_MUTED, TEXT_PRIMARY};

/// Tarjeta izquierda en negro puro (#000000) con resplandor neón inferior
/// y dígitos monocromáticos / neón cian luminosos
pub fn draw_oled_timer_card(
    ui: &mut Ui,
    rect: Rect,
    date_text: &str,
    current_time_str: &str,
    countdown_str: &str,
    countdown_label: &str,
    neon_color: Color32,
    progress: f32,
    iteration_info: &str,
) {
    let painter = ui.painter();

    // 1. Fondo negro puro
    painter.rect_filled(rect, Rounding::ZERO, CARD_BLACK);

    // 2. Halo inferior degradado neón tenue
    let glow_height = rect.height() * 0.38;
    let steps = 18;
    for i in 0..steps {
        let t = i as f32 / steps as f32;
        let y_start = rect.max.y - glow_height * (1.0 - t);
        let y_end = rect.max.y - glow_height * (1.0 - (t + 1.0 / steps as f32));
        
        let alpha = (t * t * 140.0) as u8;
        let step_color = Color32::from_rgba_premultiplied(
            ((neon_color.r() as f32) * t) as u8,
            ((neon_color.g() as f32) * t) as u8,
            ((neon_color.b() as f32) * t) as u8,
            alpha,
        );

        let strip_rect = Rect::from_min_max(
            pos2(rect.min.x, y_start),
            pos2(rect.max.x, y_end.min(rect.max.y)),
        );
        painter.rect_filled(strip_rect, Rounding::ZERO, step_color);
    }

    // 3. Borde fino
    painter.rect_stroke(
        rect,
        Rounding::ZERO,
        Stroke::new(1.0, Color32::from_rgb(35, 35, 35)),
    );

    // 4. Arriba: Fecha y Hora actual en vivo
    painter.text(
        pos2(rect.min.x + 16.0, rect.min.y + 16.0),
        egui::Align2::LEFT_TOP,
        date_text,
        FontId::proportional(16.0),
        TEXT_PRIMARY,
    );

    painter.text(
        pos2(rect.min.x + 16.0, rect.min.y + 38.0),
        egui::Align2::LEFT_TOP,
        format!("RELOJ  {}", current_time_str),
        FontId::monospace(13.0),
        NEON_CYAN,
    );

    // 5. Dígitos Neón del temporizador regresivo
    painter.text(
        pos2(rect.center().x, rect.min.y + 98.0),
        egui::Align2::CENTER_CENTER,
        countdown_str,
        FontId::monospace(32.0),
        neon_color,
    );

    painter.text(
        pos2(rect.center().x, rect.min.y + 126.0),
        egui::Align2::CENTER_CENTER,
        countdown_label,
        FontId::proportional(12.0),
        TEXT_PRIMARY,
    );

    // Barra de progreso neón
    let bar_rect = Rect::from_center_size(
        pos2(rect.center().x, rect.min.y + 146.0),
        vec2(rect.width() - 32.0, 3.0),
    );
    painter.rect_filled(
        bar_rect,
        Rounding::ZERO,
        Color32::from_rgb(25, 25, 25),
    );
    let active_w = (rect.width() - 32.0) * progress.clamp(0.0, 1.0);
    if active_w > 0.0 {
        let active_fill = Rect::from_min_size(bar_rect.min, vec2(active_w, 3.0));
        painter.rect_filled(active_fill, Rounding::ZERO, neon_color);
    }

    // 6. Abajo: Indicador de automejora recursiva / estado
    painter.text(
        pos2(rect.min.x + 16.0, rect.max.y - 14.0),
        egui::Align2::LEFT_BOTTOM,
        iteration_info,
        FontId::monospace(10.5),
        TEXT_MUTED,
    );
}

/// Fila monocromática de turno en lista derecha
pub fn draw_monochrome_turn_item(
    ui: &mut Ui,
    title: &str,
    time_range: &str,
    status_text: &str,
    neon_color: Color32,
    is_active: bool,
    is_completed: bool,
) -> Response {
    let desired_size = vec2(ui.available_width(), 44.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());
    let painter = ui.painter();

    if response.hovered() || is_active {
        let hover_bg = if is_active {
            Color32::from_rgb(18, 18, 18)
        } else {
            Color32::from_rgb(12, 12, 12)
        };
        painter.rect_filled(rect, Rounding::ZERO, hover_bg);
    }

    // Borde inferior separador muy sutil
    painter.line_segment(
        [
            pos2(rect.min.x, rect.max.y),
            pos2(rect.max.x, rect.max.y),
        ],
        Stroke::new(1.0, Color32::from_rgb(22, 22, 22)),
    );

    // Barra indicadora izquierda
    let bar_rect = Rect::from_min_size(
        pos2(rect.min.x + 2.0, rect.min.y + 6.0),
        vec2(3.0, 32.0),
    );
    let bar_color = if is_completed {
        TEXT_DARK
    } else if is_active {
        neon_color
    } else {
        Color32::from_rgb(70, 70, 70)
    };
    painter.rect_filled(bar_rect, Rounding::ZERO, bar_color);

    // Título
    let title_color = if is_completed {
        TEXT_DARK
    } else if is_active {
        Color32::WHITE
    } else {
        TEXT_PRIMARY
    };

    let title_pos = pos2(rect.min.x + 16.0, rect.min.y + 6.0);
    painter.text(
        title_pos,
        egui::Align2::LEFT_TOP,
        title,
        FontId::proportional(14.0),
        title_color,
    );

    // Tachado si completado
    if is_completed {
        let text_len = title.len() as f32 * 8.0;
        painter.line_segment(
            [
                pos2(title_pos.x, title_pos.y + 8.5),
                pos2(title_pos.x + text_len, title_pos.y + 8.5),
            ],
            Stroke::new(1.2, TEXT_MUTED),
        );
    }

    // Rango horario
    painter.text(
        pos2(rect.min.x + 16.0, rect.min.y + 24.0),
        egui::Align2::LEFT_TOP,
        time_range,
        FontId::monospace(11.5),
        TEXT_MUTED,
    );

    // Estado a la derecha
    let status_color = if is_completed {
        TEXT_DARK
    } else if is_active {
        neon_color
    } else {
        TEXT_MUTED
    };

    painter.text(
        pos2(rect.max.x - 6.0, rect.center().y),
        egui::Align2::RIGHT_CENTER,
        status_text,
        FontId::proportional(11.5),
        status_color,
    );

    response
}

/// Botón rectangular integrado "+" para la cabecera (perfectamente visible y ubicado)
pub fn draw_header_add_button(ui: &mut Ui) -> Response {
    let size = vec2(68.0, 24.0);
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());
    let painter = ui.painter();

    let is_hovered = response.hovered();
    let bg_color = if is_hovered {
        Color32::from_rgb(28, 28, 28)
    } else {
        Color32::from_rgb(14, 14, 14)
    };

    let stroke_color = if is_hovered {
        NEON_CYAN
    } else {
        Color32::from_rgb(50, 50, 50)
    };

    painter.rect_filled(rect, Rounding::ZERO, bg_color);
    painter.rect_stroke(rect, Rounding::ZERO, Stroke::new(1.0, stroke_color));

    let text_col = if is_hovered {
        Color32::WHITE
    } else {
        TEXT_PRIMARY
    };

    painter.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        "+ NUEVO",
        FontId::monospace(11.0),
        text_col,
    );

    response
}
