use clap::ArgMatches;

use crate::storage::remove_warp_point;

pub fn delete_warp_point(matches: &ArgMatches) {
    let name = matches.get_one::<String>("name").unwrap();
    remove_warp_point(name);
}
