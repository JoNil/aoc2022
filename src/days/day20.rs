pub static INPUT: &str = include_str!("../input/20.txt");
pub static TEST_INPUT: &str = include_str!("../input/20_test.txt");
pub static TORKEL_INPUT: &str = include_str!("../input/20_torkel.txt");

fn modulo(a: i64, b: usize) -> usize {
    let mut res = a % b as i64;
    while res < 0 {
        res += b as i64;
    }
    res as usize
}

#[test]
fn test_modulo() {
    assert_eq!(modulo(8, 4), 0);
    assert_eq!(modulo(9, 4), 1);
    assert_eq!(modulo(-1, 4), 3);
    assert_eq!(modulo(-2, 4), 2);
}

pub fn a(input: &str) -> i32 {
    let mut i = 0;
    let mut numbers = input
        .lines()
        .map(|s| {
            i += 1;
            (s.parse::<i64>().unwrap(), i)
        })
        .collect::<Vec<_>>();

    for i in 1..=i {
        let index = numbers
            .iter()
            .enumerate()
            .find(|(_, (_, index))| i == *index)
            .unwrap()
            .0;

        let n = numbers.remove(index);
        let pos = modulo(n.0 + index as i64, numbers.len());
        numbers.insert(pos as usize, (n.0, i));
    }

    let zero_index = numbers
        .iter()
        .enumerate()
        .find(|(_, v)| v.0 == 0)
        .unwrap()
        .0;

    let a = numbers[(zero_index + 1000) % numbers.len()].0;
    let b = numbers[(zero_index + 2000) % numbers.len()].0;
    let c = numbers[(zero_index + 3000) % numbers.len()].0;

    (a + b + c) as i32
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 3);
    assert_eq!(a(TORKEL_INPUT), 9866);
    assert_eq!(a(INPUT), 3700);
}

pub fn b(input: &str) -> i64 {
    let mut i = 0;
    let mut numbers = input
        .lines()
        .map(|s| {
            i += 1;
            (s.parse::<i64>().unwrap() * 811589153, i)
        })
        .collect::<Vec<_>>();

    for _ in 0..10 {
        for i in 1..=i {
            let index = numbers
                .iter()
                .enumerate()
                .find(|(_, (_, index))| i == *index)
                .unwrap()
                .0;

            let n = numbers.remove(index);
            let pos = modulo(n.0 + index as i64, numbers.len());
            numbers.insert(pos as usize, (n.0, i));
        }
    }

    let zero_index = numbers
        .iter()
        .enumerate()
        .find(|(_, v)| v.0 == 0)
        .unwrap()
        .0;

    let a = numbers[(zero_index + 1000) % numbers.len()].0;
    let b = numbers[(zero_index + 2000) % numbers.len()].0;
    let c = numbers[(zero_index + 3000) % numbers.len()].0;

    a + b + c
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 1623178306);
    assert_eq!(b(INPUT), 0);
}
