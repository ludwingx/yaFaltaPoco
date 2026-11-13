#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use chrono::{Datelike, Local, Timelike};
use serde::Serialize;
use tauri::{AppHandle, LogicalSize, Manager, Size, State, WebviewWindow};

mod alarm;
mod config;
mod schedule;
mod timer;

use alarm::AlarmManager;
use config::{AppConfig, AppMode};
use schedule::{evaluate_schedule, format_duration_hms, ColorRgb, ScheduleStatus, TimeBlock, WeekdayFilter};

#[derive(Default)]
pub struct TurnTracker {
    pub last_active_block_id: Option<String>,
    pub last_remaining_secs: Option<i64>,
    pub last_alarm_played_for: Option<String>,
    pub should_play_sound: bool,
}

pub struct AppState {
    pub config: Mutex<AppConfig>,
    pub alarm: Mutex<AlarmManager>,
    pub tracker: Mutex<TurnTracker>,
}

#[derive(Serialize)]
pub struct AppSnapshot {
    pub date_title: String,
    pub current_time_str: String,
    pub weekday_str: String,
    pub countdown_str: String,
    pub block_name: String,
    pub events_count_str: String,
    pub mode: String,
    pub blocks: Vec<TimeBlock>,
    pub is_free_day: bool,
    pub always_on_top: bool,
    pub alarm_active: bool,
    pub should_play_sound: bool,
    pub sound_enabled: bool,
}

#[tauri::command]
fn get_snapshot(state: State<'_, AppState>) -> AppSnapshot {
    let cfg = state.config.lock().unwrap();
    let alarm = state.alarm.lock().unwrap();
    let mut tracker = state.tracker.lock().unwrap();
    let now_local = Local::now();
    let now_dt = now_local.naive_local();
    let today_date_str = now_local.format("%Y%m%d").to_string();

    let month_name = match now_local.month() {
        1 => "Ene", 2 => "Feb", 3 => "Mar", 4 => "Abr",
        5 => "May", 6 => "Jun", 7 => "Jul", 8 => "Ago",
        9 => "Sep", 10 => "Oct", 11 => "Nov", _ => "Dic",
    };
    let is_sunday = now_local.weekday() == chrono::Weekday::Sun;
    let day_abbrev = match now_local.weekday() {
        chrono::Weekday::Mon => "Lun",
        chrono::Weekday::Tue => "Mar",
        chrono::Weekday::Wed => "Mié",
        chrono::Weekday::Thu => "Jue",
        chrono::Weekday::Fri => "Vie",
        chrono::Weekday::Sat => "Sáb",
        chrono::Weekday::Sun => "",
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

    let date_title = if is_sunday {
        format!("{:02} {} {}", now_local.day(), month_name, now_local.year())
    } else {
        format!("{} {:02} {} {}", day_abbrev, now_local.day(), month_name, now_local.year())
    };
    let current_time_str = format!(
        "{:02}:{:02}:{:02}",
        now_local.hour(),
        now_local.minute(),
        now_local.second()
    );

    let (countdown_str, block_name) = match cfg.mode {
        AppMode::Schedule => {
            let status = evaluate_schedule(&cfg.blocks, now_dt);
            match status {
                ScheduleStatus::Active { block, remaining_secs, .. } => {
                    let block_key = format!("end_{}_{}", block.id, today_date_str);
                    if remaining_secs <= 0 && tracker.last_alarm_played_for.as_deref() != Some(&block_key) {
                        if tracker.last_active_block_id.as_deref() == Some(&block.id) {
                            tracker.should_play_sound = true;
                            tracker.last_alarm_played_for = Some(block_key);
                        }
                    }
                    tracker.last_active_block_id = Some(block.id.clone());
                    tracker.last_remaining_secs = Some(remaining_secs);
                    (format_duration_hms(remaining_secs), format!("Termina: {}", block.title))
                }
                ScheduleStatus::Between { next_block, starts_in_secs } => {
                    if let Some(prev_id) = tracker.last_active_block_id.take() {
                        let prev_key = format!("end_{}_{}", prev_id, today_date_str);
                        if tracker.last_alarm_played_for.as_deref() != Some(&prev_key) {
                            tracker.should_play_sound = true;
                            tracker.last_alarm_played_for = Some(prev_key);
                        }
                    }
                    tracker.last_remaining_secs = Some(starts_in_secs);
                    (format_duration_hms(starts_in_secs), format!("Siguiente: {}", next_block.title))
                }
                ScheduleStatus::DayFinished => {
                    if let Some(prev_id) = tracker.last_active_block_id.take() {
                        let prev_key = format!("end_{}_{}", prev_id, today_date_str);
                        if tracker.last_alarm_played_for.as_deref() != Some(&prev_key) {
                            tracker.should_play_sound = true;
                            tracker.last_alarm_played_for = Some(prev_key);
                        }
                    }
                    tracker.last_remaining_secs = Some(0);
                    ("00:00:00".into(), "Día terminado".into())
                }
                ScheduleStatus::FreeDay => {
                    tracker.last_active_block_id = None;
                    tracker.last_remaining_secs = None;
                    let free_label = if is_sunday {
                        "¡Domingo Free! 🎉".to_string()
                    } else {
                        "Día libre 🎉".to_string()
                    };
                    ("LIBRE".into(), free_label)
                }
                ScheduleStatus::Empty => {
                    tracker.last_active_block_id = None;
                    tracker.last_remaining_secs = None;
                    ("--:--:--".into(), "Sin bloques".into())
                }
            }
        }
        AppMode::SimpleTimer => {
            if is_sunday {
                ("LIBRE".into(), "Domingo Free".into())
            } else {
                let (remaining_secs, reached) = cfg.timer.evaluate(now_dt);
                let timer_key = format!("timer_{}_{:02}:{:02}", today_date_str, cfg.timer.target_hour, cfg.timer.target_min);
                if (remaining_secs <= 0 || reached) && tracker.last_alarm_played_for.as_deref() != Some(&timer_key) {
                    if tracker.last_remaining_secs.map_or(false, |s| s > 0) {
                        tracker.should_play_sound = true;
                        tracker.last_alarm_played_for = Some(timer_key);
                    }
                }
                tracker.last_remaining_secs = Some(remaining_secs);
                let target_str = format!("Meta: {:02}:{:02}", cfg.timer.target_hour, cfg.timer.target_min);
                (format_duration_hms(remaining_secs), target_str)
            }
        }
    };

    let today_blocks_count = cfg.blocks
        .iter()
        .filter(|b| b.days.matches(now_local.weekday()))
        .count();
    let events_count_str = format!("{} bloques", today_blocks_count);
    let is_free_day = is_sunday || today_blocks_count == 0;

    let mode_str = match cfg.mode {
        AppMode::Schedule => "Horario",
        AppMode::SimpleTimer => "Temporizador",
    };

    let should_play = tracker.should_play_sound && cfg.sound_enabled;
    tracker.should_play_sound = false;

    AppSnapshot {
        date_title,
        current_time_str,
        weekday_str: weekday_str.into(),
        countdown_str,
        block_name,
        events_count_str,
        mode: mode_str.into(),
        blocks: cfg.blocks.clone(),
        is_free_day,
        always_on_top: cfg.always_on_top,
        alarm_active: alarm.is_active(),
        should_play_sound: should_play,
        sound_enabled: cfg.sound_enabled,
    }
}

fn get_today_last_block_end(blocks: &[TimeBlock], weekday: chrono::Weekday) -> Option<(u32, u32)> {
    blocks
        .iter()
        .filter(|b| b.days.matches(weekday))
        .max_by_key(|b| b.end_hour * 60 + b.end_min)
        .map(|b| (b.end_hour, b.end_min))
}

#[tauri::command]
fn drag_window(window: WebviewWindow) {
    let _ = window.start_dragging();
}

#[tauri::command]
fn resize_window(state: State<'_, AppState>, window: WebviewWindow, width: f64, height: f64) {
    let w = width.max(320.0);
    let h = height.max(180.0);
    let _ = window.set_size(Size::Logical(LogicalSize {
        width: w,
        height: h,
    }));
    let mut cfg = state.config.lock().unwrap();
    cfg.window_size = Some([w, h]);
    let _ = cfg.save();
}

#[tauri::command]
fn toggle_mode(state: State<'_, AppState>) -> String {
    let mut cfg = state.config.lock().unwrap();
    cfg.mode = match cfg.mode {
        AppMode::Schedule => {
            let now_local = Local::now();
            if let Some((h, m)) = get_today_last_block_end(&cfg.blocks, now_local.weekday()) {
                cfg.timer.target_hour = h;
                cfg.timer.target_min = m;
            }
            AppMode::SimpleTimer
        }
        AppMode::SimpleTimer => AppMode::Schedule,
    };
    let _ = cfg.save();
    match cfg.mode {
        AppMode::Schedule => "Horario".into(),
        AppMode::SimpleTimer => "Temporizador".into(),
    }
}

#[tauri::command]
fn set_timer_target(state: State<'_, AppState>, hour: u32, min: u32) {
    let mut cfg = state.config.lock().unwrap();
    cfg.timer.target_hour = hour.min(23);
    cfg.timer.target_min = min.min(59);
    let _ = cfg.save();
}

#[tauri::command]
fn toggle_pin(app: AppHandle, state: State<'_, AppState>) -> bool {
    let mut cfg = state.config.lock().unwrap();
    cfg.always_on_top = !cfg.always_on_top;
    if let Some(win) = app.get_webview_window("main") {
        let _ = win.set_always_on_top(cfg.always_on_top);
    }
    let _ = cfg.save();
    cfg.always_on_top
}

#[tauri::command]
fn save_block(
    state: State<'_, AppState>,
    index: Option<usize>,
    title: String,
    start_hour: u32,
    start_min: u32,
    end_hour: u32,
    end_min: u32,
    color_r: u8,
    color_g: u8,
    color_b: u8,
    icon: Option<String>,
    card_bg: Option<String>,
    monday: bool,
    tuesday: bool,
    wednesday: bool,
    thursday: bool,
    friday: bool,
    saturday: bool,
    sunday: bool,
) {
    let mut cfg = state.config.lock().unwrap();
    let mut block = TimeBlock {
        id: format!("b_{}", Local::now().timestamp_millis()),
        title,
        start_hour,
        start_min,
        end_hour,
        end_min,
        color: ColorRgb::new(color_r, color_g, color_b),
        icon,
        card_bg,
        days: WeekdayFilter {
            monday,
            tuesday,
            wednesday,
            thursday,
            friday,
            saturday,
            sunday,
        },
        notify_on_start: true,
        notify_on_end: true,
        notify_before_min: Some(5),
    };

    if let Some(idx) = index {
        if idx < cfg.blocks.len() {
            block.id = cfg.blocks[idx].id.clone();
            cfg.blocks[idx] = block;
        }
    } else {
        cfg.blocks.push(block);
    }

    let now_local = Local::now();
    if let Some((h, m)) = get_today_last_block_end(&cfg.blocks, now_local.weekday()) {
        cfg.timer.target_hour = h;
        cfg.timer.target_min = m;
    }

    let _ = cfg.save();
}

#[tauri::command]
fn delete_block(state: State<'_, AppState>, index: usize) {
    let mut cfg = state.config.lock().unwrap();
    if index < cfg.blocks.len() {
        cfg.blocks.remove(index);
        let now_local = Local::now();
        if let Some((h, m)) = get_today_last_block_end(&cfg.blocks, now_local.weekday()) {
            cfg.timer.target_hour = h;
            cfg.timer.target_min = m;
        }
        let _ = cfg.save();
    }
}

#[tauri::command]
fn toggle_sound(state: State<'_, AppState>) -> bool {
    let mut cfg = state.config.lock().unwrap();
    cfg.sound_enabled = !cfg.sound_enabled;
    let _ = cfg.save();
    cfg.sound_enabled
}

#[tauri::command]
fn stop_alarm(state: State<'_, AppState>) {
    let alarm = state.alarm.lock().unwrap();
    alarm.stop();
}

#[tauri::command]
fn close_app(app: AppHandle, state: State<'_, AppState>) {
    let cfg = state.config.lock().unwrap();
    let _ = cfg.save();
    app.exit(0);
}

#[tauri::command]
fn minimize_window(window: WebviewWindow) {
    let _ = window.minimize();
}

#[tauri::command]
fn toggle_maximize_window(window: WebviewWindow) -> bool {
    if let Ok(is_max) = window.is_maximized() {
        if is_max {
            let _ = window.unmaximize();
            false
        } else {
            let _ = window.maximize();
            true
        }
    } else {
        false
    }
}

#[tauri::command]
fn is_window_maximized(window: WebviewWindow) -> bool {
    window.is_maximized().unwrap_or(false)
}

fn main() {
    let (mut initial_config, _) = AppConfig::load();
    let alarm = AlarmManager::new();

    let now_local = Local::now();
    if let Some((h, m)) = get_today_last_block_end(&initial_config.blocks, now_local.weekday()) {
        if initial_config.timer.target_hour == 18 && initial_config.timer.target_min == 0 {
            initial_config.timer.target_hour = h;
            initial_config.timer.target_min = m;
            let _ = initial_config.save();
        }
    }

    tauri::Builder::default()
        .setup(|app| {
            if let Some(win) = app.get_webview_window("main") {
                let state = app.state::<AppState>();
                let cfg = state.config.lock().unwrap();
                if let Some([w, h]) = cfg.window_size {
                    let _ = win.set_size(Size::Logical(LogicalSize { width: w, height: h }));
                }
            }
            Ok(())
        })
        .manage(AppState {
            config: Mutex::new(initial_config),
            alarm: Mutex::new(alarm),
            tracker: Mutex::new(TurnTracker::default()),
        })
        .invoke_handler(tauri::generate_handler![
            get_snapshot,
            drag_window,
            resize_window,
            toggle_mode,
            toggle_sound,
            set_timer_target,
            toggle_pin,
            save_block,
            delete_block,
            stop_alarm,
            close_app,
            minimize_window,
            toggle_maximize_window,
            is_window_maximized,
        ])
        .run(tauri::generate_context!())
        .expect("error running tauri app");
}
