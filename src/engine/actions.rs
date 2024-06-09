use std::fs::symlink_metadata;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::process::Command;

use fs_extra::{dir, error, move_items_with_progress, TransitProcess};
use walkdir::{DirEntry, WalkDir};

use crate::configurations::Configs;
use crate::engine::models::Target;

pub fn capture(target: &Target, config: &Configs) -> Result<(), error::Error> {
    // 1.   Discriminar por target type
    let metadata = symlink_metadata(PathBuf::from(&target.path))?;
    if metadata.is_symlink() {
        log::warn!("\"{}\" is symlink. Isn't captured", &target.path);
        return Ok(());
    }

    // 2.   Move it -- I'm going to implement with strings, instead of "PathBuf" for simplicity
    let destination = format!("{}/{}", config.base_path, target.app_name);
    let options = dir::CopyOptions::new();
    let handle = |process_info: TransitProcess| {
        log::info!("{}", process_info.total_bytes);
        dir::TransitProcessResult::ContinueOrAbort
    };
    let from = vec![&target.path];
    move_items_with_progress(&from, destination, &options, handle)?;

    Ok(())
}

pub fn deploy(target: &Target, config: &Configs) -> Result<(), error::Error> {
    // 1.   get elements to binding
    let end_node = &target.path.rsplit_once('/').unwrap().1;
    let destination = format!("{}/{}", config.base_path, target.app_name);

    // 2. Do symlinks
    let symlink_start = format!("{}/{}", destination, end_node);
    let symlink_end = &target.path;
    symlink(symlink_start, symlink_end)?;

    Ok(())
}

// Función para verificar si el directorio comienza con "." (dotfile)
fn is_dotfile(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}
