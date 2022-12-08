fn get_tree_grid(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let tree_grid = get_tree_grid(input);

    let outer_visible_trees: u32 = (tree_grid.len() * 2 + (tree_grid[0].len() - 2) * 2) as u32;
    let mut inner_visible_trees: u32 = 0;

    tree_grid
        .iter()
        .enumerate()
        .for_each(|(x, row)| {
            row
                .iter()
                .enumerate()
                .for_each(|(y, tree)| {
                    if x == 0 || x == tree_grid.len() - 1 || y == 0 || y == row.len() - 1 {
                        return;
                    }

                    let (mut gt_left, mut gt_bottom, mut gt_top, mut gt_right) = (false, false, false, false);
                    // check left edge
                    for i in 0..y {
                        if tree_grid[x][i] >= *tree {
                            gt_left = true;
                            break;
                        }
                    }
                    // check bottom edge
                    for i in x + 1..tree_grid.len() {
                        if tree_grid[i][y] >= *tree {
                            gt_bottom = true;
                            break;
                        }
                    }
                    // check top edge
                    for i in 0..x {
                        if tree_grid[i][y] >= *tree {
                            gt_top = true;
                            break;
                        }
                    }
                    // check right edge
                    for i in y + 1..tree_grid[0].len() {
                        if tree_grid[x][i] >= *tree {
                            gt_right = true;
                            break;
                        }
                    }

                    if !gt_left || !gt_bottom || !gt_top || !gt_right {
                        inner_visible_trees += 1;
                    }
                });
        });

    Some(outer_visible_trees + inner_visible_trees)
}

pub fn part_two(input: &str) -> Option<u32> {
    let tree_grid = get_tree_grid(input);

    let mut max_scenic_score = 0u32;

    tree_grid
        .iter()
        .enumerate()
        .for_each(|(x, row)| {
            row
                .iter()
                .enumerate()
                .for_each(|(y, tree)| {
                    if x == 0 || x == tree_grid.len() - 1 || y == 0 || y == row.len() - 1 {
                        return;
                    }

                    let (mut gt_left, mut gt_bottom, mut gt_top, mut gt_right) = (0u32, 0u32, 0u32, 0u32);

                    // check left edge
                    for i in (0..y).rev() {
                        gt_left += 1;
                        if tree_grid[x][i] >= *tree {
                            break;
                        }
                    }

                    // check bottom edge
                    for i in x + 1..tree_grid.len() {
                        gt_bottom += 1;
                        if tree_grid[i][y] >= *tree {
                            break;
                        }
                    }

                    // check top edge
                    for i in (0..x).rev() {
                        gt_top += 1;
                        if tree_grid[i][y] >= *tree {
                            break;
                        }
                    }

                    // check right edge
                    for i in y + 1..tree_grid[0].len() {
                        gt_right += 1;
                        if tree_grid[x][i] >= *tree {
                            break;
                        }
                    }

                    let scenic_score = gt_left * gt_bottom * gt_top * gt_right;
                    max_scenic_score = max_scenic_score.max(scenic_score);
                });
        });

    Some(max_scenic_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
