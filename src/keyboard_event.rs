use std::sync::{Arc, Mutex};

use fltk::app;
use fltk::button::Button;
use fltk::enums::{Event, Key};
use fltk::prelude::{WidgetBase, WidgetExt};
use fltk::window::Window;

pub struct KeyboardEvent {}

impl KeyboardEvent {
    pub fn new(window: Arc<Mutex<Window>>, mut start_button: Button) {
        window.lock().unwrap().handle(move |_, ev| match ev {
            Event::KeyDown => {
                if app::event_key() == Key::Enter {
                    println!("Key was enter from Keyboard Event");

                    start_button.do_callback();
                }
                false
            }
            _ => false,
        });
    }
}
