use std::cmp::Ordering;

pub static INPUT: &str = include_str!("../input/20.txt");
pub static TEST_INPUT: &str = include_str!("../input/20_test.txt");

pub fn a(input: &str) -> i32 {
    let mut file = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut index_to_pos = (0..file.len()).collect::<Vec<_>>();
    let mut pos_to_index = (0..file.len()).collect::<Vec<_>>();

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
            assert_eq!(file, test);
        }

        let src_index = index_to_pos[index % index_to_pos.len()];
        println!("index {} -> {}", index % index_to_pos.len(), src_index);
        let value = file[src_index];

        if value == 0 {
            found_zero = true;
        }

        let dst_index = {
            let mut dst_index = (src_index as i32 + value) % (file.len() as i32 - 1);
            if dst_index <= 0 {
                dst_index += file.len() as i32 - 1;
            }
            dst_index as usize
        };

        println!("value {}, {} -> {}", value, src_index, dst_index,);

        println!("{:?}, {:?}, {:?} =>", &file, &index_to_pos, &pos_to_index);

        match src_index.cmp(&dst_index) {
            Ordering::Less => {
                file[src_index..=dst_index].rotate_left(1);

                for pos in src_index..=dst_index {
                    let index = pos_to_index[pos];
                    if pos == src_index {
                        index_to_pos[index] += dst_index - src_index;
                    } else {
                        index_to_pos[index] -= 1;
                    }
                }

                pos_to_index[src_index..=dst_index].rotate_left(1);
            }
            Ordering::Greater => {
                file[dst_index..=src_index].rotate_right(1);

                for pos in dst_index..=src_index {
                    let index = pos_to_index[pos];
                    if pos == dst_index {
                        index_to_pos[index] += src_index - dst_index;
                    } else {
                        index_to_pos[index] -= 1;
                    }
                }

                pos_to_index[dst_index..=src_index].rotate_right(1);
            }
            Ordering::Equal => {}
        }

        println!("{:?}, {:?}, {:?} =>", &file, &index_to_pos, &pos_to_index);

        for i in 0..file.len() {
            assert!(index_to_pos.iter().filter(|p| **p == i).count() == 1);
        }

        match count_after_zero {
            1000 => {
                sum += value;
                assert!(value == 4);
            }
            2000 => {
                sum += value;
                assert!(value == -3);
            }
            3000 => {
                sum += value;
                assert!(value == 2);
                break;
            }
            _ => (),
        }

        index += 1;

        if found_zero {
            count_after_zero += 1;
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
