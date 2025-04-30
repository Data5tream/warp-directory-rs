use std::path::{Path, PathBuf};

use crate::{
    storage::{WarpPoint, save_warp_point},
    util::get_current_directory,
};

/// Returns a vector of direct non-hidden subdirectories in the given path.
fn get_directories_in_path(path: &Path) -> Vec<PathBuf> {
    let mut directories = Vec::new();
    if let Ok(entries) = path.read_dir() {
        for entry in entries.flatten() {
            if entry.path().is_dir()
                && !entry
                    .file_name()
                    .to_str()
                    .is_none_or(|s| s.starts_with('.'))
            {
                directories.push(entry.path());
            }
        }
    }
    directories
}

/// Strips the given prefix from the path's file name if it exists.
///
/// # Returns
/// The file name without the prefix.
fn strip_prefix(path: &Path, prefix: &str) -> String {
    let mut file_name = path.file_name().unwrap().to_str().unwrap();

    if let Some(index) = file_name.find(prefix) {
        if index == 0 {
            file_name = &file_name[prefix.len()..];
        }
    }

    file_name.to_string()
}

/// Adds a warp point for each non-hidden subdirectory in the given path.
///
/// Overwrites existing warp points if the `--force` flag is used.
pub fn add_warp_directory(matches: &clap::ArgMatches) {
    let path = matches.get_one::<String>("path");
    let force = matches.get_flag("force");
    let description = matches.get_one::<String>("description");
    let prefix = matches.get_one::<String>("strip-prefix");

    let working_dir = get_current_directory();
    let path = match path {
        Some(path) => working_dir.join(path),
        None => working_dir.into(),
    };

    for dir in get_directories_in_path(&path) {
        let name = if let Some(prefix) = prefix {
            strip_prefix(&dir, prefix)
        } else {
            dir.file_name().unwrap().to_str().unwrap().to_string()
        };

        let warp_point = WarpPoint {
            name,
            path: dir.into_boxed_path(),
            description: description.cloned(),
        };

        save_warp_point(warp_point, force);
    }
}
