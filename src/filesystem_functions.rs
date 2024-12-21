use std::error::Error;
use std::fs;
use std::fs::{remove_dir_all, remove_file};
use std::path::Path;

pub fn remove_resource<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    //1. Get metadata from element
    let metadata = fs::metadata(&path).map_err(|e| Box::new(e) as Box<dyn Error>)?;

    // Eliminar archivo o directorio
    if metadata.is_file() || metadata.is_symlink() {
        remove_file(&path).map_err(|e| Box::new(e) as Box<dyn Error>)?;
    } else if metadata.is_dir() {
        remove_dir_all(&path).map_err(|e| Box::new(e) as Box<dyn Error>)?;
    }

    Ok(())
}
