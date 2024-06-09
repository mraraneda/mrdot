use std::path::PathBuf;

/// Abstrae un dotfile target
pub struct Target {
    pub app_name: String,
    pub path: String,
}

/// Underlying type of target resource
pub enum FsKind {
    Dir,
    File,
    Symlink,
}