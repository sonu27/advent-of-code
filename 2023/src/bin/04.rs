use std::collections::HashSet;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let a = input
        .lines()
        .map(|line| {
            line.split(":")
                .last()
                .unwrap()
                .split("|")
                .map(|s| s.split_whitespace().collect::<Vec<&str>>())
                .collect::<Vec<Vec<&str>>>()
        })
        .map(|s| {
            let first: HashSet<&str> = HashSet::from_iter(s[0].iter().cloned());
            let second: HashSet<&str> = HashSet::from_iter(s[1].iter().cloned());

            first
                .intersection(&second)
                .fold(0, |acc, _| if acc == 0 { 1 } else { acc * 2 })
        })
        .sum::<u32>();

    Some(a)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();

    let mut counts: Vec<usize> = vec![1; lines.len()];

    let a = lines
        .iter()
        .map(|line| {
            line.split(":")
                .last()
                .unwrap()
                .split("|")
                .map(|s| s.split_whitespace().collect::<Vec<&str>>())
                .collect::<Vec<Vec<&str>>>()
        })
        .enumerate()
        .map(|(count, s)| {
            let first: HashSet<&str> = HashSet::from_iter(s[0].iter().cloned());
            let second: HashSet<&str> = HashSet::from_iter(s[1].iter().cloned());

            let matches = first.intersection(&second)
                .count();

            for _ in 0..counts[count] {
                for c in count + 1..count + matches + 1 {
                    counts[c] += 1;
                }
            }

            counts[count]
        })
        .sum::<usize>();

    Some(a.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
