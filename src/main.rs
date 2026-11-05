#![windows_subsystem = "windows"]

use eframe::egui::vec2;
use eframe::NativeOptions;

mod alarm;
mod app;
mod config;
mod hotkey;
mod schedule;
mod theme;
mod timer;
mod tray;
mod widgets;

use app::YaFaltaPocoApp;
use config::AppConfig;

fn main() -> eframe::Result<()> {
    // Cargar configuración guardada para tamaño e icono
    let (initial_config, _) = AppConfig::load();
    let initial_size = if initial_config.compact_mode {
        vec2(280.0, 95.0)
    } else {
        vec2(620.0, 310.0)
    };

    let mut native_options = NativeOptions::default();
    let mut viewport = eframe::egui::ViewportBuilder::default()
        .with_decorations(false)
        .with_transparent(true)
        .with_inner_size(initial_size)
        .with_min_inner_size(vec2(260.0, 85.0))
        .with_always_on_top()
        .with_visible(true)
        .with_active(true)
        .with_title("Ya Falta Poco");

    if let Some(pos) = initial_config.window_pos {
        viewport = viewport.with_position(eframe::egui::pos2(pos[0], pos[1]));
    }

    native_options.viewport = viewport;

    eframe::run_native(
        "Ya Falta Poco",
        native_options,
        Box::new(|cc| Ok(Box::new(YaFaltaPocoApp::new(cc)))),
    )
}
