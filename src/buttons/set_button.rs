use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

use fltk::button::Button;
use fltk::enums;
use fltk::enums::Color;
use fltk::group::Flex;
use fltk::prelude::{WidgetBase, WidgetExt};
use fltk::window::Window;

use crate::{ChannelMessage, WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct SetButton {
    pub button: Button,
}

const EXPANDED_WINDOW_HEIGHT: i32 = 190;

impl SetButton {
    pub fn new(
        mut start_button: Button,
        window: Arc<Mutex<Window>>,
        mut flex: Flex,
        thread_tx: Arc<Mutex<Sender<ChannelMessage>>>,
    ) -> Self {
        let mut button = Button::new(105, 90, 85, 40, "Set");

        button.set_color(Color::Red);
        button.set_label_color(Color::Black);
        button.set_frame(enums::FrameType::PlasticThinUpBox);
        button.set_label_font(enums::Font::HelveticaBold);
        button.set_label_size(18);

        button.set_callback(move |_button| {
            start_button.set_label("Start");
            start_button.set_color(Color::Blue);

            let mut local_window = window.lock().unwrap();

            thread_tx
                .lock()
                .unwrap()
                .send(ChannelMessage::StopCountdown)
                .expect("Failed to get stop countdown message");

            if local_window.pixel_h() > WINDOW_HEIGHT + EXPANDED_WINDOW_HEIGHT {
                flex.hide();
                local_window.set_size(WINDOW_WIDTH, WINDOW_HEIGHT);
            } else {
                flex.show();
                local_window.set_size(WINDOW_WIDTH, EXPANDED_WINDOW_HEIGHT);
            }
        });

        Self { button }
    }
}
