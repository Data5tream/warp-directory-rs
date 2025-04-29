use std::path::Path;

pub fn get_current_directory() -> Box<Path> {
    std::env::current_dir()
        .expect("Unable to get current directory")
        .into_boxed_path()
}
