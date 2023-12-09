advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i64> {
    let a: i64 = input
        .lines()
        .map(|line| line.split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>())
        .map(|v| solve(v))
        .sum()
        ;

    Some(a)
}

pub fn part_two(input: &str) -> Option<i64> {
    let a: i64 = input
        .lines()
        .map(|line| line.split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .rev()
            .collect::<Vec<_>>())
        .map(|v| solve(v))
        .sum()
        ;

    Some(a)
}

fn solve(a: Vec<i64>) -> i64 {
    let mut nums: Vec<Vec<i64>> = vec![a];

    while !nums.last().unwrap().iter().all(|&x| x == 0) {
        nums.push(nums
            .last()
            .unwrap()
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect::<Vec<_>>())
    }

    nums
        .iter()
        .map(|v| v.last().unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
