use glam::{ivec3, IVec3};
use pathfinding::prelude::dijkstra;
use std::collections::HashSet;

pub static INPUT: &str = include_str!("../input/18.txt");
pub static TEST_INPUT: &str = include_str!("../input/18_test.txt");

static DIRS: &[IVec3] = &[
    ivec3(1, 0, 0),
    ivec3(-1, 0, 0),
    ivec3(0, 1, 0),
    ivec3(0, -1, 0),
    ivec3(0, 0, 1),
    ivec3(0, 0, -1),
];

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
        .flat_map(|p| DIRS.iter().map(|c| *c + *p))
        .filter(|p| !drop.contains(p))
        .count() as i32
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 64);
    assert_eq!(a(INPUT), 3530);
}

pub fn b(input: &str) -> i32 {
    let mut drop = parse(input);
    let mut has_path_to_outside = HashSet::new();

    loop {
        let mut new_drop = drop.clone();
        let mut has_tested = HashSet::new();

        for candidate in drop.iter().flat_map(|p| DIRS.iter().map(|c| *c + *p)) {
            if has_tested.contains(&candidate) {
                continue;
            }

            if has_path_to_outside.contains(&candidate) {
                continue;
            }

            has_tested.insert(candidate);

            let path = dijkstra(
                &candidate,
                |&p| {
                    DIRS.iter()
                        .map(move |c| (*c + p, 1))
                        .filter(|(p, _)| !drop.contains(p))
                },
                |&p| p == ivec3(-1, -1, -1),
            );
            if path.is_none() {
                new_drop.insert(candidate);
            } else {
                has_path_to_outside.insert(candidate);
            }
        }

        if drop.len() == new_drop.len() {
            break;
        }

        drop = new_drop;
    }

    drop.iter()
        .flat_map(|p| DIRS.iter().map(|c| *c + *p))
        .filter(|p| !drop.contains(p))
        .count() as i32
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 58);
    assert_eq!(b(INPUT), 2000);
}
