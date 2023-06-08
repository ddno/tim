use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};

use fltk::{app, enums::Color, frame::Frame, group, input, prelude::*, window::Window};

use crate::buttons::set_button::SetButton;
use crate::buttons::start_button::StartButton;
use crate::keyboard_event::KeyboardEvent;
use crate::window_manager::WindowManager;

mod audio;
mod buttons;
mod keyboard_event;
mod window_manager;

#[derive(Debug, Copy, Clone)]
pub enum ChannelMessage {
    UpdateCountdown(u32),
    StopCountdown,
}

fn main() {
    let app = app::App::default();

    let mut window_manager = WindowManager::new(
        Window::default()
            .with_size(200, 200)
            .with_label("Tim the Timer")
            .with_pos(50, 160),
    );

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

    let start_button = StartButton::new(
        input_minutes,
        input_seconds,
        thread_rx,
        tx,
        window_manager.get_window().clone(),
        thread_tx.clone(),
    );

    SetButton::new(
        start_button.button.clone(),
        window_manager.get_window().clone(),
        flex,
        thread_tx.clone(),
    );

    KeyboardEvent::new(window_manager.get_window(), start_button.button.clone());

    window_manager.get_window().lock().unwrap().show();

    while app.wait() {
        if let Some(ChannelMessage::UpdateCountdown(countdown)) = rx.recv() {
            window_manager.update_countdown(&mut frame, countdown);
        }
    }
}
