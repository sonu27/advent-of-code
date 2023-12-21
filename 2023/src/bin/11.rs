use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut cols_count = HashMap::new();
    let mut galaxy = Vec::new();

    input.lines().for_each(|line| {
        let mut row = 0;
        let chars = line
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if c == '#' {
                    row += 1;
                    cols_count.insert(i, cols_count.get(&i).unwrap_or(&0) + 1);
                }
                c
            })
            .collect::<Vec<_>>();

        if row == 0 {
            grid.push(chars.clone());
        }
        grid.push(chars);
    });

    let new_grid: Vec<Vec<char>> = grid
        .iter()
        .map(|row| {
            let mut new_row: Vec<char> = Vec::new();
            row.iter().enumerate().for_each(|(i, c)| {
                if cols_count.get(&i).is_none() {
                    new_row.push(*c)
                }

                new_row.push(*c);
            });
            new_row
        })
        .collect();

    for (y, row) in new_grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '#' {
                galaxy.push((y, x));
            }
        }
    }

    let mut a = 0;
    for i in 0..galaxy.len() - 1 {
        for j in i + 1..galaxy.len() {
            a += (galaxy[j].0 - galaxy[i].0) as i32
                + (galaxy[i].1 as i32 - galaxy[j].1 as i32).abs();
        }
    }

    Some(a as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
