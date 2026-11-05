use egui::{pos2, vec2, Color32, FontId, Pos2, Rect, Response, Rounding, Sense, Stroke, Ui, Vec2};
use crate::theme::{CARD_BASE_DARK, GLOW_BLUE, NEON_CYAN, NEON_MINT, TEXT_BRIGHT, TEXT_MUTED, TEXT_WHITE};

/// Dibuja la tarjeta izquierda limpia y fiel a la referencia visual:
/// Negro puro arriba, base con brillo azul profundo, fecha y contador limpios sin cajas invasivas
pub fn draw_card_with_gradient(
    ui: &mut Ui,
    rect: Rect,
    date_title: &str,
    current_time_str: &str,
    countdown_str: &str,
    block_name: &str,
    weekday_str: &str,
    events_count_str: &str,
) {
    let painter = ui.painter();
    let card_rounding = Rounding::same(24.0);

    // 1. Fondo negro de la tarjeta
    painter.rect_filled(rect, card_rounding, CARD_BASE_DARK);

    // 2. Degradado azul brillante suave desde la base inferior
    let glow_height = rect.height() * 0.45;
    let steps = 28;
    for i in 0..steps {
        let t = i as f32 / steps as f32; // 0.0 a 1.0
        let y_start = rect.max.y - glow_height * (1.0 - t);
        let y_end = rect.max.y - glow_height * (1.0 - (t + 1.0 / steps as f32));

        let alpha = (t.powf(2.2) * 230.0) as u8;
        let step_color = Color32::from_rgba_premultiplied(
            ((GLOW_BLUE.r() as f32) * t) as u8,
            ((GLOW_BLUE.g() as f32) * t) as u8,
            ((GLOW_BLUE.b() as f32) * t) as u8,
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

    // 3. Borde sutil
    painter.rect_stroke(
        rect,
        card_rounding,
        Stroke::new(1.0_f32, Color32::from_rgba_premultiplied(255, 255, 255, 25)),
    );

    // 4. Arriba: Fecha ("Oct 30" / "Nov 05") en tipografía limpia blanca
    painter.text(
        pos2(rect.min.x + 22.0, rect.min.y + 20.0),
        egui::Align2::LEFT_TOP,
        date_title,
        FontId::proportional(28.0),
        TEXT_WHITE,
    );

    // Hora actual sutil debajo de la fecha
    painter.text(
        pos2(rect.min.x + 22.0, rect.min.y + 54.0),
        egui::Align2::LEFT_TOP,
        current_time_str,
        FontId::monospace(12.5),
        Color32::from_rgb(180, 195, 215),
    );

    // 5. Centro: Contador regresivo limpio, tipografía monoespaciada grande y brillante
    painter.text(
        pos2(rect.center().x, rect.min.y + 98.0),
        egui::Align2::CENTER_CENTER,
        countdown_str,
        FontId::monospace(27.0),
        NEON_CYAN,
    );

    // Etiqueta del bloque / turno
    painter.text(
        pos2(rect.center().x, rect.min.y + 120.0),
        egui::Align2::CENTER_CENTER,
        block_name,
        FontId::proportional(12.0),
        Color32::from_rgb(210, 225, 245),
    );

    // 6. Abajo: Día de la semana ("Jueves") y número de bloques ("3 bloques")
    painter.text(
        pos2(rect.min.x + 22.0, rect.max.y - 42.0),
        egui::Align2::LEFT_BOTTOM,
        weekday_str,
        FontId::proportional(16.0),
        TEXT_WHITE,
    );
    painter.text(
        pos2(rect.min.x + 22.0, rect.max.y - 18.0),
        egui::Align2::LEFT_BOTTOM,
        events_count_str,
        FontId::proportional(14.0),
        Color32::from_rgb(175, 205, 250),
    );
}

/// Fila de la lista derecha: minimalista, sin cajas gigantes que saturen la pantalla
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

    // Fondo sutil únicamente si está activo o al pasar el cursor (hover)
    if is_active {
        painter.rect_filled(
            rect,
            Rounding::same(8.0),
            Color32::from_rgba_premultiplied(color.r(), color.g(), color.b(), 35),
        );
        painter.rect_stroke(
            rect,
            Rounding::same(8.0),
            Stroke::new(1.0_f32, Color32::from_rgba_premultiplied(color.r(), color.g(), color.b(), 100)),
        );
    } else if response.hovered() {
        painter.rect_filled(
            rect,
            Rounding::same(8.0),
            Color32::from_rgba_premultiplied(255, 255, 255, 12),
        );
    }

    // Barra vertical fina de color (como en la referencia de diseño)
    let bar_rect = Rect::from_min_size(
        pos2(rect.min.x + 4.0, rect.min.y + 6.0),
        vec2(3.0, 32.0),
    );
    let bar_col = if is_completed {
        Color32::from_rgb(75, 85, 105)
    } else {
        color
    };
    painter.rect_filled(bar_rect, Rounding::same(1.5), bar_col);

    // Título del evento
    let title_pos = pos2(rect.min.x + 16.0, rect.min.y + 6.0);
    let title_color = if is_completed {
        Color32::from_rgb(125, 135, 155)
    } else if is_active {
        TEXT_WHITE
    } else {
        TEXT_BRIGHT
    };

    painter.text(
        title_pos,
        egui::Align2::LEFT_TOP,
        title,
        FontId::proportional(15.0),
        title_color,
    );

    // Tachado sutil si ya terminó
    if is_completed {
        let text_w = title.len() as f32 * 7.8;
        painter.line_segment(
            [
                pos2(title_pos.x, title_pos.y + 9.5),
                pos2(title_pos.x + text_w, title_pos.y + 9.5),
            ],
            Stroke::new(1.2_f32, Color32::from_rgb(140, 150, 170)),
        );
    }

    // Horario limpio y legible
    let time_color = if is_active {
        Color32::from_rgb(165, 215, 255)
    } else if is_completed {
        Color32::from_rgb(105, 115, 135)
    } else {
        TEXT_MUTED
    };

    painter.text(
        pos2(rect.min.x + 16.0, rect.min.y + 25.0),
        egui::Align2::LEFT_TOP,
        time_range,
        FontId::proportional(12.5),
        time_color,
    );

    response
}

/// Botón circular flotante "+" con aro degradado sutil
pub fn draw_floating_plus(ui: &mut Ui, center: Pos2, radius: f32) -> Response {
    let size = Vec2::splat(radius * 2.0);
    let rect = Rect::from_center_size(center, size);
    let response = ui.allocate_rect(rect, Sense::click());
    let painter = ui.painter();

    let is_hovered = response.hovered();

    // Aro exterior luminoso
    let ring_alpha: u8 = if is_hovered { 255 } else { 180 };
    painter.circle_stroke(
        center,
        radius + 1.2,
        Stroke::new(2.0_f32, Color32::from_rgba_premultiplied(NEON_MINT.r(), NEON_MINT.g(), NEON_MINT.b(), ring_alpha)),
    );

    // Fondo oscuro interior
    let bg_color = if is_hovered {
        Color32::from_rgb(22, 28, 42)
    } else {
        Color32::from_rgb(14, 18, 28)
    };
    painter.circle_filled(center, radius, bg_color);

    // Cruz "+" blanca
    let plus_len = radius * 0.42;
    let plus_stroke = Stroke::new(2.0_f32, Color32::WHITE);
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

