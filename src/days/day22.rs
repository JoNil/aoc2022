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

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub enum Dir {
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

fn find_side(sides: &HashMap<i32, IVec2>, side_len: i32, pos: IVec2) -> i32 {
    *sides
        .iter()
        .find(|s| {
            pos.x >= s.1.x && pos.x < s.1.x + side_len && pos.y >= s.1.y && pos.y < s.1.y + side_len
        })
        .unwrap()
        .0
}

fn get_local_pos(pos: IVec2, dir: Dir, side: IVec2) -> i32 {
    match dir {
        Dir::R | Dir::L => pos.y - side.y,
        Dir::U | Dir::D => pos.x - side.x,
    }
}

fn wrap(side_pos: IVec2, last: i32, dir: Dir, local_pos: i32, flip: bool) -> IVec2 {
    let transformed_local = if flip { last - local_pos } else { local_pos };

    match dir {
        Dir::R => ivec2(side_pos.x, side_pos.y + transformed_local),
        Dir::L => ivec2(side_pos.x + last, side_pos.y + transformed_local),
        Dir::U => ivec2(side_pos.x + transformed_local, side_pos.y + last),
        Dir::D => ivec2(side_pos.x + transformed_local, side_pos.y),
    }
}

pub fn b(
    input: &str,
    sides: &HashMap<i32, IVec2>,
    side_len: i32,
    connections: &HashMap<(i32, Dir), (i32, Dir, bool)>,
) -> i32 {
    let (map_input, instructions_input) = input.split_once("\n\n").unwrap();
    let (map, _, _) = parse_map(map_input);
    let instructions = parse_instructions(instructions_input.trim());

    let x_start = map
        .keys()
        .filter_map(|p| if p.y == 1 { Some(p.x) } else { None })
        .min()
        .unwrap();

    let mut debug_map = map.clone();

    let last = side_len - 1;

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
                            let current_side = find_side(sides, side_len, pos);
                            let (dest_side, wrapped_dir, flip) =
                                connections.get(&(current_side, dir)).unwrap();

                            let current_side = sides.get(&current_side).unwrap();
                            let dest_side = sides.get(dest_side).unwrap();

                            let local_pos = get_local_pos(pos, dir, *current_side);

                            let wrapped_pos =
                                wrap(*dest_side, last, *wrapped_dir, local_pos, *flip);

                            if *map.get(&wrapped_pos).unwrap() != '#' {
                                pos = wrapped_pos;
                                dir = *wrapped_dir;
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
    {
        let side_len = 4;

        let sides = [
            (1, ivec2(2 * side_len + 1, 1)),
            (2, ivec2(1, side_len + 1)),
            (3, ivec2(side_len + 1, side_len + 1)),
            (4, ivec2(2 * side_len + 1, side_len + 1)),
            (5, ivec2(2 * side_len + 1, 2 * side_len + 1)),
            (6, ivec2(3 * side_len + 1, 2 * side_len + 1)),
        ]
        .into_iter()
        .collect::<HashMap<i32, IVec2>>();

        let connections = [
            ((1, Dir::U), (2, Dir::D, true)),
            ((1, Dir::R), (6, Dir::L, true)),
            ((1, Dir::L), (3, Dir::D, false)),
            ((2, Dir::U), (1, Dir::D, true)),
            ((2, Dir::D), (5, Dir::U, true)),
            ((2, Dir::L), (6, Dir::U, true)),
            ((3, Dir::U), (1, Dir::R, false)),
            ((3, Dir::D), (5, Dir::R, true)),
            ((4, Dir::R), (6, Dir::D, true)),
            ((5, Dir::D), (2, Dir::U, true)),
            ((5, Dir::L), (3, Dir::U, true)),
            ((6, Dir::U), (4, Dir::L, true)),
            ((6, Dir::R), (1, Dir::L, true)),
            ((6, Dir::D), (2, Dir::R, true)),
        ]
        .into_iter()
        .collect::<HashMap<_, _>>();

        assert_eq!(b(TEST_INPUT, &sides, side_len, &connections), 5031);
    }

    {
        let side_len = 50;

        let sides = [
            (1, ivec2(side_len + 1, 1)),
            (2, ivec2(2 * side_len + 1, 1)),
            (3, ivec2(side_len + 1, side_len + 1)),
            (4, ivec2(1, 2 * side_len + 1)),
            (5, ivec2(side_len + 1, 2 * side_len + 1)),
            (6, ivec2(1, 3 * side_len + 1)),
        ]
        .into_iter()
        .collect::<HashMap<i32, IVec2>>();

        let connections = [
            ((1, Dir::U), (6, Dir::R, false)),
            ((1, Dir::L), (4, Dir::R, true)),
            ((2, Dir::U), (6, Dir::U, false)),
            ((2, Dir::R), (5, Dir::L, true)),
            ((2, Dir::D), (3, Dir::L, false)),
            ((3, Dir::R), (2, Dir::U, false)),
            ((3, Dir::L), (4, Dir::D, false)),
            ((4, Dir::U), (3, Dir::R, false)),
            ((4, Dir::L), (1, Dir::R, true)),
            ((5, Dir::R), (2, Dir::L, true)),
            ((5, Dir::D), (6, Dir::L, false)),
            ((6, Dir::R), (5, Dir::U, false)),
            ((6, Dir::L), (1, Dir::D, false)),
            ((6, Dir::D), (2, Dir::D, false)),
        ]
        .into_iter()
        .collect::<HashMap<_, _>>();

        assert_eq!(b(INPUT, &sides, side_len, &connections), 0);
    }
}

//         1111
//         1111
//         1111
//         1111
// 222233334444
// 222233334444
// 222233334444
// 222233334444
//         55556666
//         55556666
//         55556666
//         55556666
//
//
//
//     11112222
//     11112222
//     11112222
//     11112222
//     3333
//     3333
//     3333
//     3333
// 44445555
// 44445555
// 44445555
// 44445555
// 6666
// 6666
// 6666
// 6666
