use std::sync::Mutex;
use chrono::{Datelike, Local, Timelike};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, State};

mod alarm;
mod config;
mod schedule;
mod timer;

use alarm::AlarmManager;
use config::{AppConfig, AppMode};
use schedule::{evaluate_schedule, format_duration_hms, ColorRgb, ScheduleStatus, TimeBlock, WeekdayFilter};

pub struct AppState {
    pub config: Mutex<AppConfig>,
    pub alarm: Mutex<AlarmManager>,
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
    pub always_on_top: bool,
    pub alarm_active: bool,
}

#[tauri::command]
fn get_snapshot(state: State<'_, AppState>) -> AppSnapshot {
    let mut cfg = state.config.lock().unwrap();
    let alarm = state.alarm.lock().unwrap();
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

    let date_title = format!("{} {:02}", month_name, now_local.day());
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
                    (format_duration_hms(remaining_secs), format!("Termina: {}", block.title))
                }
                ScheduleStatus::Between { next_block, starts_in_secs } => {
                    (format_duration_hms(starts_in_secs), format!("Siguiente: {}", next_block.title))
                }
                ScheduleStatus::DayFinished => ("00:00:00".into(), "Día terminado".into()),
                ScheduleStatus::Empty => ("--:--:--".into(), "Sin bloques".into()),
            }
        }
        AppMode::SimpleTimer => {
            let (remaining_secs, _) = cfg.timer.evaluate(now_dt);
            let target_str = format!("Meta: {:02}:{:02}", cfg.timer.target_hour, cfg.timer.target_min);
            (format_duration_hms(remaining_secs), target_str)
        }
    };

    let today_blocks_count = cfg.blocks
        .iter()
        .filter(|b| b.days.matches(now_local.weekday()))
        .count();
    let events_count_str = format!("{} bloques", today_blocks_count);

    let mode_str = match cfg.mode {
        AppMode::Schedule => "Horario",
        AppMode::SimpleTimer => "Temporizador",
    };

    AppSnapshot {
        date_title,
        current_time_str,
        weekday_str: weekday_str.into(),
        countdown_str,
        block_name,
        events_count_str,
        mode: mode_str.into(),
        blocks: cfg.blocks.clone(),
        always_on_top: cfg.always_on_top,
        alarm_active: alarm.is_active(),
    }
}

#[tauri::command]
fn toggle_mode(state: State<'_, AppState>) -> String {
    let mut cfg = state.config.lock().unwrap();
    cfg.mode = match cfg.mode {
        AppMode::Schedule => AppMode::SimpleTimer,
        AppMode::SimpleTimer => AppMode::Schedule,
    };
    let _ = cfg.save();
    match cfg.mode {
        AppMode::Schedule => "Horario".into(),
        AppMode::SimpleTimer => "Temporizador".into(),
    }
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
    monday: bool,
    tuesday: bool,
    wednesday: bool,
    thursday: bool,
    friday: bool,
    saturday: bool,
    sunday: bool,
) {
    let mut cfg = state.config.lock().unwrap();
    let block = TimeBlock {
        id: format!("b_{}", Local::now().timestamp_millis()),
        title,
        start_hour,
        start_min,
        end_hour,
        end_min,
        color: ColorRgb::new(color_r, color_g, color_b),
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
            cfg.blocks[idx] = block;
        }
    } else {
        cfg.blocks.push(block);
    }
    let _ = cfg.save();
}

#[tauri::command]
fn delete_block(state: State<'_, AppState>, index: usize) {
    let mut cfg = state.config.lock().unwrap();
    if index < cfg.blocks.len() {
        cfg.blocks.remove(index);
        let _ = cfg.save();
    }
}

#[tauri::command]
fn stop_alarm(state: State<'_, AppState>) {
    let mut alarm = state.alarm.lock().unwrap();
    alarm.stop();
}

#[tauri::command]
fn close_app(app: AppHandle, state: State<'_, AppState>) {
    let cfg = state.config.lock().unwrap();
    let _ = cfg.save();
    app.exit(0);
}

fn main() {
    let (initial_config, _) = AppConfig::load();
    let alarm = AlarmManager::new();

    tauri::Builder::default()
        .manage(AppState {
            config: Mutex::new(initial_config),
            alarm: Mutex::new(alarm),
        })
        .invoke_handler(tauri::generate_handler![
            get_snapshot,
            toggle_mode,
            toggle_pin,
            save_block,
            delete_block,
            stop_alarm,
            close_app,
        ])
        .run(tauri::generate_context!())
        .expect("error running tauri app");
}
