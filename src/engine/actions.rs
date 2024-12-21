use std::error::Error;
use std::{env, fs};
use std::fs::{symlink_metadata, create_dir_all};
use std::io::ErrorKind;
use std::os::unix::fs::symlink;
use std::path::PathBuf;

use fs_extra::{dir, move_items};

use crate::engine::models::{Dotfiles, Target};
use crate::filesystem_functions::remove_resource;

pub fn capture_iter(dotfiles: &Dotfiles) -> Result<(), Box<dyn Error>> {
    let current_dir = env::current_dir()?;
    log::info!("Current directory: {}", current_dir.display());
    
    
    for target in dotfiles.targets.iter() {
        capture(target, &dotfiles.base_path)?;
    }

    Ok(())
}

pub fn capture(target: &Target, base_path: &String) -> Result<(), Box<dyn Error>> {
    log::info!("Init for {}", target.application);
    // 1.   Move it -- I'm going to implement with strings, instead of "PathBuf" for simplicity
    let destination_dir = format!("{}/{}", base_path, target.application);
    fs::create_dir(&destination_dir).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::AlreadyExists {
            log::info!("Ready directory: {}", &destination_dir)
        }
    });

    let options = dir::CopyOptions::new();
    let mut elements_to_move = Vec::new();
    for element in target.elements.iter() {
        // 1. Check symlink
        let metadata = match symlink_metadata(PathBuf::from(&element.path)) {
            Ok(metadata) => metadata,
            Err(e) => match e.kind() {
                ErrorKind::NotFound => {
                    log::warn!("File not found: {}", &element.path);
                    continue;
                }
                other_error => panic!("Problem with filesystem element: {:?}", other_error),
            },
        };

        // TODO
        //  Debería agregar la lógica de seguir los symlinks y capturar los originales
        if metadata.is_symlink() {
            log::warn!("Symlink detected, skip: {}", &element.path);

            // 2. Add to move list
            elements_to_move.push(&element.path);
        }

        match move_items(&elements_to_move, &destination_dir, &options) {
            Ok(..) => log::info!("Ending {}", target.application),
            Err(e) => {
                log::error!("Move items error: {}", e);
                return Err(e.into())
            },
        }
    }

    Ok(())
}

pub fn deploy_iter(dotfiles: &Dotfiles) -> Result<(), Box<dyn Error>> {
    for target in dotfiles.targets.iter() {
        deploy(target, dotfiles.remove_on_conflict, &dotfiles.base_path)?;
    }

    Ok(())
}

pub fn deploy(
    target: &Target,
    remove_on_conflict: bool,
    base_path: &String,
) -> Result<(), Box<dyn Error>> {
    for element in target.elements.iter() {
        // TODO: Replace unwrap for unwrap_or_else
        // 1. Get element to binding
        let end_node = element.path.rsplit_once('/').unwrap().1;
        let origin_dir = format!("{}/{}", base_path, target.application);
        
        // 2. Test and create folders
        create_dir_all(&element.path).unwrap_or_else(|error| {
            log::warn!("Could not create directory. {} : {}", &origin_dir, error);
        });

        // 2. Do symlinks
        let symlink_start = format!("{}/{}", origin_dir, end_node);
        let symlink_end = &element.path;

        symlink(&symlink_start, symlink_end).unwrap_or_else(|error| {
            log::warn!("Fail to create symlink. {}: {}", error, symlink_end);

            if !remove_on_conflict {
                log::info!("Deletion disabled. Skipping: {}", symlink_end);
                return;
            }

            log::debug!("Delete element on conflict: {}", symlink_end);
            remove_resource(symlink_end).unwrap_or_else(|error| log::warn!("{}", error));
            
            symlink(symlink_start, symlink_end).unwrap_or_else(|error| log::warn!("{}", error));
        });
    }

    Ok(())
}
