use std::collections::HashSet;

pub static INPUT: &str = include_str!("../input/3.txt");
pub static TEST_INPUT: &str = include_str!("../input/3_test.txt");

pub fn a(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let (first, second) = line.split_at(line.len() / 2);

            let first_set = first.chars().collect::<HashSet<char>>();
            let second_set = second.chars().collect::<HashSet<char>>();

            *first_set.intersection(&second_set).next().unwrap()
        })
        .map(|c| match c {
            'a'..='z' => c as i32 - 'a' as i32 + 1,
            'A'..='Z' => c as i32 - 'A' as i32 + 27,
            _ => {
                panic!("Out of range");
            }
        })
        .sum::<i32>()
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 157);
    assert_eq!(a(INPUT), 7980);
}

pub fn b(input: &str) -> i32 {
    let all_lines = input.lines().collect::<Vec<_>>();

    all_lines
        .chunks_exact(3)
        .map(|group| {
            let second_set = group[1].chars().collect::<HashSet<char>>();
            let third_set = group[2].chars().collect::<HashSet<char>>();

            group[0]
                .chars()
                .find(|c| second_set.contains(c) && third_set.contains(c))
                .unwrap()
        })
        .map(|c| match c {
            'a'..='z' => c as i32 - 'a' as i32 + 1,
            'A'..='Z' => c as i32 - 'A' as i32 + 27,
            _ => {
                panic!("Out of range");
            }
        })
        .sum::<i32>()
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 70);
    assert_eq!(b(INPUT), 2881);
}
