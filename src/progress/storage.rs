use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

use super::Progress;

use directories::ProjectDirs;

const FILE: &str = "progress.json";
const BACKUP: &str = "progress.json.bak";

// The folder can be overridden to keep real progress safe while developing
pub fn data_dir() -> Option<PathBuf> {
    if let Ok(dir) = env::var("INGRAIN_DATA_DIR") {
        return Some(PathBuf::from(dir));
    }

    Some(
        ProjectDirs::from("", "", "ingrain")?
            .data_dir()
            .to_path_buf(),
    )
}

// The saved progress of a folder, or an empty one if there is none
pub fn read(dir: &Path) -> Progress {
    let path = dir.join(FILE);
    let Ok(json) = fs::read_to_string(&path) else {
        return Progress::default();
    };

    // A broken file is kept as a .bak copy and we start empty
    serde_json::from_str(&json).unwrap_or_else(|error| {
        eprintln!("{} is corrupt: {error}", path.display());
        let _ = fs::rename(&path, dir.join(BACKUP));
        Progress::default()
    })
}

pub fn write(progress: &Progress, dir: &Path) -> io::Result<()> {
    fs::create_dir_all(dir)?;

    // Write a temporary file first, then rename it, so a crash never leaves half a file
    let temporary = dir.join("progress.json.tmp");
    fs::write(&temporary, serde_json::to_string_pretty(progress)?)?;
    fs::rename(temporary, dir.join(FILE))
}
