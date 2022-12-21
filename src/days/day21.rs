use std::collections::HashMap;

use parse_display::FromStr;

pub static INPUT: &str = include_str!("../input/21.txt");
pub static TEST_INPUT: &str = include_str!("../input/21_test.txt");

#[derive(FromStr, Debug)]
enum Operation {
    #[display("{0}")]
    Num(i32),
    #[display("{1} {0} {2}")]
    Op(char, String, String),
}

impl Operation {
    fn eval(&self, refs: &HashMap<&str, Operation>) -> i128 {
        match &self {
            Operation::Num(val) => *val as i128,
            Operation::Op(op, a, b) => {
                let a = refs.get(a.as_str()).unwrap().eval(refs);
                let b = refs.get(b.as_str()).unwrap().eval(refs);

                match op {
                    '+' => a + b,
                    '-' => a - b,
                    '*' => a * b,
                    '/' => a / b,
                    _ => panic!("Unknown op {op}"),
                }
            }
        }
    }
}

pub fn a(input: &str) -> i128 {
    let ops = input
        .lines()
        .map(|line| {
            let (name, op) = line.split_once(':').unwrap();
            (name, op.trim().parse::<Operation>().unwrap())
        })
        .collect::<HashMap<_, _>>();

    ops.get("root").unwrap().eval(&ops)
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 152);
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
