use std::path::PathBuf;

pub struct Installation {
    pub path: PathBuf, // Base path.
    pub app_path: PathBuf, // Path to the app folder.
    pub branch: String, // "stable", "ptb", "canary", "development", ...
    pub is_patched: bool,
    pub is_sys_electron: bool,
    pub is_openasar: bool,
}
