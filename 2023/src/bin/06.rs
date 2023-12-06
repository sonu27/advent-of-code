advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let a: Vec<Vec<u64>> = input
        .lines()
        .map(|line| line.split(':')
            .last()
            .map(|s| s.split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect()
            )
            .unwrap())
        .collect();

    let mut results: u64 = 1;

    for race in 0..a[0].len() {
        let mut result: u64 = 0;
        let start =  (a[1][race] / a[0][race])+1;
        for press in start..a[0][race] { // per second
            let distance = get_distance(press, a[0][race]);
            if distance > a[1][race] {
                result += 1;
            }
        }
        results *= result;
    }

    Some(results as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let a: Vec<u64> = input
        .lines()
        .map(|line| line.split(':')
            .last()
            .map(|s| s.split_whitespace()
                .collect::<Vec<&str>>()
                .join("").parse::<u64>().unwrap()
            )
            .unwrap())
        .collect();

    let mut result: u64 = 0;
    let start =  (a[1] / a[0])+1;
    for press in start..a[0] { // per second
        let distance = get_distance(press, a[0]);
        if distance > a[1] {
            result += 1;
        }
    }

    Some(result as u32)
}

fn get_distance(press: u64, stop_time: u64) -> u64 {
    press * (stop_time - press)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
