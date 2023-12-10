use std::cmp::max;
advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let mut start: [usize; 2] = [0, 0];
    let map: Vec<_> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = [y, x];
                    }

                    if c == 'F' {
                        '┌'
                    } else if c == 'L' {
                        '└'
                    } else if c == 'J' {
                        '┘'
                    } else if c == '7' {
                        '┐'
                    } else if c == '|' {
                        '│'
                    } else if c == '-' {
                        '─'
                    } else if c == '.' {
                        '0'
                    } else {
                        c
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();

    print_char_map(&map.clone());

    let mut int_map: Vec<Vec<u32>> = vec![vec![0; map[0].len()]; map.len()];
    let ans = visit(&map, &mut int_map, start, 1);

    Some(ans / 2)
}

const STEPS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn visit(map: &Vec<Vec<char>>, int_map: &mut Vec<Vec<u32>>, pos: [usize; 2], count: u32) -> u32 {
    let curr_char = map[pos[0]][pos[1]];
    for step in STEPS.iter() {
        let y = pos[0] as i32 + step.0;
        let x = pos[1] as i32 + step.1;

        if y < 0 || x < 0 || y >= map.len() as i32 || x >= map[y as usize].len() as i32 {
            continue;
        }

        if map[y as usize][x as usize] == '0' || int_map[y as usize][x as usize] != 0 {
            continue;
        }

        match (curr_char, step.0, step.1) {
            ('│', 0, _) => continue,
            ('─', _, 0) => continue,
            ('└', 1, _) => continue,
            ('└', _, -1) => continue,
            ('┘', 1, _) => continue,
            ('┘', _, 1) => continue,
            ('┐', -1, _) => continue,
            ('┐', _, 1) => continue,
            ('┌', -1, _) => continue,
            ('┌', _, -1) => continue,
            ('.', _, _) => continue,
            _ => {}
        }

        int_map[y as usize][x as usize] = count;

        return max(
            count,
            visit(map, int_map, [y as usize, x as usize], count + 1),
        );
    }

    // map[pos[0]][pos[1]] = match map[pos[0]][pos[1]] {
    //     '┌' => '┏',
    //     '└' => '┗',
    //     '┘' => '┛',
    //     '┐' => '┓',
    //     '│' => '┃',
    //     '─' => '━',
    //     _ => map[pos[0]][pos[1]],
    // };
    // print_char_map(&map);

    count
}

fn print_map(map: &mut Vec<Vec<u32>>) {
    println!();
    for line in map {
        for c in line {
            print!("{},", format!("{:02}", c));
        }
        println!();
    }
}

fn print_char_map(map: &Vec<Vec<char>>) {
    let mut output = String::new();
    output.push('\n');

    for line in map {
        for c in line {
            output.push_str(&format!("{} ", c));
        }
        output.push('\n');
    }

    print!("{}", output);
    // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
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
        assert_eq!(result, None);
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
