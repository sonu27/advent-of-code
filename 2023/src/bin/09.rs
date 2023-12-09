advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i64> {
    let a: i64 = input
        .lines()
        .map(|line| line.split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>())
        .map(|v| {
            solve(v)
        })
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
        .map(|v| {
            solve(v)
        })
        .sum()
        ;

    Some(a)
}

fn solve(a: Vec<i64>) -> i64 {
    let mut nums: Vec<Vec<i64>> = vec![];
    nums.push(a);

    loop {
        for j in nums.len() - 1..nums.len() {
            nums.push(vec![0; nums[j].len() - 1]);
            for i in 0..nums[j].len() - 1 {
                nums[j + 1][i] = nums[j][i + 1] - nums[j][i];
            }
        }
        if nums[nums.len() - 1].iter().all(|&x| x == 0) {
            break;
        }
    }

    for j in (0..nums.len() - 1).rev() {
        let last = nums[j].last().unwrap().clone();
        let last2 = nums[j + 1].last().unwrap().clone();

        nums[j].push(last + last2);
    }

    nums[0].last().unwrap().clone()
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
