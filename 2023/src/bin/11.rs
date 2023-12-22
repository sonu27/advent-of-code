advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<i64> {
    Some(solve(input, 1))
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(solve(input, 999999))
}

fn solve(input: &str, expand: i64) -> i64 {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut galaxy = Vec::new();

    let mut col_contains_galaxy = vec![false; input.lines().next().unwrap().len()];
    let mut y_map = vec![0; input.lines().count()];

    input.lines().enumerate().for_each(|(y, line)| {
        if !line.contains('#') {
            y_map[y] = 1;
        }

        let chars = line
            .chars()
            .enumerate()
            .map(|(x, c)| {
                if c == '#' {
                    col_contains_galaxy[x] = true;
                    galaxy.push((y, x));
                }
                c
            })
            .collect::<Vec<_>>();

        grid.push(chars);
    });

    // pairs
    let mut ans = 0;
    for i in 0..galaxy.len() - 1 {
        for j in i + 1..galaxy.len() {
            let mut yy = [galaxy[i].0, galaxy[j].0];
            yy.sort();

            for y in yy[0]..yy[1] {
                if y_map[y] == 1 {
                    ans += expand;
                }
                ans += 1;
            }

            let mut xx = [galaxy[i].1, galaxy[j].1];
            xx.sort();

            for x in xx[0]..xx[1] {
                if !col_contains_galaxy[x] {
                    ans += expand;
                }
                ans += 1;
            }
        }
    }

    ans
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
