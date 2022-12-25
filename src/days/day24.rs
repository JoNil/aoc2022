use glam::{ivec2, IVec2};
use std::collections::HashMap;

pub static INPUT: &str = include_str!("../input/24.txt");
pub static TEST_INPUT: &str = include_str!("../input/24_test.txt");

fn parse_map(input: &str) -> HashMap<IVec2, char> {
    let mut map = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                map.insert(ivec2(x as i32, y as i32), c);
            }
        }
    }

    map
}

pub fn a(input: &str) -> i32 {
    let map = parse_map(input);
    0
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 18);
    assert_eq!(a(INPUT), 0);
}

pub fn b(input: &str) -> i32 {
    0
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 0);
    assert_eq!(b(INPUT), 0);
}
