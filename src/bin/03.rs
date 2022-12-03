use std::collections::HashSet;

const A_LOWER_PRIORITY: u32 = 1;
const A_UPPER_PRIORITY: u32 = 27;

fn compute_priority(item: char) -> u32 {
    match item {
        'a'..='z' => A_LOWER_PRIORITY + (item as u32 - 'a' as u32),
        'A'..='Z' => A_UPPER_PRIORITY + (item as u32 - 'A' as u32),
        _ => 0
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(first, second)| {
            (
                first.chars().collect::<HashSet<_>>(),
                second.chars().collect::<HashSet<_>>(),
            )
        })
        .map(|(first_set, second_set)| {
            first_set.intersection(&second_set)
                .map(|c| {
                    compute_priority(c.to_owned())
                })
                .sum::<u32>()
        })
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| {
            (
                chunk[0].chars().collect::<HashSet<_>>(),
                chunk[1].chars().collect::<HashSet<_>>(),
                chunk[2].chars().collect::<HashSet<_>>(),
            )
        })
        .map(|(first_set, second_set, third_set)| {
            first_set.intersection(&second_set)
                .map(|c| c.to_owned())
                .collect::<HashSet<char>>()
                .intersection(&third_set)
                .map(|c| {
                    compute_priority(*c)
                })
                .sum::<u32>()
        })
        .sum::<u32>()
        .into()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
