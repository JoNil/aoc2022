use std::collections::HashMap;

use parse_display::FromStr;

pub static INPUT: &str = include_str!("../input/21.txt");
pub static TEST_INPUT: &str = include_str!("../input/21_test.txt");

#[derive(FromStr, Debug, Clone)]
enum Operation {
    #[display("{0}")]
    Num(i32),
    #[display("{1} {0} {2}")]
    Op(char, String, String),
    #[display("")]
    Unknown(),
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
            Operation::Unknown() => {
                panic!("Unknown");
            }
        }
    }

    fn has_unknown(&self, refs: &HashMap<&str, Operation>) -> bool {
        match &self {
            Operation::Num(_) => false,
            Operation::Op(_, a, b) => {
                refs.get(a.as_str()).unwrap().has_unknown(refs)
                    || refs.get(b.as_str()).unwrap().has_unknown(refs)
            }
            Operation::Unknown() => true,
        }
    }

    fn solve_layer(
        &self,
        refs: &HashMap<&str, Operation>,
        current_equality: i128,
    ) -> Option<(i128, Operation)> {
        match self {
            Operation::Num(_) => panic!("Cant strip num"),
            Operation::Op(op, a, b) => {
                let a_has_unknown = refs.get(a.as_str()).unwrap().has_unknown(refs);

                let next_unknown = if a_has_unknown {
                    refs.get(a.as_str()).unwrap()
                } else {
                    refs.get(b.as_str()).unwrap()
                };

                let other = if a_has_unknown {
                    refs.get(b.as_str()).unwrap().eval(refs)
                } else {
                    refs.get(a.as_str()).unwrap().eval(refs)
                };

                Some(match op {
                    '+' => (current_equality - other, next_unknown.clone()),
                    '-' => {
                        if a_has_unknown {
                            (current_equality + other, next_unknown.clone())
                        } else {
                            (-(current_equality - other), next_unknown.clone())
                        }
                    }
                    '*' => (current_equality / other, next_unknown.clone()),
                    '/' => {
                        if a_has_unknown {
                            (current_equality * other, next_unknown.clone())
                        } else {
                            (other / current_equality, next_unknown.clone())
                        }
                    }
                    _ => panic!("Unknown op {op}"),
                })
            }
            Operation::Unknown() => None,
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
    assert_eq!(a(INPUT), 353837700405464);
}

pub fn b(input: &str) -> i128 {
    let ops = input
        .lines()
        .map(|line| {
            let (name, op) = line.split_once(':').unwrap();
            if name == "humn" {
                (name, Operation::Unknown())
            } else {
                (name, op.trim().parse::<Operation>().unwrap())
            }
        })
        .collect::<HashMap<_, _>>();

    let Operation::Op(_, root_a, root_b) = ops.get("root").unwrap() else {
        panic!("Bad root");
    };

    let a = ops.get(root_a.as_str()).unwrap();
    let b = ops.get(root_b.as_str()).unwrap();

    let (mut a, mut b) = if a.has_unknown(&ops) {
        (b.eval(&ops), a.clone())
    } else {
        (a.eval(&ops), b.clone())
    };

    while let Some((new_a, new_b)) = b.solve_layer(&ops, a) {
        a = new_a;
        b = new_b;
    }

    a
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 301);
    assert_eq!(b(INPUT), 3678125408017);
}
