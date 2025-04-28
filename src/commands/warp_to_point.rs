pub fn warp_to_point(point_name: &str) {
    if let Some(warp_point) = crate::storage::get_warp_point(point_name) {
        println!("{}", warp_point.path.display());
    } else {
        eprintln!("Warp point with name '{point_name}' not found.");
    }
}
