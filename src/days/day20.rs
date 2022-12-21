use std::cmp::Ordering;

pub static INPUT: &str = include_str!("../input/20.txt");
pub static TEST_INPUT: &str = include_str!("../input/20_test.txt");

pub fn a(input: &str) -> i32 {
    let original_file = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut scrambled_file = original_file.clone();

    let mut found_zero = false;
    let mut index = 0;
    let mut count_after_zero = 0;

    let mut sum = 0;

    loop {
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

        let value_to_move = original_file[index % original_file.len()];
        let src_index = scrambled_file
            .iter()
            .enumerate()
            .find(|(_, val)| value_to_move == **val)
            .unwrap()
            .0;

        if value_to_move == 0 {
            found_zero = true;
        }

        let dst_index = {
            let mut dst_index =
                (src_index as i32 + value_to_move) % (original_file.len() as i32 - 1);
            if dst_index <= 0 {
                dst_index += original_file.len() as i32 - 1;
            }
            dst_index as usize
        };

        //println!("value {}, {} -> {}", value_to_move, src_index, dst_index);

        //print!("{:?} => ", &scrambled_file);

        match src_index.cmp(&dst_index) {
            Ordering::Less => {
                scrambled_file[src_index..=dst_index].rotate_left(1);
            }
            Ordering::Greater => {
                scrambled_file[dst_index..=src_index].rotate_right(1);
            }
            Ordering::Equal => {}
        }

        //println!("{:?}", &scrambled_file);

        index += 1;

        if found_zero {
            count_after_zero += 1;
        }

        match count_after_zero {
            997 => {
                println!("{}", value_to_move);
            }
            998 => {
                println!("{}", value_to_move);
            }
            999 => {
                println!("{}", value_to_move);
            }
            1000 => {
                sum += value_to_move;
                println!("1000 {}", value_to_move);
                //assert!(value_to_move == 4);
            }
            1001 => {
                println!("{}", value_to_move);
            }
            1002 => {
                println!("{}", value_to_move);
            }
            1003 => {
                println!("{}", value_to_move);
            }
            2000 => {
                println!("2000 {}", value_to_move);
                sum += value_to_move;
                assert!(value_to_move == -3);
            }
            2001 => {
                println!("2001 {}", value_to_move);
            }
            2002 => {
                println!("2002 {}", value_to_move);
            }
            2003 => {
                println!("2003 {}", value_to_move);
            }
            3000 => {
                println!("3000 {}", value_to_move);
                sum += value_to_move;
                assert!(value_to_move == 2);
                break;
            }
            _ => (),
        }
    }

    sum
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
