use parse_display::FromStr;
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
    time: i32,

    ore: i32,
    clay: i32,
    obsidian: i32,
    geodes: i32,

    ore_robot: i32,
    clay_robot: i32,
    obsidian_robot: i32,
    geode_robots: i32,
}

fn solve(blueprint: &Blueprint, max_geodes: &mut i32, max_time: &mut i32, end_time: i32, s: State) {
    if s.time as i32 > *max_time {
        println!("{s:?}");
        *max_time = s.time as i32;
    }

    if s.time as i32 == end_time {
        *max_geodes = (*max_geodes).max(s.geodes as i32);
        return;
    }

    if s.ore >= blueprint.geode_robot_ore && s.obsidian >= blueprint.geode_robot_obsidian {
        solve(
            blueprint,
            max_geodes,
            max_time,
            end_time,
            State {
                time: s.time + 1,
                ore: s.ore + s.ore_robot - blueprint.geode_robot_ore,
                clay: s.clay + s.clay_robot,
                obsidian: s.obsidian + s.obsidian_robot - blueprint.geode_robot_obsidian,
                geodes: s.geodes + s.geode_robots,
                ore_robot: s.ore_robot,
                clay_robot: s.clay_robot,
                obsidian_robot: s.obsidian_robot,
                geode_robots: s.geode_robots + 1,
            },
        );
    }

    if s.ore >= blueprint.obsidian_robot_ore && s.clay >= blueprint.obsidian_robot_clay {
        solve(
            blueprint,
            max_geodes,
            max_time,
            end_time,
            State {
                time: s.time + 1,
                ore: s.ore + s.ore_robot - blueprint.obsidian_robot_ore,
                clay: s.clay + s.clay_robot - blueprint.obsidian_robot_clay,
                obsidian: s.obsidian + s.obsidian_robot,
                geodes: s.geodes + s.geode_robots,
                ore_robot: s.ore_robot,
                clay_robot: s.clay_robot,
                obsidian_robot: s.obsidian_robot + 1,
                geode_robots: s.geode_robots,
            },
        );
    }

    if s.ore >= blueprint.clay_robot_ore {
        solve(
            blueprint,
            max_geodes,
            max_time,
            end_time,
            State {
                time: s.time + 1,
                ore: s.ore + s.ore_robot - blueprint.clay_robot_ore,
                clay: s.clay + s.clay_robot,
                obsidian: s.obsidian + s.obsidian_robot,
                geodes: s.geodes + s.geode_robots,
                ore_robot: s.ore_robot,
                clay_robot: s.clay_robot + 1,
                obsidian_robot: s.obsidian_robot,
                geode_robots: s.geode_robots,
            },
        );
    }

    if s.ore >= blueprint.ore_robot_ore {
        solve(
            blueprint,
            max_geodes,
            max_time,
            end_time,
            State {
                time: s.time + 1,
                ore: s.ore + s.ore_robot - blueprint.ore_robot_ore,
                clay: s.clay + s.clay_robot,
                obsidian: s.obsidian + s.obsidian_robot,
                geodes: s.geodes + s.geode_robots,
                ore_robot: s.ore_robot + 1,
                clay_robot: s.clay_robot,
                obsidian_robot: s.obsidian_robot,
                geode_robots: s.geode_robots,
            },
        );
    }

    solve(
        blueprint,
        max_geodes,
        max_time,
        end_time,
        State {
            time: s.time + 1,
            ore: s.ore + s.ore_robot,
            clay: s.clay + s.clay_robot,
            obsidian: s.obsidian + s.obsidian_robot,
            geodes: s.geodes + s.geode_robots,
            ore_robot: s.ore_robot,
            clay_robot: s.clay_robot,
            obsidian_robot: s.obsidian_robot,
            geode_robots: s.geode_robots,
        },
    );
}

pub fn a(input: &str) -> i32 {
    let blueprints = input
        .lines()
        .map(|line| line.parse::<Blueprint>().unwrap())
        .collect::<Vec<_>>();

    blueprints
        .par_iter()
        .map(|blueprint| {
            let mut max_geodes = 0;
            let mut max_time = 0;

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

            solve(blueprint, &mut max_geodes, &mut max_time, 24, start);

            blueprint.id * max_geodes
        })
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
            let mut max_geodes = 0;
            let mut max_time = 0;

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

            solve(blueprint, &mut max_geodes, &mut max_time, 32, start);

            dbg!(blueprint.id);
            dbg!(max_geodes);

            max_geodes
        })
        .product::<i32>()
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 56 * 62);
    assert_eq!(b(INPUT), 0);
}
