use std::collections::HashSet;

fn check_if_different(sequence: &[char]) -> bool {
    let initial_length = sequence.len();
    let char_set: HashSet<char> = sequence.iter().cloned().collect();

    initial_length == char_set.len()
}

fn detect_starter_by_count(input: &str, count: usize) -> Option<u32> {
    input
        .lines()
        .collect::<Vec<_>>()
        .first()
        .unwrap()
        .to_owned()
        .chars()
        .collect::<Vec<_>>()
        .windows(count)
        .enumerate()
        .find(|(_, sequence)| check_if_different(sequence))
        .map(|(index, _)| (index + count) as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    detect_starter_by_count(input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    detect_starter_by_count(input, 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(5));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(23));
    }
}
