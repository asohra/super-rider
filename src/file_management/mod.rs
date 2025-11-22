use std::{env::current_exe, path::PathBuf};

#[derive(Debug)]
pub enum FindAssetsDirError {
    NoExePath,       // Could not find executable path.
    NoExeDir,        // Executable had not parent directory.
    DirNotFound,     // Dir not found in loop limit.
    DirLimitReached, // Ran out of parent directories while looping.
}

pub fn get_assets_dir() -> Result<PathBuf, FindAssetsDirError> {
    let Ok(exe_path) = current_exe() else {
        return Err(FindAssetsDirError::NoExePath);
    };
    let Some(exe_dir) = exe_path.parent() else {
        return Err(FindAssetsDirError::NoExeDir);
    };

    let mut loop_dir = exe_dir;
    for _ in 0..3 {
        if loop_dir.join("assets").is_dir() {
            return Ok(loop_dir.join("assets"));
        }
        let Some(parent_dir) = loop_dir.parent() else {
            return Err(FindAssetsDirError::DirLimitReached);
        };
        loop_dir = parent_dir;
    }
    Err(FindAssetsDirError::DirNotFound)
}
