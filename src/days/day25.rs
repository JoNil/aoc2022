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

fn divisor_for_digit(digit: i32) -> i32 {
    5i32.pow(digit as u32) - (0..digit).map(|f| 2 * 5i32.pow(f as u32)).sum::<i32>()
}

fn max_all_digits(digit: i32) -> i32 {
    (0..digit).map(|f| 2 * 5i32.pow(f as u32)).sum::<i32>()
}

#[derive(Debug)]
struct Snafu {
    digits: Vec<i32>,
}

impl Snafu {
    fn from_i32(mut v: i32) -> Self {
        println!("==== {v}");
        let mut digits = Vec::new();

        let mut largest_digit = 0;

        for digit in 0.. {
            if v / divisor_for_digit(digit) == 0 {
                break;
            }

            largest_digit = digit;
        }

        for digit in (0..=largest_digit).rev() {
            let two = 2 * 5i32.pow(digit as u32) - max_all_digits(digit);
            let one = 5i32.pow(digit as u32) - max_all_digits(digit);

            let n = v.signum()
                * if v.abs() >= two {
                    2
                } else {
                    (v.abs() >= one) as i32
                };

            println!("{v} {} {} => {}", two, one, n);

            v -= n * 5i32.pow(digit as u32);

            digits.push(n);
        }

        Snafu { digits }
    }

    fn as_i32(&self) -> i32 {
        let mut res = 0;

        for (position, digit) in self.digits.iter().rev().enumerate() {
            res += 5i32.pow(position as u32) * *digit;
        }

        res
    }
}

#[test]
fn test_from_i32() {
    assert_eq!("=-0-2".parse::<Snafu>().unwrap().as_i32(), -1378);
    assert_eq!("-0-2".parse::<Snafu>().unwrap().as_i32(), -128);
    assert_eq!(Snafu::from_i32(-3).to_string(), "-2".to_string());
    assert_eq!(Snafu::from_i32(-128).to_string(), "-0-2".to_string());
    assert_eq!(Snafu::from_i32(1747).to_string(), "1=-0-2".to_string());
    assert_eq!(Snafu::from_i32(906).to_string(), "12111".to_string());
    assert_eq!(Snafu::from_i32(198).to_string(), "2=0=".to_string());
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
        let n = number.as_i32();
        let s = Snafu::from_i32(n);
        println!("{} => {}", s.as_i32(), s.to_string());
        assert_eq!(number.to_string(), s.to_string());
    }

    let sum = numbers.iter().map(|s| s.as_i32()).sum::<i32>();

    Snafu::from_i32(sum).to_string()
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), "2=-1=0".to_string());
    //assert_eq!(a(INPUT), String::new());
}

pub fn b(input: &str) -> String {
    String::new()
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), String::new());
    assert_eq!(b(INPUT), String::new());
}
