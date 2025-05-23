//! Find and launch Rhythm Doctor with arguments and Steam.

use log::{error, info, trace, warn};
use opener::open;
use std::path::{Path, PathBuf};
use std::process::Command;
use steamlocate::SteamDir;
use which::which;

/// Rhythm Doctor's App ID.
#[allow(clippy::unreadable_literal)]
const APP_ID: u32 = 774181;

const APP_ID_STR: &str = "774181";

#[allow(clippy::doc_markdown)]
/// Launch Rhythm Doctor with [BepInEx](https://docs.bepinex.dev/index.html).
/// Note that launching BepInEx is different on Windows, Linux, and macOS.
pub fn launch_rhythm_doctor(path: &str, with_steam: bool) -> Result<(), String> {
    trace!("Launching Rhythm Doctor");

    info!("Launching Rhythm Doctor with BepInEx and path argument");

    if with_steam {
        // Launch by passing parameter directly to Steam
        if let Some(steam_path) = find_steam_executable() {
            Command::new(steam_path)
                .args(["-applaunch", APP_ID_STR, path])
                .status()
                .expect("Failed to launch Rhythm Doctor");
            return Ok(());
        }

        // Could't find Steam, fallback to using file + steam://
        warn!("Using file fallback method");
        match write_path_file() {
            Some(error) => {
                error!("Failed to write path file - {error}");
            }
            None => match open("steam://run/".to_owned() + APP_ID_STR) {
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
    if let Some(mut game_path) = find_rhythm_doctor() {
        if cfg!(target_os = "windows") {
            game_path = game_path.join("Rhythm Doctor.exe");
        } else if cfg!(target_os = "macos") {
            game_path = game_path.join("Rhythm Doctor.app");
        } else if cfg!(target_os = "linux") {
            game_path = game_path.join("Rhythm Doctor");
        } else {
            return Err("Unsupported OS".to_owned());
        }

        Command::new(game_path)
            .arg(path)
            .status()
            .expect("Failed to launch Rhythm Doctor");
        return Ok(());
    }

    Err("Couldn't open Rhythm Doctor".to_owned())
}

/// Write to the file the RhythmDoctor.EditorLaunch plugin will look for to load rdlevel/rdzip.
pub fn write_path_file() -> Option<String> {
    todo!(
        "Implement path file to write to when finding Steam fails (use steam://run/APPID instead)"
    );
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
    }

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
            if let Some((app, library)) = rhythm_doctor {
                let path = library.resolve_app_dir(&app);
                info!("Found Rhythm Doctor at {}", path.display());
                Some(path)
            } else {
                error!("Couldn't find Rhythm Doctor (case 2)");
                None
            }
        } else {
            error!("Couldn't find Rhythm Doctor (case 1)");
            None
        }
    } else {
        error!("Couldn't find Steam");
        None
    }
}
