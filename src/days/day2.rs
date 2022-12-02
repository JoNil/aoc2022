pub static INPUT: &str = include_str!("../input/2.txt");
pub static TEST_INPUT: &str = include_str!("../input/2_test.txt");

pub fn a(input: &str) -> i32 {
    input
        .lines()
        .map(|line| match line {
            "A X" => 1 + 3,
            "A Y" => 2 + 6,
            "A Z" => 3,
            "B X" => 1,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 1 + 6,
            "C Y" => 2,
            "C Z" => 3 + 3,
            _ => panic!("{line}"),
        })
        .sum::<i32>()
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 15);
    assert_eq!(a(INPUT), 13005);
}

pub fn b(input: &str) -> i32 {
    input
        .lines()
        .map(|line| match line {
            "A X" => 3,
            "A Y" => 1 + 3,
            "A Z" => 2 + 6,
            "B X" => 1,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 2,
            "C Y" => 3 + 3,
            "C Z" => 1 + 6,
            _ => panic!("{line}"),
        })
        .sum::<i32>()
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 12);
    assert_eq!(b(INPUT), 11373);
}
