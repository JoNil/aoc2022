use std::str::FromStr;

pub static INPUT: &str = include_str!("../input/25.txt");
pub static TEST_INPUT: &str = include_str!("../input/25_test.txt");

fn char_to_snafo(c: char) -> i64 {
    match c {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => panic!("Bad digit"),
    }
}

fn divisor_for_digit(digit: i64) -> i64 {
    5i64.pow(digit as u32) - (0..digit).map(|f| 2 * 5i64.pow(f as u32)).sum::<i64>()
}

fn max_all_digits(digit: i64) -> i64 {
    (0..digit).map(|f| 2 * 5i64.pow(f as u32)).sum::<i64>()
}

#[derive(Debug)]
struct Snafu {
    digits: Vec<i64>,
}

impl Snafu {
    fn from_i64(mut v: i64) -> Self {
        let mut digits = Vec::new();

        let mut largest_digit = 0;

        for digit in 0.. {
            if v / divisor_for_digit(digit) == 0 {
                break;
            }

            largest_digit = digit;
        }

        for digit in (0..=largest_digit).rev() {
            let two = 2 * 5i64.pow(digit as u32) - max_all_digits(digit);
            let one = 5i64.pow(digit as u32) - max_all_digits(digit);

            let n = v.signum()
                * if v.abs() >= two {
                    2
                } else {
                    (v.abs() >= one) as i64
                };

            v -= n * 5i64.pow(digit as u32);

            digits.push(n);
        }

        Snafu { digits }
    }

    fn as_i64(&self) -> i64 {
        let mut res = 0;

        for (position, digit) in self.digits.iter().rev().enumerate() {
            res += 5i64.pow(position as u32) * *digit;
        }

        res
    }
}

#[test]
fn test_from_i64() {
    assert_eq!("=-0-2".parse::<Snafu>().unwrap().as_i64(), -1378);
    assert_eq!("-0-2".parse::<Snafu>().unwrap().as_i64(), -128);
    assert_eq!(Snafu::from_i64(-3).to_string(), "-2".to_string());
    assert_eq!(Snafu::from_i64(-128).to_string(), "-0-2".to_string());
    assert_eq!(Snafu::from_i64(1747).to_string(), "1=-0-2".to_string());
    assert_eq!(Snafu::from_i64(906).to_string(), "12111".to_string());
    assert_eq!(Snafu::from_i64(198).to_string(), "2=0=".to_string());
}

impl ToString for Snafu {
    fn to_string(&self) -> String {
        let mut res = String::new();
        for n in &self.digits {
            res.push(match n {
                2 => '2',
                1 => '1',
                0 => '0',
                -1 => '-',
                -2 => '=',
                _ => panic!("Bad num"),
            });
        }
        res
    }
}

impl FromStr for Snafu {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Snafu {
            digits: s.chars().map(char_to_snafo).collect::<Vec<_>>(),
        })
    }
}

pub fn a(input: &str) -> String {
    let numbers = input
        .lines()
        .map(|line| line.parse::<Snafu>().unwrap())
        .collect::<Vec<_>>();

    for number in &numbers {
        let n = number.as_i64();
        let s = Snafu::from_i64(n);
        assert_eq!(number.to_string(), s.to_string());
    }

    let sum = numbers.iter().map(|s| s.as_i64()).sum::<i64>();

    Snafu::from_i64(sum).to_string()
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), "2=-1=0".to_string());
    assert_eq!(a(INPUT), "2=-0=01----22-0-1-10".to_string());
}

pub fn b(input: &str) -> String {
    String::new()
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), String::new());
    assert_eq!(b(INPUT), String::new());
}
