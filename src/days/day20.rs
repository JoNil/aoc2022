pub static INPUT: &str = include_str!("../input/20.txt");
pub static TEST_INPUT: &str = include_str!("../input/20_test.txt");

pub fn a(input: &str) -> i32 {
    let mut file = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut original_index = (0..file.len()).collect::<Vec<_>>();

    let mut found_zero = false;
    let mut index = 0;
    let mut count_after_zero = 0;

    let mut sum = 0;

    loop {
        let src_index = original_index[index % original_index.len()];
        let value = file[src_index];

        if value == 0 {
            found_zero = true;
        }

        let dst_index = ((src_index as i32 + value) % file.len() as i32).unsigned_abs() as usize;

        println!("{:?}", &file);

        if src_index <= dst_index {
            file[src_index..(dst_index + 1)].rotate_left(1);
            original_index[src_index..(dst_index + 1)].rotate_left(1);
        } else {
            file[dst_index..(src_index + 1)].rotate_right(1);
            original_index[dst_index..(src_index + 1)].rotate_right(1);
        }

        match count_after_zero {
            1000 => {
                sum += value;
                println!("{}", value);
            }
            2000 => {
                sum += value;
                println!("{}", value);
            }
            3000 => {
                sum += value;
                println!("{}", value);
                break;
            }
            _ => (),
        }

        if index == 10 {
            panic!("end");
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
