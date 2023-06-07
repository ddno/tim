use std::fs::File;
use std::io::BufReader;
use std::{env};

use rodio::{source::Source, Decoder, OutputStream};

pub struct Audio;

impl Audio {
    pub fn play_sound() {
        std::thread::spawn(|| {
            let alarm_file_path = Self::get_file_path();

            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let file = BufReader::new(File::open(alarm_file_path).unwrap());
            let source = Decoder::new(file).unwrap();
            stream_handle
                .play_raw(source.convert_samples())
                .expect("Failed to play audio");

            std::thread::sleep(std::time::Duration::from_secs(60));
        });
    }

    fn get_file_path() -> String {
        let current_dir = env::current_dir().unwrap();
        let mut current_dir = current_dir.to_string_lossy().to_string();

        if !current_dir.contains("Tim.app") && current_dir != "/" {
            current_dir = current_dir + "/assets/alarm.mp3";

            return current_dir;
        }

        if let Ok(exe_path) = env::current_exe() {
            return exe_path
                .to_string_lossy()
                .to_string()
                .replace("/MacOS/tim", "/Resources/assets/alarm.mp3");
        }

        return "".to_string();
    }
}
