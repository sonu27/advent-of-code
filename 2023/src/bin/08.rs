use std::collections::HashMap;
use num_integer::lcm;

use regex::Regex;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let cmds = lines.next().unwrap().chars();

    lines.next(); // skip the blank line

    let mut m = HashMap::new();
    let re = Regex::new(r"(\w+)").unwrap();
    let a: Vec<_> = lines.map(|line| {
        let aa: Vec<_> = re.captures_iter(line).map(|cap| cap[0].to_string()).collect();
        aa
    }).collect();

    for x in a {
        m.insert(x.get(0).unwrap().clone(), (x.get(1).unwrap().clone(), x.get(2).unwrap().clone()));
    }

    let mut steps = 0;
    let mut curr = "AAA";
    loop {
        for c in cmds.clone() {
            if curr == "ZZZ" {
                return Some(steps);
            }
            let go = m.get(curr).unwrap();

            if c == 'L' {
                curr = &*go.0;
            } else {
                // R
                curr = &*go.1;
            }

            steps += 1;
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {// use u64 because of overflow
    let mut lines = input.lines();
    let cmds = lines.next().unwrap().chars();

    lines.next(); // skip the blank line

    let mut m = HashMap::new();
    let re = Regex::new(r"(\w+)").unwrap();
    let a: Vec<_> = lines.map(|line| {
        let aa: Vec<_> = re.captures_iter(line).map(|cap| cap[0].to_string()).collect();
        aa
    }).collect();

    let mut currs = vec![];
    for x in a {
        let key = x.get(0).unwrap().clone();

        if key.ends_with('A') {
            currs.push(key.clone());
        }

        m.insert(key, (x.get(1).unwrap().clone(), x.get(2).unwrap().clone()));
    }

    let mut steps: u64 = 0;
    let mut step_counts = vec![0; currs.len()];
    loop {
        for c in cmds.clone() {
            // println!("{:?}, {:?}", steps, currs);
            for i in 0..currs.clone().len() {
                if currs[i].ends_with('Z') && step_counts[i] > 0 {
                    continue;
                }

                let go = m.get(&currs[i]).unwrap();

                if c == 'L' {
                    currs[i] = (&*go.0).parse().unwrap();
                } else {
                    // R
                    currs[i] = (&*go.1).parse().unwrap();
                }

                if currs[i].ends_with('Z') {
                    step_counts[i] = steps + 1;
                }
            }

            steps += 1;

            if currs.iter().all(|x| x.ends_with('Z')) {
                let mut lcm_value = step_counts[0].clone();
                for count in &step_counts[1..] {
                    lcm_value = lcm(lcm_value, count.clone());
                }
                return Some(lcm_value);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

// #[test]
    // fn test_part_one() {
    //     let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, Some(2));
    // }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
