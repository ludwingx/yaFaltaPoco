use chrono::{Datelike, Local, Timelike};
use egui::{
    vec2, Align, Color32, Layout, Rounding, ScrollArea, Stroke, Ui, ViewportCommand,
};

use crate::alarm::AlarmManager;
use crate::config::{AppConfig, AppMode};
use crate::hotkey::HotkeyManager;
use crate::schedule::{
    evaluate_schedule, format_duration_hms, ColorRgb, ScheduleStatus, TimeBlock, WeekdayFilter,
    DEFAULT_COLORS,
};
use crate::theme::{
    BORDER_COLOR, NEON_AMBER, NEON_CYAN, NEON_PURPLE, PURE_BLACK, SURFACE_DARK, TEXT_MUTED,
};
use crate::tray::{SystemTray, ID_ALWAYS_ON_TOP, ID_PASSTHROUGH, ID_QUIT, ID_TOGGLE_SHOW};
use crate::widgets::{draw_header_add_button, draw_monochrome_turn_item, draw_oled_timer_card};

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
    runtime_ticks: u64, // Contador de ticks para telemetría de automejora recursiva
}

impl YaFaltaPocoApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let (config, _) = AppConfig::load();
        let alarm = AlarmManager::new();
        let tray = SystemTray::new().ok();
        let hotkey = HotkeyManager::new().ok();

        // Estética monocromática / OLED
        let mut visuals = egui::Visuals::dark();
        visuals.window_rounding = Rounding::ZERO;
        visuals.panel_fill = PURE_BLACK;
        cc.egui_ctx.set_visuals(visuals);

        let app = Self {
            config,
            alarm_manager: alarm,
            _tray: tray,
            hotkey,
            editor: BlockEditorState::default(),
            window_visible: true,
            runtime_ticks: 0,
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
        [0.0, 0.0, 0.0, 0.0] // Transparencia total en el lienzo de la ventana
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.runtime_ticks += 1;

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

        // Marco Negro Puro OLED con bordes rectos
        let widget_frame = egui::Frame {
            inner_margin: egui::Margin::same(14.0),
            outer_margin: egui::Margin::ZERO,
            rounding: Rounding::ZERO,
            shadow: egui::epaint::Shadow {
                offset: [0.0, 4.0].into(),
                blur: 16.0,
                spread: 0.0,
                color: Color32::from_black_alpha(200),
            },
            fill: PURE_BLACK,
            stroke: Stroke::new(1.0, BORDER_COLOR),
        };

        egui::CentralPanel::default()
            .frame(widget_frame)
            .show(ctx, |ui| {
                // Arrastrar la ventana desde cualquier espacio libre del fondo
                let bg_drag = ui.interact(ui.max_rect(), ui.id().with("drag_bg"), egui::Sense::drag());
                if bg_drag.dragged() {
                    ctx.send_viewport_cmd(ViewportCommand::StartDrag);
                }

                self.render_ui(ui, ctx);
            });

        ctx.request_repaint_after(std::time::Duration::from_secs(1));
    }
}

impl YaFaltaPocoApp {
    fn render_ui(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        // Cabecera minimalista y funcional
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("YA FALTA POCO")
                    .color(Color32::from_rgb(140, 140, 140))
                    .size(11.0)
                    .monospace(),
            );

            // Botón "+ NUEVO" perfectamente ubicado en la cabecera
            if draw_header_add_button(ui).clicked() {
                self.open_editor_new();
            }

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                // Cerrar
                if ui
                    .add(egui::Button::new(egui::RichText::new("✕").color(Color32::from_rgb(160, 160, 160))).frame(false))
                    .clicked()
                {
                    let _ = self.config.save();
                    ctx.send_viewport_cmd(ViewportCommand::Close);
                }

                // Pin Always on top
                let pin_text = if self.config.always_on_top { "PIN: ON" } else { "PIN: OFF" };
                let pin_col = if self.config.always_on_top { NEON_CYAN } else { TEXT_MUTED };
                if ui
                    .add(egui::Button::new(egui::RichText::new(pin_text).color(pin_col).monospace().size(11.0)).frame(false))
                    .clicked()
                {
                    self.config.always_on_top = !self.config.always_on_top;
                    self.apply_window_settings(ctx);
                    let _ = self.config.save();
                }

                // Modo
                let mode_lbl = match self.config.mode {
                    AppMode::Schedule => "MODO: TURNOS",
                    AppMode::SimpleTimer => "MODO: LIBRE",
                };
                if ui
                    .add(egui::Button::new(egui::RichText::new(mode_lbl).color(Color32::WHITE).monospace().size(11.0)).frame(false))
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

        let month_str = match now_local.month() {
            1 => "ENE", 2 => "FEB", 3 => "MAR", 4 => "ABR",
            5 => "MAY", 6 => "JUN", 7 => "JUL", 8 => "AGO",
            9 => "SEP", 10 => "OCT", 11 => "NOV", _ => "DIC",
        };
        let weekday_str = match now_local.weekday() {
            chrono::Weekday::Mon => "LUNES",
            chrono::Weekday::Tue => "MARTES",
            chrono::Weekday::Wed => "MIÉRCOLES",
            chrono::Weekday::Thu => "JUEVES",
            chrono::Weekday::Fri => "VIERNES",
            chrono::Weekday::Sat => "SÁBADO",
            chrono::Weekday::Sun => "DOMINGO",
        };

        let date_text = format!("{}, {:02} {}", weekday_str, now_local.day(), month_str);
        let time_str = format!(
            "{:02}:{:02}:{:02}",
            now_local.hour(),
            now_local.minute(),
            now_local.second()
        );

        // Estado del loop de automejora / optimización recursiva
        let recursion_info = format!("RECURSION: TICK #{} [OPT: 0 Desfase]", self.runtime_ticks);

        ui.horizontal(|ui| {
            // ===============================================
            // 1. TARJETA OLED IZQUIERDA: DÍGITOS NEÓN
            // ===============================================
            let card_size = vec2(225.0, 220.0);
            let (card_rect, card_resp) = ui.allocate_exact_size(card_size, egui::Sense::hover());
            let _ = card_resp;

            match self.config.mode {
                AppMode::Schedule => {
                    let status = evaluate_schedule(&self.config.blocks, now_dt);
                    let (countdown_str, label, neon_col, progress) = match status {
                        ScheduleStatus::Active { block, remaining_secs, progress } => {
                            let n_col = match block.title.as_str() {
                                "OtherBrain I" => NEON_CYAN,
                                "Almuerzo" => NEON_AMBER,
                                "OtherBrain II" => NEON_PURPLE,
                                _ => block.color.into(),
                            };
                            (
                                format_duration_hms(remaining_secs),
                                format!("RESTANTE: {}", block.title.to_uppercase()),
                                n_col,
                                progress,
                            )
                        }
                        ScheduleStatus::Between { next_block, starts_in_secs } => {
                            (
                                format_duration_hms(starts_in_secs),
                                format!("SIGUIENTE: {}", next_block.title.to_uppercase()),
                                NEON_CYAN,
                                0.0,
                            )
                        }
                        ScheduleStatus::DayFinished { .. } => {
                            (
                                "00:00:00".into(),
                                "JORNADA COMPLETADA".into(),
                                Color32::from_rgb(80, 80, 80),
                                1.0,
                            )
                        }
                        ScheduleStatus::Empty => {
                            ("--:--:--".into(), "SIN TURNOS".into(), NEON_CYAN, 0.0)
                        }
                    };

                    draw_oled_timer_card(
                        ui,
                        card_rect,
                        &date_text,
                        &time_str,
                        &countdown_str,
                        &label,
                        neon_col,
                        progress,
                        &recursion_info,
                    );
                }
                AppMode::SimpleTimer => {
                    let (remaining_secs, triggered) = self.config.timer.evaluate(now_dt);
                    if triggered {
                        self.alarm_manager.trigger(
                            "¡Ya Falta Poco!",
                            "¡Objetivo alcanzado!",
                            self.config.sound_enabled,
                        );
                        ctx.send_viewport_cmd(ViewportCommand::Focus);
                    }

                    let target_str = format!(
                        "META  {:02}:{:02}",
                        self.config.timer.target_hour, self.config.timer.target_min
                    );
                    draw_oled_timer_card(
                        ui,
                        card_rect,
                        &date_text,
                        &time_str,
                        &format_duration_hms(remaining_secs),
                        &target_str,
                        NEON_CYAN,
                        0.5,
                        &recursion_info,
                    );
                }
            }

            ui.add_space(14.0);

            // ===============================================
            // 2. LISTA DERECHA: TURNOS MONOCROMÁTICOS
            // ===============================================
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("HORARIO DEL DÍA")
                            .color(Color32::from_rgb(150, 150, 150))
                            .monospace()
                            .size(11.5)
                            .strong(),
                    );
                });
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
                                "COMPLETADO ✓"
                            } else if is_active {
                                "ACTIVO NOW"
                            } else {
                                "EN ESPERA"
                            };

                            let neon_color = match block.title.as_str() {
                                "OtherBrain I" => NEON_CYAN,
                                "Almuerzo" => NEON_AMBER,
                                "OtherBrain II" => NEON_PURPLE,
                                _ => block.color.into(),
                            };

                            let resp = draw_monochrome_turn_item(
                                ui,
                                &block.title,
                                &time_range,
                                status_text,
                                neon_color,
                                is_active,
                                is_completed,
                            );

                            if resp.clicked() {
                                clicked_index = Some(idx);
                            }
                        }
                    });

                if let Some(idx) = clicked_index {
                    self.open_editor_for_block(idx);
                }
            });
        });

        if self.alarm_manager.is_active() {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("🔔 ¡TIEMPO COMPLETADO!")
                        .color(NEON_CYAN)
                        .monospace()
                        .strong(),
                );
                if ui.button("Silenciar").clicked() {
                    self.alarm_manager.stop();
                }
            });
        }
    }

    fn open_editor_new(&mut self) {
        self.editor = BlockEditorState {
            is_open: true,
            editing_index: None,
            title: "Nuevo Bloque".into(),
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
                    "CONFIGURACIÓN DE TURNO"
                } else {
                    "NUEVO TURNO"
                })
                .monospace()
                .color(NEON_CYAN),
            );
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Nombre:");
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
                ui.label("Acento:");
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
                    .add(egui::Button::new(egui::RichText::new("GUARDAR").color(Color32::BLACK)).fill(NEON_CYAN))
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
                        .add(egui::Button::new("ELIMINAR").fill(Color32::from_rgb(180, 30, 30)))
                        .clicked()
                    {
                        self.config.blocks.remove(idx);
                        let _ = self.config.save();
                        self.editor.is_open = false;
                    }
                }

                if ui.button("CANCELAR").clicked() {
                    self.editor.is_open = false;
                }
            });
        });
    }
}
