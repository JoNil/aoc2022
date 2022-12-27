#![allow(clippy::type_complexity)]

use glam::{ivec2, IVec2};
use std::collections::HashMap;

use crate::utils::map::print_map;

pub static INPUT: &str = include_str!("../input/22.txt");
pub static TEST_INPUT: &str = include_str!("../input/22_test.txt");
pub static TORKEL_INPUT: &str = include_str!("../input/22_torkel.txt");

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

    if !number.is_empty() {
        res.push(Instruction::Fwd(number.parse().unwrap()));
        number.clear();
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
                                } else {
                                    break;
                                }
                            }
                            Dir::L => {
                                let new_candidate = ivec2(
                                    y_wrapping.get(&candidate_pos.y).unwrap().1,
                                    candidate_pos.y,
                                );

                                if *map.get(&new_candidate).unwrap() != '#' {
                                    pos = new_candidate;
                                } else {
                                    break;
                                }
                            }
                            Dir::U => {
                                let new_candidate = ivec2(
                                    candidate_pos.x,
                                    x_wrapping.get(&candidate_pos.x).unwrap().1,
                                );

                                if *map.get(&new_candidate).unwrap() != '#' {
                                    pos = new_candidate;
                                } else {
                                    break;
                                }
                            }
                            Dir::D => {
                                let new_candidate = ivec2(
                                    candidate_pos.x,
                                    x_wrapping.get(&candidate_pos.x).unwrap().0,
                                );

                                if *map.get(&new_candidate).unwrap() != '#' {
                                    pos = new_candidate;
                                } else {
                                    break;
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
    assert_eq!(a(TORKEL_INPUT), 149250);
    assert_eq!(a(INPUT), 27436);
}

fn find_side(sides: &HashMap<i32, IVec2>, side: i32, pos: IVec2) -> i32 {
    *sides
        .iter()
        .find(|s| pos.x >= s.1.x && pos.x < s.1.x + side && pos.y >= s.1.y && pos.y < s.1.y + side)
        .unwrap()
        .0
}

enum Transform {
    Cw,
    Ccw,
    OneEighty,
    Identity,
    Negate,
}

pub fn b(input: &str, side: i32) -> i32 {
    let (map_input, instructions_input) = input.split_once("\n\n").unwrap();
    let (map, _, _) = parse_map(map_input);
    let instructions = parse_instructions(instructions_input.trim());

    let x_start = map
        .keys()
        .filter_map(|p| if p.y == 1 { Some(p.x) } else { None })
        .min()
        .unwrap();

    let sides = [
        (1, ivec2(2 * side + 1, 1)),
        (2, ivec2(1, side + 1)),
        (3, ivec2(side + 1, side + 1)),
        (4, ivec2(2 * side + 1, side + 1)),
        (5, ivec2(2 * side + 1, 2 * side + 1)),
        (6, ivec2(3 * side + 1, 2 * side + 1)),
    ]
    .into_iter()
    .collect::<HashMap<i32, IVec2>>();

    let conenctions = vec![(
        1,
        [
            (Dir::U, 2, Transform::OneEighty),
            (Dir::R, 6, Transform::OneEighty),
            (Dir::D, 4, Transform::Identity),
            (Dir::L, 3, Transform::Ccw),
        ],
    )];

    let mut debug_map = map.clone();

    let last = side - 1;

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
                        ' ' => {
                            let s = find_side(&sides, side, pos);

                            let (wrapped_pos, wrapped_dir) = match (dir, s) {
                                // Right
                                (Dir::R, 1) => {
                                    let y_on_side = pos.y - sides.get(&1).unwrap().y;
                                    let side_6 = sides.get(&6).unwrap();

                                    (ivec2(side_6.x + last, side_6.y + last - y_on_side), Dir::L)
                                }
                                (Dir::R, 4) => {
                                    let y_on_side = pos.y - sides.get(&4).unwrap().y;
                                    let side_6 = sides.get(&6).unwrap();

                                    (ivec2(side_6.x + last - y_on_side, side_6.y), Dir::D)
                                }
                                (Dir::R, 6) => {
                                    let y_on_side = pos.y - sides.get(&6).unwrap().y;
                                    let side_1 = sides.get(&1).unwrap();

                                    (ivec2(side_1.x + last, side_1.y + last - y_on_side), Dir::L)
                                }

                                // Up
                                (Dir::U, 2) => {
                                    let x_on_side = pos.x - sides.get(&2).unwrap().x;
                                    let side_1 = sides.get(&1).unwrap();

                                    (ivec2(side_1.x + last - x_on_side, side_1.y), Dir::D)
                                }
                                (Dir::U, 3) => {
                                    let x_on_side = pos.x - sides.get(&3).unwrap().x;
                                    let side_1 = sides.get(&1).unwrap();

                                    (ivec2(side_1.x, side_1.y + x_on_side), Dir::R)
                                }
                                (Dir::U, 1) => {
                                    let x_on_side = pos.x - sides.get(&1).unwrap().x;
                                    let side_2 = sides.get(&2).unwrap();

                                    (ivec2(side_2.x + last - x_on_side, side_2.y), Dir::D)
                                }
                                (Dir::U, 6) => {
                                    let x_on_side = pos.x - sides.get(&6).unwrap().x;
                                    let side_4 = sides.get(&4).unwrap();

                                    (ivec2(side_4.x + last, side_4.y + last - x_on_side), Dir::L)
                                }

                                // Down
                                (Dir::D, 2) => {
                                    let x_on_side = pos.x - sides.get(&2).unwrap().x;
                                    let side_5 = sides.get(&5).unwrap();

                                    (ivec2(side_5.x + last - x_on_side, side_5.y + last), Dir::U)
                                }
                                (Dir::D, 3) => {
                                    let x_on_side = pos.x - sides.get(&3).unwrap().x;
                                    let side_5 = sides.get(&5).unwrap();

                                    (ivec2(side_5.x, side_5.y + last - x_on_side), Dir::R)
                                }
                                (Dir::D, 5) => {
                                    let x_on_side = pos.x - sides.get(&5).unwrap().x;
                                    let side_2 = sides.get(&2).unwrap();

                                    (ivec2(side_2.x + last - x_on_side, side_2.y + last), Dir::U)
                                }
                                (Dir::D, 6) => {
                                    let x_on_side = pos.x - sides.get(&6).unwrap().x;
                                    let side_2 = sides.get(&2).unwrap();

                                    (ivec2(side_2.x, side_2.y + last - x_on_side), Dir::R)
                                }

                                // Left
                                (Dir::L, 1) => {
                                    let y_on_side = pos.y - sides.get(&1).unwrap().y;
                                    let side_3 = sides.get(&3).unwrap();

                                    (ivec2(side_3.x + y_on_side, side_3.y), Dir::D)
                                }
                                (Dir::L, 2) => {
                                    let y_on_side = pos.y - sides.get(&2).unwrap().y;
                                    let side_4 = sides.get(&4).unwrap();

                                    (ivec2(side_4.x, side_4.y + y_on_side), Dir::L)
                                }
                                (Dir::L, 5) => {
                                    let y_on_side = pos.y - sides.get(&5).unwrap().y;
                                    let side_3 = sides.get(&3).unwrap();

                                    (ivec2(side_3.x + last - y_on_side, side_3.y + last), Dir::U)
                                }

                                _ => panic!("Error"),
                            };

                            if *map.get(&wrapped_pos).unwrap() != '#' {
                                pos = wrapped_pos;
                                dir = wrapped_dir;
                            } else {
                                break;
                            }
                        }
                        _ => (),
                    }

                    println!("{pos:?}");
                    match dir {
                        Dir::R => {
                            debug_map.insert(pos, '>');
                        }
                        Dir::L => {
                            debug_map.insert(pos, '<');
                        }
                        Dir::U => {
                            debug_map.insert(pos, '^');
                        }
                        Dir::D => {
                            debug_map.insert(pos, 'v');
                        }
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

    print_map(&debug_map);

    1000 * pos.y + 4 * pos.x + dir.score()
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT, 4), 5031);
    assert_eq!(b(INPUT, 50), 0);
}
