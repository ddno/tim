use cocoa::appkit::NSStatusBar;
use cocoa::appkit::NSWindow;
use cocoa::base::nil;
use cocoa::foundation::NSString;
use objc::runtime::Object;

use crate::traits::convert_countdown::ConvertCountdown;

#[cfg(target_os = "macos")]
pub struct StatusBar {
    pub system_status_bar: *mut Object,
}

#[cfg(target_os = "macos")]
impl StatusBar {
    pub fn update_countdown(&mut self, countdown: u32) {
        let (minutes, seconds) = self.to_minutes_seconds(countdown);

        let mut label = format!("{}:{}", minutes, seconds);

        if countdown == 0 {
            label = "".to_string();
        }

        unsafe {
            self.system_status_bar
                .setTitle_(NSString::alloc(nil).init_str(&label));
        }
    }

    pub fn new() -> Self {
        unsafe {
            StatusBar {
                system_status_bar: NSStatusBar::systemStatusBar(nil).statusItemWithLength_(-1.0),
            }
        }
    }
}

impl ConvertCountdown for StatusBar {}
