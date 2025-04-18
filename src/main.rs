mod emulator;
mod ui;

use iced::{Application, Settings};
use ui::app::RGenApp;

fn main() -> iced::Result {
    RGenApp::run(Settings {
        antialiasing: true,
        default_font: None,
        window: iced::window::Settings {
            size: (800, 600),
            resizable: true,
            ..Default::default()
        },
        ..Default::default()
    })
}
