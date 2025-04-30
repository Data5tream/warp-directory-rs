use std::path::Path;

/// Returns the current working directory as a boxed path.
pub fn get_current_directory() -> Box<Path> {
    std::env::current_dir()
        .expect("Unable to get current directory")
        .into_boxed_path()
}
