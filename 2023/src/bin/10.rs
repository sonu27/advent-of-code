advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let mut start: [usize; 2] = [0, 0];
    let mut map: Vec<_> = input
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

    let mut visited: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];
    let mut path: Vec<[usize; 2]> = Vec::new();

    visit(&mut map, &mut visited, start, &mut path);

    // Shoelace formula
    let mut trailing: i32 = 0;

    for i in 0..path.len() {
        let j = (i + 1) % path.len();
        trailing += path[i][0] as i32 * path[j][1] as i32 - path[j][0] as i32 * path[i][1] as i32;
    }

    let area = (trailing as f32 / 2.0).abs();

    // Pick's theorem
    let inner_points = area - path.len() as f32 / 2.0 + 1.0;
    println!("Part 2: {}", inner_points);

    Some(path.len() as u32 / 2)
}

const STEPS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn visit(
    map: &mut Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    pos: [usize; 2],
    path: &mut Vec<[usize; 2]>,
) {
    let curr_char = map[pos[0]][pos[1]];
    for step in STEPS.iter() {
        let y = pos[0] as i32 + step.0;
        let x = pos[1] as i32 + step.1;

        // check bounds
        if y < 0 || x < 0 || y >= map.len() as i32 || x >= map[y as usize].len() as i32 {
            continue;
        }

        if visited[y as usize][x as usize] {
            continue;
        }

        match (curr_char, step.0, step.1) {
            ('│', 0, _)
            | ('─', _, 0)
            | ('└', 1, _)
            | ('└', _, -1)
            | ('┘', 1, _)
            | ('┘', _, 1)
            | ('┐', -1, _)
            | ('┐', _, 1)
            | ('┌', -1, _)
            | ('┌', _, -1)
            | ('0', _, _) => continue,
            _ => {}
        }

        path.push([y as usize, x as usize]);

        map[pos[0]][pos[1]] = match map[pos[0]][pos[1]] {
            '┌' => '┏',
            '└' => '┗',
            '┘' => '┛',
            '┐' => '┓',
            '│' => '┃',
            '─' => '━',
            _ => map[pos[0]][pos[1]],
        };

        visited[y as usize][x as usize] = true;

        return visit(map, visited, [y as usize, x as usize], path);
    }
    print_char_map(&map);
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

pub fn part_two(_: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
