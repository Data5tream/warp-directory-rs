use clap::ArgMatches;
use std::path::Path;

use crate::storage::{save_warp_point, WarpPoint};

fn get_current_directory() -> Box<Path> {
    std::env::current_dir()
        .expect("Unable to get current directory")
        .into_boxed_path()
}

pub fn add_warp_point(matches: &ArgMatches) {
    let name = matches.get_one::<String>("name").unwrap();
    let path = matches.get_one::<String>("path");
    let force = matches.get_flag("force");
    let description = matches.get_one::<String>("description");

    let working_dir = get_current_directory();
    let path = match path {
        Some(path) => working_dir.join(path),
        None => working_dir.into(),
    };

    let warp_point = WarpPoint {
        name: name.clone(),
        path: Box::from(path.as_path()),
        description: description.cloned(),
    };

    save_warp_point(warp_point, force);
}
