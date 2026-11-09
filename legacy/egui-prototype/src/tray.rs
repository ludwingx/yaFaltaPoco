use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem},
    Icon, TrayIcon, TrayIconBuilder,
};

pub const ID_TOGGLE_SHOW: &str = "tray_toggle_show";
pub const ID_ALWAYS_ON_TOP: &str = "tray_always_on_top";
pub const ID_PASSTHROUGH: &str = "tray_passthrough";
pub const ID_QUIT: &str = "tray_quit";

pub struct SystemTray {
    _tray_icon: TrayIcon,
}

impl SystemTray {
    pub fn new() -> Result<Self, String> {
        let tray_menu = Menu::new();
        let show_i = MenuItem::with_id(ID_TOGGLE_SHOW, "Mostrar / Ocultar", true, None);
        let on_top_i = MenuItem::with_id(ID_ALWAYS_ON_TOP, "Siempre visible (Pin)", true, None);
        let pass_i = MenuItem::with_id(ID_PASSTHROUGH, "Click-through", true, None);
        let quit_i = MenuItem::with_id(ID_QUIT, "Salir de Ya Falta Poco", true, None);

        tray_menu.append(&show_i).map_err(|e| e.to_string())?;
        tray_menu.append(&on_top_i).map_err(|e| e.to_string())?;
        tray_menu.append(&pass_i).map_err(|e| e.to_string())?;
        tray_menu.append(&quit_i).map_err(|e| e.to_string())?;

        // Ícono simple 16x16 transparente / cyan
        let icon_data = create_default_icon_bytes();
        let icon = Icon::from_rgba(icon_data, 16, 16).map_err(|e| e.to_string())?;

        let tray = TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu))
            .with_tooltip("Ya Falta Poco - Temporizador y Agenda")
            .with_icon(icon)
            .build()
            .map_err(|e| e.to_string())?;

        Ok(Self { _tray_icon: tray })
    }

    pub fn poll_event() -> Option<String> {
        if let Ok(event) = MenuEvent::receiver().try_recv() {
            Some(event.id.0)
        } else {
            None
        }
    }
}

fn create_default_icon_bytes() -> Vec<u8> {
    let mut rgba = Vec::with_capacity(16 * 16 * 4);
    for y in 0..16 {
        for x in 0..16 {
            let dist_sq = (x as i32 - 8).pow(2) + (y as i32 - 8).pow(2);
            if dist_sq <= 49 {
                // Interior cyan
                rgba.extend_from_slice(&[34, 211, 238, 255]);
            } else {
                rgba.extend_from_slice(&[0, 0, 0, 0]);
            }
        }
    }
    rgba
}
