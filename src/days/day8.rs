use std::collections::{HashMap, HashSet};

pub static INPUT: &str = include_str!("../input/8.txt");
pub static TEST_INPUT: &str = include_str!("../input/8_test.txt");

fn parse(input: &str) -> HashMap<(i32, i32), i32> {
    let mut heights = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, height) in line.chars().enumerate() {
            heights.insert((x as _, y as _), height.to_digit(10).unwrap() as i32);
        }
    }

    heights
}

fn visible_count(
    trees: &HashMap<(i32, i32), i32>,
    start_pos: (i32, i32),
    dir: (i32, i32),
    visible: &mut HashSet<(i32, i32)>,
) {
    let mut max_height = -1;
    let mut pos = start_pos;

    while let Some(height) = trees.get(&pos) {
        if *height > max_height {
            max_height = *height;
            visible.insert(pos);
        }

        pos = (pos.0 + dir.0, pos.1 + dir.1);
    }
}

pub fn a(input: &str) -> i32 {
    let trees = parse(input);

    let x_len = trees.keys().map(|(x, _)| *x).max().unwrap() + 1;
    let y_len = trees.keys().map(|(_, y)| *y).max().unwrap() + 1;

    let mut visible = HashSet::new();

    for start_y in 1..(y_len - 1) {
        visible_count(&trees, (0, start_y), (1, 0), &mut visible);
        visible_count(&trees, (x_len - 1, start_y), (-1, 0), &mut visible);
    }

    for start_x in 1..(x_len - 1) {
        visible_count(&trees, (start_x, 0), (0, 1), &mut visible);
        visible_count(&trees, (start_x, y_len - 1), (0, -1), &mut visible);
    }

    visible.len() as i32 + 4
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 21);
    assert_eq!(a(INPUT), 1533);
}

fn visible_in_dir(trees: &HashMap<(i32, i32), i32>, start_pos: (i32, i32), dir: (i32, i32)) -> i32 {
    let tree_height = *trees.get(&start_pos).unwrap();
    let mut pos = (start_pos.0 + dir.0, start_pos.1 + dir.1);
    let mut count = 0;

    while let Some(height) = trees.get(&pos) {
        if tree_height > *height {
            count += 1;
            pos = (pos.0 + dir.0, pos.1 + dir.1);
        } else {
            count += 1;
            break;
        }
    }

    count
}

pub fn b(input: &str) -> i32 {
    let trees = parse(input);

    let x_len = trees.keys().map(|(x, _)| *x).max().unwrap() + 1;
    let y_len = trees.keys().map(|(_, y)| *y).max().unwrap() + 1;

    let mut max_score = 0;

    for x in 0..x_len {
        for y in 0..y_len {
            let score = visible_in_dir(&trees, (x, y), (1, 0))
                * visible_in_dir(&trees, (x, y), (-1, 0))
                * visible_in_dir(&trees, (x, y), (0, 1))
                * visible_in_dir(&trees, (x, y), (0, -1));

            max_score = max_score.max(score);
        }
    }

    max_score
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 8);
    assert_eq!(b(INPUT), 345744);
}
