use glam::{ivec2, IVec2};
use std::collections::HashMap;

pub static INPUT: &str = include_str!("../input/17.txt");
pub static TEST_INPUT: &str = include_str!("../input/17_test.txt");

static MINUS_SHAPE: &[IVec2] = &[ivec2(0, 0), ivec2(1, 0), ivec2(2, 0), ivec2(3, 0)];
static PLUS_SHAPE: &[IVec2] = &[
    ivec2(1, 0),
    ivec2(0, -1),
    ivec2(1, -1),
    ivec2(2, -1),
    ivec2(1, -2),
];
static ANGLE_SHAPE: &[IVec2] = &[
    ivec2(0, 0),
    ivec2(1, 0),
    ivec2(2, 0),
    ivec2(2, -1),
    ivec2(2, -2),
];
static I_SHAPE: &[IVec2] = &[ivec2(0, 0), ivec2(0, -1), ivec2(0, -2), ivec2(0, -3)];
static BOX_SHAPE: &[IVec2] = &[ivec2(0, 0), ivec2(1, 0), ivec2(0, -1), ivec2(1, -1)];
static SHAPES: &[&[IVec2]] = &[MINUS_SHAPE, PLUS_SHAPE, ANGLE_SHAPE, I_SHAPE, BOX_SHAPE];

fn shape_collides(map: &HashMap<IVec2, char>, shape_count: usize, pos: IVec2) -> bool {
    SHAPES[shape_count % SHAPES.len()]
        .iter()
        .map(|s| *s + pos)
        .any(|s| map.contains_key(&s) || s.y == 1 || s.x == -1 || s.x == 7)
}

fn insert_shape(map: &mut HashMap<IVec2, char>, shape_count: usize, pos: IVec2) {
    for transformed_pos in SHAPES[shape_count % SHAPES.len()].iter().map(|s| *s + pos) {
        map.insert(transformed_pos, '#');
    }
}

pub fn a(input: &str) -> i32 {
    let wind = input.chars().collect::<Vec<_>>();

    let mut map: HashMap<IVec2, char> = HashMap::new();

    let mut wind_step = 0;

    for shape_count in 0..2022 {
        let mut shape_pos = ivec2(2, map.keys().map(|p| p.y).min().unwrap_or(1) - 4);

        loop {
            let wind_pos = shape_pos
                + match wind[wind_step % wind.len()] {
                    '>' => ivec2(1, 0),
                    '<' => ivec2(-1, 0),
                    _ => panic!("Bad input"),
                };
            wind_step += 1;

            if !shape_collides(&map, shape_count, wind_pos) {
                shape_pos = wind_pos;
            }

            let fall_pos = shape_pos + ivec2(0, 1);

            if shape_collides(&map, shape_count, fall_pos) {
                insert_shape(&mut map, shape_count, shape_pos);
                break;
            } else {
                shape_pos = fall_pos
            }
        }
    }

    -map.keys().map(|p| p.y).min().unwrap() + 1
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 3068);
    assert_eq!(a(INPUT), 3111);
}

pub fn b(input: &str, preamble_count: usize, repeat_count: usize) -> i64 {
    let wind = input.chars().collect::<Vec<_>>();

    let mut map: HashMap<IVec2, char> = HashMap::new();

    let mut wind_step = 0;

    let mut last_height = 0;

    let rest_count = (1000000000000 - preamble_count) % repeat_count;

    let mut preamble_height = 0;
    let mut repeat_height = 0;
    let mut rest_height = 0;

    for shape_count in 0..10000 {
        let mut shape_pos = ivec2(2, map.keys().map(|p| p.y).min().unwrap_or(1) - 4);

        loop {
            let wind_pos = shape_pos
                + match wind[wind_step % wind.len()] {
                    '>' => ivec2(1, 0),
                    '<' => ivec2(-1, 0),
                    _ => panic!("Bad input"),
                };
            wind_step += 1;

            if !shape_collides(&map, shape_count, wind_pos) {
                shape_pos = wind_pos;
            }

            let fall_pos = shape_pos + ivec2(0, 1);

            if shape_collides(&map, shape_count, fall_pos) {
                insert_shape(&mut map, shape_count, shape_pos);
                break;
            } else {
                shape_pos = fall_pos
            }
        }

        /*if let Some(min_y) = map.keys().map(|p| p.y).min() {
            println!("{}", -min_y - last_height);
            last_height = -min_y;
        }*/

        if shape_count == preamble_count - 1 {
            let height = -map.keys().map(|p| p.y).min().unwrap();
            preamble_height = height;
            last_height = height;
        }

        if shape_count == (rest_count + preamble_count - 1) {
            let height = -map.keys().map(|p| p.y).min().unwrap();
            rest_height = height - last_height;
        }

        if shape_count == (repeat_count + preamble_count - 1) {
            let height = -map.keys().map(|p| p.y).min().unwrap();
            repeat_height = height - last_height;
            last_height = height;
        }

        if shape_count == (2 * repeat_count + preamble_count - 1) {
            let height = -map.keys().map(|p| p.y).min().unwrap();
            assert_eq!(height - last_height, repeat_height);
            last_height = height;
        }
    }

    (((1000000000000 - preamble_count) / repeat_count * repeat_height as usize
        + preamble_height as usize
        + rest_height as usize)
        + 1) as i64
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT, 15, 35), 1514285714288);
    assert_eq!(b(INPUT, 185, 1720), 1526744186042);
}
