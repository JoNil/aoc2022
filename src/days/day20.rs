use std::cmp::Ordering;

pub static INPUT: &str = include_str!("../input/20.txt");
pub static TEST_INPUT: &str = include_str!("../input/20_test.txt");

pub fn a(input: &str) -> i32 {
    let original_file = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut scrambled_file = original_file.clone();

    for index in 0..original_file.len() {
        let value_to_move = original_file[index % original_file.len()];
        let src_index = scrambled_file
            .iter()
            .enumerate()
            .find(|(_, val)| value_to_move == **val)
            .unwrap()
            .0;

        let dst_index = {
            let mut dst_index =
                (src_index as i32 + value_to_move) % (original_file.len() as i32 - 1);
            if dst_index <= 0 {
                dst_index += original_file.len() as i32 - 1;
            }
            dst_index as usize
        };

        match src_index.cmp(&dst_index) {
            Ordering::Less => {
                scrambled_file[src_index..=dst_index].rotate_left(1);
            }
            Ordering::Greater => {
                scrambled_file[dst_index..=src_index].rotate_right(1);
            }
            Ordering::Equal => {}
        }
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
