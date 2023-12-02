use regex::Regex;
advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let games = input.lines();
    let re = Regex::new(r"(\d+) (\w+)").unwrap();
    let mut p1_ans = Vec::new();

    for game in games {
        let parts: Vec<&str> = game.split(':').collect();
        let game_number: u32 = parts[0][5..].trim().parse().unwrap();
        let mut red_count = 0;
        let mut green_count = 0;
        let mut blue_count = 0;

        for cap in re.captures_iter(parts[1]) {
            let number: u32 = cap[1].parse().unwrap();
            let color: String = cap[2].to_string();
            match color.as_str() {
                "red" => {
                    if number > red_count {
                        red_count = number
                    }
                }
                "green" => {
                    if number > green_count {
                        green_count = number
                    }
                }
                "blue" => {
                    if number > blue_count {
                        blue_count = number
                    }
                }
                _ => {
                    println!("Unknown color: {}", color)
                }
            }
        }

        if red_count <= 12 && green_count <= 13 && blue_count <= 14 {
            p1_ans.push(game_number);
        }
    }

    Some(p1_ans.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = input.lines();
    let re = Regex::new(r"(\d+) (\w+)").unwrap();
    let mut p2_ans = Vec::new();

    for game in games {
        let parts: Vec<&str> = game.split(':').collect();
        let game_number: u32 = parts[0][5..].trim().parse().unwrap();
        let mut red_count = 0;
        let mut green_count = 0;
        let mut blue_count = 0;

        for cap in re.captures_iter(parts[1]) {
            let number: u32 = cap[1].parse().unwrap();
            let color: String = cap[2].to_string();
            match color.as_str() {
                "red" => {
                    if number > red_count {
                        red_count = number
                    }
                }
                "green" => {
                    if number > green_count {
                        green_count = number
                    }
                }
                "blue" => {
                    if number > blue_count {
                        blue_count = number
                    }
                }
                _ => {
                    println!("Unknown color: {}", color)
                }
            }
        }

        p2_ans.push(red_count * green_count * blue_count);
    }

    Some(p2_ans.iter().sum())
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
