#![allow(clippy::type_complexity)]

use glam::{ivec2, IVec2};
use std::collections::HashMap;

use crate::utils::map::print_map;

pub static INPUT: &str = include_str!("../input/22.txt");
pub static TEST_INPUT: &str = include_str!("../input/22_test.txt");

fn parse_map(
    input: &str,
) -> (
    HashMap<IVec2, char>,
    HashMap<i32, (i32, i32)>,
    HashMap<i32, (i32, i32)>,
) {
    let mut map = HashMap::new();

    for (y, line) in input.lines().enumerate().map(|(y, line)| (y + 1, line)) {
        for (x, c) in line.chars().enumerate().map(|(x, c)| (x + 1, c)) {
            if c != ' ' {
                map.insert(ivec2(x as i32, y as i32), c);
            }
        }
    }

    let mut x_wrapping = HashMap::new();

    {
        let x_min = map.keys().map(|p| p.x).min().unwrap();
        let x_max = map.keys().map(|p| p.x).max().unwrap();

        for x in x_min..x_max {
            let y_min = map
                .keys()
                .filter_map(|p| if p.x == x { Some(p.y) } else { None })
                .min()
                .unwrap();
            let y_max = map
                .keys()
                .filter_map(|p| if p.x == x { Some(p.y) } else { None })
                .max()
                .unwrap();

            x_wrapping.insert(x, (y_min, y_max));
        }
    }

    let mut y_wrapping = HashMap::new();

    {
        let y_min = map.keys().map(|p| p.y).min().unwrap();
        let y_max = map.keys().map(|p| p.y).max().unwrap();

        for y in y_min..y_max {
            let x_min = map
                .keys()
                .filter_map(|p| if p.y == y { Some(p.x) } else { None })
                .min()
                .unwrap();
            let x_max = map
                .keys()
                .filter_map(|p| if p.y == y { Some(p.x) } else { None })
                .max()
                .unwrap();

            y_wrapping.insert(y, (x_min, x_max));
        }
    }

    (map, x_wrapping, y_wrapping)
}

#[derive(Debug)]
enum Instruction {
    Fwd(i32),
    Cw,
    Ccw,
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let mut res = Vec::new();
    let mut number = String::new();

    for c in input.chars() {
        if c.is_numeric() {
            number.push(c);
        } else {
            if !number.is_empty() {
                res.push(Instruction::Fwd(number.parse().unwrap()));
                number.clear();
            }

            res.push(match c {
                'R' => Instruction::Cw,
                'L' => Instruction::Ccw,
                _ => panic!("Bad char"),
            });
        }
    }

    res
}

pub fn a(input: &str) -> i32 {
    let (map_input, instructions_input) = input.split_once("\n\n").unwrap();
    let (map, x_wrapping, y_wrapping) = parse_map(map_input);
    let instructions = parse_instructions(instructions_input);

    print_map(&map);
    println!("{y_wrapping:#?}");
    println!("{instructions:#?}");

    0
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 6032);
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
