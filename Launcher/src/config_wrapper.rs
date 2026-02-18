use config::Config;
use log::{error, info, trace};
use opener::open;
use serde::Deserialize;
use std::env::current_exe;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

const LOG_FILENAME: &str = "launcher.log";
const CONFIG_FILENAME: &str = "launcher.toml";

#[derive(Deserialize)]
pub struct Settings {
    pub steam: bool,
    pub use_standalone_editor: bool,
}

pub fn get_log_path() -> PathBuf {
    get_parent_folder().join(LOG_FILENAME)
}

pub fn get_config_path() -> PathBuf {
    get_parent_folder().join(CONFIG_FILENAME)
}

pub fn get_parent_folder() -> PathBuf {
    let current_exe = current_exe().unwrap();

    #[cfg(not(target_os = "macos"))]
    return current_exe.parent().unwrap().to_path_buf();

    #[cfg(target_os = "macos")]
    {
        // MacOS has "application" *folders* (folders with the extension .app),
        // so we need to navigate up this.
        current_exe // rhythm_doctor_editor_launcher
            .parent() // MacOS
            .unwrap()
            .parent() // Contents
            .unwrap()
            .parent() // Rhythm Doctor Editor Launcher.app
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf()
    }
}

fn build_config() -> Config {
    trace!("Building config");
    match Config::builder()
        .add_source(config::File::with_name(get_config_path().to_str().unwrap()))
        .build()
    {
        Ok(config) => config,
        Err(error) => {
            error!("Failed to build config: {error}");
            write_config_file().expect("Failed to write config file");
            build_config()
        }
    }
}

/// Loads configuration from get_config_path()
/// Recreates the config file if it is damaged or missing.
pub fn load_config() -> Settings {
    trace!("Loading config");
    match build_config().try_deserialize::<Settings>() {
        Ok(config) => config,
        Err(error) => {
            error!("Failed to load settings: {error}");
            return write_config_file()
                .unwrap()
                .try_deserialize::<Settings>()
                .unwrap();
        }
    }
}

pub fn write_config_file() -> Result<Config, String> {
    info!("Creating new settings file");

    match File::create(get_config_path()) {
        Ok(mut file) => {
            let write = file.write_all(
                b"\
# Open Rhythm Doctor with Steam.
# You must have the RhythmDoctor.EditorLaunch plugin installed.
# Download it at https://github.com/nonperforming/RhythmDoctor.EditorLaunch/releases/latest
steam = true

# Use the standalone editor over the Steam game.
# You should really only need to use this option if you are on macOS and want to use the standalone editor,
# as the standalone editor already supports launching from rdlevel/rdzip files on Windows and Linux.
use_standalone_editor = false",
            );
            match write {
                Ok(()) => {
                    info!("Created new settings file");
                    return Ok(build_config());
                }
                Err(error) => Err(format!("Failed to write to new settings file: {error}")),
            }
        }
        Err(error) => Err(format!("Failed to create new settings file: {error}")),
    }
}

pub fn open_config_file() -> Result<(), String> {
    info!("Opening config file");
    if open(get_config_path()).is_ok() {
        return Ok(());
    }
    Err("Failed to open settings file".to_owned())
}
