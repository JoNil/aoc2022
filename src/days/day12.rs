use glam::{ivec2, IVec2};
use pathfinding::prelude::dijkstra;
use std::collections::HashMap;

pub static INPUT: &str = include_str!("../input/12.txt");
pub static TEST_INPUT: &str = include_str!("../input/12_test.txt");

fn parse(input: &str) -> (HashMap<IVec2, i32>, IVec2, IVec2) {
    let mut res = HashMap::new();
    let mut start = IVec2::ZERO;
    let mut end = IVec2::ZERO;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = ivec2(x as i32, y as i32);

            if c == 'S' {
                start = pos;
                res.insert(pos, 'a' as i32);
            } else if c == 'E' {
                end = pos;
                res.insert(pos, 'z' as i32);
            } else {
                res.insert(pos, c as i32);
            }
        }
    }

    (res, start, end)
}

pub fn a(input: &str) -> i32 {
    let (map, start, end) = parse(input);
    let map = &map;

    let result = dijkstra(
        &start,
        |&p| {
            let candidates = [
                p + ivec2(1, 0),
                p + ivec2(-1, 0),
                p + ivec2(0, 1),
                p + ivec2(0, -1),
            ];

            candidates.into_iter().filter_map(move |c| {
                let next_height = map.get(&c)?;
                let current_height = map.get(&p).unwrap();

                if *next_height <= *current_height + 1 {
                    Some((c, 1))
                } else {
                    None
                }
            })
        },
        |p| *p == end,
    )
    .unwrap();

    result.0.len() as i32 - 1
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 31);
    assert_eq!(a(INPUT), 462);
}

pub fn b(input: &str) -> i32 {
    let (map, _, end) = parse(input);
    let map = &map;

    map.iter()
        .filter(|(_, c)| **c == 'a' as i32)
        .filter_map(|(start, _)| {
            dijkstra(
                start,
                |&p| {
                    let candidates = [
                        p + ivec2(1, 0),
                        p + ivec2(-1, 0),
                        p + ivec2(0, 1),
                        p + ivec2(0, -1),
                    ];

                    candidates.into_iter().filter_map(move |c| {
                        let next_height = map.get(&c)?;
                        let current_height = map.get(&p).unwrap();

                        if *next_height <= *current_height + 1 {
                            Some((c, 1))
                        } else {
                            None
                        }
                    })
                },
                |p| *p == end,
            )
            .map(|r| r.0.len() as i32 - 1)
        })
        .min()
        .unwrap()
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 29);
    assert_eq!(b(INPUT), 451);
}
