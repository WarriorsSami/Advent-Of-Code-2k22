use std::collections::VecDeque;

fn execute_crane_movement(input: &str, preserve_order: bool) -> Option<String> {
    // read the initial form of stacks as it is
    let mut stack_lines = input
        .lines()
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>();

    // extract the ids of the stacks
    let no_ids = stack_lines
        .last()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<_>>()
        .into_iter()
        .map(|id| id.parse::<u32>().unwrap())
        .len();
    // mark how many lines we should skip up until the first movement
    let skip_threshold = stack_lines.len() + 1;

    // remove the ids line and reverse the order of the stacks for better performance
    stack_lines.pop();
    stack_lines.reverse();

    // create a vector of empty stacks
    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    for _ in 0..no_ids {
        stacks.push(VecDeque::new());
    }

    // construct the stacks
    stack_lines
        .into_iter()
        // parse every line as chunks of characters and extract the proper crate tag
        .map(|line| {
            line
                .chars()
                .collect::<Vec<_>>()
                .chunks(4)
                .map(|chunk| {
                    chunk[1]
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .into_iter()
        // push every crate tag to the proper stack
        .for_each(|line| {
            line
                .into_iter()
                .enumerate()
                .for_each(|(idx, c)| {
                    match c {
                        ' ' => (),
                        _ => stacks[idx].push_back(c),
                    }
                });
        });

    // read the movements
    input
        .lines()
        // skip the initial stacks, ids line and empty line
        .skip(skip_threshold)
        // parse every movement as a tuple of (qty, from, to)
        .map(|line| {
            let s = line
                .split(|c: char| {
                    c == ' ' || c.is_alphabetic()
                })
                .filter(|s| !s.is_empty())
                .collect::<Vec<&str>>();

            (
                s[0].parse::<usize>().unwrap(),
                s[1].parse::<usize>().unwrap(),
                s[2].parse::<usize>().unwrap(),
            )
        })
        // execute every movement with respect to the preserve_order flag
        .for_each(|(qty, from, to)| {
            match preserve_order {
                false => {
                    for _ in 0..qty {
                        let c = stacks[from - 1].pop_back().unwrap();
                        stacks[to - 1].push_back(c);
                    }
                }
                true => {
                    let mut temp_stack = VecDeque::new();
                    for _ in 0..qty {
                        let c = stacks[from - 1].pop_back().unwrap();
                        temp_stack.push_back(c);
                    }
                    for _ in 0..qty {
                        let c = temp_stack.pop_back().unwrap();
                        stacks[to - 1].push_back(c);
                    }
                }
            }
        });

    // extract the top of the stacks and concatenate them
    stacks
        .into_iter()
        .map(|mut stack| stack.pop_back().unwrap())
        .collect::<String>()
        .into()
}

pub fn part_one(input: &str) -> Option<String> {
    execute_crane_movement(input, false)
}

pub fn part_two(input: &str) -> Option<String> {
    execute_crane_movement(input, true)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
