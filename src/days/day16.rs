use parse_display::FromStr;
use pathfinding::prelude::dijkstra;
use std::{collections::HashMap, hash::Hash, str::FromStr};

pub static INPUT: &str = include_str!("../input/16.txt");
pub static TEST_INPUT: &str = include_str!("../input/16_test.txt");

#[derive(Clone, PartialEq, Debug, Hash, Eq)]
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

#[derive(Clone, FromStr, PartialEq, Debug, Hash, Eq)]
#[display("Valve {name} has flow rate={rate}; {tunnels}")]
struct Valve {
    name: String,
    rate: i32,
    tunnels: Tunnels,
}

#[derive(Clone, Hash, Debug, Eq, PartialEq)]
struct State {
    time: i32,
    location: String,
    remaining: Vec<String>,
    rate: i32,
    total: i32,
}

pub fn a(input: &str) -> i32 {
    let valves = input
        .lines()
        .map(|l| l.parse::<Valve>().unwrap())
        .map(|v| (v.name.clone(), v))
        .collect::<HashMap<_, _>>();

    let all_paths = {
        let mut all_paths = HashMap::new();

        let all_valves = valves.values().map(|v| v.name.clone()).collect::<Vec<_>>();

        for (i, v1) in all_valves.iter().enumerate() {
            for v2 in &all_valves[i + 1..] {
                let path = dijkstra(
                    v1,
                    |s| {
                        valves
                            .get(s)
                            .unwrap()
                            .tunnels
                            .0
                            .iter()
                            .map(|c| (c.clone(), 1))
                    },
                    |s| s == v2,
                )
                .unwrap();
                all_paths.insert((v1.clone(), v2.clone()), path.1);
                all_paths.insert((v2.clone(), v1.clone()), path.1);
            }
        }
        all_paths
    };

    let total_rate = valves.values().map(|v| v.rate).sum();

    let start = State {
        time: 0,
        location: "AA".to_string(),
        remaining: valves
            .values()
            .map(|v| v.name.clone())
            .filter(|name| name != "AA")
            .collect(),
        rate: 0,
        total: 0,
    };

    let result = dijkstra(
        &start,
        |s| {
            let mut candidates = Vec::new();

            if s.time == 30 {
                return candidates.into_iter();
            }

            for candidate in &s.remaining {
                let steps = *all_paths
                    .get(&(s.location.clone(), candidate.clone()))
                    .unwrap();
                let candidate_rate = valves.get(candidate).unwrap().rate;

                candidates.push((
                    State {
                        time: s.time + steps + 1,
                        location: candidate.clone(),
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
                    location: s.location.clone(),
                    remaining: s.remaining.clone(),
                    rate: s.rate,
                    total: s.total + s.rate,
                },
                total_rate - s.rate,
            ));

            candidates.into_iter()
        },
        |s| s.time == 30 && s.rate == total_rate,
    )
    .unwrap();

    for p in &result.0 {
        println!("{p:?}");
    }

    result.0.last().unwrap().total
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 1651);
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
