use crate::config_wrapper::get_parent_folder;

use log::{error, info, trace};
use std::{path::PathBuf, process::Command};

pub fn find_standalone_editor() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        todo!("Windows standalone editor");
    }

    #[cfg(target_os = "linux")]
    {
        todo!("Linux standalone editor");
    }

    #[cfg(target_os = "macos")]
    {
        // Check if the editor is in the folder we're in
        trace!("Checking for standalone editor at parent folder");
        let path = get_parent_folder().join("Rhythm Doctor Editor.app/Contents/MacOS/Rhythm Doctor Editor");
        if path.exists() {
            info!("Found standalone editor at parent folder");
            return Some(path);
        }

        // Check if the editor is in ~/Applications
        trace!("Checking for standalone editor at ~/Applications/Rhythm Doctor Editor.app/Contents/MacOS/Rhythm Doctor Editor");
        let path = PathBuf::from("~/Applications/Rhythm Doctor Editor.app/Contents/MacOS/Rhythm Doctor Editor");
        if path.exists() {
            info!("Found standalone editor at ~/Applications/Rhythm Doctor Editor.app/Contents/MacOS/Rhythm Doctor Editor");
            return Some(path);
        }

        // Check if the editor is in /Applications
        trace!("Checking for standalone editor at /Applications/Rhythm Doctor Editor.app/Contents/MacOS/Rhythm Doctor Editor");
        let path = PathBuf::from("/Applications/Rhythm Doctor Editor.app/Contents/MacOS/Rhythm Doctor Editor");
        if path.exists() {
            info!("Found standalone editor at /Applications/Rhythm Doctor Editor.app/Contents/MacOS/Rhythm Doctor Editor");
            return Some(path);
        }
        None
    }
}

pub fn open_standalone_editor_with_level(level: &str) -> Result<(), String> {
    if let Some(path) = find_standalone_editor() {
        info!("Opening standalone editor with level {level}");
        if let Err(error) = Command::new(path).arg(level).status() {
            error!("Failed to open standalone editor - {error}");
            return Err(format!("Failed to open standalone editor - {error}").to_owned());
        }
        Ok(())
    } else {
        Err("Couldn't find standalone editor".to_owned())
    }
}
