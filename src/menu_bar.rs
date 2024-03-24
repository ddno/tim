use fltk::menu::SysMenuBar;
use fltk::prelude::*;

#[cfg(target_os = "macos")]
pub struct MenuBar;

#[cfg(target_os = "macos")]
impl MenuBar {
    pub fn new() {
        let choice_open_in_browser = "Open Manual in Browser";

        let mut menu_bar = SysMenuBar::default();
        menu_bar.add_choice(&format!("&Help/{}", choice_open_in_browser));

        menu_bar.set_callback(move |menu| match menu.choice() {
            Some(value) if value == choice_open_in_browser => {
                let _ = webbrowser::open("https://github.com/ddno/tim/blob/main/README.md");
            }
            _ => (),
        });
    }
}
