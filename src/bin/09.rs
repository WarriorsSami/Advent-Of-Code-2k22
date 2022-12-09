use std::collections::HashSet;
use phf::{Map, phf_map};

const DIRECTION_TO_DELTA: Map<&str, (isize, isize)> = phf_map! {
    "R" => (0, 1),
    "L" => (0, -1),
    "U" => (-1, 0),
    "D" => (1, 0),
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct MatrixPoint {
    pub x: i64,
    pub y: i64,
}

impl MatrixPoint {
    pub fn new(x: i64, y: i64) -> MatrixPoint {
        MatrixPoint { x, y }
    }

    pub fn update_position_by_delta(&mut self, delta: (isize, isize)) {
        self.x = (self.x as isize + delta.0) as i64;
        self.y = (self.y as isize + delta.1) as i64;
    }

    pub fn is_adjacent_with(self, other: MatrixPoint) -> bool {
        let delta_x = (self.x as isize - other.x as isize).abs();
        let delta_y = (self.y as isize - other.y as isize).abs();

        delta_x <= 1 && delta_y <= 1
    }

    pub fn update_position_by_neighbor(&mut self, neighbor: MatrixPoint) {
        if self.x == neighbor.x {
            if self.y < neighbor.y {
                self.y += 1;
            } else {
                self.y -= 1;
            }
        } else if self.y == neighbor.y {
            if self.x < neighbor.x {
                self.x += 1;
            } else {
                self.x -= 1;
            }
        } else if self.x < neighbor.x && self.y < neighbor.y {
            self.x += 1;
            self.y += 1;
        } else if self.x < neighbor.x && self.y > neighbor.y {
            self.x += 1;
            self.y -= 1;
        } else if self.x > neighbor.x && self.y < neighbor.y {
            self.x -= 1;
            self.y += 1;
        } else if self.x > neighbor.x && self.y > neighbor.y {
            self.x -= 1;
            self.y -= 1;
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut head = MatrixPoint::new(1, 1);
    let mut tail = MatrixPoint::new(1, 1);

    let mut motion_matrix: HashSet<MatrixPoint> = HashSet::new();

    input
        .lines()
        .map(|line| line.split(' '))
        .map(|mut line| {
            let direction = line.next().unwrap();
            let distance = line.next().unwrap().parse::<usize>().unwrap();
            (direction, distance)
        })
        .for_each(|(direction, distance)| {
            let delta = DIRECTION_TO_DELTA[direction];
            for _ in 0..distance {
                motion_matrix.insert(tail);
                head.update_position_by_delta(delta);
                if !head.is_adjacent_with(tail) {
                    tail.update_position_by_neighbor(head);
                }
            }
        });

    let ans = motion_matrix.len() as u32;
    ans.into()
}

struct Rope {
    pub head: MatrixPoint,
    pub tail: MatrixPoint,
    pub body: Vec<MatrixPoint>,
}

impl Rope {
    pub fn new(len: i64) -> Rope {
        let body= (0..len)
            .map(|_| MatrixPoint::new(0, 0))
            .collect::<Vec<_>>();

        Rope {
            head: body[0],
            tail: body[len as usize - 1],
            body,
        }
    }

    pub fn from_rope(rope: &Rope) -> Rope {
        Rope {
            head: rope.head,
            tail: rope.tail,
            body: rope.body.clone(),
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut rope = Rope::new(10);
    let mut motion_matrix: HashSet<MatrixPoint> = HashSet::new();

    input
        .lines()
        .map(|line| line.split(' '))
        .map(|mut line| {
            let direction = line.next().unwrap();
            let distance = line.next().unwrap().parse::<usize>().unwrap();
            (direction, distance)
        })
        .for_each(|(direction, distance)| {
            let delta = DIRECTION_TO_DELTA[direction];
            for _ in 0..distance {
                motion_matrix.insert(rope.tail);
                rope.head.update_position_by_delta(delta);

                let mut prev_knot = rope.head;
                let body: Vec<MatrixPoint> = rope.body
                    .clone()
                    .into_iter()
                    .enumerate()
                    .map(|(index, mut knot_position)| {
                        if index == 0 {
                            return rope.head;
                        }

                        let prev_knot_position = prev_knot;
                        if !knot_position.is_adjacent_with(prev_knot_position) {
                            knot_position
                                .update_position_by_neighbor(prev_knot_position);
                        }

                        prev_knot = knot_position;
                        knot_position
                    })
                    .collect::<Vec<_>>();

                rope = Rope::from_rope(&Rope {
                    head: body[0],
                    tail: body[body.len() - 1],
                    body,
                });
            }
        });

    let ans = motion_matrix.len() as u32;
    ans.into()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(36));
    }
}
