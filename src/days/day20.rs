use std::cmp::Ordering;

pub static INPUT: &str = include_str!("../input/20.txt");
pub static TEST_INPUT: &str = include_str!("../input/20_test.txt");
pub static TORKEL_INPUT: &str = include_str!("../input/20_torkel.txt");

fn modulo(a: i32, b: usize) -> usize {
    let mut res = a % b as i32;
    while res < 0 {
        res += b as i32;
    }
    res as usize
}

#[test]
fn test_modulo() {
    assert_eq!(modulo(8, 4), 0);
    assert_eq!(modulo(9, 4), 1);
    assert_eq!(modulo(-1, 4), 3);
    assert_eq!(modulo(-2, 4), 2);
}

pub fn a(input: &str, assert_test: bool) -> i32 {
    let original_file = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut scrambled_file = original_file.clone();
    let len = scrambled_file.len();

    for index in 0..len {
        if assert_test {
            let test = match index {
                0 => Some([1, 2, -3, 3, -2, 0, 4]),
                1 => Some([2, 1, -3, 3, -2, 0, 4]),
                2 => Some([1, -3, 2, 3, -2, 0, 4]),
                3 => Some([1, 2, 3, -2, -3, 0, 4]),
                4 => Some([1, 2, -2, -3, 0, 3, 4]),
                5 => Some([1, 2, -3, 0, 3, 4, -2]),
                6 => Some([1, 2, -3, 0, 3, 4, -2]),
                7 => Some([1, 2, -3, 4, 0, 3, -2]),
                _ => None,
            };

            if let Some(test) = test {
                assert_eq!(scrambled_file, test);
            }
        }

        let value_to_move = original_file[index % len];
        let src_index = scrambled_file
            .iter()
            .enumerate()
            .find(|(_, val)| value_to_move == **val)
            .unwrap()
            .0;

        //print!("{value_to_move} {src_index}: {:?} => ", scrambled_file);

        match value_to_move.cmp(&0) {
            Ordering::Greater => {
                let mut rotations = 0;

                for count in 0..value_to_move {
                    let from = modulo(src_index as i32 + count, len);
                    let next = modulo(src_index as i32 + count + 1, len);
                    scrambled_file.swap(from, next);

                    if next == len - 1 {
                        rotations += 1;
                    }
                }

                for _ in 0..rotations {
                    scrambled_file.rotate_right(1);
                }
            }
            Ordering::Less => {
                let value_to_move = value_to_move.abs();
                let mut rotations = 0;

                for count in 0..value_to_move {
                    let from = modulo(src_index as i32 - count, len);
                    let next = modulo(src_index as i32 - count - 1, len);
                    scrambled_file.swap(from, next);

                    if next == 0 {
                        rotations += 1;
                    }
                }

                for _ in 0..rotations {
                    scrambled_file.rotate_left(1);
                }
            }
            Ordering::Equal => (),
        }

        //println!("{:?}", scrambled_file)
    }

    if assert_test {
        assert_eq!(scrambled_file, [1, 2, -3, 4, 0, 3, -2]);
    }

    let zero_index = scrambled_file
        .iter()
        .enumerate()
        .find(|(_, v)| **v == 0)
        .unwrap()
        .0;

    let a = scrambled_file[(zero_index + 1000) % scrambled_file.len()];
    let b = scrambled_file[(zero_index + 2000) % scrambled_file.len()];
    let c = scrambled_file[(zero_index + 3000) % scrambled_file.len()];

    // 225, 1013, 8628
    println!("{a}, {b}, {c}");

    a + b + c
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT, true), 3);
    assert_eq!(a(TORKEL_INPUT, false), 9866);

    // not -35..
    // not 4387
    // not 8579
    // not -339
    assert_eq!(a(INPUT, false), 0);
}

pub fn b(input: &str) -> i32 {
    0
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 0);
    assert_eq!(b(INPUT), 0);
}
