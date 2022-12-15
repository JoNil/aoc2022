use glam::{ivec2, IVec2};
use parse_display::FromStr;

pub static INPUT: &str = include_str!("../input/15.txt");
pub static TEST_INPUT: &str = include_str!("../input/15_test.txt");

#[derive(FromStr, PartialEq, Debug)]
#[display("Sensor at x={pos.x}, y={pos.y}: closest beacon is at x={closest_beacon.x}, y={closest_beacon.y}")]
struct Sensor {
    #[from_str(default)]
    pos: IVec2,
    #[from_str(default)]
    closest_beacon: IVec2,
}

fn manhattan_distance(a: IVec2, b: IVec2) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

pub fn a(input: &str, row: i32) -> i32 {
    let sensors = input
        .lines()
        .map(|l| l.parse::<Sensor>().unwrap())
        .collect::<Vec<_>>();

    let mut max_x = i32::MIN;
    let mut min_x = i32::MAX;

    for sensor in &sensors {
        let dist = manhattan_distance(sensor.pos, sensor.closest_beacon);

        max_x = max_x.max(sensor.pos.x + dist);
        min_x = min_x.min(sensor.pos.x - dist);
    }

    let mut not_covered = 0;

    for x in min_x..=max_x {
        let pos = ivec2(x, row);

        if sensors
            .iter()
            .any(|s| s.closest_beacon == pos || s.pos == pos)
        {
            continue;
        }

        if sensors
            .iter()
            .any(|s| manhattan_distance(pos, s.pos) <= manhattan_distance(s.pos, s.closest_beacon))
        {
            not_covered += 1;
        }
    }

    not_covered
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT, 10), 26);
    assert_eq!(a(INPUT, 2000000), 5394423);
}

pub fn b(input: &str, max: i32) -> i64 {
    let sensors = input
        .lines()
        .map(|l| l.parse::<Sensor>().unwrap())
        .collect::<Vec<_>>();

    for candidate_pos in sensors.iter().flat_map(|s| {
        let dist = manhattan_distance(s.pos, s.closest_beacon) + 1;

        (0..=dist)
            .map(move |i| ivec2(s.pos.x + dist - i, s.pos.y + i))
            .chain((0..=dist).map(move |i| ivec2(s.pos.x + dist - i, s.pos.y - i)))
            .chain((0..=dist).map(move |i| ivec2(s.pos.x - dist - i, s.pos.y + i)))
            .chain((0..=dist).map(move |i| ivec2(s.pos.x - dist - i, s.pos.y - i)))
    }) {
        if candidate_pos.x < 0
            || candidate_pos.y < 0
            || candidate_pos.x > max
            || candidate_pos.y > max
        {
            continue;
        }

        if sensors
            .iter()
            .any(|s| s.closest_beacon == candidate_pos || s.pos == candidate_pos)
        {
            continue;
        }

        if sensors.iter().all(|s| {
            manhattan_distance(candidate_pos, s.pos) > manhattan_distance(s.pos, s.closest_beacon)
        }) {
            return 4000000 * candidate_pos.x as i64 + candidate_pos.y as i64;
        }
    }

    0
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT, 20), 56000011);
    assert_eq!(b(INPUT, 4000000), 11840879211051);
}
