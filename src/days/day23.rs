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

fn west() -> [IVec2; 3] {
    [ivec2(-1, -1), ivec2(-1, 0), ivec2(-1, 1)]
}

fn east() -> [IVec2; 3] {
    [ivec2(1, -1), ivec2(1, 0), ivec2(1, 1)]
}

fn rules() -> [(IVec2, [IVec2; 3]); 4] {
    [
        (ivec2(0, -1), north()),
        (ivec2(0, 1), south()),
        (ivec2(-1, 0), west()),
        (ivec2(1, 0), east()),
    ]
}

pub fn a(input: &str) -> i32 {
    let mut map = parse_map(input);

    for round in 0..=10 {
        let mut proposed_moves = Vec::new();
        let mut proposed_moves_count = HashMap::<IVec2, i32>::new();

        for elf in map.keys() {
            if all_direction()
                .iter()
                .all(|d| !map.contains_key(&(*elf + *d)))
            {
                continue;
            }

            for rule_index in 0..4 {
                let rule_index = (rule_index + round) % 4;

                let rule = rules()[rule_index];

                if rule.1.iter().all(|d| !map.contains_key(&(*elf + *d))) {
                    let proposed_pos = *elf + rule.0;
                    proposed_moves.push((*elf, proposed_pos));
                    *proposed_moves_count.entry(proposed_pos).or_default() += 1;

                    break;
                }
            }
        }

        for (old_pos, new_pos) in proposed_moves {
            if *proposed_moves_count.get(&new_pos).unwrap() == 1 {
                map.remove(&old_pos);
                map.insert(new_pos, '#');
            }
        }
    }

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

    let mut count = 0;

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if !map.contains_key(&ivec2(x, y)) {
                count += 1;
            }
        }
    }

    count
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 110);
    assert_eq!(a(INPUT), 3947);
}

pub fn b(input: &str) -> i32 {
    0
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 0);
    assert_eq!(b(INPUT), 0);
}
