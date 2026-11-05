use chrono::{Datelike, Local};
use egui::{
    pos2, vec2, Align, Align2, Color32, FontId, Layout, Rect, Rounding, ScrollArea, Stroke, Ui,
    ViewportCommand,
};

use crate::alarm::AlarmManager;
use crate::config::{AppConfig, AppMode};
use crate::hotkey::HotkeyManager;
use crate::schedule::{
    evaluate_schedule, format_duration_hms, ColorRgb, ScheduleStatus, TimeBlock, WeekdayFilter,
    DEFAULT_COLORS,
};
use crate::theme::{WIDGET_BG, WIDGET_BORDER};
use crate::timer::TimerState;
use crate::tray::{SystemTray, ID_ALWAYS_ON_TOP, ID_PASSTHROUGH, ID_QUIT, ID_TOGGLE_SHOW};
use crate::widgets::{draw_glowing_add_button, draw_reference_card, draw_upcoming_item};

#[derive(Default)]
struct BlockEditorState {
    is_open: bool,
    editing_index: Option<usize>,
    title: String,
    start_hour: u32,
    start_min: u32,
    end_hour: u32,
    end_min: u32,
    color: ColorRgb,
    days: WeekdayFilter,
}

pub struct YaFaltaPocoApp {
    pub config: AppConfig,
    alarm_manager: AlarmManager,
    _tray: Option<SystemTray>,
    hotkey: Option<HotkeyManager>,
    editor: BlockEditorState,
    window_visible: bool,
}

impl YaFaltaPocoApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let (config, _) = AppConfig::load();
        let alarm = AlarmManager::new();
        let tray = SystemTray::new().ok();
        let hotkey = HotkeyManager::new().ok();

        // Tipografía limpia eframe
        let mut visuals = egui::Visuals::dark();
        visuals.window_rounding = Rounding::same(28.0);
        visuals.panel_fill = WIDGET_BG;
        cc.egui_ctx.set_visuals(visuals);

        let app = Self {
            config,
            alarm_manager: alarm,
            _tray: tray,
            hotkey,
            editor: BlockEditorState::default(),
            window_visible: true,
        };

        app.apply_window_settings(&cc.egui_ctx);
        app
    }

    fn apply_window_settings(&self, ctx: &egui::Context) {
        let level = if self.config.always_on_top {
            egui::viewport::WindowLevel::AlwaysOnTop
        } else {
            egui::viewport::WindowLevel::Normal
        };
        ctx.send_viewport_cmd(ViewportCommand::WindowLevel(level));
        ctx.send_viewport_cmd(ViewportCommand::MousePassthrough(self.config.mouse_passthrough));
    }

    fn toggle_visibility(&mut self, ctx: &egui::Context) {
        self.window_visible = !self.window_visible;
        ctx.send_viewport_cmd(ViewportCommand::Visible(self.window_visible));
        if self.window_visible {
            ctx.send_viewport_cmd(ViewportCommand::Focus);
        }
    }
}

impl eframe::App for YaFaltaPocoApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.08, 0.10, 0.16, 1.0] // Fondo de ventana exactamente acoplado al widget
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(hk) = &self.hotkey {
            if hk.poll_toggle() {
                self.toggle_visibility(ctx);
            }
        }

        if let Some(action) = SystemTray::poll_event() {
            match action.as_str() {
                ID_TOGGLE_SHOW => self.toggle_visibility(ctx),
                ID_ALWAYS_ON_TOP => {
                    self.config.always_on_top = !self.config.always_on_top;
                    self.apply_window_settings(ctx);
                    let _ = self.config.save();
                }
                ID_PASSTHROUGH => {
                    self.config.mouse_passthrough = !self.config.mouse_passthrough;
                    ctx.send_viewport_cmd(ViewportCommand::MousePassthrough(
                        self.config.mouse_passthrough,
                    ));
                    let _ = self.config.save();
                }
                ID_QUIT => {
                    let _ = self.config.save();
                    ctx.send_viewport_cmd(ViewportCommand::Close);
                }
                _ => {}
            }
        }

        let widget_frame = egui::Frame {
            inner_margin: egui::Margin::same(18.0),
            outer_margin: egui::Margin::ZERO,
            rounding: Rounding::same(28.0),
            shadow: egui::epaint::Shadow {
                offset: [0.0, 10.0].into(),
                blur: 30.0,
                spread: 0.0,
                color: Color32::from_black_alpha(140),
            },
            fill: WIDGET_BG,
            stroke: Stroke::new(1.0, WIDGET_BORDER),
        };

        egui::CentralPanel::default()
            .frame(widget_frame)
            .show(ctx, |ui| {
                self.render_ui(ui, ctx);
            });

        ctx.request_repaint_after(std::time::Duration::from_secs(1));
    }
}

impl YaFaltaPocoApp {
    fn render_ui(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        // Barra superior mínima transparente e interactiva para arrastrar
        ui.horizontal(|ui| {
            let (drag_rect, drag_resp) =
                ui.allocate_exact_size(vec2(ui.available_width() - 65.0, 18.0), egui::Sense::drag());
            if drag_resp.dragged() {
                ctx.send_viewport_cmd(ViewportCommand::StartDrag);
            }

            ui.painter().text(
                drag_rect.left_center(),
                Align2::LEFT_CENTER,
                "YA FALTA POCO",
                FontId::proportional(10.5),
                Color32::from_rgb(90, 105, 135),
            );

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if ui
                    .add(egui::Button::new("✕").frame(false))
                    .clicked()
                {
                    let _ = self.config.save();
                    ctx.send_viewport_cmd(ViewportCommand::Close);
                }

                let pin_col = if self.config.always_on_top {
                    Color32::from_rgb(56, 189, 248)
                } else {
                    Color32::from_rgb(100, 115, 140)
                };
                if ui
                    .add(egui::Button::new("📌").stroke(Stroke::NONE).frame(false))
                    .clicked()
                {
                    self.config.always_on_top = !self.config.always_on_top;
                    self.apply_window_settings(ctx);
                    let _ = self.config.save();
                }

                let _ = pin_col;
            });
        });

        ui.add_space(8.0);

        if self.editor.is_open {
            self.render_editor(ui);
            return;
        }

        let now_local = Local::now();
        let now_dt = now_local.naive_local();

        ui.horizontal(|ui| {
            // ============================================
            // 1. TARJETA IZQUIERDA (DISEÑO EXACTO A LA IMAGEN)
            // ============================================
            let card_size = vec2(215.0, 215.0);
            let (card_rect, card_resp) = ui.allocate_exact_size(card_size, egui::Sense::click());
            if card_resp.clicked() {
                // Alternar modo con un clic sobre la tarjeta
                self.config.mode = match self.config.mode {
                    AppMode::Schedule => AppMode::SimpleTimer,
                    AppMode::SimpleTimer => AppMode::Schedule,
                };
                let _ = self.config.save();
            }

            let month_name = match now_local.month() {
                1 => "Jan", 2 => "Feb", 3 => "Mar", 4 => "Apr",
                5 => "May", 6 => "Jun", 7 => "Jul", 8 => "Aug",
                9 => "Sep", 10 => "Oct", 11 => "Nov", _ => "Dec",
            };
            let weekday_str = match now_local.weekday() {
                chrono::Weekday::Mon => "Monday",
                chrono::Weekday::Tue => "Tuesday",
                chrono::Weekday::Wed => "Wednesday",
                chrono::Weekday::Thu => "Thursday",
                chrono::Weekday::Fri => "Friday",
                chrono::Weekday::Sat => "Saturday",
                chrono::Weekday::Sun => "Sunday",
            };

            let today_events_count = self
                .config
                .blocks
                .iter()
                .filter(|b| b.days.matches(now_local.weekday()))
                .count();

            match self.config.mode {
                AppMode::Schedule => {
                    let status = evaluate_schedule(&self.config.blocks, now_dt);
                    let (top_title, bottom_glow) = match status {
                        ScheduleStatus::Active { block, remaining_secs, .. } => {
                            (
                                format_duration_hms(remaining_secs),
                                block.color.into(),
                            )
                        }
                        ScheduleStatus::Between { starts_in_secs, .. } => {
                            (
                                format_duration_hms(starts_in_secs),
                                Color32::from_rgb(37, 99, 235), // Azul vibrante
                            )
                        }
                        _ => (
                            format!("{} {:02}", month_name, now_local.day()),
                            Color32::from_rgb(37, 99, 235), // Azul vibrante de la foto
                        ),
                    };

                    let count_text = format!("{} events", today_events_count);
                    draw_reference_card(
                        ui,
                        card_rect,
                        &top_title,
                        weekday_str,
                        &count_text,
                        bottom_glow,
                    );
                }
                AppMode::SimpleTimer => {
                    let (remaining_secs, triggered) = self.config.timer.evaluate(now_dt);
                    if triggered {
                        self.alarm_manager.trigger(
                            "¡Ya Falta Poco!",
                            "Hora objetivo alcanzada",
                            self.config.sound_enabled,
                        );
                        ctx.send_viewport_cmd(ViewportCommand::Focus);
                    }

                    let timer_text = format_duration_hms(remaining_secs);
                    let target_sub = format!(
                        "Target {:02}:{:02}",
                        self.config.timer.target_hour, self.config.timer.target_min
                    );
                    draw_reference_card(
                        ui,
                        card_rect,
                        &timer_text,
                        "Timer Mode",
                        &target_sub,
                        Color32::from_rgb(56, 189, 248), // Cian
                    );
                }
            }

            ui.add_space(20.0);

            // ============================================
            // 2. LISTA DERECHA ("UPCOMING")
            // ============================================
            ui.vertical(|ui| {
                ui.label(
                    egui::RichText::new("UPCOMING")
                        .color(Color32::from_rgb(130, 145, 175))
                        .size(13.0)
                        .strong(),
                );
                ui.add_space(8.0);

                let mut clicked_index = None;
                ScrollArea::vertical()
                    .max_height(175.0)
                    .show(ui, |ui| {
                        let current_weekday = now_local.weekday();
                        for (idx, block) in self.config.blocks.iter().enumerate() {
                            if !block.days.matches(current_weekday) {
                                continue;
                            }

                            let is_active = block.is_active_at(now_dt);
                            let time_range = format!(
                                "{:02}:{:02} - {:02}:{:02}",
                                block.start_hour,
                                block.start_min,
                                block.end_hour,
                                block.end_min
                            );

                            let resp = draw_upcoming_item(
                                ui,
                                &block.title,
                                &time_range,
                                block.color.into(),
                                is_active,
                            );

                            if resp.clicked() {
                                clicked_index = Some(idx);
                            }
                            ui.add_space(4.0);
                        }
                    });

                if let Some(idx) = clicked_index {
                    self.open_editor_for_block(idx);
                }
            });
        });

        // ============================================
        // 3. BOTÓN FLOTANTE CON RESPLANDOR
        // ============================================
        let base_rect = ui.min_rect();
        let add_center = pos2(base_rect.max.x - 32.0, base_rect.max.y - 32.0);
        if draw_glowing_add_button(ui, add_center, 18.0).clicked() {
            self.open_editor_new();
        }

        if self.alarm_manager.is_active() {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("🔔 ¡Ya es la hora!")
                        .color(Color32::from_rgb(52, 211, 153))
                        .strong(),
                );
                if ui.button("Detener").clicked() {
                    self.alarm_manager.stop();
                }
            });
        }
    }

    fn open_editor_new(&mut self) {
        self.editor = BlockEditorState {
            is_open: true,
            editing_index: None,
            title: "Focus Time".into(),
            start_hour: 11,
            start_min: 0,
            end_hour: 13,
            end_min: 0,
            color: DEFAULT_COLORS[3].1, // Violeta
            days: WeekdayFilter::all(),
        };
    }

    fn open_editor_for_block(&mut self, idx: usize) {
        if let Some(b) = self.config.blocks.get(idx) {
            self.editor = BlockEditorState {
                is_open: true,
                editing_index: Some(idx),
                title: b.title.clone(),
                start_hour: b.start_hour,
                start_min: b.start_min,
                end_hour: b.end_hour,
                end_min: b.end_min,
                color: b.color,
                days: b.days,
            };
        }
    }

    fn render_editor(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.heading(
                egui::RichText::new(if self.editor.editing_index.is_some() {
                    "Editar Bloque"
                } else {
                    "Nuevo Bloque"
                })
                .color(Color32::WHITE),
            );
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Título:");
                ui.text_edit_singleline(&mut self.editor.title);
            });

            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.label("Horario:");
                ui.add(egui::DragValue::new(&mut self.editor.start_hour).range(0..=23));
                ui.label(":");
                ui.add(egui::DragValue::new(&mut self.editor.start_min).range(0..=59));
                ui.label("—");
                ui.add(egui::DragValue::new(&mut self.editor.end_hour).range(0..=23));
                ui.label(":");
                ui.add(egui::DragValue::new(&mut self.editor.end_min).range(0..=59));
            });

            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.label("Días:");
                ui.checkbox(&mut self.editor.days.monday, "L");
                ui.checkbox(&mut self.editor.days.tuesday, "M");
                ui.checkbox(&mut self.editor.days.wednesday, "X");
                ui.checkbox(&mut self.editor.days.thursday, "J");
                ui.checkbox(&mut self.editor.days.friday, "V");
                ui.checkbox(&mut self.editor.days.saturday, "S");
                ui.checkbox(&mut self.editor.days.sunday, "D");
            });

            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.label("Color:");
                for (_, c) in &DEFAULT_COLORS {
                    let c32: Color32 = (*c).into();
                    let (rect, resp) = ui.allocate_exact_size(vec2(18.0, 18.0), egui::Sense::click());
                    let is_sel = self.editor.color == *c;
                    ui.painter().rect_filled(rect, Rounding::same(5.0), c32);
                    if is_sel {
                        ui.painter().rect_stroke(
                            rect,
                            Rounding::same(5.0),
                            Stroke::new(2.0, Color32::WHITE),
                        );
                    }
                    if resp.clicked() {
                        self.editor.color = *c;
                    }
                }
            });

            ui.add_space(14.0);
            ui.horizontal(|ui| {
                if ui
                    .add(egui::Button::new("Guardar").fill(Color32::from_rgb(37, 99, 235)))
                    .clicked()
                {
                    let b = TimeBlock {
                        id: format!("b_{}", Local::now().timestamp_millis()),
                        title: self.editor.title.clone(),
                        start_hour: self.editor.start_hour,
                        start_min: self.editor.start_min,
                        end_hour: self.editor.end_hour,
                        end_min: self.editor.end_min,
                        color: self.editor.color,
                        days: self.editor.days,
                        notify_on_start: true,
                        notify_on_end: true,
                        notify_before_min: Some(5),
                    };

                    if let Some(idx) = self.editor.editing_index {
                        self.config.blocks[idx] = b;
                    } else {
                        self.config.blocks.push(b);
                    }
                    let _ = self.config.save();
                    self.editor.is_open = false;
                }

                if let Some(idx) = self.editor.editing_index {
                    if ui
                        .add(egui::Button::new("Eliminar").fill(Color32::from_rgb(220, 38, 38)))
                        .clicked()
                    {
                        self.config.blocks.remove(idx);
                        let _ = self.config.save();
                        self.editor.is_open = false;
                    }
                }

                if ui.button("Cancelar").clicked() {
                    self.editor.is_open = false;
                }
            });
        });
    }
}
