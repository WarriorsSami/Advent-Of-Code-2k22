use phf::phf_map;

static MOVES: phf::Map<char, i32> = phf_map! {
    'A' => 1,
    'B' => 2,
    'C' => 3,
    'X' => 1,
    'Y' => 2,
    'Z' => 3,
};

// Rock-Paper-Scissors
// A-B-C
// X-Y-Z
// 1-2-3
// X(1), Y(2), Z(3)
// lose(0), draw(3), win(6)
fn get_score_part_one(foe: char, me: char) -> i32 {
    let game_status = (MOVES[&me] - MOVES[&foe] + 1).rem_euclid(3);
    MOVES[&me] + game_status * 3
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            let foe = line.chars().next().unwrap();
            let me = line.chars().nth(2).unwrap();
            get_score_part_one(foe, me) as u32
        })
        .sum::<u32>()
        .into()
}

// Rock-Paper-Scissors
// A-B-C
// X-Y-Z
// A(1), B(2), C(3)
// X(lose), Y(draw), Z(win)
// lose(0), draw(3), win(6)
// 1 -> 2, 2 -> 3, 3 -> 1 (win)
// 1 -> 3, 2 -> 1, 3 -> 2 (lose)
fn get_score_part_two(foe: char, me: char) -> i32 {
    let game_status = (MOVES[&me] + 2) % 3;
    let move_score = match game_status {
        0 => match MOVES[&foe] - 1 {
            0 => 3,
            x => x,
        },
        1 => MOVES[&foe],
        2 => match (MOVES[&foe] + 1) % 3 {
            0 => 3,
            x => x,
        },
        _ => panic!("Invalid game status"),
    };
    game_status * 3 + move_score
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            let foe = line.chars().next().unwrap();
            let me = line.chars().nth(2).unwrap();
            get_score_part_two(foe, me) as u32
        })
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
