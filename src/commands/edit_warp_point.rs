use clap::ArgMatches;

use crate::storage::{get_warp_point, save_warp_point};
use crate::util::get_current_directory;

pub fn edit_warp_point(matches: &ArgMatches) {
    let name = matches.get_one::<String>("name").unwrap();

    let Some(mut warp_point) = get_warp_point(name) else {
        eprintln!("Warp point with name '{name}' not found.");
        return;
    };

    if let Some(new_path) = matches.get_one::<String>("path") {
        let working_dir = get_current_directory();
        let path = match new_path.as_str() {
            "." => working_dir.into(),
            p => working_dir.join(p),
        };

        warp_point.path = Box::from(path.as_path());
    }

    if let Some(new_description) = matches.get_one::<String>("description") {
        warp_point.description = Some(new_description.clone());
    }

    save_warp_point(warp_point, true);
}
