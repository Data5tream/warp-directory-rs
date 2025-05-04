use std::path::Path;

use directories::ProjectDirs;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct WarpPoint {
    pub(crate) name: String,
    pub(crate) path: Box<Path>,
    pub(crate) description: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct WarpPointFile {
    warp_points: Vec<WarpPoint>,
}

pub fn get_storage_file() -> Box<Path> {
    if let Some(dirs) = ProjectDirs::from("at", "b1t", "warp-directory") {
        let storage_dir = dirs.data_local_dir();
        if !storage_dir.exists() {
            std::fs::create_dir_all(storage_dir).expect("Unable to create storage directory");
        }

        return dirs.data_local_dir().join("warp_points.json").into();
    }

    eprintln!("Could not find storage directory");
    std::process::exit(1);
}

/// Loads all warp points from the storage file.
pub fn load_warp_points() -> Vec<WarpPoint> {
    let warp_points_file = get_storage_file();
    if !warp_points_file.exists() {
        return Vec::new();
    }

    let file = std::fs::File::open(warp_points_file).expect("Unable to open warp points file");
    let reader = std::io::BufReader::new(file);
    serde_json::from_reader(reader).expect("Unable to parse warp points file")
}

/// Saves the warp points to the storage file.
fn save_warp_points(warp_points: &[WarpPoint]) {
    let warp_points_file = get_storage_file();
    let file = std::fs::File::create(warp_points_file).expect("Unable to create warp points file");
    let writer = std::io::BufWriter::new(file);
    serde_json::to_writer(writer, &warp_points).expect("Unable to write warp points file");
}

/// Tries to find a warp point by name.
pub fn get_warp_point(name: &str) -> Option<WarpPoint> {
    let warp_points = load_warp_points();
    warp_points
        .into_iter()
        .find(|warp_point| warp_point.name == name)
}

/// Add a new warp point to the storage file.
///
/// Doesn't do anything if the warp point already exists and `force` is not `true`.
pub fn save_warp_point(new_warp_point: WarpPoint, force: bool) {
    let mut warp_points = load_warp_points();

    let mut found = false;
    for warp_point in &mut warp_points {
        if warp_point.name.eq(&new_warp_point.name) {
            found = true;
            if force {
                warp_point.path = new_warp_point.path.clone();
                warp_point
                    .description
                    .clone_from(&new_warp_point.description);
            } else {
                eprintln!(
                    "Warp point with name '{}' already exists. Use --force to overwrite.",
                    warp_point.name
                );
                return;
            }
        }
    }

    if !found {
        warp_points.push(new_warp_point);
    }

    save_warp_points(&warp_points);
}

/// Removes a warp point from the storage file.
///
/// Doesn't do anything if the warp point doesn't exist.
pub fn remove_warp_point(name: &str) {
    let mut warp_points = load_warp_points();
    warp_points.retain(|warp_point| warp_point.name != name);

    save_warp_points(&warp_points);
}
