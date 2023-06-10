use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use rodio::{source::Source, Decoder, OutputStream};

pub struct Audio;

const DEFAULT_ALARM_FILE: &str = "default_alarm.wav";
const CUSTOM_ALARM_FILE: &str = "alarm.mp3";

impl Audio {
    pub fn play_sound() {
        std::thread::spawn(|| {
            let alarm_file_path = Self::get_file_path();

            let (_stream, stream_handle) = OutputStream::try_default().unwrap();

            if alarm_file_path.contains(DEFAULT_ALARM_FILE) {
                for _ in 1..=10 {
                    let file = BufReader::new(File::open(alarm_file_path.clone()).unwrap());
                    let source = Decoder::new(file).unwrap();
                    stream_handle
                        .play_raw(source.convert_samples())
                        .expect("Failed to play audio");

                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
            } else {
                let file = BufReader::new(File::open(alarm_file_path).unwrap());
                let source = Decoder::new(file).unwrap();
                stream_handle
                    .play_raw(source.convert_samples())
                    .expect("Failed to play audio");

                std::thread::sleep(std::time::Duration::from_secs(60));
            }
        });
    }

    fn get_file_path() -> String {
        let assets_path = Self::get_assets_path();

        if assets_path == "" {
            return "".to_string();
        }

        let custom_alarm_path = assets_path.clone() + CUSTOM_ALARM_FILE;

        if Path::new(&custom_alarm_path).exists() {
            return custom_alarm_path;
        }

        return assets_path.clone() + DEFAULT_ALARM_FILE;
    }

    fn get_assets_path() -> String {
        let current_dir = env::current_dir().unwrap();
        let current_dir = current_dir.to_string_lossy().to_string();

        if !current_dir.contains("Tim.app") && current_dir != "/" {
            return current_dir + "/assets/";
        } else {
            if let Ok(exe_path) = env::current_exe() {
                return exe_path
                    .to_string_lossy()
                    .to_string()
                    .replace("/MacOS/tim", "/Resources/assets/");
            }
        }

        return "".to_string();
    }
}
