use chrono::{Datelike, Local, Timelike};
use egui::{
    pos2, vec2, Align, Align2, Color32, FontId, Layout, Rounding, ScrollArea, Stroke, Ui,
    ViewportCommand,
};

use crate::alarm::AlarmManager;
use crate::config::{AppConfig, AppMode};
use crate::hotkey::HotkeyManager;
use crate::schedule::{
    evaluate_schedule, format_duration_hms, ColorRgb, ScheduleStatus, TimeBlock, WeekdayFilter,
    DEFAULT_COLORS,
};
use crate::theme::{TEXT_MUTED, WIDGET_BG, WIDGET_BORDER};
use crate::tray::{SystemTray, ID_ALWAYS_ON_TOP, ID_PASSTHROUGH, ID_QUIT, ID_TOGGLE_SHOW};
use crate::widgets::{draw_add_button, draw_timer_card, draw_turn_item};

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

        // Estilo moderno con bordes rectos (Rounding::ZERO)
        let mut visuals = egui::Visuals::dark();
        visuals.window_rounding = Rounding::ZERO;
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
        [0.0, 0.0, 0.0, 0.0] // Transparente total detrás del widget recto
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

        // Marco del widget con bordes RECTOS (sin redondear)
        let widget_frame = egui::Frame {
            inner_margin: egui::Margin::same(16.0),
            outer_margin: egui::Margin::ZERO,
            rounding: Rounding::ZERO,
            shadow: egui::epaint::Shadow {
                offset: [0.0, 6.0].into(),
                blur: 24.0,
                spread: 0.0,
                color: Color32::from_black_alpha(150),
            },
            fill: WIDGET_BG,
            stroke: Stroke::new(1.0, WIDGET_BORDER),
        };

        egui::CentralPanel::default()
            .frame(widget_frame)
            .show(ctx, |ui| {
                // Arrastrar la ventana desde cualquier punto del fondo
                let bg_drag = ui.interact(ui.max_rect(), ui.id().with("drag_window"), egui::Sense::drag());
                if bg_drag.dragged() {
                    ctx.send_viewport_cmd(ViewportCommand::StartDrag);
                }

                self.render_ui(ui, ctx);
            });

        // Repintar cada segundo para descontar en tiempo real sin gastar CPU
        ctx.request_repaint_after(std::time::Duration::from_secs(1));
    }
}

impl YaFaltaPocoApp {
    fn render_ui(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        // Barra superior compacta con controles
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("YA FALTA POCO")
                    .color(Color32::from_rgb(100, 118, 150))
                    .size(11.0)
                    .strong(),
            );

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if ui.add(egui::Button::new("✕").frame(false)).clicked() {
                    let _ = self.config.save();
                    ctx.send_viewport_cmd(ViewportCommand::Close);
                }

                let pin_text = if self.config.always_on_top { "📌 Fijo" } else { "📌 Libre" };
                let pin_color = if self.config.always_on_top {
                    Color32::from_rgb(56, 189, 248)
                } else {
                    TEXT_MUTED
                };
                if ui
                    .add(egui::Button::new(egui::RichText::new(pin_text).color(pin_color).size(11.0)).frame(false))
                    .clicked()
                {
                    self.config.always_on_top = !self.config.always_on_top;
                    self.apply_window_settings(ctx);
                    let _ = self.config.save();
                }

                let mode_str = match self.config.mode {
                    AppMode::Schedule => "Modo: Turnos",
                    AppMode::SimpleTimer => "Modo: Temporizador",
                };
                if ui
                    .add(egui::Button::new(egui::RichText::new(mode_str).color(Color32::WHITE).size(11.0)).frame(false))
                    .clicked()
                {
                    self.config.mode = match self.config.mode {
                        AppMode::Schedule => AppMode::SimpleTimer,
                        AppMode::SimpleTimer => AppMode::Schedule,
                    };
                    let _ = self.config.save();
                }
            });
        });

        ui.add_space(8.0);

        if self.editor.is_open {
            self.render_editor(ui);
            return;
        }

        let now_local = Local::now();
        let now_dt = now_local.naive_local();

        let month_name = match now_local.month() {
            1 => "Ene", 2 => "Feb", 3 => "Mar", 4 => "Abr",
            5 => "May", 6 => "Jun", 7 => "Jul", 8 => "Ago",
            9 => "Sep", 10 => "Oct", 11 => "Nov", _ => "Dic",
        };
        let weekday_str = match now_local.weekday() {
            chrono::Weekday::Mon => "Lunes",
            chrono::Weekday::Tue => "Martes",
            chrono::Weekday::Wed => "Miércoles",
            chrono::Weekday::Thu => "Jueves",
            chrono::Weekday::Fri => "Viernes",
            chrono::Weekday::Sat => "Sábado",
            chrono::Weekday::Sun => "Domingo",
        };

        let date_text = format!("{}, {} {:02}", weekday_str, month_name, now_local.day());
        let current_time_str = format!(
            "{:02}:{:02}:{:02}",
            now_local.hour(),
            now_local.minute(),
            now_local.second()
        );

        ui.horizontal(|ui| {
            // ===============================================
            // 1. TARJETA IZQUIERDA: CONTADOR Y HORA EN VIVO
            // ===============================================
            let card_size = vec2(220.0, 220.0);
            let (card_rect, card_resp) = ui.allocate_exact_size(card_size, egui::Sense::hover());
            let _ = card_resp;

            match self.config.mode {
                AppMode::Schedule => {
                    let status = evaluate_schedule(&self.config.blocks, now_dt);
                    let (countdown_str, label, glow_col, progress) = match status {
                        ScheduleStatus::Active { block, remaining_secs, progress } => {
                            (
                                format_duration_hms(remaining_secs),
                                format!("Termina: {}", block.title),
                                block.color.into(),
                                progress,
                            )
                        }
                        ScheduleStatus::Between { next_block, starts_in_secs } => {
                            (
                                format_duration_hms(starts_in_secs),
                                format!("Siguiente: {}", next_block.title),
                                Color32::from_rgb(16, 185, 129),
                                0.0,
                            )
                        }
                        ScheduleStatus::DayFinished { .. } => {
                            (
                                "00:00:00".into(),
                                "Turnos de hoy terminados ✓".into(),
                                Color32::from_rgb(71, 85, 105),
                                1.0,
                            )
                        }
                        ScheduleStatus::Empty => {
                            (
                                "--:--:--".into(),
                                "Sin turnos creados".into(),
                                Color32::from_rgb(37, 99, 235),
                                0.0,
                            )
                        }
                    };

                    draw_timer_card(
                        ui,
                        card_rect,
                        &date_text,
                        &current_time_str,
                        &countdown_str,
                        &label,
                        glow_col,
                        progress,
                    );
                }
                AppMode::SimpleTimer => {
                    let (remaining_secs, triggered) = self.config.timer.evaluate(now_dt);
                    if triggered {
                        self.alarm_manager.trigger(
                            "¡Ya Falta Poco!",
                            "¡Hora objetivo alcanzada!",
                            self.config.sound_enabled,
                        );
                        ctx.send_viewport_cmd(ViewportCommand::Focus);
                    }

                    let target_str = format!(
                        "Objetivo: {:02}:{:02}",
                        self.config.timer.target_hour, self.config.timer.target_min
                    );
                    draw_timer_card(
                        ui,
                        card_rect,
                        &date_text,
                        &current_time_str,
                        &format_duration_hms(remaining_secs),
                        &target_str,
                        Color32::from_rgb(34, 211, 238),
                        0.5,
                    );
                }
            }

            ui.add_space(18.0);

            // ===============================================
            // 2. LISTA DERECHA: TURNOS DE HOY
            // ===============================================
            ui.vertical(|ui| {
                ui.label(
                    egui::RichText::new("TURNOS DE HOY")
                        .color(Color32::from_rgb(140, 155, 185))
                        .size(12.5)
                        .strong(),
                );
                ui.add_space(6.0);

                let mut clicked_index = None;
                ScrollArea::vertical()
                    .max_height(180.0)
                    .show(ui, |ui| {
                        let current_weekday = now_local.weekday();
                        for (idx, block) in self.config.blocks.iter().enumerate() {
                            if !block.days.matches(current_weekday) {
                                continue;
                            }

                            let is_active = block.is_active_at(now_dt);
                            let is_completed = block.is_completed_at(now_dt);

                            let time_range = format!(
                                "{:02}:{:02} - {:02}:{:02}",
                                block.start_hour,
                                block.start_min,
                                block.end_hour,
                                block.end_min
                            );

                            let status_text = if is_completed {
                                "Terminado ✓"
                            } else if is_active {
                                "En curso..."
                            } else {
                                "Pendiente"
                            };

                            let resp = draw_turn_item(
                                ui,
                                &block.title,
                                &time_range,
                                status_text,
                                block.color.into(),
                                is_active,
                                is_completed,
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

        // ===============================================
        // 3. BOTÓN FLOTANTE "+"
        // ===============================================
        let base_rect = ui.min_rect();
        let add_center = pos2(base_rect.max.x - 30.0, base_rect.max.y - 30.0);
        if draw_add_button(ui, add_center, 17.0).clicked() {
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
                if ui.button("Detener alarma").clicked() {
                    self.alarm_manager.stop();
                }
            });
        }
    }

    fn open_editor_new(&mut self) {
        self.editor = BlockEditorState {
            is_open: true,
            editing_index: None,
            title: "Nuevo Turno".into(),
            start_hour: 8,
            start_min: 0,
            end_hour: 12,
            end_min: 0,
            color: DEFAULT_COLORS[0].1,
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
                    "Editar Turno"
                } else {
                    "Nuevo Turno"
                })
                .color(Color32::WHITE),
            );
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Nombre del turno:");
                ui.text_edit_singleline(&mut self.editor.title);
            });

            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.label("Horario:");
                ui.add(egui::DragValue::new(&mut self.editor.start_hour).range(0..=23));
                ui.label(":");
                ui.add(egui::DragValue::new(&mut self.editor.start_min).range(0..=59));
                ui.label("a");
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
                    ui.painter().rect_filled(rect, Rounding::ZERO, c32);
                    if is_sel {
                        ui.painter().rect_stroke(
                            rect,
                            Rounding::ZERO,
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
