use std::collections::HashMap;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // move all the Os all the way up
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'O' {
                let mut k = i;
                while k > 0 && grid[k - 1][j] == '.' {
                    grid[k][j] = '.';
                    grid[k - 1][j] = 'O';
                    k -= 1;
                }
            }
        }
    }

    Some(calculate_load(&grid) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut m: HashMap<String, usize> = HashMap::new();
    let mut cycle_found_at = 0;

    for j in 0..1000 {
        cycle(&mut grid);

        let joined_grid = grid
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("");
        let found = m.contains_key(&joined_grid);
        if found {
            if cycle_found_at == 0 {
                cycle_found_at = j;
                m.clear();
                m.insert(joined_grid, j);
                continue;
            }

            let mut remaining = 1000000000 - j - 1;
            remaining %= j - cycle_found_at;
            for _ in 0..remaining {
                cycle(&mut grid);
            }

            return Some(calculate_load(&grid) as u32);
        }

        m.insert(joined_grid, j);
    }

    None
}

fn cycle(grid: &mut Vec<Vec<char>>) {
    // move all the Os all the way up
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'O' {
                let mut k = i;
                while k > 0 && grid[k - 1][j] == '.' {
                    grid[k][j] = '.';
                    grid[k - 1][j] = 'O';
                    k -= 1;
                }
            }
        }
    }

    // move all the Os all the way left
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'O' {
                let mut k = j;
                while k > 0 && grid[i][k - 1] == '.' {
                    grid[i][k] = '.';
                    grid[i][k - 1] = 'O';
                    k -= 1;
                }
            }
        }
    }

    // move all the Os all the way down
    for i in (0..grid.len()).rev() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'O' {
                let mut k = i;
                while k < grid.len() - 1 && grid[k + 1][j] == '.' {
                    grid[k][j] = '.';
                    grid[k + 1][j] = 'O';
                    k += 1;
                }
            }
        }
    }

    // move all the Os all the way right
    for i in 0..grid.len() {
        for j in (0..grid[i].len()).rev() {
            if grid[i][j] == 'O' {
                let mut k = j;
                while k < grid[i].len() - 1 && grid[i][k + 1] == '.' {
                    grid[i][k] = '.';
                    grid[i][k + 1] = 'O';
                    k += 1;
                }
            }
        }
    }
}

// calculate the load of each O, which is equal to the number of rows from the O to the south edge of the grid
fn calculate_load(grid: &Vec<Vec<char>>) -> usize {
    let mut load = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'O' {
                load += grid.len() - i;
            }
        }
    }
    load
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
