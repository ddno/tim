pub trait ConvertCountdown {
    fn to_minutes_seconds(&self, countdown: u32) -> (String, String) {
        let mut seconds = countdown;
        let mut minutes = 0;

        if seconds >= 60 {
            minutes = seconds / 60;
        }

        seconds = seconds - (minutes * 60);

        let mut minutes_string = minutes.to_string();
        if minutes < 10 {
            minutes_string = "0".to_string() + &minutes.to_string();
        }

        let mut seconds_string = seconds.to_string();
        if seconds < 10 {
            seconds_string = "0".to_string() + &seconds.to_string();
        }

        (minutes_string, seconds_string)
    }
}
