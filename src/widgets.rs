use egui::{pos2, vec2, Color32, FontId, Pos2, Rect, Response, Rounding, Sense, Stroke, Ui, Vec2};
use crate::theme::{CARD_BASE_DARK, GLOW_BLUE, TEXT_GRAY, TEXT_WHITE};

/// Dibuja la tarjeta izquierda con el degradado azul vertical en la base,
/// esquinas redondeadas (24px) y la fecha, reloj y temporizador
pub fn draw_card_with_gradient(
    ui: &mut Ui,
    rect: Rect,
    date_title: &str,
    current_time_str: &str,
    countdown_str: &str,
    block_name: &str,
    weekday_str: &str,
    events_count_str: &str,
    glow_color: Color32,
) {
    let painter = ui.painter();
    let card_rounding = Rounding::same(24.0);

    // 1. Fondo oscuro de la tarjeta
    painter.rect_filled(rect, card_rounding, CARD_BASE_DARK);

    // 2. Degradado vertical azul/color en la base (idéntico a la imagen)
    let glow_height = rect.height() * 0.46;
    let steps = 24;
    for i in 0..steps {
        let t = i as f32 / steps as f32; // 0.0 a 1.0
        let y_start = rect.max.y - glow_height * (1.0 - t);
        let y_end = rect.max.y - glow_height * (1.0 - (t + 1.0 / steps as f32));

        let alpha = (t * t * 230.0) as u8;
        let step_color = Color32::from_rgba_premultiplied(
            ((glow_color.r() as f32) * t) as u8,
            ((glow_color.g() as f32) * t) as u8,
            ((glow_color.b() as f32) * t) as u8,
            alpha,
        );

        let round = if i == steps - 1 {
            Rounding { nw: 0.0, ne: 0.0, sw: 24.0, se: 24.0 }
        } else {
            Rounding::ZERO
        };

        let strip_rect = Rect::from_min_max(
            pos2(rect.min.x, y_start),
            pos2(rect.max.x, y_end.min(rect.max.y)),
        );
        painter.rect_filled(strip_rect, round, step_color);
    }

    // 3. Borde de luz sutil
    painter.rect_stroke(
        rect,
        card_rounding,
        Stroke::new(1.0, Color32::from_rgba_premultiplied(255, 255, 255, 22)),
    );

    // 4. Arriba en grande y negrita: Fecha ("Oct 30") y Hora actual
    painter.text(
        pos2(rect.min.x + 20.0, rect.min.y + 20.0),
        egui::Align2::LEFT_TOP,
        date_title,
        FontId::proportional(28.0),
        TEXT_WHITE,
    );

    painter.text(
        pos2(rect.min.x + 20.0, rect.min.y + 54.0),
        egui::Align2::LEFT_TOP,
        format!("Hora: {}", current_time_str),
        FontId::monospace(12.5),
        TEXT_GRAY,
    );

    // 5. Centro: Temporizador regresivo en vivo
    painter.text(
        pos2(rect.center().x, rect.min.y + 104.0),
        egui::Align2::CENTER_CENTER,
        countdown_str,
        FontId::monospace(28.0),
        TEXT_WHITE,
    );

    painter.text(
        pos2(rect.center().x, rect.min.y + 128.0),
        egui::Align2::CENTER_CENTER,
        block_name,
        FontId::proportional(12.0),
        Color32::from_rgb(220, 235, 255),
    );

    // 6. Abajo: Día de la semana ("Jueves") y número de bloques ("3 bloques")
    painter.text(
        pos2(rect.min.x + 20.0, rect.max.y - 42.0),
        egui::Align2::LEFT_BOTTOM,
        weekday_str,
        FontId::proportional(15.0),
        TEXT_WHITE,
    );
    painter.text(
        pos2(rect.min.x + 20.0, rect.max.y - 20.0),
        egui::Align2::LEFT_BOTTOM,
        events_count_str,
        FontId::proportional(13.5),
        Color32::from_rgb(180, 210, 255),
    );
}

/// Fila de la lista derecha (estilo limpio de la foto)
pub fn draw_event_row(
    ui: &mut Ui,
    title: &str,
    time_range: &str,
    color: Color32,
    is_active: bool,
    is_completed: bool,
) -> Response {
    let desired_size = vec2(ui.available_width(), 44.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());
    let painter = ui.painter();

    if response.hovered() || is_active {
        let hover_bg = if is_active {
            Color32::from_rgba_premultiplied(color.r(), color.g(), color.b(), 28)
        } else {
            Color32::from_rgba_premultiplied(255, 255, 255, 10)
        };
        painter.rect_filled(rect, Rounding::same(8.0), hover_bg);
    }

    // Barra vertical de color (exacta a la de la foto)
    let bar_rect = Rect::from_min_size(
        pos2(rect.min.x + 6.0, rect.min.y + 6.0),
        vec2(3.5, 32.0),
    );
    let bar_col = if is_completed {
        Color32::from_rgb(70, 80, 100)
    } else {
        color
    };
    painter.rect_filled(bar_rect, Rounding::same(2.0), bar_col);

    // Nombre del bloque
    let title_pos = pos2(rect.min.x + 18.0, rect.min.y + 7.0);
    let title_color = if is_completed {
        Color32::from_rgb(115, 125, 145)
    } else if is_active {
        Color32::WHITE
    } else {
        Color32::from_rgb(235, 240, 255)
    };

    painter.text(
        title_pos,
        egui::Align2::LEFT_TOP,
        title,
        FontId::proportional(14.5),
        title_color,
    );

    // Subrayado / tachado si ya terminó
    if is_completed {
        let text_w = title.len() as f32 * 7.8;
        painter.line_segment(
            [
                pos2(title_pos.x, title_pos.y + 9.0),
                pos2(title_pos.x + text_w, title_pos.y + 9.0),
            ],
            Stroke::new(1.3, Color32::from_rgb(140, 150, 170)),
        );
    }

    // Horario (ej: 08:00 - 12:00)
    painter.text(
        pos2(rect.min.x + 18.0, rect.min.y + 25.0),
        egui::Align2::LEFT_TOP,
        time_range,
        FontId::proportional(12.0),
        TEXT_GRAY,
    );

    response
}

/// Botón circular flotante "+" con aro luminoso verde/cian idéntico a la imagen
pub fn draw_floating_plus(ui: &mut Ui, center: Pos2, radius: f32) -> Response {
    let size = Vec2::splat(radius * 2.0);
    let rect = Rect::from_center_size(center, size);
    let response = ui.allocate_rect(rect, Sense::click());
    let painter = ui.painter();

    let is_hovered = response.hovered();

    // Aro exterior brillante con degradado suave
    let ring_alpha: u8 = if is_hovered { 255 } else { 180 };
    painter.circle_stroke(
        center,
        radius + 1.5,
        Stroke::new(2.2, Color32::from_rgba_premultiplied(52, 211, 153, ring_alpha)),
    );

    // Fondo oscuro interior del botón
    let bg_color = if is_hovered {
        Color32::from_rgb(26, 32, 48)
    } else {
        Color32::from_rgb(16, 20, 32)
    };
    painter.circle_filled(center, radius, bg_color);

    // Signo "+" blanco
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
