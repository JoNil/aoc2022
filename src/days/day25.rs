use std::str::FromStr;

pub static INPUT: &str = include_str!("../input/25.txt");
pub static TEST_INPUT: &str = include_str!("../input/25_test.txt");

fn char_to_snafo(c: char) -> i32 {
    match c {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => panic!("Bad digit"),
    }
}

#[derive(Debug)]
struct Snafo {
    digits: Vec<i32>,
}

impl Snafo {
    fn as_i32(&self) -> i32 {
        let mut res = 0;
        let mut factor = 0;

        for (position, digit) in self.digits.iter().rev().enumerate() {
            res += 5i32.pow(position as u32) * *digit;
        }

        res
    }
}

impl FromStr for Snafo {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Snafo {
            digits: s.chars().map(char_to_snafo).collect::<Vec<_>>(),
        })
    }
}

pub fn a(input: &str) -> String {
    let numbers = input
        .lines()
        .map(|line| line.parse::<Snafo>().unwrap())
        .collect::<Vec<_>>();

    for number in &numbers {
        println!("{:?} => {}", number, number.as_i32());
    }

    let sum = numbers.iter().map(|s| s.as_i32()).sum::<i32>();

    println!("{sum}");

    String::new()
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), "2=-1=0".to_string());
    assert_eq!(a(INPUT), String::new());
}

pub fn b(input: &str) -> String {
    String::new()
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), String::new());
    assert_eq!(b(INPUT), String::new());
}
