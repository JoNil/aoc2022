use std::cmp::Ordering;

pub static INPUT: &str = include_str!("../input/20.txt");
pub static TEST_INPUT: &str = include_str!("../input/20_test.txt");

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

pub fn a(input: &str) -> i32 {
    let original_file = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut scrambled_file = original_file.clone();
    let len = scrambled_file.len();

    for index in 0..len {
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

        let value_to_move = original_file[index % len];
        let src_index = scrambled_file
            .iter()
            .enumerate()
            .find(|(_, val)| value_to_move == **val)
            .unwrap()
            .0;

        print!("{value_to_move} {src_index}: {:?} => ", scrambled_file);

        match value_to_move.cmp(&0) {
            Ordering::Greater => {
                for count in 0..value_to_move {
                    let from = modulo(src_index as i32 + count, len);
                    let next = modulo(src_index as i32 + count + 1, len);
                    scrambled_file.swap(from, next);
                }
            }
            Ordering::Less => {
                let value_to_move = value_to_move.abs();
                let mut needs_rotation = false;

                for count in 0..value_to_move {
                    let from = modulo(src_index as i32 - count, len);
                    let next = modulo(src_index as i32 - count - 1, len);
                    scrambled_file.swap(from, next);

                    if next == 0 {
                        needs_rotation = true;
                    }
                }

                if needs_rotation {
                    scrambled_file.rotate_left(1);
                }
            }
            Ordering::Equal => (),
        }

        println!("{:?}", scrambled_file);
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

    a + b + c
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 3);
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
