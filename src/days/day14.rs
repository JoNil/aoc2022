use glam::{ivec2, IVec2};
use std::collections::HashMap;

pub static INPUT: &str = include_str!("../input/14.txt");
pub static TEST_INPUT: &str = include_str!("../input/14_test.txt");

fn parse_lines(input: &str) -> HashMap<IVec2, char> {
    let mut res = HashMap::new();

    for line in input.lines() {
        let positions = line
            .split(" -> ")
            .map(|p| {
                let (x, y) = p.split_once(',').unwrap();
                ivec2(x.parse().unwrap(), y.parse().unwrap())
            })
            .collect::<Vec<_>>();

        for line in positions.windows(2) {
            let a = line[0];
            let b = line[1];

            if a.x == b.x {
                let min_y = a.y.min(b.y);
                let max_y = a.y.max(b.y);

                for y in min_y..=max_y {
                    res.insert(ivec2(a.x, y), '#');
                }
            } else {
                let min_x = a.x.min(b.x);
                let max_x = a.x.max(b.x);

                for x in min_x..=max_x {
                    res.insert(ivec2(x, a.y), '#');
                }
            }
        }
    }

    res
}

pub fn a(input: &str) -> i32 {
    let mut map = parse_lines(input);

    let max_y = map.keys().map(|p| p.y).max().unwrap();

    'outer: loop {
        let mut new_sand = ivec2(500, 0);

        loop {
            let old_sand = new_sand;

            for candidate in [ivec2(0, 1), ivec2(-1, 1), ivec2(1, 1)]
                .iter()
                .map(|c| new_sand + *c)
            {
                if !map.contains_key(&candidate) {
                    new_sand = candidate;
                    break;
                }
            }

            if old_sand == new_sand {
                map.insert(new_sand, 'o');
                break;
            }

            if new_sand.y > max_y {
                break 'outer;
            }
        }
    }

    map.values().filter(|c| **c == 'o').count() as i32
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 24);
    assert_eq!(a(INPUT), 964);
}

pub fn b(input: &str) -> i32 {
    let mut map = parse_lines(input);

    let floor = map.keys().map(|p| p.y).max().unwrap() + 2;

    'outer: loop {
        let mut new_sand = ivec2(500, 0);

        loop {
            let old_sand = new_sand;

            for candidate in [ivec2(0, 1), ivec2(-1, 1), ivec2(1, 1)]
                .iter()
                .map(|c| new_sand + *c)
            {
                if !(map.contains_key(&candidate) || candidate.y == floor) {
                    new_sand = candidate;
                    break;
                }
            }

            if new_sand == ivec2(500, 0) {
                map.insert(new_sand, 'o');
                break 'outer;
            }

            if old_sand == new_sand {
                map.insert(new_sand, 'o');
                break;
            }
        }
    }

    map.values().filter(|c| **c == 'o').count() as i32
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 93);
    assert_eq!(b(INPUT), 32041);
}
