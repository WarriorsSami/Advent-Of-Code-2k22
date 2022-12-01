use std::cmp::max;

pub fn part_one(input: &str) -> Option<u32> {
    let mut curr_sum = 0u32;
    input
        .lines()
        .fold(Some(0u32), |acc, line| {
            match line.parse::<u32>() {
                Ok(num) => {
                    curr_sum += num;
                    acc
                }
                Err(_) => {
                    let result = acc.map(|acc| max(acc, curr_sum));
                    curr_sum = 0;
                    result
                }
            }
        })
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut curr_sum = 0u32;
    let mut answer = input
        .lines()
        .map(|line| {
            match line.parse::<u32>() {
                Ok(num) => {
                    curr_sum += num;
                    0
                }
                Err(_) => {
                    let result = curr_sum;
                    curr_sum = 0;
                    result
                }
            }
        })
        .collect::<Vec<u32>>();
    answer.push(curr_sum);
    answer.sort_by(|a, b| b.cmp(a));

    Some(answer[0..3].iter().sum::<u32>())
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
