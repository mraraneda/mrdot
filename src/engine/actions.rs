use std::error::Error;
use std::fs;
use std::fs::symlink_metadata;
use std::io::ErrorKind;
//use std::os::unix::fs::symlink;
use std::path::PathBuf;

use fs_extra::{dir, move_items};

use crate::engine::models::{Dotfiles, Target};

pub fn capture_iter(dotfiles: &Dotfiles) -> Result<(), Box<dyn Error>> {
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
                    log::error!("File not found: {}", &element.path);
                    continue;
                }
                other_error => panic!("Problem with filesystem element: {:?}", other_error),
            },
        };

        if metadata.is_symlink() {
            log::warn!("Symlink detected, skip: {}", &element.path);
            continue;
        }

        // 2. Add to move list
        elements_to_move.push(&element.path);
    }

    match move_items(&elements_to_move, destination_dir, &options) {
        Ok(..) => log::info!("Ending {}", target.application),
        Err(e) => log::error!("Move items error: {}", e),
    };

    Ok(())
}

// pub fn deploy(target: &Target, remove_on_conflict: bool, base_path: &String) -> Result<(), Box<dyn Error>> {
//     // 1.   get elements to binding
//     let end_node = &target.path.rsplit_once('/').unwrap().1;
//     let destination = format!("{}/{}", config.algo, target.app_name);
//
//     // 2. Do symlinks
//     let symlink_start = format!("{}/{}", destination, end_node);
//     let symlink_end = &target.path;
//     match symlink(&symlink_start, symlink_end) {
//         Ok(..) => (),
//         Err(e) => {
//             log::warn!(
//                 "Stopper at trying to create the symlink over \"{}\". {}",
//                 symlink_end,
//                 e
//             );
//             // Aplicando política ante colisiones, obtenida desde la configuración
//             // Apply politic
//
//             // Apply politic - guard clause
//             if !config.remove_on_conflict {
//                 log::info!("Skipping creation symlink for \"{}\"", symlink_end);
//                 return Err(e.into());
//             }
//
//             let element = vec![symlink_end];
//             match remove_items(&element) {
//                 Ok(..) => (),
//                 Err(e) => return Err(e.into()),
//             };
//
//             match symlink(symlink_start, symlink_end) {
//                 Ok(..) => (),
//                 Err(e) => return Err(e.into()),
//             }
//         }
//     }
//
//     Ok(())
// }
