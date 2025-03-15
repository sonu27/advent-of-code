use std::collections::HashSet;
advent_of_code::solution!(21);

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

                    c
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let mut on: HashSet<[usize; 2]> = HashSet::new();

    visit(&map, start, 64, &mut on);

    println!("{:?}", on);

    None
}

const STEPS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn visit(map: &Vec<Vec<char>>, pos: [usize; 2], steps_left: u32, on: &mut HashSet<[usize; 2]>) {
    let curr_char = map[pos[0]][pos[1]];

    if steps_left == 0 {
        on.insert(pos);
        return;
    }

    for step in STEPS.iter() {
        let y = pos[0] as i32 + step.0;
        let x = pos[1] as i32 + step.1;

        // check bounds
        if y < 0 || x < 0 || y >= map.len() as i32 || x >= map[y as usize].len() as i32 {
            continue;
        }

        if map[y as usize][x as usize] == '#' {
            continue;
        }

        visit(map, [y as usize, x as usize], steps_left - 1, on)
    }
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

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
