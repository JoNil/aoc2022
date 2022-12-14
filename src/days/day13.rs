use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{char, one_of},
    combinator::{cut, map, map_res, opt, recognize},
    error::{context, ContextError, FromExternalError, ParseError, VerboseError},
    multi::{many1, separated_list0},
    sequence::{delimited, preceded, terminated},
    IResult,
};
use std::cmp::Ordering;

pub static INPUT: &str = include_str!("../input/13.txt");
pub static TEST_INPUT: &str = include_str!("../input/13_test.txt");

fn sp<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n";
    take_while(move |c| chars.contains(c))(i)
}

fn number<'a, E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>>(
    i: &'a str,
) -> IResult<&'a str, i32, E> {
    alt((
        map_res(recognize(many1(one_of("0123456789"))), |digit_str: &str| {
            digit_str.parse::<i32>()
        }),
        map_res(
            preceded(tag("-"), recognize(many1(one_of("0123456789")))),
            |digit_str: &str| digit_str.parse::<i32>().map(|v| -v),
        ),
    ))(i)
}

fn list<
    'a,
    E: ParseError<&'a str>
        + ContextError<&'a str>
        + FromExternalError<&'a str, std::num::ParseIntError>,
>(
    i: &'a str,
) -> IResult<&'a str, Vec<Value>, E> {
    context(
        "list",
        preceded(
            char('['),
            cut(terminated(
                separated_list0(preceded(sp, char(',')), value),
                preceded(sp, char(']')),
            )),
        ),
    )(i)
}

fn value<
    'a,
    E: ParseError<&'a str>
        + ContextError<&'a str>
        + FromExternalError<&'a str, std::num::ParseIntError>,
>(
    i: &'a str,
) -> IResult<&'a str, Value, E> {
    preceded(sp, alt((map(list, Value::Vec), map(number, Value::I32))))(i)
}

fn root<
    'a,
    E: ParseError<&'a str>
        + ContextError<&'a str>
        + FromExternalError<&'a str, std::num::ParseIntError>,
>(
    i: &'a str,
) -> IResult<&'a str, Value, E> {
    delimited(sp, value, opt(sp))(i)
}

fn parse_packet(line: &str) -> Value {
    root::<VerboseError<&str>>(line).unwrap().1
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Value {
    I32(i32),
    Vec(Vec<Value>),
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match (self, other) {
            (Value::I32(a), Value::I32(b)) => a.cmp(b),
            (a @ Value::I32(_), b @ Value::Vec(_)) => {
                Value::Vec(vec![a.clone()]).partial_cmp(b).unwrap()
            }
            (a @ Value::Vec(_), b @ Value::I32(_)) => {
                a.partial_cmp(&Value::Vec(vec![b.clone()])).unwrap()
            }
            (Value::Vec(a), Value::Vec(b)) => {
                for (a, b) in a.iter().zip(b.iter()) {
                    match a.partial_cmp(b).unwrap() {
                        Ordering::Less => return Some(Ordering::Less),
                        Ordering::Equal => continue,
                        Ordering::Greater => return Some(Ordering::Greater),
                    }
                }
                a.len().cmp(&b.len())
            }
        })
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn a(input: &str) -> i32 {
    input
        .split("\n\n")
        .map(|p| {
            let (a, b) = p.split_once('\n').unwrap();
            (parse_packet(a), parse_packet(b))
        })
        .enumerate()
        .filter_map(|(i, (a, b))| if a < b { Some(i + 1) } else { None })
        .sum::<usize>() as i32
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 13);
    assert_eq!(a(INPUT), 5720);
}

pub fn b(input: &str) -> i32 {
    let div_a = Value::Vec(vec![Value::Vec(vec![Value::I32(2)])]);
    let div_b = Value::Vec(vec![Value::Vec(vec![Value::I32(6)])]);

    let mut packets = input
        .split('\n')
        .filter(|l| !l.trim().is_empty())
        .map(parse_packet)
        .collect::<Vec<_>>();

    packets.push(div_a.clone());
    packets.push(div_b.clone());

    packets.sort();

    (packets
        .iter()
        .enumerate()
        .find(|(_, p)| *p == &div_a)
        .unwrap()
        .0 as i32
        + 1)
        * (packets
            .iter()
            .enumerate()
            .find(|(_, p)| *p == &div_b)
            .unwrap()
            .0 as i32
            + 1)
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 140);
    assert_eq!(b(INPUT), 23504);
}
