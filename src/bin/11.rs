use std::collections::vec_deque::VecDeque;
use num::{BigInt, Zero};
use num::bigint::ToBigInt;

#[derive(Debug, Clone)]
enum Operation {
    AddOld,
    Add(BigInt),
    MulOld,
    Mul(BigInt),
}

impl Operation {
    pub fn new(input: Vec<&str>) -> Self {
        match input[0] {
            "+" => {
                if let Ok(value) = input[1].parse::<u64>() {
                    Operation::Add(ToBigInt::to_bigint(&value).unwrap())
                } else {
                    Operation::AddOld
                }
            }
            "*" => {
                if let Ok(value) = input[1].parse::<u64>() {
                    Operation::Mul(ToBigInt::to_bigint(&value).unwrap())
                } else {
                    Operation::MulOld
                }
            }
            value => panic!("Unknown operation: {}", value),
        }
    }

    pub fn apply_operation(self, old_value: BigInt) -> BigInt {
        match self {
            Operation::Add(value) => old_value + value,
            Operation::AddOld => 2 * old_value,
            Operation::Mul(value) => old_value * value,
            Operation::MulOld => BigInt::pow(&old_value, 2),
        }
    }
}

#[derive(Debug, Clone)]
struct DivTest {
    pub rate: (BigInt, BigInt),
    pub true_monkey: usize,
    pub false_monkey: usize,
}

impl DivTest {
    pub fn new(input: Vec<Vec<&str>>) -> Self {
        let rate = input[0]
            .last()
            .unwrap()
            .parse::<u64>()
            .expect("Invalid rate");

        let true_monkey = input[1]
            .last()
            .unwrap()
            .parse::<usize>()
            .expect("Invalid true monkey");

        let false_monkey = input[2]
            .last()
            .unwrap()
            .parse::<usize>()
            .expect("Invalid false monkey");

        DivTest {
            rate: (ToBigInt::to_bigint(&rate).unwrap(), ToBigInt::to_bigint(&rate).unwrap()),
            true_monkey,
            false_monkey,
        }
    }

    pub fn apply_div_test(self, value: BigInt) -> usize {
        if value % self.rate.0 == BigInt::zero() {
            self.true_monkey
        } else {
            self.false_monkey
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    pub worry_levels_queue: VecDeque<BigInt>,
    pub operation: Operation,
    pub div_test: DivTest,
}

impl Monkey {
    pub fn new(input: Vec<&str>) -> Self {
        let items_line = input[0]
            .split_whitespace()
            .collect::<Vec<_>>()
            .into_iter()
            .skip(2);

        let operation_line = input[1]
            .split_whitespace()
            .collect::<Vec<_>>()
            .into_iter()
            .skip(4)
            .collect::<Vec<_>>();

        let div_test_lines = input[2..]
            .iter()
            .map(|line| {
                line
                    .split_whitespace()
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let items_queue = items_line
            .into_iter()
            .map(|item| {
                let item = String::from(item);
                if item.ends_with(',') {
                    item[..item.len() - 1].parse::<u64>().unwrap()
                } else {
                    item.parse::<u64>().unwrap()
                }
            })
            .map(|item| ToBigInt::to_bigint(&item).unwrap())
            .collect::<VecDeque<_>>();

        let operation = Operation::new(operation_line);

        let div_test = DivTest::new(div_test_lines);

        Monkey {
            worry_levels_queue: items_queue,
            operation,
            div_test,
        }
    }
}

fn get_monkeys(input: &str) -> Vec<Monkey> {
    let monkeys = input
        .lines()
        .collect::<Vec<_>>()
        .chunks(7)
        .map(|chunk| {
            chunk
                .iter()
                .copied()
                .skip(1)
                .filter(|line| !line.is_empty())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .into_iter()
        .map(Monkey::new)
        .collect::<Vec<_>>();

    let common_div_test_rate = monkeys
        .iter()
        .map(|monkey| monkey.div_test.rate.0.clone())
        .reduce(|a, b| a * b)
        .unwrap();

    monkeys
        .iter()
        .map(|monkey| {
            let new_div_test = DivTest {
                rate: (monkey.div_test.rate.0.clone(), common_div_test_rate.clone()),
                true_monkey: monkey.div_test.true_monkey,
                false_monkey: monkey.div_test.false_monkey,
            };
            Monkey {
                worry_levels_queue: monkey.worry_levels_queue.clone(),
                operation: monkey.operation.clone(),
                div_test: new_div_test,
            }
        })
        .collect::<Vec<_>>()
}

fn execute_monkey_keep_away(mut monkeys: Vec<Monkey>, rounds: u64, worry_level_threshold: u64) -> u64 {
    let mut items_inspected_per_monkey = vec![0u64; monkeys.len()];

    for _ in 0..rounds {
        for monkey_index in 0..monkeys.len() {
            let mut monkey = monkeys[monkey_index].clone();

            while let Some(worry_level) = monkey.worry_levels_queue.pop_front() {
                items_inspected_per_monkey[monkey_index] += 1;

                let mut new_worry_level = monkey.operation.clone()
                    .apply_operation(worry_level) / worry_level_threshold;
                new_worry_level %= monkey.div_test.rate.1.clone();

                let new_monkey = monkey.div_test.clone()
                    .apply_div_test(new_worry_level.clone());

                monkeys[new_monkey].worry_levels_queue
                    .push_back(new_worry_level.clone());
            }
            monkeys[monkey_index] = monkey;
        }
    }

    items_inspected_per_monkey.sort_by(|a, b| b.cmp(a));
    (items_inspected_per_monkey[0] * items_inspected_per_monkey[1]) as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    let monkeys = get_monkeys(input);
    Some(execute_monkey_keep_away(monkeys, 20, 3))
}

pub fn part_two(input: &str) -> Option<u64> {
    let monkeys = get_monkeys(input);
    Some(execute_monkey_keep_away(monkeys, 10000, 1))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
