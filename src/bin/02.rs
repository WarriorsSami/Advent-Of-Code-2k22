use std::collections::HashMap;

// Rock-Paper-Scissors
// A-B-C
// X-Y-Z
// X(1), Y(2), Z(3)
// lose(0), draw(3), win(6)
fn get_combinations_part_one() -> HashMap<String, u32> {
    let mut combinations = HashMap::new();
    combinations.insert(String::from("A X"), 4);
    combinations.insert(String::from("A Y"), 8);
    combinations.insert(String::from("A Z"), 3);
    combinations.insert(String::from("B X"), 1);
    combinations.insert(String::from("B Y"), 5);
    combinations.insert(String::from("B Z"), 9);
    combinations.insert(String::from("C X"), 7);
    combinations.insert(String::from("C Y"), 2);
    combinations.insert(String::from("C Z"), 6);
    combinations
}

pub fn part_one(input: &str) -> Option<u32> {
    let combinations = get_combinations_part_one();
    input
        .lines()
        .map(|line| combinations.get(line).unwrap())
        .sum::<u32>()
        .into()
}

// Rock-Paper-Scissors
// A-B-C
// X-Y-Z
// A(1), B(2), C(3)
// X(lose), Y(draw), Z(win)
// lose(0), draw(3), win(6)
fn get_combinations_part_two() -> HashMap<String, u32> {
    let mut combinations = HashMap::new();
    combinations.insert(String::from("A X"), 3); // lose against rock with scissors
    combinations.insert(String::from("A Y"), 4); // draw against rock with rock
    combinations.insert(String::from("A Z"), 8); // win against rock with paper
    combinations.insert(String::from("B X"), 1); // lose against paper with rock
    combinations.insert(String::from("B Y"), 5); // draw against paper with paper
    combinations.insert(String::from("B Z"), 9); // win against paper with scissors
    combinations.insert(String::from("C X"), 2); // lose against scissors with paper
    combinations.insert(String::from("C Y"), 6); // draw against scissors with scissors
    combinations.insert(String::from("C Z"), 7); // win against scissors with rock
    combinations
}

pub fn part_two(input: &str) -> Option<u32> {
    let combinations = get_combinations_part_two();
    input
        .lines()
        .map(|line| combinations.get(line).unwrap())
        .sum::<u32>()
        .into()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
