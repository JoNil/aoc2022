use parse_display::{Display, FromStr};

pub static INPUT: &str = include_str!("../input/5.txt");
pub static TEST_INPUT: &str = include_str!("../input/5_test.txt");

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("move {count} from {from} to {to}")]
struct Command {
    count: i32,
    from: i32,
    to: i32,
}

struct Yard {
    piles: Vec<Vec<char>>,
}

fn parse(input: &str) -> (Yard, Vec<Command>) {
    let (stack, commands) = input.split_once("\n\n").unwrap();

    let mut stack = stack.lines().collect::<Vec<_>>();

    let stack_numbers = stack
        .pop()
        .unwrap()
        .split("   ")
        .map(|n| n.trim().parse::<i32>().unwrap())
        .max()
        .unwrap();

    let mut piles = Vec::new();
    piles.resize(stack_numbers as _, Vec::new());

    for stack_line in stack.iter().rev() {
        for i in 0..stack_numbers {
            let char_index = 1 + 4 * i;
            let char = stack_line.as_bytes()[char_index as usize] as char;
            if char != ' ' {
                piles[i as usize].push(char);
            }
        }
    }

    let yard = Yard { piles };

    let commands = commands
        .lines()
        .map(|l| l.parse::<Command>().unwrap())
        .collect();

    (yard, commands)
}

pub fn a(input: &str) -> String {
    let (mut yard, commands) = parse(input);

    for command in commands {
        for _ in 0..command.count {
            if let Some(thing) = yard.piles[command.from as usize - 1].pop() {
                yard.piles[command.to as usize - 1].push(thing);
            }
        }
    }

    yard.piles
        .iter_mut()
        .map(|p| p.pop().unwrap_or(' '))
        .collect()
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), "CMZ".to_string());
    assert_eq!(a(INPUT), "WCZTHTMPS".to_string());
}

pub fn b(input: &str) -> String {
    let (mut yard, commands) = parse(input);

    for command in commands {
        let mut things = Vec::new();

        for _ in 0..command.count {
            if let Some(thing) = yard.piles[command.from as usize - 1].pop() {
                things.push(thing);
            }
        }

        for thing in things.iter().rev() {
            yard.piles[command.to as usize - 1].push(*thing);
        }
    }

    yard.piles
        .iter_mut()
        .map(|p| p.pop().unwrap_or(' '))
        .collect()
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), "MCD".to_string());
    assert_eq!(b(INPUT), "BLSGJSDTS".to_string());
}
