#![windows_subsystem = "windows"]

mod built;
mod config_wrapper;
mod launcher;
mod rhythm_doctor;
mod rhythm_doctor_editor_standalone;

use built::{BUILT_TIME_UTC, GIT_VERSION, PKG_VERSION, PROFILE, RUSTC_VERSION, TARGET};
use config_wrapper::{get_config_path, get_log_path, load_config};
use rhythm_doctor::launch_rhythm_doctor;

use log::{debug, error, info};
use rhythm_doctor_editor_standalone::open_standalone_editor_with_level;
use std::env::{args, current_exe};
use std::process::ExitCode;

// MacOS
#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
use macos::event_loop;

// Not MacOS
#[cfg(not(target_os = "macos"))]
use config_wrapper::open_config_file;

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

    let settings = load_config();

    let args: Vec<String> = args().collect();

    let Some(level) = args.get(1) else {
        info!("Run with no arguments");

        #[cfg(target_os = "macos")]
        event_loop(settings);

        #[cfg(not(target_os = "macos"))]
        {
            let _ = open_config_file();
            return ExitCode::from(1);
        }
    };

    if settings.use_standalone_editor {
        info!("Opening standalone editor with level {level}");
        if let Err(error) = open_standalone_editor_with_level(level) {
            error!("Failed to launch standalone editor: {error}");
        }
    }
    info!("Opening Rhythm Doctor with level {level}");
    if let Err(error) = launch_rhythm_doctor(level, settings.steam) {
        error!("Failed to launch Rhythm Doctor: {error}");
        return ExitCode::from(2);
    }
    ExitCode::from(0)
}

/// Sets up logging
fn init_log() {
    #[cfg(debug_assertions)]
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

    #[cfg(not(debug_assertions))]
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
