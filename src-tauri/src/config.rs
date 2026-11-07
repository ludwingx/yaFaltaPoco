use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

use crate::schedule::{default_sample_blocks, ColorRgb, TimeBlock};
use crate::timer::SimpleTimer;

pub const CONFIG_FILE_NAME: &str = "yafaltapoco.json";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppMode {
    Schedule,
    SimpleTimer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub mode: AppMode,
    pub blocks: Vec<TimeBlock>,
    pub timer: SimpleTimer,
    pub window_pos: Option<[f32; 2]>,
    #[serde(default)]
    pub window_size: Option<[f64; 2]>,
    pub always_on_top: bool,
    pub opacity: f32,
    pub compact_mode: bool,
    pub accent_color: ColorRgb,
    pub hide_taskbar_icon: bool,
    pub mouse_passthrough: bool,
    pub sound_enabled: bool,
    pub notifications_enabled: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            mode: AppMode::Schedule,
            blocks: default_sample_blocks(),
            timer: SimpleTimer::default(),
            window_pos: None,
            window_size: None,
            always_on_top: true,
            opacity: 0.95,
            compact_mode: false,
            accent_color: ColorRgb::new(34, 211, 238), // Neon cyan #22D3EE
            hide_taskbar_icon: false,
            mouse_passthrough: false,
            sound_enabled: true,
            notifications_enabled: true,
        }
    }
}

impl AppConfig {
    pub fn get_exe_dir() -> PathBuf {
        std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .unwrap_or_else(|| PathBuf::from("."))
    }

    pub fn get_config_path() -> PathBuf {
        Self::get_exe_dir().join(CONFIG_FILE_NAME)
    }

    /// Carga la configuración desde el archivo JSON portable.
    /// Si ocurre un error al deserializar, hace copia de seguridad del archivo dañado
    /// y retorna la configuración por defecto para no perder la app.
    pub fn load() -> (Self, Option<String>) {
        let path = Self::get_config_path();
        if !path.exists() {
            return (Self::default(), None);
        }

        match fs::read_to_string(&path) {
            Ok(content) => match serde_json::from_str::<AppConfig>(&content) {
                Ok(config) => (config, None),
                Err(err) => {
                    // Copia de seguridad del archivo corrupto
                    let backup_path = path.with_extension("json.corrupted.bak");
                    let _ = fs::copy(&path, &backup_path);
                    let warn = format!(
                        "Error al leer {}: {}. Se respaldó en {:?} y se restauraron valores por defecto.",
                        CONFIG_FILE_NAME, err, backup_path
                    );
                    (Self::default(), Some(warn))
                }
            },
            Err(err) => {
                let warn = format!("No se pudo leer {}: {}", CONFIG_FILE_NAME, err);
                (Self::default(), Some(warn))
            }
        }
    }

    /// Guarda la configuración con escritura atómica (archivo temporal y reemplazo)
    pub fn save(&self) -> Result<(), String> {
        let path = Self::get_config_path();
        let temp_path = path.with_extension("json.tmp");

        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Error al serializar configuración: {}", e))?;

        // Probar si el directorio es escribible
        let mut file = fs::File::create(&temp_path).map_err(|e| {
            format!(
                "No se puede escribir en {}: {}. Verifique permisos de carpeta.",
                temp_path.display(),
                e
            )
        })?;

        file.write_all(json.as_bytes())
            .map_err(|e| format!("Error escribiendo datos temporales: {}", e))?;
        file.sync_all().map_err(|e| format!("Error sincronizando archivo: {}", e))?;
        drop(file);

        // Reemplazar atómicamente
        if path.exists() {
            let _ = fs::remove_file(&path);
        }
        fs::rename(&temp_path, &path).map_err(|e| {
            format!("Error al reemplazar {} por temporal: {}", path.display(), e)
        })?;

        Ok(())
    }

    #[allow(dead_code)]
    pub fn export_to_file(&self, path: &Path) -> Result<(), String> {
        let json = serde_json::to_string_pretty(&self.blocks)
            .map_err(|e| format!("Error serializando agenda: {}", e))?;
        fs::write(path, json).map_err(|e| format!("Error guardando archivo: {}", e))
    }

    #[allow(dead_code)]
    pub fn import_from_file(&mut self, path: &Path) -> Result<usize, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Error leyendo archivo de agenda: {}", e))?;
        let blocks: Vec<TimeBlock> = serde_json::from_str(&content)
            .map_err(|e| format!("Formato de agenda inválido: {}", e))?;
        let count = blocks.len();
        self.blocks = blocks;
        Ok(count)
    }
}
