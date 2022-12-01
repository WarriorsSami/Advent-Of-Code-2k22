use std::num::ParseIntError;
use sorted_vec::partial::ReverseSortedVec;

fn vec_from(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| line.parse::<u32>())
        .collect::<Vec<Result<u32, ParseIntError>>>()
        .split(|element| element.is_err())
        .map(|slice|
            slice
                .iter()
                .map(|element|
                    element
                        .as_ref()
                        .unwrap()
                )
                .sum()
        )
        .collect::<Vec<u32>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    vec_from(input)
        .iter()
        .max()
        .copied()
}

pub fn part_two(input: &str) -> Option<u32> {
    let unsorted_vec = vec_from(input).to_vec();
    ReverseSortedVec::from_unsorted(unsorted_vec)
        .iter()
        .take(3)
        .sum::<u32>()
        .into()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
