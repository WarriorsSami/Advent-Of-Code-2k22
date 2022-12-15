const Y_QUERY: i64 = 2000000;

type Point = (i64, i64);
type Interval = (Point, Point);

struct Sensor {
    pub origin: Point,
    pub closest_beacon: Point,
    pub distance: i64,
}

impl Sensor {
    pub fn new(origin: Point, closest_beacon: Point) -> Self {
        let distance = Self::distance(origin, closest_beacon);

        Self {
            origin,
            closest_beacon,
            distance,
        }
    }

    fn distance(origin: Point, closest_beacon: Point) -> i64 {
        let (x1, y1) = origin;
        let (x2, y2) = closest_beacon;

        (x1.abs_diff(x2) + y1.abs_diff(y2)) as i64
    }

    pub fn get_influence_interval(&self, relative_y: i64) -> Option<Interval> {
        let (x, y) = self.origin;

        if relative_y.abs_diff(y) > self.distance as u64 {
            return None;
        }

        let remaining_distance = self.distance - (relative_y.abs_diff(y) as i64);
        let x_min = x - remaining_distance;
        let x_max = x + remaining_distance;

        Some(((x_min, relative_y), (x_max, relative_y)))
    }
}

fn get_sensors(input: &str) -> Vec<Sensor> {
    input
        .lines()
        .map(|line| line.split(' ').collect::<Vec<_>>())
        .map(|line| {
            (
                line[2],
                line[3],
                line[8],
                line[9],
            )
        })
        .map(|(x1, y1, x2, y2)| {
            let origin = (
                x1[2..x1.len() - 1].parse::<i64>().unwrap(),
                y1[2..y1.len() - 1].parse::<i64>().unwrap(),
            );
            let closest_beacon = (
                x2[2..x2.len() - 1].parse::<i64>().unwrap(),
                y2[2..].parse::<i64>().unwrap(),
            );

            Sensor::new(origin, closest_beacon)
        })
        .collect::<Vec<_>>()
}

fn get_influence_intervals(sensors: &[Sensor], relative_y: i64) -> Vec<Point> {
    let mut intervals = sensors
        .iter()
        .filter_map(|sensor| sensor.get_influence_interval(relative_y))
        .map(|interval| (interval.0.0, interval.1.0) as Point)
        .collect::<Vec<_>>();

    intervals.sort_by(|a, b| {
        if a.0 == b.0 {
            a.1.cmp(&b.1)
        } else {
            a.0.cmp(&b.0)
        }
    });

    let mut merged_intervals: Vec<Point> = vec![];
    let mut current_interval = intervals[0];

    for interval in intervals.clone() {
        if interval.0 > current_interval.1 && interval.0.abs_diff(current_interval.1) > 1 {
            merged_intervals.push(current_interval);
            current_interval = interval;
        } else if interval.0.abs_diff(current_interval.1) == 1 {
            current_interval.1 = interval.1;
        } else {
            current_interval.1 = current_interval.1.max(interval.1);
        }
    }
    merged_intervals.push(current_interval);

    merged_intervals
}

pub fn part_one(input: &str) -> Option<i64> {
    let sensors = get_sensors(input);
    let intervals = get_influence_intervals(&sensors, Y_QUERY);

    let mut influence_score = 0i64;
    println!();
    for interval in &intervals {
        influence_score += (interval.0.abs_diff(interval.1) + 1) as i64;
        println!("{:?}", interval);
    }

    let mut beacons = sensors
        .iter()
        .map(|sensor| sensor.closest_beacon)
        .collect::<Vec<_>>();

    beacons.sort();
    beacons.dedup();

    for beacon in beacons {
        if beacon.1 == Y_QUERY {
            influence_score -= 1;
        }
    }

    influence_score.into()
}

const Y_START: i64 = 0;
const Y_END: i64 = 4000000;

pub fn part_two(input: &str) -> Option<i64> {
    let sensors = get_sensors(input);

    let (mut start_y, mut end_y) = (Y_END, Y_START);

    sensors
        .iter()
        .map(|sensor| sensor.origin)
        .for_each(|point| {
            start_y = start_y.min(point.1);
            end_y = end_y.max(point.1);
        });

    let mut ans_beacon: Point = (0, 0);

    for y in start_y..end_y + 1 {
        let intervals = get_influence_intervals(&sensors, y);
        if intervals.len() > 1 {
            ans_beacon.1 = y;
            ans_beacon.0 = intervals[0].1 + 1;
        }
    }

    (ans_beacon.0 * Y_END + ans_beacon.1).into()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), Some(56000011));
    }
}
