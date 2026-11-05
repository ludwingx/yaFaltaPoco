use egui::{pos2, vec2, Color32, FontId, Pos2, Rect, Response, Rounding, Sense, Stroke, Ui, Vec2};
use crate::theme::{CARD_BASE_DARK, GLOW_BLUE, NEON_CYAN, NEON_MINT, TEXT_BRIGHT, TEXT_MUTED, TEXT_WHITE};

/// Dibuja la tarjeta izquierda idéntica a la imagen de referencia:
/// Base con degradado azul intenso brillante y esquinas redondeadas
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

    // 1. Fondo negro profundo para máximo contraste
    painter.rect_filled(rect, card_rounding, CARD_BASE_DARK);

    // 2. Degradado azul brillante de la base
    let glow_height = rect.height() * 0.44;
    let steps = 24;
    for i in 0..steps {
        let t = i as f32 / steps as f32; // 0.0 a 1.0
        let y_start = rect.max.y - glow_height * (1.0 - t);
        let y_end = rect.max.y - glow_height * (1.0 - (t + 1.0 / steps as f32));

        let alpha = (t * t * 240.0) as u8;
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

    // 3. Borde sutil de luz
    painter.rect_stroke(
        rect,
        card_rounding,
        Stroke::new(1.0_f32, Color32::from_rgba_premultiplied(255, 255, 255, 30)),
    );

    // 4. Arriba en grande: Fecha ("Oct 30") en blanco ultra nítido
    painter.text(
        pos2(rect.min.x + 22.0, rect.min.y + 18.0),
        egui::Align2::LEFT_TOP,
        date_title,
        FontId::proportional(28.0),
        TEXT_WHITE,
    );

    // Hora actual en blanco suave
    painter.text(
        pos2(rect.min.x + 22.0, rect.min.y + 52.0),
        egui::Align2::LEFT_TOP,
        format!("Hora: {}", current_time_str),
        FontId::monospace(13.0),
        TEXT_BRIGHT,
    );

    // 5. Centro: Contador regresivo con caja oscura de fondo y dígitos neón ultra vivos
    let countdown_box = Rect::from_center_size(
        pos2(rect.center().x, rect.min.y + 106.0),
        vec2(rect.width() - 32.0, 48.0),
    );
    // Fondo negro puro con borde suave para hacer resaltar los números neón
    painter.rect_filled(
        countdown_box,
        Rounding::same(12.0),
        Color32::from_rgba_premultiplied(0, 0, 0, 190),
    );
    painter.rect_stroke(
        countdown_box,
        Rounding::same(12.0),
        Stroke::new(1.0_f32, Color32::from_rgba_premultiplied(34, 211, 238, 45)),
    );

    // Dígitos del temporizador en Cian Neón vibrante
    painter.text(
        pos2(rect.center().x, rect.min.y + 98.0),
        egui::Align2::CENTER_CENTER,
        countdown_str,
        FontId::monospace(26.0),
        NEON_CYAN,
    );

    // Etiqueta del turno/bloque en texto claro
    painter.text(
        pos2(rect.center().x, rect.min.y + 120.0),
        egui::Align2::CENTER_CENTER,
        block_name,
        FontId::proportional(11.5),
        TEXT_BRIGHT,
    );

    // 6. Abajo: Día de la semana ("Jueves") y número de bloques ("3 bloques") en blanco nítido
    painter.text(
        pos2(rect.min.x + 22.0, rect.max.y - 44.0),
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
        Color32::from_rgb(200, 225, 255),
    );
}

/// Fila de la lista derecha
pub fn draw_event_row(
    ui: &mut Ui,
    title: &str,
    time_range: &str,
    color: Color32,
    is_active: bool,
    is_completed: bool,
) -> Response {
    let desired_size = vec2(ui.available_width(), 46.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());
    let painter = ui.painter();

    // Fondo oscuro en las filas para contraste perfecto
    let row_bg = if is_active {
        Color32::from_rgba_premultiplied(color.r(), color.g(), color.b(), 40)
    } else if response.hovered() {
        Color32::from_rgba_premultiplied(255, 255, 255, 14)
    } else {
        Color32::from_rgba_premultiplied(0, 0, 0, 70)
    };
    painter.rect_filled(rect, Rounding::same(10.0), row_bg);

    if is_active {
        painter.rect_stroke(
            rect,
            Rounding::same(10.0),
            Stroke::new(1.0_f32, Color32::from_rgba_premultiplied(color.r(), color.g(), color.b(), 120)),
        );
    }

    // Barra vertical de color
    let bar_rect = Rect::from_min_size(
        pos2(rect.min.x + 8.0, rect.min.y + 7.0),
        vec2(4.0, 32.0),
    );
    let bar_col = if is_completed {
        Color32::from_rgb(80, 90, 110)
    } else {
        color
    };
    painter.rect_filled(bar_rect, Rounding::same(2.0), bar_col);

    // Título en blanco nítido
    let title_pos = pos2(rect.min.x + 20.0, rect.min.y + 8.0);
    let title_color = if is_completed {
        Color32::from_rgb(130, 142, 160)
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

    // Tachado si ya terminó
    if is_completed {
        let text_w = title.len() as f32 * 8.0;
        painter.line_segment(
            [
                pos2(title_pos.x, title_pos.y + 10.0),
                pos2(title_pos.x + text_w, title_pos.y + 10.0),
            ],
            Stroke::new(1.3_f32, Color32::from_rgb(150, 162, 180)),
        );
    }

    // Horario con dígitos muy legibles
    let time_color = if is_active {
        NEON_CYAN
    } else if is_completed {
        Color32::from_rgb(120, 130, 150)
    } else {
        TEXT_MUTED
    };

    painter.text(
        pos2(rect.min.x + 20.0, rect.min.y + 27.0),
        egui::Align2::LEFT_TOP,
        time_range,
        FontId::monospace(12.5),
        time_color,
    );

    response
}

/// Botón circular flotante "+" con margen holgado para que no se corte
pub fn draw_floating_plus(ui: &mut Ui, center: Pos2, radius: f32) -> Response {
    let size = Vec2::splat(radius * 2.0);
    let rect = Rect::from_center_size(center, size);
    let response = ui.allocate_rect(rect, Sense::click());
    let painter = ui.painter();

    let is_hovered = response.hovered();

    // Aro exterior brillante neón
    let ring_alpha: u8 = if is_hovered { 255 } else { 200 };
    painter.circle_stroke(
        center,
        radius + 1.5,
        Stroke::new(2.2_f32, Color32::from_rgba_premultiplied(NEON_MINT.r(), NEON_MINT.g(), NEON_MINT.b(), ring_alpha)),
    );

    // Fondo negro interior
    let bg_color = if is_hovered {
        Color32::from_rgb(18, 24, 38)
    } else {
        Color32::from_rgb(8, 10, 18)
    };
    painter.circle_filled(center, radius, bg_color);

    // Cruz "+" blanca pura
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

