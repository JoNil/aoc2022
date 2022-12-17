#![allow(clippy::comparison_chain)]

use parse_display::FromStr;
use pathfinding::prelude::dijkstra;
use std::{collections::HashMap, hash::Hash, str::FromStr};

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

#[derive(Debug, Eq)]
struct Valve {
    id: i32,
    rate: i32,
    tunnels: Vec<i32>,
}

impl Hash for Valve {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Valve {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

fn parse_input(input: &str) -> (Vec<Valve>, i32) {
    let raw_valves = input
        .lines()
        .map(|l| l.parse::<RawValve>().unwrap())
        .collect::<Vec<_>>();

    (
        raw_valves
            .iter()
            .enumerate()
            .map(|(id, rv)| Valve {
                id: id as i32,
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

fn find_all_paths(
    valves: &[Valve],
    start_index: i32,
    interesting_paths: &[i32],
) -> HashMap<(i32, i32), i32> {
    let mut all_paths = HashMap::new();

    let all_interesting_valves = [start_index]
        .into_iter()
        .chain(interesting_paths.iter().copied())
        .collect::<Vec<_>>();

    for (i, v1) in all_interesting_valves.iter().copied().enumerate() {
        for v2 in all_interesting_valves[i + 1..].iter().copied() {
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

#[derive(Clone, Hash, Debug, Eq, PartialEq)]
struct State {
    time: i32,
    location: i32,
    remaining: Vec<i32>,
    rate: i32,
    total: i32,
}

fn solve(valves: &[Valve], start_index: i32, interesting_paths: Vec<i32>, max_t: i32) -> i32 {
    let all_paths = find_all_paths(valves, start_index, &interesting_paths);

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
        |s| {
            let mut candidates = Vec::new();

            if s.time == max_t {
                return candidates.into_iter();
            }

            for candidate in &s.remaining {
                let steps = *all_paths.get(&(s.location, *candidate)).unwrap();
                let candidate_rate = valves.get(*candidate as usize).unwrap().rate;

                candidates.push((
                    State {
                        time: s.time + steps + 1,
                        location: *candidate,
                        remaining: s
                            .remaining
                            .iter()
                            .cloned()
                            .filter(|r| *r != *candidate)
                            .collect(),
                        rate: s.rate + candidate_rate,
                        total: s.total + (steps + 1) * s.rate,
                    },
                    steps * (total_rate - s.rate) + total_rate - (s.rate + candidate_rate),
                ));
            }

            candidates.push((
                State {
                    time: s.time + 1,
                    location: s.location,
                    remaining: s.remaining.clone(),
                    rate: s.rate,
                    total: s.total + s.rate,
                },
                total_rate - s.rate,
            ));

            candidates.into_iter()
        },
        |s| s.time == max_t,
    )
    .unwrap();

    result.0.last().unwrap().total
}

pub fn a(input: &str) -> i32 {
    let (valves, start_index) = parse_input(input);

    let remaining = valves
        .iter()
        .filter(|v| v.id != start_index)
        .filter(|v| v.rate != 0)
        .map(|c| c.id)
        .collect::<Vec<_>>();

    solve(&valves, start_index, remaining, 30)
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
