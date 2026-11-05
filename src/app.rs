use chrono::{Datelike, Local, Timelike};
use egui::{
    pos2, vec2, Align, Align2, Color32, FontId, Layout, Rect, Rounding, ScrollArea, Stroke, Ui,
    ViewportCommand,
};

use crate::alarm::AlarmManager;
use crate::config::{AppConfig, AppMode};
use crate::hotkey::HotkeyManager;
use crate::schedule::{
    detect_overlaps, evaluate_schedule, format_duration_hms, ColorRgb, ScheduleStatus, TimeBlock,
    WeekdayFilter, DEFAULT_COLORS,
};
use crate::theme::{rounded_frame, DEFAULT_ACCENT, TEXT_MUTED, TEXT_PRIMARY};
use crate::timer::TimerState;
use crate::tray::{SystemTray, ID_ALWAYS_ON_TOP, ID_PASSTHROUGH, ID_QUIT, ID_TOGGLE_SHOW};
use crate::widgets::{draw_gradient_card, floating_add_button, mini_progress_bar};

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
    notify_on_start: bool,
    notify_on_end: bool,
}

pub struct YaFaltaPocoApp {
    pub config: AppConfig,
    alarm_manager: AlarmManager,
    _tray: Option<SystemTray>,
    hotkey: Option<HotkeyManager>,
    editor: BlockEditorState,
    _status_message: Option<(String, f64)>,
    window_visible: bool,
}

impl YaFaltaPocoApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let (config, warn) = AppConfig::load();
        let alarm = AlarmManager::new();
        let tray = SystemTray::new().ok();
        let hotkey = HotkeyManager::new().ok();

        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        let app = Self {
            config,
            alarm_manager: alarm,
            _tray: tray,
            hotkey,
            editor: BlockEditorState::default(),
            _status_message: warn.map(|w| (w, 10.0)),
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

    fn toggle_compact_mode(&mut self, ctx: &egui::Context) {
        self.config.compact_mode = !self.config.compact_mode;
        let size = if self.config.compact_mode {
            vec2(280.0, 95.0)
        } else {
            vec2(620.0, 310.0)
        };
        ctx.send_viewport_cmd(ViewportCommand::InnerSize(size));
        let _ = self.config.save();
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
        [0.08, 0.10, 0.15, 1.0]
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
                    let level = if self.config.always_on_top {
                        egui::viewport::WindowLevel::AlwaysOnTop
                    } else {
                        egui::viewport::WindowLevel::Normal
                    };
                    ctx.send_viewport_cmd(ViewportCommand::WindowLevel(level));
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

        let opacity = self.config.opacity.clamp(0.4, 1.0);
        let mut frame_style = rounded_frame();
        let current_fill = frame_style.fill;
        frame_style.fill = Color32::from_rgba_premultiplied(
            current_fill.r(),
            current_fill.g(),
            current_fill.b(),
            (current_fill.a() as f32 * opacity) as u8,
        );

        egui::CentralPanel::default()
            .frame(frame_style)
            .show(ctx, |ui| {
                if self.config.compact_mode {
                    self.render_compact_ui(ui, ctx);
                } else {
                    self.render_full_ui(ui, ctx);
                }
            });

        ctx.request_repaint_after(std::time::Duration::from_secs(1));
    }
}

impl YaFaltaPocoApp {
    fn render_title_bar(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        ui.horizontal(|ui| {
            let (drag_rect, drag_resp) =
                ui.allocate_exact_size(vec2(ui.available_width() - 110.0, 22.0), egui::Sense::drag());
            if drag_resp.dragged() {
                ctx.send_viewport_cmd(ViewportCommand::StartDrag);
            }
            if drag_resp.double_clicked() {
                self.toggle_compact_mode(ctx);
            }

            ui.painter().text(
                drag_rect.left_center(),
                Align2::LEFT_CENTER,
                "Ya falta poco…",
                FontId::proportional(11.0),
                TEXT_MUTED,
            );

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if ui.button("✕").clicked() {
                    let _ = self.config.save();
                    ctx.send_viewport_cmd(ViewportCommand::Close);
                }

                if ui.button("—").clicked() {
                    self.toggle_visibility(ctx);
                }

                let comp_icon = if self.config.compact_mode { "⤢" } else { "⤡" };
                if ui.button(comp_icon).clicked() {
                    self.toggle_compact_mode(ctx);
                }

                let pin_color = if self.config.always_on_top {
                    DEFAULT_ACCENT
                } else {
                    TEXT_MUTED
                };
                if ui
                    .add(egui::Button::new("📌").stroke(Stroke::new(1.0_f32, pin_color)))
                    .clicked()
                {
                    self.config.always_on_top = !self.config.always_on_top;
                    let level = if self.config.always_on_top {
                        egui::viewport::WindowLevel::AlwaysOnTop
                    } else {
                        egui::viewport::WindowLevel::Normal
                    };
                    ctx.send_viewport_cmd(ViewportCommand::WindowLevel(level));
                    let _ = self.config.save();
                }

                let mode_label = match self.config.mode {
                    AppMode::Schedule => "Modo: Agenda",
                    AppMode::SimpleTimer => "Modo: Libre",
                };
                if ui.button(mode_label).clicked() {
                    self.config.mode = match self.config.mode {
                        AppMode::Schedule => AppMode::SimpleTimer,
                        AppMode::SimpleTimer => AppMode::Schedule,
                    };
                    let _ = self.config.save();
                }
            });
        });
    }

    fn render_full_ui(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        self.render_title_bar(ui, ctx);
        ui.add_space(8.0);

        if self.editor.is_open {
            self.render_block_editor(ui);
            return;
        }

        let now_local = Local::now();
        let now_dt = now_local.naive_local();

        ui.horizontal(|ui| {
            let card_width = 220.0;
            let card_height = 230.0;
            let (card_rect, _) =
                ui.allocate_exact_size(vec2(card_width, card_height), egui::Sense::hover());

            let mut bottom_color = Color32::from_rgb(30, 144, 255);

            match self.config.mode {
                AppMode::Schedule => {
                    let status = evaluate_schedule(&self.config.blocks, now_dt);
                    let (headline, _sub_label, remaining_str, progress) = match status {
                        ScheduleStatus::Active {
                            block,
                            remaining_secs,
                            progress,
                        } => {
                            bottom_color = block.color.into();
                            (
                                block.title.clone(),
                                "Bloque actual",
                                format_duration_hms(remaining_secs),
                                progress,
                            )
                        }
                        ScheduleStatus::Between {
                            next_block,
                            starts_in_secs,
                        } => {
                            bottom_color = Color32::from_rgb(16, 185, 129);
                            (
                                format!("Siguiente: {}", next_block.title),
                                "Comienza en",
                                format_duration_hms(starts_in_secs),
                                0.0,
                            )
                        }
                        ScheduleStatus::DayFinished {
                            next_block_tomorrow,
                            starts_in_secs,
                        } => {
                            let label = if let Some(nb) = next_block_tomorrow {
                                format!("Mañana: {}", nb.title)
                            } else {
                                "Sin bloques".into()
                            };
                            let time = starts_in_secs
                                .map(format_duration_hms)
                                .unwrap_or_else(|| "--:--:--".into());
                            (label, "Día terminado", time, 1.0)
                        }
                        ScheduleStatus::Empty => {
                            ("Sin bloques creados".into(), "Agenda vacía", "--:--:--".into(), 0.0)
                        }
                    };

                    draw_gradient_card(ui, card_rect, bottom_color);

                    let painter = ui.painter();
                    let month_str = match now_local.month() {
                        1 => "Ene",
                        2 => "Feb",
                        3 => "Mar",
                        4 => "Abr",
                        5 => "May",
                        6 => "Jun",
                        7 => "Jul",
                        8 => "Ago",
                        9 => "Sep",
                        10 => "Oct",
                        11 => "Nov",
                        _ => "Dic",
                    };
                    let date_title = format!("{} {:02}", month_str, now_local.day());
                    painter.text(
                        pos2(card_rect.min.x + 18.0, card_rect.min.y + 24.0),
                        Align2::LEFT_TOP,
                        date_title,
                        FontId::proportional(26.0),
                        TEXT_PRIMARY,
                    );

                    let time_str = format!(
                        "{:02}:{:02}:{:02}",
                        now_local.hour(),
                        now_local.minute(),
                        now_local.second()
                    );
                    painter.text(
                        pos2(card_rect.min.x + 18.0, card_rect.min.y + 60.0),
                        Align2::LEFT_TOP,
                        time_str,
                        FontId::monospace(14.0),
                        TEXT_MUTED,
                    );

                    painter.text(
                        pos2(card_rect.center().x, card_rect.min.y + 115.0),
                        Align2::CENTER_CENTER,
                        remaining_str,
                        FontId::monospace(26.0),
                        TEXT_PRIMARY,
                    );
                    painter.text(
                        pos2(card_rect.center().x, card_rect.min.y + 140.0),
                        Align2::CENTER_CENTER,
                        headline,
                        FontId::proportional(13.0),
                        Color32::WHITE,
                    );

                    let bar_rect = Rect::from_center_size(
                        pos2(card_rect.center().x, card_rect.min.y + 160.0),
                        vec2(160.0, 4.0),
                    );
                    painter.rect_filled(
                        bar_rect,
                        Rounding::same(2.0),
                        Color32::from_rgba_premultiplied(255, 255, 255, 40),
                    );
                    let active_w = 160.0 * progress.clamp(0.0, 1.0);
                    if active_w > 0.0 {
                        let fill_rect = Rect::from_min_size(bar_rect.min, vec2(active_w, 4.0));
                        painter.rect_filled(fill_rect, Rounding::same(2.0), bottom_color);
                    }

                    let weekday_name = match now_local.weekday() {
                        chrono::Weekday::Mon => "Lunes",
                        chrono::Weekday::Tue => "Martes",
                        chrono::Weekday::Wed => "Miércoles",
                        chrono::Weekday::Thu => "Jueves",
                        chrono::Weekday::Fri => "Viernes",
                        chrono::Weekday::Sat => "Sábado",
                        chrono::Weekday::Sun => "Domingo",
                    };
                    let blocks_today_count = self
                        .config
                        .blocks
                        .iter()
                        .filter(|b| b.days.matches(now_local.weekday()))
                        .count();

                    painter.text(
                        pos2(card_rect.min.x + 18.0, card_rect.max.y - 42.0),
                        Align2::LEFT_BOTTOM,
                        weekday_name,
                        FontId::proportional(15.0),
                        TEXT_PRIMARY,
                    );
                    painter.text(
                        pos2(card_rect.min.x + 18.0, card_rect.max.y - 22.0),
                        Align2::LEFT_BOTTOM,
                        format!("{} eventos", blocks_today_count),
                        FontId::proportional(12.0),
                        TEXT_MUTED,
                    );
                }
                AppMode::SimpleTimer => {
                    let (remaining_secs, triggered) = self.config.timer.evaluate(now_dt);
                    if triggered {
                        self.alarm_manager.trigger(
                            "¡Ya Falta Poco!",
                            "¡Ha llegado la hora objetivo!",
                            self.config.sound_enabled,
                        );
                        ctx.send_viewport_cmd(ViewportCommand::Focus);
                    }

                    draw_gradient_card(ui, card_rect, DEFAULT_ACCENT);

                    let painter = ui.painter();
                    painter.text(
                        pos2(card_rect.min.x + 18.0, card_rect.min.y + 24.0),
                        Align2::LEFT_TOP,
                        "TEMPORIZADOR",
                        FontId::proportional(16.0),
                        TEXT_PRIMARY,
                    );

                    let target_str = format!(
                        "Objetivo: {:02}:{:02}",
                        self.config.timer.target_hour, self.config.timer.target_min
                    );
                    painter.text(
                        pos2(card_rect.min.x + 18.0, card_rect.min.y + 55.0),
                        Align2::LEFT_TOP,
                        target_str,
                        FontId::proportional(13.0),
                        TEXT_MUTED,
                    );

                    painter.text(
                        pos2(card_rect.center().x, card_rect.min.y + 115.0),
                        Align2::CENTER_CENTER,
                        format_duration_hms(remaining_secs),
                        FontId::monospace(26.0),
                        TEXT_PRIMARY,
                    );

                    let state_desc = match self.config.timer.state {
                        TimerState::Running => "Contando...",
                        TimerState::Paused => "Pausado",
                        TimerState::Stopped => "Detenido",
                    };
                    painter.text(
                        pos2(card_rect.center().x, card_rect.min.y + 150.0),
                        Align2::CENTER_CENTER,
                        state_desc,
                        FontId::proportional(13.0),
                        DEFAULT_ACCENT,
                    );
                }
            }

            ui.add_space(16.0);

            // LISTA DERECHA ("UPCOMING" / "HOY")
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("UPCOMING")
                            .color(TEXT_MUTED)
                            .size(13.0)
                            .strong(),
                    );
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(format!("{:.0}%", self.config.opacity * 100.0));
                        ui.add(
                            egui::Slider::new(&mut self.config.opacity, 0.4..=1.0)
                                .show_value(false),
                        );
                    });
                });

                ui.add_space(4.0);

                let today_weekday = now_local.weekday();
                let overlaps = detect_overlaps(&self.config.blocks, now_dt.date());
                if !overlaps.is_empty() {
                    ui.label(
                        egui::RichText::new("⚠ Hay bloques con horarios solapados")
                            .color(Color32::from_rgb(245, 158, 11))
                            .size(11.0),
                    );
                }

                ScrollArea::vertical()
                    .max_height(180.0)
                    .show(ui, |ui| {
                        let mut block_to_edit = None;
                        for (idx, block) in self.config.blocks.iter().enumerate() {
                            if !block.days.matches(today_weekday) {
                                continue;
                            }

                            let is_active = block.is_active_at(now_dt);
                            let bg = if is_active {
                                Color32::from_rgba_premultiplied(
                                    block.color.r,
                                    block.color.g,
                                    block.color.b,
                                    35,
                                )
                            } else {
                                Color32::TRANSPARENT
                            };

                            let frame = egui::Frame::none()
                                .fill(bg)
                                .inner_margin(egui::Margin::symmetric(8.0, 4.0))
                                .rounding(Rounding::same(8.0));

                            frame.show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    let bar_color: Color32 = block.color.into();
                                    let (bar_rect, _) = ui.allocate_exact_size(
                                        vec2(3.5, 24.0),
                                        egui::Sense::hover(),
                                    );
                                    ui.painter().rect_filled(
                                        bar_rect,
                                        Rounding::same(2.0),
                                        bar_color,
                                    );

                                    ui.add_space(6.0);

                                    ui.vertical(|ui| {
                                        let title_resp = ui.link(
                                            egui::RichText::new(&block.title)
                                                .color(if is_active {
                                                    Color32::WHITE
                                                } else {
                                                    TEXT_PRIMARY
                                                })
                                                .size(13.0)
                                                .strong(),
                                        );
                                        if title_resp.clicked() {
                                            block_to_edit = Some(idx);
                                        }

                                        let range_str = format!(
                                            "{:02}:{:02} - {:02}:{:02}",
                                            block.start_hour,
                                            block.start_min,
                                            block.end_hour,
                                            block.end_min
                                        );
                                        ui.label(
                                            egui::RichText::new(range_str)
                                                .color(TEXT_MUTED)
                                                .size(11.0),
                                        );
                                    });
                                });
                            });
                        }

                        if let Some(idx) = block_to_edit {
                            self.open_editor_for_block(idx);
                        }
                    });
            });
        });

        // BOTÓN "+" FLOTANTE
        let widget_rect = ui.min_rect();
        let add_center = pos2(widget_rect.max.x - 30.0, widget_rect.max.y - 30.0);
        if floating_add_button(ui, add_center, 18.0).clicked() {
            self.open_editor_new();
        }

        if self.alarm_manager.is_active() {
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("🔔 ¡Ya es la hora!")
                        .color(DEFAULT_ACCENT)
                        .strong(),
                );
                if ui.button("Detener sonido").clicked() {
                    self.alarm_manager.stop();
                }
            });
        }
    }

    fn render_compact_ui(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        let (_drag_rect, drag_resp) =
            ui.allocate_exact_size(vec2(ui.available_width(), 16.0), egui::Sense::drag());
        if drag_resp.dragged() {
            ctx.send_viewport_cmd(ViewportCommand::StartDrag);
        }
        if drag_resp.double_clicked() {
            self.toggle_compact_mode(ctx);
        }

        let now_dt = Local::now().naive_local();
        let status = evaluate_schedule(&self.config.blocks, now_dt);

        let (title, time_str, progress, color) = match status {
            ScheduleStatus::Active {
                block,
                remaining_secs,
                progress,
            } => (
                block.title.clone(),
                format_duration_hms(remaining_secs),
                progress,
                block.color.into(),
            ),
            ScheduleStatus::Between {
                next_block,
                starts_in_secs,
            } => (
                format!("Sig: {}", next_block.title),
                format_duration_hms(starts_in_secs),
                0.0,
                DEFAULT_ACCENT,
            ),
            ScheduleStatus::DayFinished { .. } => (
                "Día terminado".into(),
                "--:--:--".into(),
                1.0,
                Color32::from_rgb(100, 116, 139),
            ),
            ScheduleStatus::Empty => {
                ("Sin bloques".into(), "--:--:--".into(), 0.0, DEFAULT_ACCENT)
            }
        };

        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(title).strong().size(13.0));
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if ui.button("⤢").clicked() {
                    self.toggle_compact_mode(ctx);
                }
            });
        });

        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(time_str)
                    .color(DEFAULT_ACCENT)
                    .size(20.0)
                    .monospace(),
            );
        });

        mini_progress_bar(ui, progress, color, ui.available_width());
    }

    fn open_editor_new(&mut self) {
        self.editor = BlockEditorState {
            is_open: true,
            editing_index: None,
            title: "Nuevo Bloque".into(),
            start_hour: 9,
            start_min: 0,
            end_hour: 10,
            end_min: 0,
            color: DEFAULT_COLORS[0].1,
            days: WeekdayFilter::all(),
            notify_on_start: true,
            notify_on_end: true,
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
                notify_on_start: b.notify_on_start,
                notify_on_end: b.notify_on_end,
            };
        }
    }

    fn render_block_editor(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.heading(if self.editor.editing_index.is_some() {
                "Editar Bloque"
            } else {
                "Crear Nuevo Bloque"
            });
            ui.add_space(6.0);

            ui.horizontal(|ui| {
                ui.label("Nombre:");
                ui.text_edit_singleline(&mut self.editor.title);
            });

            ui.horizontal(|ui| {
                ui.label("Inicio (HH:MM):");
                ui.add(egui::DragValue::new(&mut self.editor.start_hour).range(0..=23));
                ui.label(":");
                ui.add(egui::DragValue::new(&mut self.editor.start_min).range(0..=59));

                ui.add_space(10.0);
                ui.label("Fin (HH:MM):");
                ui.add(egui::DragValue::new(&mut self.editor.end_hour).range(0..=23));
                ui.label(":");
                ui.add(egui::DragValue::new(&mut self.editor.end_min).range(0..=59));
            });

            let s = self.editor.start_hour * 60 + self.editor.start_min;
            let e = self.editor.end_hour * 60 + self.editor.end_min;
            if e <= s {
                ui.label(
                    egui::RichText::new("ℹ Cruza la medianoche (termina al día siguiente)")
                        .color(Color32::from_rgb(245, 158, 11))
                        .size(11.0),
                );
            }

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

            ui.horizontal(|ui| {
                ui.label("Color:");
                for (_, c) in &DEFAULT_COLORS {
                    let c32: Color32 = (*c).into();
                    let (rect, resp) = ui.allocate_exact_size(vec2(16.0, 16.0), egui::Sense::click());
                    let is_selected = self.editor.color == *c;
                    ui.painter().rect_filled(rect, Rounding::same(4.0), c32);
                    if is_selected {
                        ui.painter().rect_stroke(
                            rect,
                            Rounding::same(4.0),
                            Stroke::new(2.0_f32, Color32::WHITE),
                        );
                    }
                    if resp.clicked() {
                        self.editor.color = *c;
                    }
                }
            });

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                if ui.button("Guardar").clicked() {
                    let new_block = TimeBlock {
                        id: uuid_simple(),
                        title: self.editor.title.clone(),
                        start_hour: self.editor.start_hour,
                        start_min: self.editor.start_min,
                        end_hour: self.editor.end_hour,
                        end_min: self.editor.end_min,
                        color: self.editor.color,
                        days: self.editor.days,
                        notify_on_start: self.editor.notify_on_start,
                        notify_on_end: self.editor.notify_on_end,
                        notify_before_min: Some(5),
                    };

                    if let Some(idx) = self.editor.editing_index {
                        self.config.blocks[idx] = new_block;
                    } else {
                        self.config.blocks.push(new_block);
                    }

                    let _ = self.config.save();
                    self.editor.is_open = false;
                }

                if let Some(idx) = self.editor.editing_index {
                    if ui
                        .add(egui::Button::new("Eliminar").fill(Color32::from_rgb(239, 68, 68)))
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

fn uuid_simple() -> String {
    let now = Local::now().timestamp_nanos_opt().unwrap_or(0);
    format!("b_{}", now)
}
