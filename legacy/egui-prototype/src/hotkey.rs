use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};

pub struct HotkeyManager {
    _manager: GlobalHotKeyManager,
    hotkey: HotKey,
}

impl HotkeyManager {
    pub fn new() -> Result<Self, String> {
        let manager = GlobalHotKeyManager::new().map_err(|e| e.to_string())?;
        // Atajo global Ctrl + Alt + T
        let hotkey = HotKey::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyT);
        manager.register(hotkey).map_err(|e| e.to_string())?;

        Ok(Self {
            _manager: manager,
            hotkey,
        })
    }

    pub fn poll_toggle(&self) -> bool {
        if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
            return event.id == self.hotkey.id();
        }
        false
    }
}
