use glam::{ivec2, IVec2};
use pathfinding::prelude::dijkstra;
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

pub static INPUT: &str = include_str!("../input/24.txt");
pub static TEST_INPUT: &str = include_str!("../input/24_test.txt");

struct BoundingBox {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
                '.' => (),
                _ => panic!("Bad tile {c}"),
            }
        }
    }

    (map, blizzards, bounding_box)
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct State {
    pos: IVec2,
    step: i32,
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("State").field("pos", &self.pos).finish()
    }
}

pub fn a(input: &str) -> i32 {
    let (map, blizzards, bb) = parse_map(input);
    let map = &map;

    let x_len = bb.max_x - bb.min_x - 1;
    let y_len = bb.max_y - bb.min_y - 1;

    let (x_blizzards, y_blizzards) = {
        let mut x_blizzards = Vec::new();
        let mut y_blizzards = Vec::new();

        {
            let mut state = blizzards
                .iter()
                .copied()
                .filter(|b| b.dir.y == 0)
                .collect::<Vec<_>>();

            let initial_state = state.clone();

            for _ in 0..x_len {
                x_blizzards.push(state.iter().map(|b| b.pos).collect::<HashSet<_>>());
                state = update_blizzards(&bb, &state);
            }

            assert_eq!(state, initial_state);
        }

        {
            let mut state = blizzards
                .iter()
                .copied()
                .filter(|b| b.dir.x == 0)
                .collect::<Vec<_>>();

            let initial_state = state.clone();

            for _ in 0..y_len {
                y_blizzards.push(state.iter().map(|b| b.pos).collect::<HashSet<_>>());
                state = update_blizzards(&bb, &state);
            }

            assert_eq!(state, initial_state);
        }

        (x_blizzards, y_blizzards)
    };

    let start_x = (bb.min_x..=bb.max_x)
        .find(|x| !map.contains_key(&ivec2(*x, bb.min_y)))
        .unwrap();
    let end_x = (bb.min_x..=bb.max_x)
        .find(|x| !map.contains_key(&ivec2(*x, bb.max_y)))
        .unwrap();

    let start = State {
        pos: ivec2(start_x, bb.min_y),
        step: 0,
    };

    let end = ivec2(end_x, bb.max_y);

    let res = dijkstra(
        &start,
        |s| {
            let candidates = [
                (s.pos + ivec2(1, 0), s.step + 1),
                (s.pos + ivec2(-1, 0), s.step + 1),
                (s.pos + ivec2(0, 1), s.step + 1),
                (s.pos + ivec2(0, -1), s.step + 1),
                (s.pos, s.step + 1),
            ];

            candidates.into_iter().filter_map(|(c, step)| {
                if (c.x == bb.max_x || c.x == bb.min_x || c.y == bb.max_y || c.y == bb.min_y)
                    && c != end
                {
                    return None;
                }

                if x_blizzards[step as usize % x_blizzards.len()].contains(&c) {
                    return None;
                }

                if y_blizzards[step as usize % y_blizzards.len()].contains(&c) {
                    return None;
                }

                Some((State { pos: c, step }, 1))
            })
        },
        |s| s.pos == end,
    )
    .unwrap();

    //println!("{:#?}", res.0);

    res.0.len() as i32 - 1
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 18);
    //panic!("Te");
    // Not 153
    //assert_eq!(a(INPUT), 0);
}

pub fn b(input: &str) -> i32 {
    0
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 0);
    assert_eq!(b(INPUT), 0);
}
