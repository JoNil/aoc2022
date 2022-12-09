use std::collections::HashSet;

use glam::{ivec2, IVec2};

pub static INPUT: &str = include_str!("../input/9.txt");
pub static TEST_INPUT: &str = include_str!("../input/9_test.txt");
pub static TEST_INPUT_2: &str = include_str!("../input/9_test_2.txt");

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
            head += dir;
            if (head.x - tail.x).abs() > 1 || (head.y - tail.y).abs() > 1 {
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
    assert_eq!(a(INPUT), 6023);
}

pub fn b(input: &str) -> i32 {
    let mut tail_pos = HashSet::new();
    let mut rope = [ivec2(0, 0); 10];

    for line in input.lines() {
        let (dir, count) = line
            .split_once(' ')
            .map(|(d, s)| (parse_dir(d), s.parse::<i32>().unwrap()))
            .unwrap();

        for _ in 0..count {
            rope[0] += dir;
            for i in 1..10 {
                let head = rope[i - 1];
                let tail = &mut rope[i];

                if (head.x - tail.x).abs() > 1 && (head.y - tail.y).abs() > 1 {
                    *tail = head - ivec2((head.x - tail.x).signum(), (head.y - tail.y).signum());
                } else if (head.x - tail.x).abs() > 1 {
                    *tail = head - ivec2((head.x - tail.x).signum(), 0);
                } else if (head.y - tail.y).abs() > 1 {
                    *tail = head - ivec2(0, (head.y - tail.y).signum());
                }
            }
            tail_pos.insert(rope[9]);
        }
    }

    tail_pos.len() as i32
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 1);
    assert_eq!(b(TEST_INPUT_2), 36);
    assert_eq!(b(INPUT), 2533);
}
