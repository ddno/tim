use std::sync::{Arc, Mutex};

use fltk::{enums::Color, frame::Frame, prelude::*, window::Window};

use crate::traits::convert_countdown::ConvertCountdown;

pub struct WindowManager {
    pub window: Arc<Mutex<Window>>,
}

impl WindowManager {
    pub fn new(window: Window) -> WindowManager {
        WindowManager {
            window: Arc::new(Mutex::new(window)),
        }
    }

    pub fn get_window(&mut self) -> Arc<Mutex<Window>> {
        let wind = Arc::clone(&self.window);

        wind
    }

    pub fn set_color(&mut self, color: Color) {
        let binding = self.get_window();
        let mut w = binding.lock().unwrap();

        w.set_color(color);
    }

    pub fn update_countdown(&mut self, frame: &mut Frame, countdown: u32, update_background: bool) {
        let (minutes, seconds) = self.to_minutes_seconds(countdown);

        frame.set_label(&format!("{}:{}", minutes, seconds));

        if update_background == false {
            return;
        }

        if countdown == 60 {
            self.set_color(Color::DarkBlue);
        } else if countdown <= 10 && countdown > 0 {
            self.set_color(Color::Red);
        } else if countdown == 0 {
            crate::audio::Audio::play_sound();
        }
    }
}

impl ConvertCountdown for WindowManager {}
