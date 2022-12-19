use parse_display::FromStr;
use pathfinding::prelude::dijkstra;

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
struct State<'a> {
    time: i32,

    ore: i32,
    clay: i32,
    obsidian: i32,
    geodes: i32,

    ore_robot: i32,
    clay_robot: i32,
    obsidian_robot: i32,
    geode_robots: i32,

    blueprint: &'a Blueprint,
}

pub fn a(input: &str) -> i32 {
    let blueprints = input
        .lines()
        .map(|line| line.parse::<Blueprint>().unwrap())
        .collect::<Vec<_>>();

    let mut sum_of_quality_level = 0;

    for blueprint in &blueprints {
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
            blueprint,
        };

        let res = dijkstra(
            &start,
            |s| {
                let mut candidates = Vec::new();

                println!("{s:?}");

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
                        blueprint,
                    },
                    1000 - s.geodes,
                ));

                if s.ore >= s.blueprint.ore_robot_ore {
                    candidates.push((
                        State {
                            time: s.time + 1,
                            ore: s.ore + s.ore_robot - s.blueprint.ore_robot_ore,
                            clay: s.clay + s.clay_robot,
                            obsidian: s.obsidian + s.obsidian_robot,
                            geodes: s.geodes + s.geode_robots,
                            ore_robot: s.ore_robot + 1,
                            clay_robot: s.clay_robot,
                            obsidian_robot: s.obsidian_robot,
                            geode_robots: s.geode_robots,
                            blueprint,
                        },
                        1000 - s.geodes,
                    ));
                }

                if s.ore >= s.blueprint.clay_robot_ore {
                    candidates.push((
                        State {
                            time: s.time + 1,
                            ore: s.ore + s.ore_robot - s.blueprint.clay_robot_ore,
                            clay: s.clay + s.clay_robot,
                            obsidian: s.obsidian + s.obsidian_robot,
                            geodes: s.geodes + s.geode_robots,
                            ore_robot: s.ore_robot,
                            clay_robot: s.clay_robot + 1,
                            obsidian_robot: s.obsidian_robot,
                            geode_robots: s.geode_robots,
                            blueprint,
                        },
                        1000 - s.geodes,
                    ));
                }

                if s.ore >= s.blueprint.obsidian_robot_ore
                    && s.clay >= s.blueprint.obsidian_robot_clay
                {
                    candidates.push((
                        State {
                            time: s.time + 1,
                            ore: s.ore + s.ore_robot - s.blueprint.obsidian_robot_ore,
                            clay: s.clay + s.clay_robot - s.blueprint.obsidian_robot_clay,
                            obsidian: s.obsidian + s.obsidian_robot,
                            geodes: s.geodes + s.geode_robots,
                            ore_robot: s.ore_robot,
                            clay_robot: s.clay_robot,
                            obsidian_robot: s.obsidian_robot + 1,
                            geode_robots: s.geode_robots,
                            blueprint,
                        },
                        1000 - s.geodes,
                    ));
                }

                if s.ore >= s.blueprint.geode_robot_ore
                    && s.obsidian >= s.blueprint.geode_robot_obsidian
                {
                    candidates.push((
                        State {
                            time: s.time + 1,
                            ore: s.ore + s.ore_robot - s.blueprint.geode_robot_ore,
                            clay: s.clay + s.clay_robot,
                            obsidian: s.obsidian + s.obsidian_robot
                                - s.blueprint.geode_robot_obsidian,
                            geodes: s.geodes + s.geode_robots,
                            ore_robot: s.ore_robot,
                            clay_robot: s.clay_robot,
                            obsidian_robot: s.obsidian_robot,
                            geode_robots: s.geode_robots + 1,
                            blueprint,
                        },
                        1000 - s.geodes,
                    ));
                }

                candidates.into_iter()
            },
            |s| s.time == 24,
        )
        .unwrap();

        sum_of_quality_level += blueprint.id * res.0.last().unwrap().geodes;
    }

    sum_of_quality_level
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 33);
    //assert_eq!(a(INPUT), 0);
}

pub fn b(input: &str) -> i32 {
    0
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 0);
    assert_eq!(b(INPUT), 0);
}
