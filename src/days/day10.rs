use parse_display::{Display, FromStr};
use std::io::Write;

pub static INPUT: &str = include_str!("../input/10.txt");
pub static TEST_INPUT: &str = include_str!("../input/10_test.txt");
pub static TEST_RESULT: &str = include_str!("../input/10_test_result.txt");
pub static RESULT: &str = include_str!("../input/10_result.txt");

#[derive(Display, FromStr, PartialEq, Debug)]
enum Instruction {
    #[display("noop")]
    Nop,
    #[display("addx {0}")]
    Addx(i32),
}

struct Cpu {
    x: i32,
    in_add: bool,
}

impl Cpu {
    fn execute(&mut self, instruction: &Instruction) -> i32 {
        match instruction {
            Instruction::Nop => 1,
            Instruction::Addx(v) => {
                if self.in_add {
                    self.x += *v;
                    self.in_add = false;
                    1
                } else {
                    self.in_add = true;
                    0
                }
            }
        }
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            x: 1,
            in_add: false,
        }
    }
}

pub fn a(input: &str) -> i32 {
    let instructions = input
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect::<Vec<_>>();
    let mut instruction_counter = 0;

    let mut cpu = Cpu::default();

    let mut signal_strength = 0;

    for cycle in 1..=220 {
        if let 20 | 60 | 100 | 140 | 180 | 220 = cycle {
            signal_strength += cycle * cpu.x;
        }

        instruction_counter += cpu.execute(&instructions[instruction_counter as usize]);
    }

    signal_strength
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 13140);
    assert_eq!(a(INPUT), 14420);
}

pub fn b(input: &str) -> String {
    let instructions = input
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect::<Vec<_>>();
    let mut instruction_counter = 0;

    let mut cpu = Cpu::default();

    let mut res = Vec::new();

    for cycle in 0..=239 {
        if let 40 | 80 | 120 | 160 | 200 = cycle {
            writeln!(&mut res).unwrap();
        }

        let crt = cycle % 40;

        if cpu.x == crt || cpu.x - 1 == crt || cpu.x + 1 == crt {
            write!(&mut res, "#").unwrap();
        } else {
            write!(&mut res, ".").unwrap();
        }

        instruction_counter += cpu.execute(&instructions[instruction_counter as usize]);
    }

    String::from_utf8(res).unwrap()
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), TEST_RESULT);
    assert_eq!(b(INPUT), RESULT);
}
