pub mod models;
pub mod styles;
pub mod graphs;
pub mod suggestions;
pub mod system_monitor;
pub mod view;
pub mod app;
mod platform;
mod util;

use app::ProcMonApp;
use iced::Application;

fn main() -> iced::Result {
    ProcMonApp::run(iced::Settings::default())
}
