use parse_display::FromStr;
use pathfinding::prelude::{dijkstra, dijkstra_all};
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
struct State<'a> {
    time: i32,
    valve: &'a Valve,
    opened: HashSet<String>,
}

impl<'a> Hash for State<'a> {
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
        valve: valves.get("AA").unwrap(),
        opened: HashSet::new(),
    };

    let result = dijkstra_all(&start, |s| {
        let mut candidates = Vec::new();

        if s.time == 30 {
            return candidates.into_iter();
        }

        if !s.opened.contains(&s.valve.name) {
            let mut new_opened = s.opened.clone();
            new_opened.insert(s.valve.name.clone());

            candidates.push((
                State {
                    time: s.time + 1,
                    valve: s.valve,
                    opened: new_opened,
                },
                -s.valve.rate * (30 - (s.time + 1)),
            ));
        }

        for v in s.valve.tunnels.0.iter().map(|t| valves.get(t).unwrap()) {
            candidates.push((
                State {
                    time: s.time + 1,
                    valve: v,
                    opened: s.opened.clone(),
                },
                0,
            ));
        }
        candidates.into_iter()
    });

    for a in &result {
        println!("{:?}", a.1 .1);
    }
    -result.iter().map(|r| r.1 .1).min().unwrap()
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
