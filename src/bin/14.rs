use num::range;

const MATRIX_DIM: usize = 1000;

fn get_matrix(input: &str) -> [[char; MATRIX_DIM]; MATRIX_DIM] {
    let mut matrix = [['.'; MATRIX_DIM]; MATRIX_DIM];

    input
        .lines()
        .map(|line| {
            line
                .split(" -> ")
                .collect::<Vec<_>>()
                .into_iter()
                .map(|token| {
                    token
                        .split(',')
                        .collect::<Vec<_>>()
                })
                .map(|token| {
                    (
                        token[0].parse::<usize>().unwrap(),
                        token[1].parse::<usize>().unwrap(),
                    )
                })
                .collect::<Vec<_>>()
        })
        .for_each(|positions| {
            positions
                .windows(2)
                .for_each(|adj_pos| {
                    let (x1, y1) = adj_pos[0];
                    let (x2, y2) = adj_pos[1];

                    if x1 == x2 {
                        for y in range(y1.min(y2), y1.max(y2) + 1) {
                            matrix[y][x1] = '#';
                        }
                    } else if y1 == y2 {
                        for x in range(x1.min(x2), x1.max(x2) + 1) {
                            matrix[y1][x] = '#';
                        }
                    }
                });
        });

    matrix
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut matrix = get_matrix(input);
    let (sand_src_x, sand_src_y) = (0, 500);
    matrix[sand_src_x][sand_src_y] = '+';

    let mut sand_count = 0u32;
    loop {
        let (mut sand_x, mut sand_y) = (sand_src_x, sand_src_y);

        loop {
            if sand_x == MATRIX_DIM - 1 {
                break;
            }

            if matrix[sand_x + 1][sand_y] == '.' {
                sand_x += 1;
            } else if matrix[sand_x + 1][sand_y - 1] == '.' {
                sand_x += 1;
                sand_y -= 1;
            } else if matrix[sand_x + 1][sand_y + 1] == '.' {
                sand_x += 1;
                sand_y += 1;
            } else {
                break;
            }
        }

        if sand_x == MATRIX_DIM - 1 {
            break;
        }

        matrix[sand_x][sand_y] = 'o';
        sand_count += 1;
    }

    sand_count.into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut matrix = get_matrix(input);
    let (sand_src_x, sand_src_y) = (0, 500);
    matrix[sand_src_x][sand_src_y] = '+';

    let mut sand_bottom_x = MATRIX_DIM;
    for x in range(0, MATRIX_DIM).rev() {
        for y in range(0, MATRIX_DIM) {
            if matrix[x][y] == '#' {
                sand_bottom_x = x;
                break;
            }
        }

        if sand_bottom_x != MATRIX_DIM {
            break;
        }
    }

    sand_bottom_x += 2;
    for y in range(0, MATRIX_DIM) {
        matrix[sand_bottom_x][y] = '#';
    }
    for x in range(0, sand_bottom_x) {
        matrix[x][0] = '#';
        matrix[x][MATRIX_DIM - 1] = '#';
    }

    println!("{}", sand_bottom_x);

    let mut sand_count = 0u32;
    loop {
        let (mut sand_x, mut sand_y) = (sand_src_x, sand_src_y);

        loop {
            if matrix[sand_x + 1][sand_y] == '.' {
                sand_x += 1;
            } else if matrix[sand_x + 1][sand_y - 1] == '.' {
                sand_x += 1;
                sand_y -= 1;
            } else if matrix[sand_x + 1][sand_y + 1] == '.' {
                sand_x += 1;
                sand_y += 1;
            } else {
                break;
            }

            if sand_x == sand_bottom_x || sand_x == sand_src_x {
                break;
            }
        }

        if sand_x == sand_src_x {
            break;
        }

        matrix[sand_x][sand_y] = 'o';
        sand_count += 1;
    }

    matrix[sand_src_x][sand_src_y] = 'o';
    sand_count += 1;

    println!("Matrix:");
    for x in range(0, MATRIX_DIM) {
        for y in range(0, MATRIX_DIM) {
            print!("{}", matrix[x][y]);
        }
        println!();
    }

    sand_count.into()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
