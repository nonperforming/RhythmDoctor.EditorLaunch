use config::{Config, ConfigError};
use log::info;
use opener::open;
use std::env::current_exe;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

const LOG_FILENAME: &str = "launcher.log";
const CONFIG_FILENAME: &str = "launcher.toml";

pub fn get_log_path() -> PathBuf {
    get_parent_folder().join(LOG_FILENAME)
}

pub fn get_config_path() -> PathBuf {
    get_parent_folder().join(CONFIG_FILENAME)
}

fn get_parent_folder() -> PathBuf {
    let current_exe = current_exe().unwrap();

    if cfg!(target_os = "windows") || cfg!(target_os = "linux") {
        return current_exe.parent().unwrap().to_path_buf();
    } else if cfg!(target_os = "macos") {
        // MacOS has "application" *folders* (folders with the extension .app),
        // so we need to navigate up this.
        return current_exe // rhythm_doctor_editor_launcher
            .parent() // MacOS
            .unwrap()
            .parent() // Contents
            .unwrap()
            .parent() // Rhythm Doctor Editor Launcher.app
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf();
    }
    panic!("Unsupported OS");
}

pub fn build_config() -> Result<Config, ConfigError> {
    Config::builder()
        .add_source(config::File::with_name(get_config_path().to_str().unwrap()))
        .build()
}

pub fn write_config_file() -> Result<Config, String> {
    info!("Creating new settings file");

    match File::create(get_config_path()) {
        Ok(mut file) => {
            let write = file.write_all(b"# Open Rhythm Doctor with Steam.\nsteam = true");
            match write {
                Ok(()) => {
                    info!("Created new settings file");
                    match build_config() {
                        Ok(config) => {
                            info!("Loaded new settings file");
                            Ok(config)
                        }
                        Err(error) => Err(format!("Failed to load new settings file: {error}")),
                    }
                }
                Err(error) => Err(format!("Failed to write to new settings file: {error}")),
            }
        }
        Err(error) => Err(format!("Failed to create new settings file: {error}")),
    }
}

pub fn open_config_file() -> Result<(), String> {
    if open(get_config_path()).is_ok() {
        return Ok(());
    }
    Err("Failed to open settings file".to_owned())
}
