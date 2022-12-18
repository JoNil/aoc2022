use std::collections::HashSet;

use glam::{ivec3, IVec3};

pub static INPUT: &str = include_str!("../input/18.txt");
pub static TEST_INPUT: &str = include_str!("../input/18_test.txt");

fn parse(input: &str) -> HashSet<IVec3> {
    input
        .lines()
        .map(|l| {
            let v = l
                .split(',')
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            ivec3(v[0], v[1], v[2])
        })
        .collect()
}

pub fn a(input: &str) -> i32 {
    let drop = parse(input);

    drop.iter()
        .flat_map(|p| {
            [
                ivec3(1, 0, 0),
                ivec3(-1, 0, 0),
                ivec3(0, 1, 0),
                ivec3(0, -1, 0),
                ivec3(0, 0, 1),
                ivec3(0, 0, -1),
            ]
            .into_iter()
            .map(|c| c + *p)
        })
        .filter(|p| !drop.contains(p))
        .count() as i32
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 64);
    assert_eq!(a(INPUT), 3530);
}

pub fn b(input: &str) -> i32 {
    0
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 0);
    assert_eq!(b(INPUT), 0);
}
