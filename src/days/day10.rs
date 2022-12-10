use parse_display::{Display, FromStr};

pub static INPUT: &str = include_str!("../input/10.txt");
pub static TEST_INPUT: &str = include_str!("../input/10_test.txt");

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
        match cycle {
            20 | 60 | 100 | 140 | 180 | 220 => {
                signal_strength += cycle * cpu.x;
            }
            _ => {}
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

pub fn b(input: &str) -> i32 {
    0
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 0);
    assert_eq!(b(INPUT), 0);
}
