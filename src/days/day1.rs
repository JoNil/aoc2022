pub static INPUT: &str = include_str!("../input/1.txt");
pub static TEST_INPUT: &str = include_str!("../input/1_test.txt");

pub fn a(input: &str) -> i32 {
    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|cal| cal.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .max()
        .unwrap()
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 24000);
    assert_eq!(a(INPUT), 69310);
}

pub fn b(input: &str) -> i32 {
    let mut sum_of_elfs = input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|cal| cal.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<_>>();

    sum_of_elfs.sort_by(|a, b| b.partial_cmp(a).unwrap());
    sum_of_elfs.iter().take(3).sum::<i32>()
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 45000);
    assert_eq!(b(INPUT), 206104);
}
