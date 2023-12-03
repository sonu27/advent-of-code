use std::collections::HashMap;
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<_> = input.lines().collect();

    let mut nums: Vec<u32> = Vec::new();
    let mut current: Vec<char> = Vec::new();
    let mut save_current = false;

    // iterate of each char in each line
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                current.push(c);

                if save_current {
                    continue;
                }

                // check if char is within one step in all directions including diagonals
                for (dy, dx) in &[
                    (0, 1),
                    (0, -1),
                    (1, 0),
                    (-1, 0),
                    (-1, -1),
                    (-1, 1),
                    (1, -1),
                    (1, 1),
                ] {
                    let (ny, nx) = ((y as i32) + dy, (x as i32) + dx);

                    if ny >= 0
                        && nx >= 0
                        && (ny as usize) < lines.len()
                        && (nx as usize) < line.len()
                        && !lines[ny as usize]
                            .chars()
                            .nth(nx as usize)
                            .unwrap()
                            .is_digit(10)
                        && lines[ny as usize].chars().nth(nx as usize).unwrap() != '.'
                    {
                        save_current = true;
                        break;
                    }
                }
            } else {
                if save_current {
                    nums.push(current.iter().collect::<String>().parse::<u32>().unwrap());
                    save_current = false;
                }
                current.clear();
            }
        }
        if save_current {
            nums.push(current.iter().collect::<String>().parse::<u32>().unwrap());
            save_current = false;
        }
        current.clear();
    }

    Some(nums.iter().sum::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<_> = input.lines().collect();

    let mut map: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    let mut current: Vec<char> = Vec::new();
    let mut save_current = false;
    let mut current_pos: (usize, usize) = (0, 0);

    // iterate of each char in each line
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                current.push(c);

                if save_current {
                    continue;
                }

                // check if char is within one step in all directions including diagonals
                for (dy, dx) in &[
                    (0, 1),
                    (0, -1),
                    (1, 0),
                    (-1, 0),
                    (-1, -1),
                    (-1, 1),
                    (1, -1),
                    (1, 1),
                ] {
                    let (ny, nx) = ((y as i32) + dy, (x as i32) + dx);

                    if ny >= 0
                        && nx >= 0
                        && (ny as usize) < lines.len()
                        && (nx as usize) < line.len()
                        && lines[ny as usize].chars().nth(nx as usize).unwrap() == '*'
                    {
                        save_current = true;
                        current_pos = (ny as usize, nx as usize);
                    }
                }
            } else {
                if save_current {
                    map.entry(current_pos)
                        .or_insert(Vec::new())
                        .push(current.iter().collect::<String>().parse::<u32>().unwrap());
                    save_current = false;
                }
                current.clear();
            }
        }
        if save_current {
            map.entry(current_pos)
                .or_insert(Vec::new())
                .push(current.iter().collect::<String>().parse::<u32>().unwrap());
            save_current = false;
        }
        current.clear();
    }

    let ans = map
        .iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v.iter().product::<u32>())
        .sum::<u32>();
    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
