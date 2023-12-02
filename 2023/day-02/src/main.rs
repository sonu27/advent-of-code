use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    let games = input.lines();
    let re = Regex::new(r"(\d+) (\w+)").unwrap();
    let mut p1_ans = Vec::new();
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
                "red" => if number > red_count { red_count = number },
                "green" => if number > green_count { green_count = number },
                "blue" => if number > blue_count { blue_count = number },
                _ => { println!("Unknown color: {}", color) }
            }
        }

        if red_count <= 12 && green_count <= 13 && blue_count <= 14 {
            p1_ans.push(game_number);
        }

        p2_ans.push(red_count * green_count * blue_count);
    }

    println!("Part 1: {}", p1_ans.iter().sum::<u32>());
    println!("Part 2: {}", p2_ans.iter().sum::<u32>());
}
