use parse_display::FromStr;
use std::{mem, str::FromStr};

pub static INPUT: &str = include_str!("../input/11.txt");
pub static TEST_INPUT: &str = include_str!("../input/11_test.txt");

#[derive(PartialEq, Debug)]
struct Items(Vec<i32>);

impl FromStr for Items {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Items(
            s.split(',')
                .map(|i| i.trim())
                .map(|i| i.parse().unwrap())
                .collect(),
        ))
    }
}

#[derive(FromStr, PartialEq, Debug)]
enum Input {
    #[display("{0}")]
    Const(i32),
    #[display("old")]
    Old,
}

impl Input {
    fn get(&self, old: i32) -> i32 {
        match self {
            Input::Const(c) => *c,
            Input::Old => old,
        }
    }
}

#[derive(FromStr, PartialEq, Debug)]
enum Operation {
    #[display("new = {0} * {1}")]
    Mul(Input, Input),
    #[display("new = {0} + {1}")]
    Add(Input, Input),
}

impl Operation {
    fn apply(&self, old: i32) -> i32 {
        match self {
            Operation::Mul(a, b) => a.get(old) * b.get(old),
            Operation::Add(a, b) => a.get(old) + b.get(old),
        }
    }
}

#[derive(FromStr, PartialEq, Debug)]
#[display(
    "Monkey {index}:
  Starting items: {items}
  Operation: {op}
  Test: divisible by {divisor}
    If true: throw to monkey {on_true}
    If false: throw to monkey {on_false}"
)]
struct Monkey {
    index: i32,
    items: Items,
    op: Operation,
    divisor: i32,
    on_true: i32,
    on_false: i32,
}

pub fn a(input: &str) -> i32 {
    let mut monkeys = input
        .split("\n\n")
        .map(|l| l.parse::<Monkey>().unwrap())
        .collect::<Vec<_>>();

    let mut inspection_count = vec![0; monkeys.len()];

    for _ in 0..20 {
        for monkey_index in 0..monkeys.len() {
            let current_items = mem::replace(&mut monkeys[monkey_index].items, Items(Vec::new()));

            for item in current_items.0 {
                let item = monkeys[monkey_index].op.apply(item) / 3;
                inspection_count[monkey_index] += 1;

                if item % monkeys[monkey_index].divisor == 0 {
                    let new_monkey_index = monkeys[monkey_index].on_true;
                    monkeys[new_monkey_index as usize].items.0.push(item);
                } else {
                    let new_monkey_index = monkeys[monkey_index].on_false;
                    monkeys[new_monkey_index as usize].items.0.push(item);
                }
            }
        }
    }

    inspection_count.sort();
    inspection_count.iter().rev().take(2).product()
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 10605);
    assert_eq!(a(INPUT), 58794);
}

pub fn b(input: &str) -> i64 {
    let mut monkeys = input
        .split("\n\n")
        .map(|l| l.parse::<Monkey>().unwrap())
        .collect::<Vec<_>>();

    let mut inspection_count = vec![0; monkeys.len()];

    for _ in 0..10000 {
        for monkey_index in 0..monkeys.len() {
            let current_items = mem::replace(&mut monkeys[monkey_index].items, Items(Vec::new()));

            for item in current_items.0 {
                let item = monkeys[monkey_index].op.apply(item);
                inspection_count[monkey_index] += 1;

                if item % monkeys[monkey_index].divisor == 0 {
                    let new_monkey_index = monkeys[monkey_index].on_true;
                    monkeys[new_monkey_index as usize].items.0.push(item);
                } else {
                    let new_monkey_index = monkeys[monkey_index].on_false;
                    monkeys[new_monkey_index as usize].items.0.push(item);
                }
            }
        }
    }

    inspection_count.sort();
    inspection_count.iter().rev().take(2).product()
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 2713310158);
    assert_eq!(b(INPUT), 0);
}
