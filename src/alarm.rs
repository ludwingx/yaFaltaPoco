use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

#[derive(Clone, Default)]
pub struct AlarmManager {
    is_ringing: Arc<AtomicBool>,
}

impl AlarmManager {
    pub fn new() -> Self {
        Self {
            is_ringing: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn is_active(&self) -> bool {
        self.is_ringing.load(Ordering::SeqCst)
    }

    pub fn trigger(&self, _title: &str, _message: &str, sound_enabled: bool) {
        if self.is_active() {
            return;
        }
        self.is_ringing.store(true, Ordering::SeqCst);

        if sound_enabled {
            let ringing = Arc::clone(&self.is_ringing);
            thread::spawn(move || {
                let mut count = 0;
                while ringing.load(Ordering::SeqCst) && count < 8 {
                    play_bell_sound();
                    thread::sleep(Duration::from_millis(600));
                    count += 1;
                }
                ringing.store(false, Ordering::SeqCst);
            });
        }
    }

    pub fn stop(&self) {
        self.is_ringing.store(false, Ordering::SeqCst);
    }
}

fn play_bell_sound() {
    print!("\x07"); // ASCII Bell terminal beep estándar multiplataforma
}
