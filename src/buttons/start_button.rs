use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use fltk::button::Button;
use fltk::enums;
use fltk::enums::Color;
use fltk::group::Flex;
use fltk::input::IntInput;
use fltk::prelude::{InputExt, WidgetBase, WidgetExt};
use fltk::window::Window;

use crate::{ChannelMessage, WINDOW_HEIGHT, WINDOW_WIDTH};

#[derive(Debug, Copy, Clone)]
enum State {
    Start,
    Pause,
    Resume(u32),
}

pub struct StartButton {
    pub button: Button,
}

impl StartButton {
    pub fn new(
        input_minutes: IntInput,
        input_seconds: IntInput,
        thread_rx: Arc<Mutex<Receiver<ChannelMessage>>>,
        tx: ::fltk::app::Sender<ChannelMessage>,
        window: Arc<Mutex<Window>>,
        thread_tx: Arc<Mutex<Sender<ChannelMessage>>>,
        mut flex: Flex,
    ) -> Self {
        let mut button = Button::new(10, 90, 85, 40, "Start");
        let mut state = State::Pause;
        let countdown = Arc::new(Mutex::new(60 * 5));

        button.set_color(Color::Blue);
        button.set_label_color(Color::Black);
        button.set_frame(enums::FrameType::PlasticThinUpBox);
        button.set_label_font(enums::Font::HelveticaBold);
        button.set_label_size(18);

        button.set_callback(move |_button| {
            _button.set_color(Color::Blue);

            let start_time = Instant::now();
            let mut duration = Duration::from_secs(
                (input_minutes.value().parse::<i32>().unwrap() * 60
                    + input_seconds.value().parse::<i32>().unwrap()) as u64,
            );

            if _button.label() == "Start" {
                state = State::Start;
            }

            match state {
                State::Start => {
                    _button.set_label("Pause");
                    state = State::Pause;

                    _button.set_color(Color::Green);
                }
                State::Pause => {
                    _button.set_label("Resume");

                    state = State::Resume(*countdown.lock().unwrap());

                    thread_tx
                        .lock()
                        .unwrap()
                        .send(ChannelMessage::StopCountdown)
                        .expect("Failed to get stop countdown message");

                    println!("Did send message to stop countdown");

                    println!("Seconds left: {}", duration.as_secs());

                    return;
                }
                State::Resume(seconds_left) => {
                    _button.set_label("Pause");
                    state = State::Pause;
                    _button.set_color(Color::Green);

                    duration = Duration::from_secs(seconds_left as u64);
                }
            }

            flex.hide();
            window.lock().unwrap().set_size(WINDOW_WIDTH, WINDOW_HEIGHT);
            window.lock().unwrap().set_color(Color::Black);

            // Empty channel to make sure there are no remaining messages to stop countdown before starting (again)
            while let Ok(ChannelMessage::StopCountdown) = thread_rx.lock().unwrap().try_recv() {}

            let thread_rx_clone = thread_rx.clone();

            let countdown_clone = countdown.clone();
            thread::spawn(move || loop {
                let elapsed_time = start_time.elapsed();
                let remaining_time = duration
                    .checked_sub(elapsed_time)
                    .unwrap_or_else(|| Duration::new(0, 0));

                let thread_rx = thread_rx_clone.lock().unwrap();

                if let Ok(ChannelMessage::StopCountdown) = thread_rx.try_recv() {
                    println!("Got message to stop countdown");

                    break;
                }

                tx.send(ChannelMessage::UpdateCountdown(
                    remaining_time.as_secs() as u32,
                    true,
                ));

                *countdown_clone.lock().unwrap() = remaining_time.as_secs() as u32;

                if remaining_time.as_secs() == 0 {
                    break;
                }

                thread::sleep(Duration::from_millis(100));
            });
        });

        Self { button }
    }
}
