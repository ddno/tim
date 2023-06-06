use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::{env, fs};

use fltk::button::Button;
use fltk::enums::{Event, Key};
use fltk::{app, enums, enums::Color, frame::Frame, group, input, prelude::*, window::Window};

use crate::window_manager::WindowManager;

mod audio;
mod window_manager;

#[derive(Debug, Copy, Clone)]
enum ChannelMessage {
    UpdateCountdown(u32),
    StopCountdown,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum StartButtonStatus {
    Start,
    Pause,
    Resume(u32),
}

struct SharedState {
    pub countdown: u32,
}

impl SharedState {
    pub fn new() -> SharedState {
        SharedState { countdown: 5 * 60 }
    }
}

fn main() {
    let app = app::App::default();

    let mut start_button_status = StartButtonStatus::Start;

    let mut window_manager = WindowManager::new(
        Window::default()
            .with_size(200, 200)
            .with_label("Tim the Timer")
            .with_pos(50, 160),
    );

    let shared_state = Arc::new(Mutex::new(SharedState::new()));

    window_manager.set_color(Color::Black);

    let mut frame = Frame::default()
        .with_pos(0, -50)
        .size_of(&*window_manager.get_window().lock().unwrap());

    window_manager.update_countdown(&mut frame, 60 * 5);

    frame.set_label_size(70);

    frame.set_label_color(Color::White);

    let mut flex = group::Flex::default()
        .with_size(100, 100)
        .column()
        .with_pos(10, 210);
    let mut input_minutes = input::IntInput::default();
    let mut input_seconds = input::IntInput::default();

    input_minutes.set_value(&"5".to_owned());
    input_seconds.set_value(&"0".to_owned());

    flex.end();

    flex.hide();

    let (tx, rx) = app::channel::<ChannelMessage>();

    let (thread_tx, thread_rx) = channel();
    let thread_tx = Arc::new(Mutex::new(thread_tx));
    let thread_rx = Arc::new(Mutex::new(thread_rx));

    let mut start_button = Button::new(10, 90, 80, 40, "Start");
    let start_button_window_clone = window_manager.get_window();

    let thread_tx_clone_1 = thread_tx.clone();

    let shared_state_1 = shared_state.clone();

    start_button.set_callback(move |_button| {
        println!("Clicked button");

        println!("Input minutes value: {}", input_minutes.value());
        println!("Input minutes value: {}", input_seconds.value());

        let start_time = Instant::now();
        let mut duration = Duration::from_secs(
            (input_minutes.value().parse::<i32>().unwrap() * 60
                + input_seconds.value().parse::<i32>().unwrap()) as u64,
        );

        if _button.label() == "Start" {
            println!("Start button label is start");

            start_button_status = StartButtonStatus::Start;
        }

        match start_button_status {
            StartButtonStatus::Start => {
                _button.set_label("Pause");
                start_button_status = StartButtonStatus::Pause;
            }
            StartButtonStatus::Pause => {
                _button.set_label("Resume");

                start_button_status =
                    StartButtonStatus::Resume(shared_state_1.lock().unwrap().countdown);

                thread_tx_clone_1
                    .lock()
                    .unwrap()
                    .send(ChannelMessage::StopCountdown)
                    .expect("Failed to get stop countdown message");

                println!("Did send message to stop countdown");

                println!("Seconds left: {}", duration.as_secs());

                return;
            }
            StartButtonStatus::Resume(seconds_left) => {
                _button.set_label("Pause");
                start_button_status = StartButtonStatus::Pause;

                println!("Will resume countdown");

                println!("{}", shared_state_1.lock().unwrap().countdown);

                println!("Seconds left: {}", seconds_left);

                duration = Duration::from_secs(seconds_left as u64);
            }
        }

        start_button_window_clone.lock().unwrap().set_size(200, 200);
        start_button_window_clone
            .lock()
            .unwrap()
            .set_color(Color::Black);

        while let Ok(ChannelMessage::StopCountdown) = thread_rx.lock().unwrap().try_recv() {
            println!("Clearing channel");
        }

        let thread_rx_clone = thread_rx.clone();
        let shared_state_2 = shared_state_1.clone();

        std::thread::spawn(move || loop {
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

            shared_state_2.lock().unwrap().countdown = remaining_time.as_secs() as u32;

            if remaining_time.as_secs() == 0 {
                break;
            }

            sleep(Duration::from_millis(500));
        });
    });

    let wind = window_manager.get_window();
    let mut start_button_clone = start_button.clone();
    wind.lock().unwrap().handle(move |_, ev| match ev {
        Event::KeyDown => {
            if app::event_key() == Key::Enter {
                println!("Key was enter");

                start_button_clone.do_callback();
            }
            false
        }
        _ => false,
    });

    let mut set_button = Button::new(100, 90, 80, 40, "Set");
    let set_button_window_clone = window_manager.get_window();

    let thread_tx_clone = thread_tx.clone();

    set_button.set_callback(move |_button| {
        println!("Clicked set_button");

        start_button.set_label("Start");

        let mut local_window = set_button_window_clone.lock().unwrap();

        thread_tx_clone
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

    window_manager.get_window().lock().unwrap().show();

    while app.wait() {
        if let Some(ChannelMessage::UpdateCountdown(countdown)) = rx.recv() {
            window_manager.update_countdown(&mut frame, countdown);
        }
    }
}
