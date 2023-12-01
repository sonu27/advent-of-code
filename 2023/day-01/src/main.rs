use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let first_digit = line.chars().find(|c| c.is_digit(10));
            let last_digit = line.chars().rev().find(|c| c.is_digit(10));
            match (first_digit, last_digit) {
                (Some(first), Some(last)) => Some(first.to_string() + &last.to_string()),
                _ => None,
            }
        })
        .map(|s| s.parse::<u32>().unwrap())
        .sum()
}

fn part2(input: &str) -> u32 {
    let re = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let word_to_num = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]
        .iter()
        .cloned()
        .collect::<std::collections::HashMap<_, _>>();

    input
        .lines()
        .filter_map(|line| {
            let mut matches = Vec::new();
            for start in 0..line.len() {
                if let Some(match_) = re.find(&line[start..]) {
                    let match_str = &line[start..][match_.start()..match_.end()];
                    let num_str = word_to_num.get(match_str).unwrap_or(&match_str);
                    matches.push(num_str.to_string());
                }
            }

            let first_match = matches.first();
            let last_match = matches.last();
            match (first_match, last_match) {
                (Some(first), Some(last)) => Some(first.to_string() + &last.to_string()),
                _ => None,
            }
        })
        .map(|s| s.parse::<u32>().unwrap())
        .sum()
}
