#![windows_subsystem = "windows"]

mod built;
mod config_wrapper;
mod rhythm_doctor;

use built::{BUILT_TIME_UTC, GIT_VERSION, PKG_VERSION, PROFILE, RUSTC_VERSION, TARGET};
use config_wrapper::*;
use rhythm_doctor::*;

use log::{debug, error, info, warn};
use std::collections::HashMap;
use std::env::{args, current_exe};
use std::process::ExitCode;

/// Forward the given launch option to Rhythm Doctor.
fn main() -> ExitCode {
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

    let level = match args.get(1) {
        Some(level) => level,
        None => {
            info!("Run with no arguments");
            if cfg!(target_os = "macos") {
                // Handle .rdlevel/.rdzip/.zip files opened in macOS
                debug!("Creating event loop to listen for file");

                use tao::event::Event;
                use tao::event_loop::ControlFlow;
                use tao::event_loop::EventLoop;

                let event_loop = EventLoop::new();
                let mut empty_polls = 0;

                event_loop.run(move |event, _, control_flow| {
                    *control_flow = ControlFlow::Wait;

                    match event {
                        Event::Opened { urls } => {
                            info!("Opened event");
                            if let Some(level) = urls.get(0) {
                                open_rhythm_doctor_with_level(
                                    with_steam,
                                    level.to_file_path().unwrap().display().to_string(),
                                );
                                *control_flow = ControlFlow::ExitWithCode(0);
                            } else {
                                warn!("No URL found in opened event")
                            }
                        }
                        _ => {
                            empty_polls += 1;
                            // == 50 as we do not want to open the config file multiple times
                            if empty_polls == 50 {
                                warn!(
                                    "No events for 50 polls, opening configuration file and closing"
                                );
                                let _ = open_config_file().unwrap();
                                *control_flow = ControlFlow::ExitWithCode(1);
                            }
                        }
                    }
                });
            } else {
                info!("Opening configuration file");
                let _ = open_config_file();
                return ExitCode::from(1);
            }
        }
    };

    open_rhythm_doctor_with_level(with_steam, level.to_string());
    ExitCode::from(0)
}

fn open_rhythm_doctor_with_level(with_steam: bool, level: String) -> ExitCode {
    info!("Opening Rhythm Doctor with level {}", level);
    if let Err(error) = launch_rhythm_doctor(level, with_steam) {
        error!("Failed to launch Rhythm Doctor: {error}");
        return ExitCode::from(2);
    }

    return ExitCode::from(0);
}
