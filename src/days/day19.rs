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
    time: u16,

    ore: u16,
    clay: u16,
    obsidian: u16,
    geodes: u16,

    ore_robot: u16,
    clay_robot: u16,
    obsidian_robot: u16,
    geode_robots: u16,
}

pub fn a(input: &str) -> i32 {
    let blueprints = input
        .lines()
        .map(|line| line.parse::<Blueprint>().unwrap())
        .collect::<Vec<_>>();

    blueprints
        .par_iter()
        .map(|blueprint| {
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

            let res = dijkstra_all(&start, |s| {
                let mut candidates = Vec::new();
                if s.time == 24 {
                    return candidates;
                }

                if s.ore >= blueprint.geode_robot_ore as u16
                    && s.obsidian >= blueprint.geode_robot_obsidian as u16
                {
                    candidates.push((
                        State {
                            time: s.time + 1,
                            ore: s.ore + s.ore_robot - blueprint.geode_robot_ore as u16,
                            clay: s.clay + s.clay_robot,
                            obsidian: s.obsidian + s.obsidian_robot
                                - blueprint.geode_robot_obsidian as u16,
                            geodes: s.geodes + s.geode_robots,
                            ore_robot: s.ore_robot,
                            clay_robot: s.clay_robot,
                            obsidian_robot: s.obsidian_robot,
                            geode_robots: s.geode_robots + 1,
                        },
                        1000 - s.geodes - s.geode_robots - 1,
                    ));
                } else {
                    if s.ore >= blueprint.obsidian_robot_ore as u16
                        && s.clay >= blueprint.obsidian_robot_clay as u16
                    {
                        candidates.push((
                            State {
                                time: s.time + 1,
                                ore: s.ore + s.ore_robot - blueprint.obsidian_robot_ore as u16,
                                clay: s.clay + s.clay_robot - blueprint.obsidian_robot_clay as u16,
                                obsidian: s.obsidian + s.obsidian_robot,
                                geodes: s.geodes + s.geode_robots,
                                ore_robot: s.ore_robot,
                                clay_robot: s.clay_robot,
                                obsidian_robot: s.obsidian_robot + 1,
                                geode_robots: s.geode_robots,
                            },
                            1,
                        ));
                    }

                    if s.ore >= blueprint.clay_robot_ore as u16 {
                        candidates.push((
                            State {
                                time: s.time + 1,
                                ore: s.ore + s.ore_robot - blueprint.clay_robot_ore as u16,
                                clay: s.clay + s.clay_robot,
                                obsidian: s.obsidian + s.obsidian_robot,
                                geodes: s.geodes + s.geode_robots,
                                ore_robot: s.ore_robot,
                                clay_robot: s.clay_robot + 1,
                                obsidian_robot: s.obsidian_robot,
                                geode_robots: s.geode_robots,
                            },
                            1,
                        ));
                    }

                    if s.ore >= blueprint.ore_robot_ore as u16 {
                        candidates.push((
                            State {
                                time: s.time + 1,
                                ore: s.ore + s.ore_robot - blueprint.ore_robot_ore as u16,
                                clay: s.clay + s.clay_robot,
                                obsidian: s.obsidian + s.obsidian_robot,
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
                            ore: s.ore + s.ore_robot,
                            clay: s.clay + s.clay_robot,
                            obsidian: s.obsidian + s.obsidian_robot,
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
            .unwrap();

            blueprint.id as u16 * res
        })
        .sum::<u16>() as i32
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

    let mut max_time = 0;

    blueprints
        .iter()
        .take(3)
        .map(|blueprint| {
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

            let res = dijkstra_all(&start, |s| {
                let mut candidates = Vec::new();

                if s.time > max_time {
                    println!("{s:?}");
                    max_time = s.time;
                }

                if s.time == 32 {
                    return candidates;
                }

                if s.ore >= blueprint.geode_robot_ore as u16
                    && s.obsidian >= blueprint.geode_robot_obsidian as u16
                {
                    candidates.push((
                        State {
                            time: s.time + 1,
                            ore: s.ore + s.ore_robot - blueprint.geode_robot_ore as u16,
                            clay: s.clay + s.clay_robot,
                            obsidian: s.obsidian + s.obsidian_robot
                                - blueprint.geode_robot_obsidian as u16,
                            geodes: s.geodes + s.geode_robots,
                            ore_robot: s.ore_robot,
                            clay_robot: s.clay_robot,
                            obsidian_robot: s.obsidian_robot,
                            geode_robots: s.geode_robots + 1,
                        },
                        1000 - s.geodes - s.geode_robots - 1,
                    ));
                } else {
                    if s.ore >= blueprint.obsidian_robot_ore as u16
                        && s.clay >= blueprint.obsidian_robot_clay as u16
                    {
                        candidates.push((
                            State {
                                time: s.time + 1,
                                ore: s.ore + s.ore_robot - blueprint.obsidian_robot_ore as u16,
                                clay: s.clay + s.clay_robot - blueprint.obsidian_robot_clay as u16,
                                obsidian: s.obsidian + s.obsidian_robot,
                                geodes: s.geodes + s.geode_robots,
                                ore_robot: s.ore_robot,
                                clay_robot: s.clay_robot,
                                obsidian_robot: s.obsidian_robot + 1,
                                geode_robots: s.geode_robots,
                            },
                            1,
                        ));
                    }

                    if s.ore >= blueprint.clay_robot_ore as u16 {
                        candidates.push((
                            State {
                                time: s.time + 1,
                                ore: s.ore + s.ore_robot - blueprint.clay_robot_ore as u16,
                                clay: s.clay + s.clay_robot,
                                obsidian: s.obsidian + s.obsidian_robot,
                                geodes: s.geodes + s.geode_robots,
                                ore_robot: s.ore_robot,
                                clay_robot: s.clay_robot + 1,
                                obsidian_robot: s.obsidian_robot,
                                geode_robots: s.geode_robots,
                            },
                            1,
                        ));
                    }

                    if s.ore >= blueprint.ore_robot_ore as u16 {
                        candidates.push((
                            State {
                                time: s.time + 1,
                                ore: s.ore + s.ore_robot - blueprint.ore_robot_ore as u16,
                                clay: s.clay + s.clay_robot,
                                obsidian: s.obsidian + s.obsidian_robot,
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
                            ore: s.ore + s.ore_robot,
                            clay: s.clay + s.clay_robot,
                            obsidian: s.obsidian + s.obsidian_robot,
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
            .unwrap();

            dbg!(blueprint.id);
            dbg!(res);

            res as i32
        })
        .product::<i32>()
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 56 * 62);
    assert_eq!(b(INPUT), 0);
}
