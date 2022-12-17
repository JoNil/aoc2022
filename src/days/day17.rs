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

fn shape_collides(map: &HashMap<IVec2, char>, shape_count: i64, pos: IVec2) -> bool {
    SHAPES[shape_count as usize % SHAPES.len()]
        .iter()
        .map(|s| *s + pos)
        .any(|s| map.contains_key(&s) || s.y == 1 || s.x == -1 || s.x == 7)
}

fn insert_shape(map: &mut HashMap<IVec2, char>, shape_count: i64, pos: IVec2) {
    for transformed_pos in SHAPES[shape_count as usize % SHAPES.len()]
        .iter()
        .map(|s| *s + pos)
    {
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

pub fn b(input: &str) -> i64 {
    let wind = input.chars().collect::<Vec<_>>();

    let mut map: HashMap<IVec2, char> = HashMap::new();

    let mut wind_step = 0;

    for shape_count in 0..1_000_000_000_000i64 {
        if shape_count % 1000 == 0 {
            println!("{shape_count}");
        }

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

    -map.keys().map(|p| p.y).min().unwrap() as i64 + 1
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 1514285714288);
    assert_eq!(b(INPUT), 0);
}
