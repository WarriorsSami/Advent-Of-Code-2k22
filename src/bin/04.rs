fn is_full_overlap(section1: (u32, u32), section2: (u32, u32)) -> bool {
    let (start1, end1) = section1;
    let (start2, end2) = section2;
    let full_section = (start1.min(start2), end1.max(end2));

    full_section == section1 || full_section == section2
}

fn is_partial_overlap(section1: (u32, u32), section2: (u32, u32)) -> bool {
    let (start1, end1) = section1;
    let (start2, end2) = section2;
    let (start_max, end_min) = (start1.max(start2), end1.min(end2));

    start_max <= end_min
}

fn count_overlapping_pairs<F>(input: &str, is_overlap: F) -> Option<u32>
    where F: Fn((u32, u32), (u32, u32)) -> bool {
    let answer = input
        .lines()
        .map(|line| {
            line
                .split(',')
                .collect::<Vec<_>>()
                .into_iter()
                .map(|section| {
                    section
                        .split('-')
                        .collect::<Vec<_>>()
                        .into_iter()
                        .map(|num| num.parse::<u32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .map(|section| (section[0], section[1]))
                .collect::<Vec<_>>()
        })
        .map(|sections| {
            let first_section = sections[0];
            let second_section = sections[1];
            is_overlap(first_section, second_section)
        })
        .filter(|value| value == &true)
        .count() as u32;

    answer.into()
}

pub fn part_one(input: &str) -> Option<u32> {
    count_overlapping_pairs(input, is_full_overlap)
}

pub fn part_two(input: &str) -> Option<u32> {
    count_overlapping_pairs(input, is_partial_overlap)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
