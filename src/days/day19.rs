use parse_display::FromStr;
use pathfinding::prelude::dijkstra_all;
use rayon::prelude::*;

pub static INPUT: &str = include_str!("../input/19.txt");
pub static TEST_INPUT: &str = include_str!("../input/19_test.txt");

#[derive(FromStr, Debug, Hash, Clone, PartialEq, Eq)]
#[display("Blueprint {id}: Each ore robot costs {ore_robot_ore} ore. Each clay robot costs {clay_robot_ore} ore. Each obsidian robot costs {obsidian_robot_ore} ore and {obsidian_robot_clay} clay. Each geode robot costs {geode_robot_ore} ore and {geode_robot_obsidian} obsidian.")]
struct Blueprint {
    id: i32,
    ore_robot_ore: i32,
    clay_robot_ore: i32,
    obsidian_robot_ore: i32,
    obsidian_robot_clay: i32,
    geode_robot_ore: i32,
    geode_robot_obsidian: i32,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct State {
    time: u8,
    geodes: u8,

    ore: u8,
    clay: u8,
    obsidian: u8,

    ore_robot: u8,
    clay_robot: u8,
    obsidian_robot: u8,
    geode_robots: u8,
}

fn solve(blueprint: &Blueprint, end_time: i32) -> i32 {
    let start = State {
        time: 0,
        ore: 0,
        clay: 0,
        obsidian: 0,
        geodes: 0,
        ore_robot: 1,
        clay_robot: 0,
        obsidian_robot: 0,
        geode_robots: 0,
    };

    // TODO(JoNil): Maybe limit search space by holding array of max geode_robots per level and don't search if there has been a better one found

    dijkstra_all(&start, |s| {
        let mut candidates = Vec::new();

        if s.time as i32 == end_time {
            return candidates;
        }

        if s.ore >= blueprint.geode_robot_ore as u8
            && s.obsidian >= blueprint.geode_robot_obsidian as u8
        {
            candidates.push((
                State {
                    time: s.time + 1,
                    ore: s.ore + s.ore_robot as u8 - blueprint.geode_robot_ore as u8,
                    clay: s.clay + s.clay_robot as u8,
                    obsidian: s.obsidian + s.obsidian_robot as u8
                        - blueprint.geode_robot_obsidian as u8,
                    geodes: s.geodes + s.geode_robots,
                    ore_robot: s.ore_robot,
                    clay_robot: s.clay_robot,
                    obsidian_robot: s.obsidian_robot,
                    geode_robots: s.geode_robots + 1,
                },
                1,
            ));
        } else {
            if s.ore >= blueprint.obsidian_robot_ore as u8
                && s.clay >= blueprint.obsidian_robot_clay as u8
            {
                candidates.push((
                    State {
                        time: s.time + 1,
                        ore: s.ore + s.ore_robot as u8 - blueprint.obsidian_robot_ore as u8,
                        clay: s.clay + s.clay_robot as u8 - blueprint.obsidian_robot_clay as u8,
                        obsidian: s.obsidian + s.obsidian_robot as u8,
                        geodes: s.geodes + s.geode_robots,
                        ore_robot: s.ore_robot,
                        clay_robot: s.clay_robot,
                        obsidian_robot: s.obsidian_robot + 1,
                        geode_robots: s.geode_robots,
                    },
                    1,
                ));
            }

            if s.ore >= blueprint.clay_robot_ore as u8 {
                candidates.push((
                    State {
                        time: s.time + 1,
                        ore: s.ore + s.ore_robot as u8 - blueprint.clay_robot_ore as u8,
                        clay: s.clay + s.clay_robot as u8,
                        obsidian: s.obsidian + s.obsidian_robot as u8,
                        geodes: s.geodes + s.geode_robots,
                        ore_robot: s.ore_robot,
                        clay_robot: s.clay_robot + 1,
                        obsidian_robot: s.obsidian_robot,
                        geode_robots: s.geode_robots,
                    },
                    1,
                ));
            }

            if s.ore >= blueprint.ore_robot_ore as u8 {
                candidates.push((
                    State {
                        time: s.time + 1,
                        ore: s.ore + s.ore_robot as u8 - blueprint.ore_robot_ore as u8,
                        clay: s.clay + s.clay_robot as u8,
                        obsidian: s.obsidian + s.obsidian_robot as u8,
                        geodes: s.geodes + s.geode_robots,
                        ore_robot: s.ore_robot + 1,
                        clay_robot: s.clay_robot,
                        obsidian_robot: s.obsidian_robot,
                        geode_robots: s.geode_robots,
                    },
                    1,
                ));
            }

            candidates.push((
                State {
                    time: s.time + 1,
                    ore: s.ore + s.ore_robot as u8,
                    clay: s.clay + s.clay_robot as u8,
                    obsidian: s.obsidian + s.obsidian_robot as u8,
                    geodes: s.geodes + s.geode_robots,
                    ore_robot: s.ore_robot,
                    clay_robot: s.clay_robot,
                    obsidian_robot: s.obsidian_robot,
                    geode_robots: s.geode_robots,
                },
                1,
            ));
        }

        candidates
    })
    .keys()
    .map(|s| s.geodes)
    .max()
    .unwrap() as i32
}

pub fn a(input: &str) -> i32 {
    let blueprints = input
        .lines()
        .map(|line| line.parse::<Blueprint>().unwrap())
        .collect::<Vec<_>>();

    blueprints
        .par_iter()
        .map(|blueprint| blueprint.id * solve(blueprint, 24))
        .sum::<i32>() as i32
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 33);
    assert_eq!(a(INPUT), 1589);
}

pub fn b(input: &str) -> i32 {
    let blueprints = input
        .lines()
        .map(|line| line.parse::<Blueprint>().unwrap())
        .collect::<Vec<_>>();

    blueprints
        .iter()
        .take(3)
        .map(|blueprint| {
            let res = solve(blueprint, 32);

            dbg!(blueprint.id);
            dbg!(res);

            res as i32
        })
        .product::<i32>()
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 56 * 62);
    assert_eq!(b(INPUT), 29348);
}
