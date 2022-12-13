use std::cmp::Ordering;
use either::Either;
use crate::PacketState::{Right, Skip, Wrong};

#[derive(Debug, Eq, PartialEq)]
enum PacketState {
    Wrong,
    Right,
    Skip
}

fn mark_outer_commas(input: &str) -> String {
    let mut active_brackets = 0;
    let mut result = String::new();

    for c in input.chars() {
        match c {
            '[' => {
                active_brackets += 1;
                result.push(c);
            }
            ']' => {
                active_brackets -= 1;
                result.push(c);
            }
            ',' => {
                if active_brackets == 0 {
                    result.push('_');
                } else {
                    result.push(',');
                }
            }
            _ => result.push(c),
        }
    }

    result
}

struct TreeNode {
    value: Either<Vec<TreeNode>, Option<u32>>,
}

impl TreeNode {
    pub fn new(input: &str) -> Self {
        Self::parse(input)
    }

    fn parse(input: &str) -> Self {
        // println!("Parsing {}", input);
        if !input.contains(',') {
            return match input.contains('[') {
                true => Self {
                    value: Either::Left(vec![
                        Self::parse(&input[1..input.len() - 1])
                    ]),
                },
                false => match input {
                    "" => Self {
                        value: Either::Right(None),
                    },
                    _ => Self {
                        value: Either::Right(Some(input.parse::<u32>().unwrap())),
                    }
                }
            }
        }

        let children = mark_outer_commas(&input[1..input.len() - 1])
            .split('_')
            .into_iter()
            .map(TreeNode::new)
            .collect::<Vec<_>>();

        let value = Either::Left(children);

        TreeNode {
            value,
        }
    }

    pub fn compare_to(&self, other: &Self) -> PacketState {
        match (&self.value, &other.value) {
            (Either::Right(None), Either::Right(None)) => Skip,
            (Either::Right(Some(a)), Either::Right(Some(b))) => {
                match a.cmp(b) {
                    Ordering::Equal => Skip,
                    Ordering::Less => Right,
                    Ordering::Greater => Wrong,
                }
            },
            (Either::Right(None), Either::Right(Some(_))) => Right,
            (Either::Right(Some(_)), Either::Right(None)) => Wrong,
            (Either::Left(a), Either::Left(b)) => {
                for (a, b) in a.iter().zip(b.iter()) {
                    match a.compare_to(b) {
                        Right => return Right,
                        Wrong => return Wrong,
                        Skip => continue,
                    }
                }

                match a.len().cmp(&b.len()) {
                    Ordering::Equal => Skip,
                    Ordering::Less => Right,
                    Ordering::Greater => Wrong,
                }
            },
            (Either::Left(a), Either::Right(b)) => {
                if a.is_empty() {
                    return Right;
                }

                if b.is_none() {
                    return Wrong;
                }

                match a[0].compare_to(&TreeNode {
                    value: Either::Right(*b),
                }) {
                    Skip => match a.len() {
                        1 => Skip,
                        _ => Wrong,
                    },
                    result => result,
                }
            },
            (Either::Right(a), Either::Left(b)) => {
                if b.is_empty() {
                    return Wrong;
                }

                if a.is_none() {
                    return Right;
                }

                match (TreeNode {
                    value: Either::Right(*a),
                }.compare_to(&b[0])) {
                    Skip => match b.len() {
                        1 => Skip,
                        _ => Right,
                    },
                    result => result,
                }
            },
        }
    }

    pub fn print(&self) {
        match &self.value {
            Either::Left(children) => {
                print!("[");
                for index in 0..children.len() {
                    children[index].print();
                    if index != children.len() - 1 {
                        print!(",");
                    }
                }
                print!("]");
            }
            Either::Right(value) => {
                print!("{}", match value {
                    Some(value) => value.to_string(),
                    None => "".to_string(),
                });
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .collect::<Vec<_>>()
        .into_iter()
        .map(|chunk| (TreeNode::new(chunk[0]), TreeNode::new(chunk[1])))
        .collect::<Vec<_>>()
        .into_iter()
        .enumerate()
        .map(|(index, (a, b))| {
            match a.compare_to(&b) {
                Right => (index + 1) as u32,
                _ => 0,
            }
        })
        .sum::<u32>()
        .into()
}

fn compare_tree_nodes(a: &TreeNode, b: &TreeNode) -> Ordering {
    match a.compare_to(b) {
        Right => Ordering::Less,
        Wrong => Ordering::Greater,
        Skip => Ordering::Equal,
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut signals = input
        .lines()
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .into_iter()
        .map(TreeNode::new)
        .collect::<Vec<_>>();

    let (divider_2, divider_6) = ("[[2]]", "[[6]]");
    signals.push(TreeNode::new(divider_2));
    signals.push(TreeNode::new(divider_6));

    let mut result = 1u32;
    signals.sort_by(compare_tree_nodes);
    signals
        .into_iter()
        .enumerate()
        .for_each(|(index, signal)| {
            if signal.compare_to(&TreeNode::new(divider_2)) == Skip
                || signal.compare_to(&TreeNode::new(divider_6)) == Skip {
                result *= (index + 1) as u32;
            }
        });

    result.into()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
