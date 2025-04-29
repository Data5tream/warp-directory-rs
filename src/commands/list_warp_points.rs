use crate::storage::load_warp_points;
use std::iter::repeat;

fn pad_string(s: &str, len: usize, padding_char: Option<char>) -> String {
    let padding_char = padding_char.unwrap_or(' ');

    s.chars().chain(repeat(padding_char)).take(len).collect()
}

pub fn list_warp_points() {
    let mut warp_points = load_warp_points();
    warp_points.sort_by(|a, b| a.name.cmp(&b.name));

    if warp_points.is_empty() {
        println!("No warp points found.");
        return;
    }

    let max_name_length = warp_points
        .iter()
        .map(|warp_point| warp_point.name.len())
        .max()
        .unwrap_or(0);

    let max_path_length = warp_points
        .iter()
        .map(|warp_point| warp_point.path.display().to_string().len())
        .max()
        .unwrap_or(0);

    for warp_point in warp_points {
        let mut warp_info = format!(
            "{}  >  {}",
            pad_string(&warp_point.name, max_name_length, None),
            pad_string(warp_point.path.to_str().unwrap(), max_path_length, None)
        );
        if let Some(description) = &warp_point.description {
            warp_info = format!("{warp_info}  {description}");
        }
        println!("{warp_info}");
    }
}
