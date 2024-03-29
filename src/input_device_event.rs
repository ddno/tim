use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use fltk::app;
use fltk::app::{MouseWheel, Sender};
use fltk::button::Button;
use fltk::enums::{Event, Key};
use fltk::input::IntInput;
use fltk::prelude::{InputExt, WidgetBase, WidgetExt};
use fltk::window::Window;

use crate::buttons::set_button::EXPANDED_WINDOW_HEIGHT;
use crate::{ChannelMessage, WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct InputDeviceEvent {}

impl InputDeviceEvent {
    pub fn new(
        window: Arc<Mutex<Window>>,
        mut start_button: Button,
        input_minutes: IntInput,
        input_seconds: IntInput,
        tx: Sender<ChannelMessage>,
    ) {
        let mut input_minutes_clone = input_minutes.clone();

        let mut input_minutes = input_minutes.clone();
        let mut input_seconds = input_seconds.clone();

        let change_countdown = move |increase_minutes: i32, increase_seconds: i32| {
            let mut new_input_minutes =
                input_minutes.value().parse::<i32>().unwrap() + increase_minutes;

            if new_input_minutes < 0 {
                new_input_minutes = 0;
            }
            if new_input_minutes > 99 {
                new_input_minutes = 99;
            }

            let mut new_input_seconds =
                input_seconds.value().parse::<i32>().unwrap() + increase_seconds;

            input_minutes.set_value(&*format!("{}", new_input_minutes));

            if new_input_seconds < 0 {
                new_input_seconds = 0;
            }
            if new_input_seconds >= 60 {
                new_input_seconds = 59;
            }

            input_seconds.set_value(&*format!("{}", new_input_seconds));

            let countdown = (input_minutes.value().parse::<i32>().unwrap() * 60
                + input_seconds.value().parse::<i32>().unwrap()) as u32;

            tx.send(ChannelMessage::UpdateCountdown(countdown, false));
        };

        let mut change_countdown_minutes = change_countdown.clone();
        let mut change_minutes = move |minutes: i32| {
            change_countdown_minutes(minutes, 0);
        };

        let mut change_countdown_seconds = change_countdown.clone();
        let mut change_seconds = move |seconds: i32| {
            change_countdown_seconds(0, seconds);
        };

        const MIDDLE_OF_WINDOW: i32 = WINDOW_WIDTH / 2;
        const SCROLL_REST_TIME: u64 = 30;

        window
            .lock()
            .unwrap()
            .handle(move |local_window, ev| match ev {
                Event::MouseWheel => {
                    if start_button.label() != "Start" {
                        return false;
                    }

                    let mouse_pos_x = app::event_x();

                    match app::event_dy() {
                        MouseWheel::Up => {
                            if mouse_pos_x < MIDDLE_OF_WINDOW {
                                change_minutes(-1);
                            } else {
                                change_seconds(-1);
                            }
                            thread::sleep(Duration::from_millis(SCROLL_REST_TIME));
                        }
                        MouseWheel::Down => {
                            if mouse_pos_x < MIDDLE_OF_WINDOW {
                                change_minutes(1);
                            } else {
                                change_seconds(1);
                            }

                            thread::sleep(Duration::from_millis(SCROLL_REST_TIME));
                        }
                        _ => {}
                    }
                    true
                }

                Event::KeyUp => {
                    if app::event_key() == Key::Enter {
                        start_button.do_callback();
                    }

                    if start_button.label() != "Start" {
                        return false;
                    }

                    let is_ctrl = app::is_event_ctrl();
                    let is_shift = app::is_event_shift();
                    let is_ctrl_shift = is_ctrl && is_shift;
                    let event_key = app::event_key();

                    (0..=9)
                        .map(|digit| digit.to_string().parse::<char>().unwrap())
                        .for_each(|ch| {
                            if event_key == Key::from_char(ch) {
                                let mut countdown = ch.to_string();

                                if countdown == "0" {
                                    countdown = "10".to_string();
                                }

                                let expanded_window_height = WINDOW_HEIGHT + EXPANDED_WINDOW_HEIGHT;

                                if local_window.pixel_h() < expanded_window_height {
                                    input_minutes_clone.set_value(&*format!("{}", countdown));
                                    tx.send(ChannelMessage::UpdateCountdown(
                                        60 * countdown.parse::<u32>().unwrap(),
                                        false,
                                    ));
                                }
                            }
                        });

                    if is_ctrl_shift && event_key == Key::Up {
                        change_seconds(5);
                    } else if is_ctrl_shift && event_key == Key::Down {
                        change_seconds(-5);
                    } else if is_shift && event_key == Key::Up {
                        change_seconds(1);
                    } else if is_shift && event_key == Key::Down {
                        change_seconds(-1);
                    } else if is_ctrl && event_key == Key::Up {
                        change_minutes(5);
                    } else if is_ctrl && event_key == Key::Down {
                        change_minutes(-5);
                    } else if event_key == Key::Up {
                        change_minutes(1);
                    } else if event_key == Key::Down {
                        change_minutes(-1);
                    }
                    false
                }
                _ => false,
            });
    }
}
