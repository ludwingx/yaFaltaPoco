use egui::{
    pos2, vec2, Color32, Pos2, Rect, Response, Rounding, Sense, Stroke, Ui, Vec2,
};

pub fn draw_gradient_card(ui: &mut Ui, rect: Rect, bottom_color: Color32) {
    let painter = ui.painter();
    let top_color = Color32::from_rgb(12, 16, 26);
    let rounding = Rounding::same(20.0);

    // Fondo base con redondeo suave
    painter.rect_filled(rect, rounding, top_color);

    // Degradado/resplandor en la base
    let bottom_h = rect.height() * 0.45;
    let glow_rect = Rect::from_min_max(
        pos2(rect.min.x, rect.max.y - bottom_h),
        rect.max,
    );

    let fade_bottom = Color32::from_rgba_premultiplied(
        bottom_color.r(),
        bottom_color.g(),
        bottom_color.b(),
        140,
    );

    painter.rect(
        glow_rect,
        Rounding {
            nw: 0.0,
            ne: 0.0,
            sw: 20.0,
            se: 20.0,
        },
        fade_bottom,
        Stroke::NONE,
    );

    // Borde de luz sutil estilo glass
    painter.rect_stroke(
        rect,
        rounding,
        Stroke::new(1.0_f32, Color32::from_rgba_premultiplied(255, 255, 255, 25)),
    );
}

pub fn floating_add_button(ui: &mut Ui, center: Pos2, radius: f32) -> Response {
    let button_size = Vec2::splat(radius * 2.0);
    let rect = Rect::from_center_size(center, button_size);
    let response = ui.allocate_rect(rect, Sense::click());

    let painter = ui.painter();
    let is_hovered = response.hovered();

    let ring_alpha = if is_hovered { 255 } else { 180 };
    let ring_thickness: f32 = if is_hovered { 2.5 } else { 1.8 };

    // Aro exterior con brillo
    let ring_color = Color32::from_rgba_premultiplied(52, 211, 153, ring_alpha);
    painter.circle_stroke(
        center,
        radius + 2.0,
        Stroke::new(ring_thickness, ring_color),
    );

    // Círculo oscuro interior
    let bg_color = if is_hovered {
        Color32::from_rgb(30, 36, 51)
    } else {
        Color32::from_rgb(20, 24, 35)
    };
    painter.circle_filled(center, radius, bg_color);

    // Signo más "+" blanco
    let plus_len = radius * 0.45;
    let plus_stroke = Stroke::new(2.2_f32, Color32::WHITE);
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

pub fn mini_progress_bar(ui: &mut Ui, progress: f32, color: Color32, width: f32) {
    let height = 4.0;
    let (rect, _) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
    let painter = ui.painter();

    painter.rect_filled(
        rect,
        Rounding::same(2.0),
        Color32::from_rgba_premultiplied(255, 255, 255, 30),
    );

    let active_w = rect.width() * progress.clamp(0.0, 1.0);
    if active_w > 0.0 {
        let active_rect = Rect::from_min_size(rect.min, vec2(active_w, height));
        painter.rect_filled(active_rect, Rounding::same(2.0), color);
    }
}
