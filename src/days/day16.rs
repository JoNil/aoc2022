use parse_display::FromStr;
use pathfinding::prelude::dijkstra;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    str::FromStr,
};

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

#[derive(Clone, Debug, Eq, PartialEq)]
struct State {
    time: i32,
    valve: Valve,
    opened: HashSet<String>,
}

impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.time.hash(state);
        self.valve.hash(state);
        for v in &self.opened {
            v.hash(state);
        }
    }
}

pub fn a(input: &str) -> i32 {
    let valves = input
        .lines()
        .map(|l| l.parse::<Valve>().unwrap())
        .map(|v| (v.name.clone(), v))
        .collect::<HashMap<_, _>>();

    let start = State {
        time: 0,
        valve: valves.get("AA").unwrap().clone(),
        opened: HashSet::new(),
    };

    let result = dijkstra(
        &start,
        |s| {
            s.valve
                .tunnels
                .0
                .iter()
                .map(|t| valves.get(t).unwrap())
                .flat_map(|v| {
                    let mut new_paths = vec![(
                        State {
                            time: s.time + 1,
                            valve: v.clone(),
                            opened: s.opened.clone(),
                        },
                        0,
                    )];

                    if !s.opened.contains(&v.name) {
                        let mut new_opened = s.opened.clone();
                        new_opened.insert(v.name.clone());

                        new_paths.push((
                            State {
                                time: s.time + 2,
                                valve: v.clone(),
                                opened: new_opened,
                            },
                            -v.rate * (30 - (s.time + 2)),
                        ));
                    }

                    new_paths
                })
        },
        |state| state.time == 30,
    )
    .unwrap();

    for a in result.0 {
        println!("{:?}", a);
    }

    println!("{}", result.1);
    0
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
