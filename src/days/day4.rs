use std::ops::RangeInclusive;

pub static INPUT: &str = include_str!("../input/4.txt");
pub static TEST_INPUT: &str = include_str!("../input/4_test.txt");

fn parse(line: &str) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let (first, second) = line.split_once(',').unwrap();
    let (f1, f2) = first.split_once('-').unwrap();
    let (s1, s2) = second.split_once('-').unwrap();

    (
        f1.parse().unwrap()..=f2.parse().unwrap(),
        s1.parse().unwrap()..=s2.parse().unwrap(),
    )
}

pub fn a(input: &str) -> i32 {
    input
        .lines()
        .map(parse)
        .map(|(first, second)| {
            (first.clone().all(|v| second.contains(&v))
                || second.clone().all(|v| first.contains(&v))) as i32
        })
        .sum::<i32>()
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 2);
    assert_eq!(a(INPUT), 444);
}

pub fn b(input: &str) -> i32 {
    input
        .lines()
        .map(parse)
        .map(|(first, second)| {
            (first.clone().any(|v| second.contains(&v))
                || second.clone().any(|v| first.contains(&v))) as i32
        })
        .sum::<i32>()
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 4);
    assert_eq!(b(INPUT), 801);
}
