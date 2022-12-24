use crate::utils::map::print_map;
use glam::{ivec2, IVec2};
use std::collections::HashMap;

pub static INPUT: &str = include_str!("../input/23.txt");
pub static TEST_INPUT: &str = include_str!("../input/23_test.txt");

fn parse_map(input: &str) -> HashMap<IVec2, char> {
    let mut map = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                map.insert(ivec2(x as i32, y as i32), '#');
            }
        }
    }

    map
}

fn all_direction() -> [IVec2; 8] {
    [
        ivec2(-1, -1),
        ivec2(0, -1),
        ivec2(1, -1),
        ivec2(-1, 0),
        ivec2(1, 0),
        ivec2(-1, 1),
        ivec2(0, 1),
        ivec2(1, 1),
    ]
}

fn north() -> [IVec2; 3] {
    [ivec2(-1, -1), ivec2(0, -1), ivec2(1, -1)]
}

fn south() -> [IVec2; 3] {
    [ivec2(-1, 1), ivec2(0, 1), ivec2(1, 1)]
}

fn east() -> [IVec2; 3] {
    [ivec2(1, -1), ivec2(1, 0), ivec2(1, 1)]
}

fn west() -> [IVec2; 3] {
    [ivec2(-1, -1), ivec2(-1, 0), ivec2(-1, 1)]
}

fn rules() -> [(IVec2, [IVec2; 3]); 4] {
    [
        (ivec2(0, -1), north()),
        (ivec2(0, 1), south()),
        (ivec2(1, 0), east()),
        (ivec2(-1, 0), west()),
    ]
}

pub fn a(input: &str) -> i32 {
    let map = parse_map(input);

    print_map(&map);

    for round in 0..10 {
        for elf in map.keys() {
            if all_direction()
                .iter()
                .all(|d| !map.contains_key(&(*elf + *d)))
            {
                continue;
            }

            let mut proposed_moves = HashMap::new();
            let mut proposed_moves_count = HashMap::new();

            for rule_index in 0..4 {
                let rule_index = (rule_index + round) % 4;

                let rule = rules()[rule_index];

                if rule.1.iter().all(|d| !map.contains_key(&(*elf + *d))) {
                    let proposed_pos = *elf + rule.0;
                    proposed_moves.insert(proposed_pos, (*elf, proposed_pos));
                    *proposed_moves_count.entry(proposed_pos).or_default() += 1;

                    break;
                }
            }
        }

        ru
    }

    0
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 110);
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
