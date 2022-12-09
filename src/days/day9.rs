use glam::ivec2;

pub static INPUT: &str = include_str!("../input/9.txt");
pub static TEST_INPUT: &str = include_str!("../input/9_test.txt");

pub fn a(input: &str) -> i32 {
    let head = ivec2(0, 0);
    let tail = ivec2(0, 0);

    for line in input.lines() {
        match line
            .split_once(' ')
            .map(|(d, s)| (d, s.parse::<i32>().unwrap()))
            .unwrap()
        {
            ("R", s) => {}
            ("L", s) => {}
            ("U", s) => {}
            ("D", s) => {}
            _ => panic!(),
        }
    }

    0
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 13);
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
