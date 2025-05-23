//! Handle .rdlevel/.rdzip/.zip files opened in macOS using an event loop

#![cfg(target_os = "macos")]

use log::{debug, info, warn};
use tao::event::Event;
use tao::event_loop::ControlFlow;
use tao::event_loop::EventLoop;

use crate::config_wrapper::open_config_file;
use crate::open_rhythm_doctor_with_level;

/// Handle .rdlevel/.rdzip/.zip files opened in macOS
pub fn event_loop(with_steam: bool) -> ! {
    debug!("Creating event loop to listen for file");

    let event_loop = EventLoop::new();
    let mut empty_polls: u8 = 0;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::Opened { urls } = event {
            info!("Opened event");
            if let Some(level) = urls.first() {
                open_rhythm_doctor_with_level(
                    with_steam,
                    &level.to_file_path().unwrap().display().to_string(),
                );
                *control_flow = ControlFlow::ExitWithCode(0);
            } else {
                warn!("No URL found in opened event");
            }
        } else {
            // #[allow(clippy::arithmetic_side_effects)] // blocked by https://github.com/rust-lang/rust/issues/15701
            empty_polls += 1;
            // == 50 as we do not want to open the config file multiple times
            if empty_polls == 50 {
                warn!("No events for 50 polls, opening configuration file and closing");
                open_config_file().unwrap();
                *control_flow = ControlFlow::ExitWithCode(1);
            }
        }
    });
}
