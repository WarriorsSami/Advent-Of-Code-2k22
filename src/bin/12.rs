use std::collections::VecDeque;

type UPoint = (usize, usize);
type IPoint = (isize, isize);

fn get_hill_matrix(input: &str) -> (UPoint, UPoint, Vec<Vec<char>>) {
    let mut hill_matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (n, m) = (hill_matrix.len(), hill_matrix[0].len());
    let (mut start_x, mut start_y) = (0, 0);
    let (mut end_x, mut end_y) = (n - 1, m - 1);

    hill_matrix
        .clone()
        .into_iter()
        .enumerate()
        .for_each(|(i, row)| {
            row.into_iter()
                .enumerate()
                .for_each(|(j, c)| {
                    match c {
                        'S' => (start_x, start_y) = (i, j),
                        'E' => (end_x, end_y) = (i, j),
                        _ => (),
                    }
                });
        });

    hill_matrix[start_x][start_y] = 'a';
    hill_matrix[end_x][end_y] = 'z';
    ((start_x, start_y), (end_x, end_y), hill_matrix)
}

fn bfs<F>(start: UPoint, end: UPoint, hill_matrix: Vec<Vec<char>>, is_safe: F) -> Vec<Vec<u32>>
where F: Fn(IPoint, UPoint, Vec<Vec<u32>>, Vec<Vec<char>>) -> bool {
    let (n, m) = (hill_matrix.len(), hill_matrix[0].len());
    let mut cost_matrix = vec![vec![0; m]; n];

    let mut q: VecDeque<UPoint> = VecDeque::new();
    q.push_back((start.0, start.1));
    cost_matrix[start.0][start.1] = 1;

    let (dx, dy) = (vec![0, 0, 1, -1], vec![1, -1, 0, 0]);

    while !q.is_empty() {
        let (u_x, u_y) = q.pop_front().unwrap();
        for i in 0..4 {
            let (v_x, v_y) = (u_x as isize + dx[i], u_y as isize + dy[i]);
            if is_safe((v_x, v_y), (u_x, u_y), cost_matrix.clone(), hill_matrix.clone()) {
                cost_matrix[v_x as usize][v_y as usize] = cost_matrix[u_x][u_y] + 1;
                q.push_back((v_x as usize, v_y as usize));
            }
        }
    }

    cost_matrix
}

fn is_safe_part_one(child: IPoint, parent: UPoint, cost_matrix: Vec<Vec<u32>>, hill_matrix: Vec<Vec<char>>) -> bool {
    let (n, m) = (cost_matrix.len(), cost_matrix[0].len());
    let in_matrix = child.0 >= 0 && child.0 < n as isize && child.1 >= 0 && child.1 < m as isize;
    if !in_matrix {
        return false;
    }

    cost_matrix[child.0 as usize][child.1 as usize] == 0 &&
        (hill_matrix[child.0 as usize][child.1 as usize] <= hill_matrix[parent.0][parent.1]
            || hill_matrix[child.0 as usize][child.1 as usize] == ((hill_matrix[parent.0][parent.1] as u8) + 1) as char)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (start, end, hill_matrix) = get_hill_matrix(input);
    let cost_matrix = bfs(start, end, hill_matrix, is_safe_part_one);
    Some(cost_matrix[end.0][end.1] - 1)
}

fn is_safe_part_two(child: IPoint, parent: UPoint, cost_matrix: Vec<Vec<u32>>, hill_matrix: Vec<Vec<char>>) -> bool {
    let (n, m) = (cost_matrix.len(), cost_matrix[0].len());
    let in_matrix = child.0 >= 0 && child.0 < n as isize && child.1 >= 0 && child.1 < m as isize;
    if !in_matrix {
        return false;
    }

    cost_matrix[child.0 as usize][child.1 as usize] == 0 &&
        (hill_matrix[child.0 as usize][child.1 as usize] >= hill_matrix[parent.0][parent.1]
            || hill_matrix[child.0 as usize][child.1 as usize] == ((hill_matrix[parent.0][parent.1] as u8) - 1) as char)
}

pub fn part_two(input: &str) -> Option<i32> {
    let (start, end, hill_matrix) = get_hill_matrix(input);
    let cost_matrix = bfs(end, start, hill_matrix.clone(), is_safe_part_two);

    let mut min_cost: i32 = cost_matrix[start.0][start.1] as i32;
    hill_matrix
        .into_iter()
        .enumerate()
        .for_each(|(i, row)| {
            row.into_iter()
                .enumerate()
                .for_each(|(j, c)| {
                    if c == 'a' && cost_matrix[i][j] != 0 {
                        min_cost = min_cost.min(cost_matrix[i][j] as i32);
                    }
                });
        });

    Some(min_cost - 1)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
