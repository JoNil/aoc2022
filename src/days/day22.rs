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

        for x in x_min..=x_max {
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

        for y in y_min..=y_max {
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

#[derive(Copy, Clone)]
enum Dir {
    R,
    L,
    U,
    D,
}

impl Dir {
    fn rotate_cw(self) -> Dir {
        match self {
            Dir::R => Dir::D,
            Dir::L => Dir::U,
            Dir::U => Dir::R,
            Dir::D => Dir::L,
        }
    }

    fn rotate_ccw(self) -> Dir {
        match self {
            Dir::R => Dir::U,
            Dir::L => Dir::D,
            Dir::U => Dir::L,
            Dir::D => Dir::R,
        }
    }

    fn dir(self) -> IVec2 {
        match self {
            Dir::R => ivec2(1, 0),
            Dir::L => ivec2(-1, 0),
            Dir::U => ivec2(0, -1),
            Dir::D => ivec2(0, 1),
        }
    }

    fn score(self) -> i32 {
        match self {
            Dir::R => 0,
            Dir::L => 2,
            Dir::U => 3,
            Dir::D => 1,
        }
    }
}

pub fn a(input: &str) -> i32 {
    let (map_input, instructions_input) = input.split_once("\n\n").unwrap();
    let (map, x_wrapping, y_wrapping) = parse_map(map_input);
    let instructions = parse_instructions(instructions_input.trim());

    print_map(&map);
    println!("{x_wrapping:#?}");
    println!("{instructions:#?}");

    let x_start = map
        .keys()
        .filter_map(|p| if p.y == 1 { Some(p.x) } else { None })
        .min()
        .unwrap();

    let mut pos = ivec2(x_start, 1);
    let mut dir = Dir::R;

    for instruction in &instructions {
        match instruction {
            Instruction::Fwd(steps) => {
                for _ in 0..(*steps) {
                    let candidate_pos = pos + dir.dir();

                    match map.get(&candidate_pos).unwrap_or(&' ') {
                        '.' => {
                            pos = candidate_pos;
                        }
                        '#' => break,
                        ' ' => match dir {
                            Dir::R => {
                                let new_candidate = ivec2(
                                    y_wrapping.get(&candidate_pos.y).unwrap().0,
                                    candidate_pos.y,
                                );

                                if *map.get(&new_candidate).unwrap() != '#' {
                                    pos = new_candidate;
                                }
                            }
                            Dir::L => {
                                let new_candidate = ivec2(
                                    y_wrapping.get(&candidate_pos.y).unwrap().1,
                                    candidate_pos.y,
                                );

                                if *map.get(&new_candidate).unwrap() != '#' {
                                    pos = new_candidate;
                                }
                            }
                            Dir::U => {
                                let new_candidate = ivec2(
                                    candidate_pos.x,
                                    x_wrapping.get(&candidate_pos.x).unwrap().1,
                                );

                                if *map.get(&new_candidate).unwrap() != '#' {
                                    pos = new_candidate;
                                }
                            }
                            Dir::D => {
                                let new_candidate = ivec2(
                                    candidate_pos.x,
                                    x_wrapping.get(&candidate_pos.x).unwrap().0,
                                );

                                if *map.get(&new_candidate).unwrap() != '#' {
                                    pos = new_candidate;
                                }
                            }
                        },
                        _ => (),
                    }
                }
            }
            Instruction::Cw => {
                dir = dir.rotate_cw();
            }
            Instruction::Ccw => {
                dir = dir.rotate_ccw();
            }
        }
    }

    1000 * pos.y + 4 * pos.x + dir.score()
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
