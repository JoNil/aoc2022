#![allow(clippy::comparison_chain)]

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

fn find_all_paths(valves: &HashMap<String, Valve>) -> HashMap<(String, String), i32> {
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
}

#[derive(Clone, Hash, Debug, Eq, PartialEq)]
struct StateA<'a> {
    time: i32,
    location: &'a Valve,
    remaining: Vec<&'a Valve>,
    rate: i32,
    total: i32,
}

pub fn a(input: &str) -> i32 {
    let valves = input
        .lines()
        .map(|l| l.parse::<Valve>().unwrap())
        .map(|v| (v.name.clone(), v))
        .collect::<HashMap<_, _>>();

    let all_paths = find_all_paths(&valves);

    let total_rate = valves.values().map(|v| v.rate).sum::<i32>();

    let mut remaining = valves
        .values()
        .filter(|v| v.name != "AA")
        .filter(|v| v.rate != 0)
        .collect::<Vec<_>>();
    remaining.sort_by(|a, b| b.rate.cmp(&a.rate));

    let start = StateA {
        time: 0,
        location: valves.get("AA").unwrap(),
        remaining,
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
                    .get(&(s.location.name.clone(), candidate.name.clone()))
                    .unwrap();

                candidates.push((
                    StateA {
                        time: s.time + steps + 1,
                        location: candidate,
                        remaining: s
                            .remaining
                            .iter()
                            .cloned()
                            .filter(|r| *r != *candidate)
                            .collect(),
                        rate: s.rate + candidate.rate,
                        total: s.total + (steps + 1) * s.rate,
                    },
                    steps * (total_rate - s.rate) + total_rate - (s.rate + candidate.rate),
                ));
            }

            candidates.push((
                StateA {
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
        |s| s.time == 30,
    )
    .unwrap();

    result.0.last().unwrap().total
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 1651);
    assert_eq!(a(INPUT), 2119);
}

#[derive(Clone, Hash, Debug, Eq, PartialEq)]
enum Location<'a> {
    At(&'a Valve),
    Going(&'a Valve, i32),
}

#[derive(Clone, Hash, Debug, Eq, PartialEq)]
struct StateB<'a> {
    time: i32,
    m_location: Location<'a>,
    e_location: Location<'a>,
    remaining: Vec<&'a Valve>,
    rate: i32,
    total: i32,
}

pub fn b(input: &str) -> i32 {
    let valves = input
        .lines()
        .map(|l| l.parse::<Valve>().unwrap())
        .map(|v| (v.name.clone(), v))
        .collect::<HashMap<_, _>>();

    let all_paths = find_all_paths(&valves);

    let total_rate = valves.values().map(|v| v.rate).sum::<i32>();

    let mut remaining = valves
        .values()
        .filter(|v| v.name != "AA")
        .filter(|v| v.rate != 0)
        .collect::<Vec<_>>();
    remaining.sort_by(|a, b| b.rate.cmp(&a.rate));

    let start = StateB {
        time: 0,
        m_location: Location::At(valves.get("AA").unwrap()),
        e_location: Location::At(valves.get("AA").unwrap()),
        remaining,
        rate: 0,
        total: 0,
    };

    let result = dijkstra(
        &start,
        |s| {
            let mut candidates = Vec::new();

            if s.time == 26 {
                return candidates.into_iter();
            }

            println!("{} {}", s.time, s.total);

            match (&s.m_location, &s.e_location) {
                (Location::At(m_location), Location::At(e_location)) => {
                    for (i, candidate_1) in s.remaining.iter().enumerate() {
                        for candidate_2 in &s.remaining[i..] {
                            if m_location.name != candidate_1.name
                                && e_location.name != candidate_2.name
                            {
                                // M to 1 and E to 2
                                let steps_m_to_1 = *all_paths
                                    .get(&(m_location.name.clone(), candidate_1.name.clone()))
                                    .unwrap();
                                let steps_e_to_2 = *all_paths
                                    .get(&(e_location.name.clone(), candidate_2.name.clone()))
                                    .unwrap();
                                if steps_m_to_1 < steps_e_to_2 {
                                    candidates.push((
                                        StateB {
                                            time: s.time + steps_m_to_1 + 1,
                                            m_location: Location::At(candidate_1),
                                            e_location: Location::Going(
                                                candidate_2,
                                                steps_e_to_2 - steps_m_to_1 - 1,
                                            ),
                                            remaining: s
                                                .remaining
                                                .iter()
                                                .cloned()
                                                .filter(|r| *r != *candidate_1)
                                                .collect(),
                                            rate: s.rate + candidate_1.rate,
                                            total: s.total + (steps_m_to_1 + 1) * s.rate,
                                        },
                                        steps_m_to_1 * (total_rate - s.rate) + total_rate
                                            - (s.rate + candidate_1.rate),
                                    ));
                                } else if steps_m_to_1 > steps_e_to_2 {
                                    candidates.push((
                                        StateB {
                                            time: s.time + steps_e_to_2 + 1,
                                            m_location: Location::Going(
                                                candidate_1,
                                                steps_m_to_1 - steps_e_to_2 - 1,
                                            ),
                                            e_location: Location::At(candidate_2),
                                            remaining: s
                                                .remaining
                                                .iter()
                                                .cloned()
                                                .filter(|r| *r != *candidate_2)
                                                .collect(),
                                            rate: s.rate + candidate_2.rate,
                                            total: s.total + (steps_e_to_2 + 1) * s.rate,
                                        },
                                        steps_e_to_2 * (total_rate - s.rate) + total_rate
                                            - (s.rate + candidate_2.rate),
                                    ));
                                } else {
                                    candidates.push((
                                        StateB {
                                            time: s.time + steps_m_to_1 + 1,
                                            m_location: Location::At(candidate_1),
                                            e_location: Location::At(candidate_2),
                                            remaining: s
                                                .remaining
                                                .iter()
                                                .cloned()
                                                .filter(|r| {
                                                    *r != *candidate_1 || *r != *candidate_2
                                                })
                                                .collect(),
                                            rate: s.rate + candidate_1.rate + candidate_2.rate,
                                            total: s.total + (steps_m_to_1 + 1) * s.rate,
                                        },
                                        steps_m_to_1 * (total_rate - s.rate) + total_rate
                                            - (s.rate + candidate_1.rate + candidate_2.rate),
                                    ));
                                }
                            }

                            if m_location.name != candidate_2.name
                                && e_location.name != candidate_1.name
                            {
                                // M to 2 and E to 1

                                let steps_m_to_2 = *all_paths
                                    .get(&(m_location.name.clone(), candidate_2.name.clone()))
                                    .unwrap();
                                let steps_e_to_1 = *all_paths
                                    .get(&(e_location.name.clone(), candidate_1.name.clone()))
                                    .unwrap();

                                if steps_m_to_2 < steps_e_to_1 {
                                    candidates.push((
                                        StateB {
                                            time: s.time + steps_m_to_2 + 1,
                                            m_location: Location::At(candidate_2),
                                            e_location: Location::Going(
                                                candidate_1,
                                                steps_e_to_1 - steps_m_to_2 - 1,
                                            ),
                                            remaining: s
                                                .remaining
                                                .iter()
                                                .cloned()
                                                .filter(|r| *r != *candidate_1)
                                                .collect(),
                                            rate: s.rate + candidate_1.rate,
                                            total: s.total + (steps_m_to_2 + 1) * s.rate,
                                        },
                                        steps_m_to_2 * (total_rate - s.rate) + total_rate
                                            - (s.rate + candidate_1.rate),
                                    ));
                                } else if steps_m_to_2 > steps_e_to_1 {
                                    candidates.push((
                                        StateB {
                                            time: s.time + steps_e_to_1 + 1,
                                            m_location: Location::Going(
                                                candidate_2,
                                                steps_m_to_2 - steps_e_to_1 - 1,
                                            ),
                                            e_location: Location::At(candidate_1),
                                            remaining: s
                                                .remaining
                                                .iter()
                                                .cloned()
                                                .filter(|r| *r != *candidate_2)
                                                .collect(),
                                            rate: s.rate + candidate_2.rate,
                                            total: s.total + (steps_e_to_1 + 1) * s.rate,
                                        },
                                        steps_e_to_1 * (total_rate - s.rate) + total_rate
                                            - (s.rate + candidate_2.rate),
                                    ));
                                } else {
                                    candidates.push((
                                        StateB {
                                            time: s.time + steps_m_to_2 + 1,
                                            m_location: Location::At(candidate_2),
                                            e_location: Location::At(candidate_1),
                                            remaining: s
                                                .remaining
                                                .iter()
                                                .cloned()
                                                .filter(|r| {
                                                    *r != *candidate_1 || *r != *candidate_2
                                                })
                                                .collect(),
                                            rate: s.rate + candidate_1.rate + candidate_2.rate,
                                            total: s.total + (steps_m_to_2 + 1) * s.rate,
                                        },
                                        steps_m_to_2 * (total_rate - s.rate) + total_rate
                                            - (s.rate + candidate_1.rate + candidate_2.rate),
                                    ));
                                }
                            }
                        }
                    }
                }
                (Location::At(m_location), Location::Going(e_destination, e_remaining)) => {
                    for candidate in &s.remaining {
                        if m_location.name != candidate.name {
                            let steps = *all_paths
                                .get(&(m_location.name.clone(), candidate.name.clone()))
                                .unwrap();

                            if steps < *e_remaining {
                                candidates.push((
                                    StateB {
                                        time: s.time + steps + 1,
                                        m_location: Location::At(candidate),
                                        e_location: Location::Going(
                                            e_destination,
                                            e_remaining - steps - 1,
                                        ),
                                        remaining: s
                                            .remaining
                                            .iter()
                                            .cloned()
                                            .filter(|r| *r != *candidate)
                                            .collect(),
                                        rate: s.rate + candidate.rate,
                                        total: s.total + (steps + 1) * s.rate,
                                    },
                                    steps * (total_rate - s.rate) + total_rate
                                        - (s.rate + candidate.rate),
                                ));
                            } else if *e_remaining < steps {
                                candidates.push((
                                    StateB {
                                        time: s.time + e_remaining + 1,
                                        m_location: Location::Going(
                                            m_location,
                                            steps - e_remaining - 1,
                                        ),
                                        e_location: Location::At(e_destination),
                                        remaining: s
                                            .remaining
                                            .iter()
                                            .cloned()
                                            .filter(|r| *r != *e_destination)
                                            .collect(),
                                        rate: s.rate + e_destination.rate,
                                        total: s.total + (e_remaining + 1) * s.rate,
                                    },
                                    e_remaining * (total_rate - s.rate) + total_rate
                                        - (s.rate + e_destination.rate),
                                ));
                            } else {
                                candidates.push((
                                    StateB {
                                        time: s.time + steps + 1,
                                        m_location: Location::At(candidate),
                                        e_location: Location::At(e_destination),
                                        remaining: s
                                            .remaining
                                            .iter()
                                            .cloned()
                                            .filter(|r| *r != *candidate || *r != *e_destination)
                                            .collect(),
                                        rate: s.rate + candidate.rate + e_destination.rate,
                                        total: s.total + (steps + 1) * s.rate,
                                    },
                                    steps * (total_rate - s.rate) + total_rate
                                        - (s.rate + candidate.rate + e_destination.rate),
                                ));
                            }
                        }
                    }
                }
                (Location::Going(m_destination, m_remaining), Location::At(e_location)) => {
                    for candidate in &s.remaining {
                        if e_location.name != candidate.name {
                            let steps = *all_paths
                                .get(&(e_location.name.clone(), candidate.name.clone()))
                                .unwrap();

                            if steps < *m_remaining {
                                candidates.push((
                                    StateB {
                                        time: s.time + steps + 1,
                                        m_location: Location::Going(
                                            m_destination,
                                            steps - m_remaining - 1,
                                        ),
                                        e_location: Location::At(candidate),
                                        remaining: s
                                            .remaining
                                            .iter()
                                            .cloned()
                                            .filter(|r| *r != *candidate)
                                            .collect(),
                                        rate: s.rate + candidate.rate,
                                        total: s.total + (steps + 1) * s.rate,
                                    },
                                    steps * (total_rate - s.rate) + total_rate
                                        - (s.rate + candidate.rate),
                                ));
                            } else if *m_remaining < steps {
                                candidates.push((
                                    StateB {
                                        time: s.time + m_remaining + 1,
                                        m_location: Location::At(m_destination),
                                        e_location: Location::Going(
                                            e_location,
                                            steps - m_remaining - 1,
                                        ),
                                        remaining: s
                                            .remaining
                                            .iter()
                                            .cloned()
                                            .filter(|r| *r != *m_destination)
                                            .collect(),
                                        rate: s.rate + m_destination.rate,
                                        total: s.total + (m_remaining + 1) * s.rate,
                                    },
                                    m_remaining * (total_rate - s.rate) + total_rate
                                        - (s.rate + m_destination.rate),
                                ));
                            } else {
                                candidates.push((
                                    StateB {
                                        time: s.time + steps + 1,
                                        m_location: Location::At(m_destination),
                                        e_location: Location::At(candidate),
                                        remaining: s
                                            .remaining
                                            .iter()
                                            .cloned()
                                            .filter(|r| *r != *candidate || *r != *m_destination)
                                            .collect(),
                                        rate: s.rate + candidate.rate + m_destination.rate,
                                        total: s.total + (steps + 1) * s.rate,
                                    },
                                    steps * (total_rate - s.rate) + total_rate
                                        - (s.rate + candidate.rate + m_destination.rate),
                                ));
                            }
                        }
                    }
                }
                (Location::Going(_, _), Location::Going(_, _)) => panic!("Bad code!"),
            }

            candidates.into_iter()
        },
        |s| s.time == 26,
    )
    .unwrap();

    result.0.last().unwrap().total
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 1707);
    //assert_eq!(b(INPUT), 0);
}
