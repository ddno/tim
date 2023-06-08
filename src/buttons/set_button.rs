use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

use fltk::button::Button;
use fltk::group::Flex;
use fltk::prelude::{WidgetBase, WidgetExt};
use fltk::window::Window;

use crate::ChannelMessage;

pub struct SetButton {
    pub button: Button,
}

impl SetButton {
    pub fn new(
        mut start_button: Button,
        window: Arc<Mutex<Window>>,
        mut flex: Flex,
        thread_tx: Arc<Mutex<Sender<ChannelMessage>>>,
    ) -> Self {
        let mut button = Button::new(100, 90, 80, 40, "Set");

        button.set_callback(move |_button| {
            println!("Clicked SetButton");

            start_button.set_label("Start");

            let mut local_window = window.lock().unwrap();

            thread_tx
                .lock()
                .unwrap()
                .send(ChannelMessage::StopCountdown)
                .expect("Failed to get stop countdown message");

            if local_window.pixel_h() > 450 {
                flex.hide();
                local_window.set_size(200, 200);
            } else {
                flex.show();
                local_window.set_size(200, 320);
            }
        });

        Self { button }
    }
}
