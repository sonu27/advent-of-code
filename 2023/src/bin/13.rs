advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u32> {
    let grids = parse_input(input);

    let mut ans = 0;
    grids.iter().for_each(|grid| {
        let a = find_reflection(grid);
        if a.0 == 'x' {
            ans += a.1.unwrap() + 1;
        } else if a.0 == 'y' {
            ans += (a.1.unwrap() + 1) * 100;
        }
    });

    Some(ans as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grids = parse_input(input);

    let mut ans = 0;
    grids.iter().for_each(|grid| {
        let a = find_reflection_count(grid);
        if a.0 == 'x' {
            ans += a.1.unwrap() + 1;
        } else if a.0 == 'y' {
            ans += (a.1.unwrap() + 1) * 100;
        }
    });

    Some(ans as u32)
}

fn parse_input(input: &str) -> Vec<Vec<Vec<char>>> {
    let mut grids: Vec<Vec<Vec<char>>> = Vec::new();
    let mut curr = Vec::new();
    input.lines().for_each(|line| {
        let chars = line.chars().collect::<Vec<char>>();
        if chars.is_empty() {
            let result = curr.clone();
            curr.clear();
            grids.push(result)
        } else {
            curr.push(chars);
        }
    });

    let result = curr.clone();
    curr.clear();
    grids.push(result);

    grids
}

fn find_reflection(grid: &Vec<Vec<char>>) -> (char, Option<usize>) {
    for i in 0..grid.len() - 1 {
        let curr_match = lines_match(grid, i, i + 1);
        if curr_match.is_none() {
            continue;
        }

        let mut y1 = i;
        let mut y2 = i + 1;
        loop {
            if y1 == 0 || y2 == grid.len() - 1 {
                return ('y', curr_match);
            }
            y1 -= 1;
            y2 += 1;

            let match_y = lines_match(grid, y1, y2);
            if match_y.is_none() {
                break;
            }
        }
    }

    for i in 0..grid[0].len() - 1 {
        let curr_match = lines_match_vertical(grid, i, i + 1);
        if curr_match.is_none() {
            continue;
        }

        let mut x1 = i;
        let mut x2 = i + 1;
        loop {
            if x1 == 0 || x2 == grid[0].len() - 1 {
                return ('x', curr_match);
            }
            x1 -= 1;
            x2 += 1;

            let match_x = lines_match_vertical(grid, x1, x2);
            if match_x.is_none() {
                break;
            }
        }
    }

    ('z', None)
}

fn find_reflection_count(grid: &Vec<Vec<char>>) -> (char, Option<usize>) {
    for i in 0..grid.len() - 1 {
        let (curr_match, mut diff) = lines_match_count(grid, i, i + 1);
        if curr_match.is_none() {
            continue;
        }

        let mut y1 = i;
        let mut y2 = i + 1;
        loop {
            if y1 == 0 || y2 == grid.len() - 1 {
                if diff == 1 {
                    return ('y', curr_match);
                }
                break;
            }
            y1 -= 1;
            y2 += 1;

            let (match_y, diff2) = lines_match_count(grid, y1, y2);
            diff += diff2;
            if match_y.is_none() {
                break;
            }
        }
    }

    for i in 0..grid[0].len() - 1 {
        let (curr_match, mut diff) = lines_match_count_vertical(grid, i, i + 1);
        if curr_match.is_none() {
            continue;
        }

        let mut x1 = i;
        let mut x2 = i + 1;
        loop {
            if x1 == 0 || x2 == grid[0].len() - 1 {
                if diff == 1 {
                    return ('x', curr_match);
                }
                break;
            }
            x1 -= 1;
            x2 += 1;

            let (match_x, diff2) = lines_match_count_vertical(grid, x1, x2);
            diff += diff2;
            if match_x.is_none() {
                break;
            }
        }
    }

    ('z', None)
}

fn lines_match(grid: &[Vec<char>], y1: usize, y2: usize) -> Option<usize> {
    for i in 0..grid[0].len() {
        if grid[y1][i] != grid[y2][i] {
            break;
        }
        if i == grid[0].len() - 1 {
            return Some(y1);
        }
    }
    None
}

fn lines_match_vertical(grid: &[Vec<char>], x1: usize, x2: usize) -> Option<usize> {
    for i in 0..grid.len() {
        if grid[i][x1] != grid[i][x2] {
            break;
        }
        if i == grid.len() - 1 {
            return Some(x1);
        }
    }
    None
}

fn lines_match_count(grid: &[Vec<char>], y1: usize, y2: usize) -> (Option<usize>, usize) {
    let mut diff = 0;
    for i in 0..grid[0].len() {
        if grid[y1][i] != grid[y2][i] {
            diff += 1;
        }
        if diff > 1 {
            return (None, diff);
        }
    }
    (Some(y1), diff)
}

fn lines_match_count_vertical(grid: &[Vec<char>], x1: usize, x2: usize) -> (Option<usize>, usize) {
    let mut diff = 0;
    for i in 0..grid.len() {
        if grid[i][x1] != grid[i][x2] {
            diff += 1;
        }
        if diff > 1 {
            return (None, diff);
        }
    }
    (Some(x1), diff)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
