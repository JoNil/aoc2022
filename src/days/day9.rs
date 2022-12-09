use std::collections::HashSet;

use glam::{ivec2, IVec2};

pub static INPUT: &str = include_str!("../input/9.txt");
pub static TEST_INPUT: &str = include_str!("../input/9_test.txt");

fn parse_dir(d: &str) -> IVec2 {
    match d {
        "R" => ivec2(1, 0),
        "L" => ivec2(-1, 0),
        "U" => ivec2(0, 1),
        "D" => ivec2(0, -1),
        _ => panic!("Invalid dir"),
    }
}

pub fn a(input: &str) -> i32 {
    let mut head = ivec2(0, 0);
    let mut tail = ivec2(0, 0);
    let mut tail_pos = HashSet::new();

    for line in input.lines() {
        let (dir, count) = line
            .split_once(' ')
            .map(|(d, s)| (parse_dir(d), s.parse::<i32>().unwrap()))
            .unwrap();

        for _ in 0..count {
            if head * dir.abs() == tail * dir.abs() {
                head += dir;
            } else {
                head += dir;
                tail = head - dir;
            }
            tail_pos.insert(tail);
        }
    }

    tail_pos.len() as i32
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 13);
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
