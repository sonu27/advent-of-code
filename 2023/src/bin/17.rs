use std::cmp::min;
advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<u32> {
    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut visited = vec![vec![false; map[0].len()]; map.len()];

    let mut min_sum = u32::MAX;
    visited[0][0] = true;
    visit(
        &map,
        &mut visited,
        0,
        0,
        'R',
        1,
        0,
        &mut min_sum,
        'R',
        vec![[0, 0]],
    );

    Some(min_sum)
}

fn visit(
    map: &Vec<Vec<u32>>,
    visited: &mut Vec<Vec<bool>>,
    x: i32,
    y: i32,
    direction: char,
    mut moves: u32,
    mut sum: u32,
    min_sum: &mut u32,
    prev_dir: char,
    mut path: Vec<[usize; 2]>,
) {
    let sum = sum + map[y as usize][x as usize];
    // println!("{} {} {} {}", y, x, direction, sum);

    if sum >= *min_sum {
        return;
    }

    if y == map.len() as i32 - 1 && x == map[0].len() as i32 - 1 {
        *min_sum = min(sum, *min_sum);
        // panic!("end {}", min_sum);
        println!("end {}, {:?}", min_sum, path);
        return;
    }

    if prev_dir != direction {
        moves = 0;
    }

    let change_direction = moves > 2;

    let options = match (direction, change_direction) {
        ('U', false) => vec!['R', 'L', 'U'],
        ('U', true) => vec!['R', 'L'],
        ('D', false) => vec!['D', 'R', 'L'],
        ('D', true) => vec!['R', 'L'],
        ('L', false) => vec!['D', 'U', 'L'],
        ('L', true) => vec!['D', 'U'],
        ('R', false) => vec!['R', 'D', 'U'],
        ('R', true) => vec!['D', 'U'],
        _ => vec![],
    };

    for c in options {
        let (dy, dx) = match c {
            'U' => (-1, 0),
            'D' => (1, 0),
            'L' => (0, -1),
            'R' => (0, 1),
            _ => (0, 0),
        };

        let new_y = y + dy;
        let new_x = x + dx;

        // println!("{} {} {} {}", new_x, new_y, direction, sum);

        if new_x < 0
            || new_y < 0
            || new_x >= map[0].len() as i32
            || new_y >= map.len() as i32
            || visited[new_y as usize][new_x as usize]
        {
            continue;
        }

        visited[new_y as usize][new_x as usize] = true;
        path.push([new_y as usize, new_x as usize]);
        visit(
            map,
            visited,
            new_x,
            new_y,
            c,
            moves + 1,
            sum,
            min_sum,
            direction,
            path.clone(),
        );
        visited[new_y as usize][new_x as usize] = false;
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
