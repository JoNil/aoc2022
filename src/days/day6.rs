use std::collections::HashSet;

pub static INPUT: &str = include_str!("../input/6.txt");
pub static TEST_INPUT_1: &str = include_str!("../input/6_test_1.txt");
pub static TEST_INPUT_2: &str = include_str!("../input/6_test_2.txt");
pub static TEST_INPUT_3: &str = include_str!("../input/6_test_3.txt");
pub static TEST_INPUT_4: &str = include_str!("../input/6_test_4.txt");
pub static TEST_INPUT_5: &str = include_str!("../input/6_test_5.txt");

pub fn a(input: &str) -> i32 {
    input
        .as_bytes()
        .windows(4)
        .enumerate()
        .find(|(_, w)| w.iter().collect::<HashSet<_>>().len() == 4)
        .map(|(count, _)| count + 4)
        .unwrap() as i32
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT_1), 5);
    assert_eq!(a(TEST_INPUT_2), 6);
    assert_eq!(a(TEST_INPUT_3), 10);
    assert_eq!(a(TEST_INPUT_4), 11);
    assert_eq!(a(TEST_INPUT_5), 7);
    assert_eq!(a(INPUT), 1175);
}

pub fn b(input: &str) -> i32 {
    input
        .as_bytes()
        .windows(14)
        .enumerate()
        .find(|(_, w)| w.iter().collect::<HashSet<_>>().len() == 14)
        .map(|(count, _)| count + 14)
        .unwrap() as i32
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT_1), 23);
    assert_eq!(b(TEST_INPUT_2), 23);
    assert_eq!(b(TEST_INPUT_3), 29);
    assert_eq!(b(TEST_INPUT_4), 26);
    assert_eq!(b(TEST_INPUT_5), 19);
    assert_eq!(b(INPUT), 3217);
}
