use glam::{ivec2, IVec2};
use std::collections::HashMap;

pub fn print_map(map: &HashMap<IVec2, char>) {
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;

    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;

    for p in map.keys() {
        min_x = min_x.min(p.x);
        min_y = min_y.min(p.y);
        max_x = max_x.max(p.x);
        max_y = max_y.max(p.y);
    }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            print!("{}", map.get(&ivec2(x, y)).unwrap_or(&'.'));
        }
        println!();
    }

    println!();
}
