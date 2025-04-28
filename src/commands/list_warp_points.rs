use crate::storage::load_warp_points;

pub fn list_warp_points() {
    let warp_points = load_warp_points();

    if warp_points.is_empty() {
        println!("No warp points found.");
        return;
    }

    for warp_point in warp_points {
        let mut warp_info = format!("{}  ->  {}", warp_point.name, warp_point.path.display());
        if let Some(description) = &warp_point.description {
            warp_info = format!("{warp_info}\t{description}");
        }
        println!("{warp_info}");
    }
}
