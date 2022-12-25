use crate::utils::map::print_map;
use glam::{ivec2, IVec2};
use std::collections::HashMap;

pub static INPUT: &str = include_str!("../input/24.txt");
pub static TEST_INPUT: &str = include_str!("../input/24_test.txt");

struct BoundingBox {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

struct Blizzard {
    pos: IVec2,
    dir: IVec2,
}

fn update_blizzards(bb: &BoundingBox, blizzards: &[Blizzard]) -> Vec<Blizzard> {
    let mut new_blizzards = Vec::with_capacity(blizzards.len());

    for blizzard in blizzards {
        let mut new_pos = blizzard.pos + blizzard.dir;

        if new_pos.x == bb.max_x {
            new_pos = ivec2(bb.min_x + 1, blizzard.pos.y);
        }

        if new_pos.x == bb.min_x {
            new_pos = ivec2(bb.max_x - 1, blizzard.pos.y);
        }

        if new_pos.y == bb.max_y {
            new_pos = ivec2(blizzard.pos.x, bb.min_y + 1);
        }

        if new_pos.y == bb.min_y {
            new_pos = ivec2(blizzard.pos.x, bb.max_y - 1);
        }

        new_blizzards.push(Blizzard {
            pos: new_pos,
            dir: blizzard.dir,
        });
    }

    new_blizzards
}

fn parse_map(input: &str) -> (HashMap<IVec2, char>, Vec<Blizzard>, BoundingBox) {
    let mut map = HashMap::new();
    let mut blizzards = Vec::new();
    let mut bounding_box = BoundingBox {
        min_x: i32::MAX,
        max_x: i32::MIN,
        min_y: i32::MAX,
        max_y: i32::MIN,
    };

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    map.insert(ivec2(x as i32, y as i32), c);
                    bounding_box.min_x = bounding_box.min_x.min(x as i32);
                    bounding_box.max_x = bounding_box.max_x.max(x as i32);
                    bounding_box.min_y = bounding_box.min_y.min(y as i32);
                    bounding_box.max_y = bounding_box.max_y.max(y as i32);
                }
                '<' => {
                    blizzards.push(Blizzard {
                        pos: ivec2(x as i32, y as i32),
                        dir: ivec2(-1, 0),
                    });
                }
                '>' => {
                    blizzards.push(Blizzard {
                        pos: ivec2(x as i32, y as i32),
                        dir: ivec2(1, 0),
                    });
                }
                '^' => {
                    blizzards.push(Blizzard {
                        pos: ivec2(x as i32, y as i32),
                        dir: ivec2(0, -1),
                    });
                }
                'v' => {
                    blizzards.push(Blizzard {
                        pos: ivec2(x as i32, y as i32),
                        dir: ivec2(0, 1),
                    });
                }
                _ => panic!("Bad tile"),
            }
        }
    }

    (map, blizzards, bounding_box)
}

pub fn a(input: &str) -> i32 {
    let map = parse_map(input);
    0
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 18);
    assert_eq!(a(TEST_INPUT_2), 18);
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
