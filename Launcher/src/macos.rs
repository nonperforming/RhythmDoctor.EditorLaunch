//! Handle .rdlevel/.rdzip/.zip files opened in macOS using an event loop

use log::{debug, info, trace, warn};
use tao::event::Event;
use tao::event_loop::ControlFlow;
use tao::event_loop::EventLoop;

use crate::config_wrapper::open_config_file;
use crate::open_rhythm_doctor_with_level;

/// Handle .rdlevel/.rdzip/.zip files opened in macOS
pub fn event_loop(with_steam: bool) -> ! {
    debug!("Creating event loop to listen for file");

    let mut exiting = false;
    EventLoop::new().run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if exiting {
            // Do not process events when we're exiting.
            // In theory we should be able to check `*control_flow == ControlFlow::Exit`,
            //  as it is "sticky" but control_flow always seems to be set to ControlFlow::Wait
            return;
        }

        trace!("Control flow: {:?}", control_flow);
        trace!("Event: {:?}", event);

        if let Event::Opened { urls } = event {
            info!("Got Opened event");
            if let Some(level) = urls.first() {
                open_rhythm_doctor_with_level(
                    with_steam,
                    &level.to_file_path().unwrap().display().to_string(),
                );
                *control_flow = ControlFlow::ExitWithCode(0);
                exiting = true;
            } else {
                warn!("No URL found in opened event");
                *control_flow = ControlFlow::ExitWithCode(2);
                exiting = true;
            }
        } else {
            warn!("No Opened event, opening configuration file");
            open_config_file().unwrap();
            *control_flow = ControlFlow::ExitWithCode(1);
            exiting = true;
        }
    });
}
