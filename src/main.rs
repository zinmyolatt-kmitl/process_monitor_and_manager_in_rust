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
    #[cfg(target_family = "unix")]
    std::env::set_var("WINIT_X11_SCALE_FACTOR", "1.27");

    ProcMonApp::run(iced::Settings::default())
}
