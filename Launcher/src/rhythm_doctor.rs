use log::{error, info, trace, warn};
use opener::open;
use std::path::{Path, PathBuf};
use std::process::Command;
use steamlocate::SteamDir;
use which::which;

use crate::config_wrapper::write_path_file;

/// Rhythm Doctor's App ID.
const APP_ID: u32 = 774181;

/// Launch Rhythm Doctor with BepInEx.
/// Note that launching BepInEx is different on Windows, Linux, and macOS.
pub fn launch_rhythm_doctor(path: String, with_steam: bool) -> Result<(), String> {
    trace!("Launching Rhythm Doctor");

    info!("Launching Rhythm Doctor with BepInEx and path argument");

    if with_steam {
        // Launch by passing parameter directly to Steam
        if let Some(steam_path) = find_steam_executable() {
            if cfg!(target_os = "windows") {
                // No additional action required
                Command::new(steam_path)
                    .args(["-applaunch", APP_ID.to_string().as_str(), &path])
                    .status()
                    .expect("Failed to launch Rhythm Doctor");
                return Ok(());
            } else if cfg!(target_os = "macos") {
                Command::new(steam_path)
                    .args(["-applaunch", APP_ID.to_string().as_str(), &path])
                    .status()
                    .expect("Failed to launch Rhythm Doctor");
                return Ok(());
            } else if cfg!(target_os = "linux") {
                // Requires run_bepinex.sh script
                Command::new(steam_path)
                    .args([
                        "-applaunch",
                        APP_ID.to_string().as_str(),
                        "./run_bepinex.sh %command%",
                        &path,
                    ])
                    .status()
                    .expect("Failed to launch Rhythm Doctor");
                return Ok(());
            }
        }

        // Could't find Steam, fallback to using file + steam://
        warn!("Using file fallback method");
        match write_path_file() {
            Some(error) => {
                error!("Failed to write path file - {error}");
            }
            None => match open("steam://run/".to_owned() + &APP_ID.to_string()) {
                Ok(()) => {
                    return Ok(());
                }
                Err(error) => {
                    error!("Failed to open steam:// - {error}");
                }
            },
        }
    }

    // Couldn't open Steam, fallback to using Rhythm Doctor (slow!)
    warn!("Opening Rhythm Doctor directly");
    if let Some(mut path) = find_rhythm_doctor() {
        if cfg!(target_os = "windows") {
            path = path.join("Rhythm Doctor.exe");
        } else if cfg!(target_os = "macos") {
            path = path.join("Rhythm Doctor.app");
        } else if cfg!(target_os = "linux") {
            path = path.join("Rhythm Doctor");
        } else {
            panic!("Unsupported OS");
        }

        open(path).expect("Failed to open Rhythm Doctor");
        return Ok(());
    }

    return Err("Couldn't open Rhythm Doctor".to_string());
}

/// Attempt to find the Steam executable.
fn find_steam_executable() -> Option<PathBuf> {
    trace!("Trying to find Steam on path");
    // TODO: Check if steam-native binary on some systems exists?
    match which("steam") {
        Ok(path) => return Some(path),
        Err(error) => {
            warn!("Couldn't find Steam on path - {error}");
        }
    };

    // Try to find Steam based on default install locations
    trace!("Trying to find Steam based on default install locations");
    if cfg!(target_os = "windows") {
        trace!(r"Checking for Steam executable at C:\Program Files (x86)\Steam\steam.exe");
        if Path::new(r"C:\Program Files (x86)\Steam\steam.exe").exists() {
            info!(r"Found Steam executable at C:\Program Files (x86)\Steam\steam.exe");
            return Some(PathBuf::from(r"C:\Program Files (x86)\Steam\steam.exe"));
        }
    } else if cfg!(target_os = "macos") {
        trace!("Checking for Steam executable at /Applications/Steam.app/Contents/MacOS/steam_osx");
        if Path::new("/Applications/Steam.app/Contents/MacOS/steam_osx").exists() {
            info!("Found Steam executable at /Applications/Steam.app/Contents/MacOS/steam_osx");
            return Some(PathBuf::from(
                "/Applications/Steam.app/Contents/MacOS/steam_osx",
            ));
        }
    } else if cfg!(target_os = "linux") {
        // TODO: Add flatpak, native, etc
        //       We should have better luck searching on PATH anyways.
        // Official .deb package from https://cdn.cloudflare.steamstatic.com/client/installer/steam.deb
        trace!("Checking for Steam executable at /usr/games/steam (Official)");
        if Path::new("/usr/games/steam").exists() {
            info!("Found Steam executable at /usr/games/steam (Official)");
            return Some(PathBuf::from("/usr/games/steam"));
        }
    }

    // Couldn't find Steam
    error!("Couldn't find Steam executable");
    None
}

/// Get the path to Rhythm Doctor's install location
fn find_rhythm_doctor() -> Option<PathBuf> {
    trace!("Trying to find Rhythm Doctor install path");
    if let Ok(steam) = SteamDir::locate() {
        if let Ok(rhythm_doctor) = steam.find_app(APP_ID) {
            match rhythm_doctor {
                Some((app, library)) => {
                    let path = library.resolve_app_dir(&app);
                    info!("Found Rhythm Doctor at {}", path.display());
                    return Some(path);
                }
                None => {
                    error!("Couldn't find Rhythm Doctor (case 2)");
                    return None;
                }
            }
        } else {
            error!("Couldn't find Rhythm Doctor (case 1)");
            return None;
        }
    } else {
        error!("Couldn't find Steam");
        return None;
    }
}
