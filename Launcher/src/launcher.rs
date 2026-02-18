use log::{error, info, warn};

use crate::config_wrapper::Settings;
use crate::rhythm_doctor::launch_rhythm_doctor;
use crate::rhythm_doctor_editor_standalone::open_standalone_editor_with_level;

pub fn launch_with_level(settings: &Settings, level: &str) -> Result<(), String> {
    info!("Launching Rhythm Doctor with level {level}");

    if settings.use_standalone_editor {
        info!("Opening standalone editor with level {level}");
        if let Ok(()) = open_standalone_editor_with_level(level) {
            return Ok(());
        }
        warn!("Failed to launch standalone editor");
    }

    info!(
        "Opening Rhythm Doctor with level {level} and Steam: {})",
        settings.steam
    );
    if let Ok(()) = launch_rhythm_doctor(level, settings.steam) {
        return Ok(());
    }
    warn!("Failed to launch Rhythm Doctor");

    if !settings.use_standalone_editor {
        info!("Opening standalone editor with level {level}");
        if let Ok(()) = open_standalone_editor_with_level(level) {
            return Ok(());
        }
        warn!("Failed to launch standalone editor");
    }

    error!("Failed to launch with level");
    Err("Failed to launch with level".to_owned())
}
