use num_integer::lcm;
use regex::Regex;
use std::collections::HashMap;

advent_of_code::solution!(8);

fn parse_input(input: &str) -> (Vec<usize>, HashMap<String, Vec<String>>) {
    let mut lines = input.lines();
    let cmds: Vec<usize> = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => {
                panic!("Invalid input")
            }
        })
        .collect();

    lines.next(); // skip the blank line

    let mut m = HashMap::new();
    let re = Regex::new(r"(\w+)").unwrap();
    lines
        .map(|line| {
            re.captures_iter(line)
                .map(|cap| cap[0].to_string())
                .collect::<Vec<_>>()
        })
        .for_each(|v| {
            m.insert(
                v.get(0).unwrap().clone(),
                vec![v.get(1).unwrap().clone(), v.get(2).unwrap().clone()],
            );
        });

    (cmds, m)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (cmds, m) = parse_input(input);
    let mut steps = 0;
    let mut curr = "AAA";
    loop {
        for c in cmds.clone() {
            if curr == "ZZZ" {
                return Some(steps);
            }
            curr = &m.get(curr).unwrap()[c];
            steps += 1;
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let (cmds, m) = parse_input(input);

    let mut currs = vec![];
    m.iter().for_each(|(k, _)| {
        if k.ends_with('A') {
            currs.push(k.clone());
        }
    });

    let mut steps: u64 = 0;
    let mut step_counts = vec![0; currs.len()];
    loop {
        for c in cmds.clone() {
            for i in 0..currs.clone().len() {
                if currs[i].ends_with('Z') && step_counts[i] > 0 {
                    continue;
                }

                currs[i] = m.get(&currs[i]).unwrap()[c].parse().unwrap();

                if currs[i].ends_with('Z') {
                    step_counts[i] = steps + 1;
                }
            }

            steps += 1;

            if currs.iter().all(|x| x.ends_with('Z')) {
                let mut lcm_value = step_counts[0];
                for count in &step_counts[1..] {
                    lcm_value = lcm(lcm_value, *count);
                }
                return Some(lcm_value);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
