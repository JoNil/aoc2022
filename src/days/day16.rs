#![allow(clippy::comparison_chain)]

use parse_display::FromStr;
use pathfinding::prelude::dijkstra;
use std::{collections::HashMap, fmt::Display, hash::Hash, iter, str::FromStr};

pub static INPUT: &str = include_str!("../input/16.txt");
pub static TEST_INPUT: &str = include_str!("../input/16_test.txt");

struct Tunnels(Vec<String>);

impl FromStr for Tunnels {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Tunnels(
            s.trim_start_matches("tunnels lead to valves")
                .trim_start_matches("tunnel leads to valve")
                .split(',')
                .map(|i| i.trim())
                .map(|i| i.to_string())
                .collect(),
        ))
    }
}

#[derive(FromStr)]
#[display("Valve {name} has flow rate={rate}; {tunnels}")]
struct RawValve {
    name: String,
    rate: i32,
    tunnels: Tunnels,
}

struct Valve {
    rate: i32,
    tunnels: Vec<i32>,
}

fn parse_input(input: &str) -> (Vec<Valve>, i32) {
    let raw_valves = input
        .lines()
        .map(|l| l.parse::<RawValve>().unwrap())
        .collect::<Vec<_>>();

    (
        raw_valves
            .iter()
            .map(|rv| Valve {
                rate: rv.rate,
                tunnels: rv
                    .tunnels
                    .0
                    .iter()
                    .map(|t| raw_valves.iter().position(|rw| *t == rw.name).unwrap() as i32)
                    .collect(),
            })
            .collect(),
        raw_valves.iter().position(|rv| rv.name == "AA").unwrap() as i32,
    )
}

fn find_all_paths(valves: &[Valve]) -> HashMap<(i32, i32), i32> {
    let mut all_paths = HashMap::new();

    for v1 in 0..(valves.len() as i32) {
        for v2 in (v1 + 1)..(valves.len() as i32) {
            let path = dijkstra(
                &v1,
                |s| {
                    valves
                        .get(*s as usize)
                        .unwrap()
                        .tunnels
                        .iter()
                        .map(|t| (*t, 1))
                },
                |&c| c == v2,
            )
            .unwrap();
            all_paths.insert((v1, v2), path.1);
            all_paths.insert((v2, v1), path.1);
        }
    }
    all_paths
}

#[derive(Copy, Clone, Hash, Debug, Eq, PartialEq)]
struct State {
    time: i32,
    location: i32,
    remaining: u64,
    rate: i32,
    total: i32,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "time: {}, loc: {}, rem: 0b{:064b}, rate: {}, tot: {}",
            self.time, self.location, self.remaining, self.rate, self.total
        )
    }
}

fn solve(
    valves: &[Valve],
    all_paths: &HashMap<(i32, i32), i32>,
    start_index: i32,
    interesting_paths: u64,
    max_t: i32,
) -> i32 {
    let total_rate = valves.iter().map(|v| v.rate).sum::<i32>();

    let start = State {
        time: 0,
        location: start_index,
        remaining: interesting_paths,
        rate: 0,
        total: 0,
    };

    let result = dijkstra(
        &start,
        |&s| {
            //println!("{}", s);

            (0..(valves.len() as i32))
                .filter(move |i| (s.remaining & (1 << i)) > 0)
                .map(move |candidate| {
                    let steps = *all_paths.get(&(s.location, candidate)).unwrap();
                    let candidate_rate = valves.get(candidate as usize).unwrap().rate;
                    (
                        State {
                            time: s.time + steps + 1,
                            location: candidate,
                            remaining: s.remaining & !(1 << candidate),
                            rate: s.rate + candidate_rate,
                            total: s.total + (steps + 1) * s.rate,
                        },
                        steps * (total_rate - s.rate) + total_rate - (s.rate + candidate_rate),
                    )
                })
                .chain(iter::once((
                    State {
                        time: s.time + 1,
                        location: s.location,
                        remaining: s.remaining,
                        rate: s.rate,
                        total: s.total + s.rate,
                    },
                    total_rate - s.rate,
                )))
        },
        |s| s.time == max_t,
    )
    .unwrap();

    result.0.last().unwrap().total
}

pub fn a(input: &str) -> i32 {
    let (valves, start_index) = parse_input(input);
    let all_paths = find_all_paths(&valves);

    let remaining = valves
        .iter()
        .enumerate()
        .filter(|(id, _)| *id as i32 != start_index)
        .filter(|(_, v)| v.rate != 0)
        .map(|(id, _)| 1 << id as i32)
        .sum::<u64>();

    solve(&valves, &all_paths, start_index, remaining, 30)
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 1651);
    assert_eq!(a(INPUT), 2119);
}

pub fn b(input: &str) -> i32 {
    let (valves, start_index) = parse_input(input);

    0
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 1707);
    //assert_eq!(b(INPUT), 0);
}
