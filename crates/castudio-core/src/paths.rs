use directories::ProjectDirs;
use std::path::PathBuf;

pub fn data_dir() -> PathBuf {
    if let Some(dirs) = ProjectDirs::from("com", "fathforce", "castudio") {
        dirs.data_dir().to_path_buf()
    } else {
        PathBuf::from(".castudio_data")
    }
}

pub fn db_path() -> PathBuf {
    let dir = data_dir();
    std::fs::create_dir_all(&dir).ok();
    dir.join("castudio.db")
}

pub fn export_dir() -> PathBuf {
    let dir = data_dir().join("exports");
    std::fs::create_dir_all(&dir).ok();
    dir
}
