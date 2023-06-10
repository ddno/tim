use std::sync::{Arc, Mutex};

use fltk::{enums::Color, frame::Frame, prelude::*, window::Window};

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

    pub fn update_countdown(&mut self, frame: &mut Frame, countdown: u32) {
        let mut seconds = countdown;
        let mut minutes = 0;

        if seconds >= 60 {
            minutes = seconds / 60;
        }

        seconds = seconds - (minutes * 60);

        let mut minutes_string = minutes.to_string();
        if minutes < 10 {
            minutes_string = "0".to_string() + &minutes.to_string();
        }

        let mut seconds_string = seconds.to_string();
        if seconds < 10 {
            seconds_string = "0".to_string() + &seconds.to_string();
        }

        frame.set_label(&*String::from(minutes_string + ":" + &seconds_string));

        if countdown == 60 {
            self.set_color(Color::DarkBlue);
        } else if countdown <= 10 && countdown > 0 {
            self.set_color(Color::Red);
        } else if countdown == 0 {
            crate::audio::Audio::play_sound();
        }
    }
}
