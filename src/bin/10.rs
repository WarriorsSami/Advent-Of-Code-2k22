pub fn part_one(input: &str) -> Option<i64> {
    let mut register_x_value = 1i64;
    let mut clock_cycle_count = 1i64;
    let mut signal_strength_sum = 0i64;

    input
        .lines()
        .map(|line| line.split(' ').collect::<Vec<&str>>())
        .for_each(|line| {
            let instruction = line[0];
            match instruction {
                "addx" => {
                    let value = line[1].parse::<i64>().unwrap();
                    for _ in 0..2 {
                        if clock_cycle_count == 20 || (clock_cycle_count - 20) % 40 == 0 {
                            signal_strength_sum += clock_cycle_count * register_x_value;
                        }
                        clock_cycle_count += 1;
                    }
                    register_x_value += value;
                }
                "noop" => {
                    if clock_cycle_count == 20 || (clock_cycle_count - 20) % 40 == 0 {
                        signal_strength_sum += clock_cycle_count * register_x_value;
                    }
                    clock_cycle_count += 1;
                }
                _ => panic!("Unknown instruction: {}", instruction)
            }
        });

    Some(signal_strength_sum)
}

const CRT_DIM: usize = 240;
const CHUNK_DIM: usize = 40;

fn get_overlap_between_rx_and_cc(rx: i64, cc: i64) -> Option<i64> {
    let mut overlap = None;

    if rx == cc {
        overlap = Some(rx);
    } else if rx + 1 == cc {
        overlap = Some(rx + 1);
    } else if rx + 2 == cc {
        overlap = Some(rx + 2);
    }

    overlap
}

pub fn part_two(input: &str) -> Option<String> {
    let mut crt_screen = ['.'; CRT_DIM];
    let mut register_x_value = 1i64;
    let mut clock_cycle_count = 1i64;

    input
        .lines()
        .map(|line| line.split(' ').collect::<Vec<&str>>())
        .for_each(|line| {
            let instruction = line[0];
            match instruction {
                "addx" => {
                    let value = line[1].parse::<i64>().unwrap();
                    for _ in 0..2 {
                        let overlap = get_overlap_between_rx_and_cc(
                            register_x_value - 1 + CHUNK_DIM as i64 * ((clock_cycle_count - 1) / CHUNK_DIM as i64),
                            clock_cycle_count - 1
                        );
                        if let Some(overlap) = overlap {
                            crt_screen[overlap as usize] = '#';
                        }
                        clock_cycle_count += 1;
                    }
                    register_x_value += value;
                }
                "noop" => {
                    let overlap = get_overlap_between_rx_and_cc(
                        register_x_value - 1 + CHUNK_DIM as i64 * ((clock_cycle_count - 1) / CHUNK_DIM as i64),
                        clock_cycle_count - 1
                    );
                    if let Some(overlap) = overlap {
                        crt_screen[overlap as usize] = '#';
                    }
                    clock_cycle_count += 1;
                }
                _ => panic!("Unknown instruction: {}", instruction)
            }
        });

    let crt_screen_by_chunks = crt_screen
        .chunks(CHUNK_DIM)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>();

    println!("{}", crt_screen_by_chunks.join("\n"));

    Some(crt_screen.iter().collect())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), Some("##..##..##..##..##..##..##..##..##..##..###...###...###...###...###...###...###.####....####....####....####....####....#####.....#####.....#####.....#####.....######......######......######......###########.......#######.......#######.....".to_string()));
    }
}
