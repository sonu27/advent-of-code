use crate::Direction::{Down, Left, Right, Up};
use std::collections::HashSet;
advent_of_code::solution!(16);

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn part_one(input: &str) -> Option<u32> {
    let map: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<_>>();

    Some(solve(&map, Point { x: 0, y: 0 }, Right) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<_>>();

    let mut starts: Vec<(Point, Direction)> = vec![
        (Point { x: 0, y: 0 }, Right),
        (Point { x: 0, y: 0 }, Down),
        (
            Point {
                x: 0,
                y: map.len() as i32 - 1,
            },
            Up,
        ),
        (
            Point {
                x: 0,
                y: map.len() as i32 - 1,
            },
            Right,
        ),
        (
            Point {
                x: map[0].len() as i32 - 1,
                y: 0,
            },
            Left,
        ),
        (
            Point {
                x: map[0].len() as i32 - 1,
                y: 0,
            },
            Down,
        ),
        (
            Point {
                x: map[0].len() as i32 - 1,
                y: map.len() as i32 - 1,
            },
            Left,
        ),
        (
            Point {
                x: map[0].len() as i32 - 1,
                y: map.len() as i32 - 1,
            },
            Up,
        ),
    ];
    for i in 1..map.len() - 1 {
        starts.push((Point { x: 0, y: i as i32 }, Right));
        starts.push((
            Point {
                x: map[0].len() as i32 - 1,
                y: i as i32,
            },
            Left,
        ));
    }
    for i in 1..map[0].len() - 1 {
        starts.push((Point { x: i as i32, y: 0 }, Down));
        starts.push((
            Point {
                x: i as i32,
                y: map.len() as i32 - 1,
            },
            Up,
        ));
    }

    let ans = starts
        .iter()
        .map(|(pos, dir)| solve(&map, *pos, *dir))
        .max()
        .unwrap();

    Some(ans as u32)
}

fn solve(map: &Vec<Vec<char>>, starting_pos: Point, dir: Direction) -> usize {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut v2: HashSet<(Point, Direction)> = HashSet::new();

    visit(&map, &mut visited, &mut v2, starting_pos, dir);

    visited.len()
}

fn visit(
    map: &Vec<Vec<char>>,
    visited: &mut HashSet<Point>,
    v2: &mut HashSet<(Point, Direction)>,
    pos: Point,
    dir: Direction,
) {
    if v2.contains(&(pos, dir)) {
        return;
    }
    let curr_char = map[pos.y as usize][pos.x as usize];
    visited.insert(pos);
    v2.insert((pos, dir));

    let new_dir = match (dir, curr_char) {
        (Right, '|') | (Left, '|') => {
            vec![Up, Down]
        }
        (Up, '-') | (Down, '-') => vec![Left, Right],
        (Right, '/') | (Left, '\\') => vec![Up],
        (Left, '/') | (Right, '\\') => vec![Down],
        (Up, '/') | (Down, '\\') => vec![Right],
        (Down, '/') | (Up, '\\') => vec![Left],
        _ => vec![dir],
    };

    for d in new_dir {
        let x = match d {
            Right => pos.x + 1,
            Left => pos.x - 1,
            _ => pos.x,
        };
        let y = match d {
            Up => pos.y - 1,
            Down => pos.y + 1,
            _ => pos.y,
        };

        if y < 0 || y >= map.len() as i32 || x < 0 || x >= map[0].len() as i32 {
            continue;
        }
        visit(map, visited, v2, Point { x, y }, d);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
