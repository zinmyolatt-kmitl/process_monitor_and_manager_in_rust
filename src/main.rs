mod app;
mod platform;
mod util;

use app::ProcMonApp;
use iced::Application;

fn main() -> iced::Result {
    ProcMonApp::run(iced::Settings::default())
}
