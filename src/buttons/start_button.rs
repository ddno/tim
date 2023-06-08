use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use fltk::button::Button;
use fltk::enums::Color;
use fltk::input::IntInput;
use fltk::prelude::{InputExt, WidgetBase, WidgetExt};
use fltk::window::Window;

use crate::ChannelMessage;

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
    ) -> Self {
        let mut button = Button::new(10, 90, 80, 40, "Start");
        let mut state = State::Pause;
        let countdown = Arc::new(Mutex::new(60 * 5));

        button.set_callback(move |_button| {
            println!("Clicked StartButton");

            println!("Input minutes value: {}", input_minutes.value());
            println!("Input minutes value: {}", input_seconds.value());

            dbg!(state);

            let start_time = Instant::now();
            let mut duration = Duration::from_secs(
                (input_minutes.value().parse::<i32>().unwrap() * 60
                    + input_seconds.value().parse::<i32>().unwrap()) as u64,
            );

            if _button.label() == "Start" {
                println!("Start button label is start");

                state = State::Start;
            }

            match state {
                State::Start => {
                    _button.set_label("Pause");
                    state = State::Pause;
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

                    duration = Duration::from_secs(seconds_left as u64);
                }
            }

            window.lock().unwrap().set_size(200, 200);
            window.lock().unwrap().set_color(Color::Black);

            while let Ok(ChannelMessage::StopCountdown) = thread_rx.lock().unwrap().try_recv() {
                println!("Clearing channel");
            }

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
                    remaining_time.as_secs() as u32
                ));

                *countdown_clone.lock().unwrap() = remaining_time.as_secs() as u32;

                if remaining_time.as_secs() == 0 {
                    break;
                }

                thread::sleep(Duration::from_millis(500));
            });
        });

        Self { button }
    }
}
