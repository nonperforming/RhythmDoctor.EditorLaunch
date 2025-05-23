#![windows_subsystem = "windows"]

use built::{BUILT_TIME_UTC, GIT_VERSION, PKG_VERSION, PROFILE, RUSTC_VERSION, TARGET};
use config_wrapper::{
    build_config, get_config_path, get_log_path, open_config_file, write_config_file,
};
#[cfg(target_os = "macos")]
use macos::event_loop;
use rhythm_doctor::launch_rhythm_doctor;

use log::{debug, error, info, warn};
use std::collections::HashMap;
use std::env::{args, current_exe};
use std::process::ExitCode;

mod built;
mod config_wrapper;
mod macos;
mod rhythm_doctor;

/// Forward the given launch option to Rhythm Doctor.
fn main() -> ExitCode {
    init_log();

    info!(
        "Started - {} v{} ({}) compiled with {} on {} for {}",
        PROFILE,
        PKG_VERSION,
        GIT_VERSION.unwrap_or("Unknown"),
        RUSTC_VERSION,
        BUILT_TIME_UTC,
        TARGET
    );
    debug!("Running under {}", current_exe().unwrap().display());
    debug!("Config path is {}", get_config_path().display());

    let config_built = build_config();

    let settings_config = match config_built {
        Ok(settings) => settings,
        Err(error) => {
            warn!("Failed to load settings: {error}");
            match write_config_file() {
                Ok(config) => config,
                Err(error) => {
                    error!("Failed to create new settings file: {error}");
                    return ExitCode::from(2);
                }
            }
        }
    };

    let settings = settings_config
        .try_deserialize::<HashMap<String, String>>()
        .unwrap();

    let args: Vec<String> = args().collect();
    let with_steam = settings.get("steam").is_some_and(|steam| steam == "true");

    let Some(level) = args.get(1) else {
        info!("Run with no arguments");
        if cfg!(target_os = "macos") {
            event_loop(with_steam);
        } else {
            info!("Opening configuration file");
            let _ = open_config_file();
            return ExitCode::from(1);
        }
    };

    open_rhythm_doctor_with_level(with_steam, level);
    ExitCode::from(0)
}

/// Sets up logging
fn init_log() {
    if cfg!(debug_assertions) {
        let _ = simplelog::CombinedLogger::init(vec![
            simplelog::TermLogger::new(
                simplelog::LevelFilter::Trace,
                simplelog::Config::default(),
                simplelog::TerminalMode::Stdout,
                simplelog::ColorChoice::Auto,
            ),
            simplelog::WriteLogger::new(
                simplelog::LevelFilter::Trace,
                simplelog::Config::default(),
                std::fs::File::create(get_log_path()).unwrap(),
            ),
        ]);
    } else {
        let _ = simplelog::CombinedLogger::init(vec![
            simplelog::TermLogger::new(
                simplelog::LevelFilter::Info,
                simplelog::Config::default(),
                simplelog::TerminalMode::Stdout,
                simplelog::ColorChoice::Auto,
            ),
            simplelog::WriteLogger::new(
                simplelog::LevelFilter::Info,
                simplelog::Config::default(),
                std::fs::File::create(get_log_path()).unwrap(),
            ),
        ]);
    }
}

fn open_rhythm_doctor_with_level(with_steam: bool, level: &str) -> ExitCode {
    info!("Opening Rhythm Doctor with level {level}");
    if let Err(error) = launch_rhythm_doctor(level, with_steam) {
        error!("Failed to launch Rhythm Doctor: {error}");
        return ExitCode::from(2);
    }

    ExitCode::from(0)
}