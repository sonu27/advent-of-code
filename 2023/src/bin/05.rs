use std::collections::HashSet;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let a = input
        .trim_start_matches("seeds: ")
        .split(" map:")
        .map(|s| {
            s.lines()
                .filter(|s| {
                    if let Some(c) = s.chars().next() {
                        c.is_numeric()
                    } else {
                        false
                    }
                })
                .map(|s| {
                    s.split(' ')
                        .map(|s| s.parse::<i64>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut seeds = a[0][0].clone();
    for i in 1..a.len() {
        seeds = get_ans(seeds, &a[i]);
    }

    seeds.iter().min().map(|s| *s as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let orig_seeds: Vec<i64> = input
        .lines()
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let mut seeds: Vec<i64> = orig_seeds.clone();
    for i in (0..seeds.len()).step_by(2) {
        seeds[i + 1] = seeds[i] + seeds[i + 1] - 1;
    }
    seeds.sort();

    let a = input
        .trim_start_matches("seeds: ")
        .split(" map:")
        .map(|s| {
            s.lines()
                .filter_map(|s| {
                    if let Some(c) = s.chars().next() {
                        if c.is_numeric() {
                            let x = s
                                .split(' ')
                                .map(|s| s.parse::<i64>().unwrap())
                                .collect::<Vec<_>>();
                            return Some((x[0], x[1], x[2]));
                        }
                    }
                    None
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut seed_endpts = vec![0, i64::MAX];
    for i in (1..a.len()).rev() {
        seed_endpts = invert_map(&a[i], &seed_endpts);
    }

    fn in_seed_range(seeds: &Vec<i64>, s: i64) -> bool {
        (0..seeds.len())
            .step_by(2)
            .any(|i| seeds[i] <= s && s <= seeds[i + 1])
    }

    let seed_endpts: Vec<i64> = seed_endpts
        .into_iter()
        .filter(|&s| in_seed_range(&seeds, s))
        .collect();

    let mut ans = seed_endpts.clone();
    for i in a {
        ans = get_ans2(ans, &i);
    }
    ans.iter().min().map(|s| *s as u32)
}

fn inverse_lookup_map(map_info: &Vec<(i64, i64, i64)>, y: i64) -> i64 {
    for &(d_start, s_start, rlen) in map_info {
        if d_start <= y && y < d_start + rlen {
            return s_start + (y - d_start);
        }
    }
    y
}

fn invert_map(map_info: &Vec<(i64, i64, i64)>, output_endpts: &[i64]) -> Vec<i64> {
    let mut map_endpts: Vec<(i64, i64)> = map_info
        .iter()
        .flat_map(|&(d_start, s_start, rlen)| {
            vec![(d_start, s_start), (d_start + rlen - 1, s_start + rlen - 1)]
        })
        .collect();
    map_endpts.sort_by(|a, b| a.0.cmp(&b.0));

    let mut output_src_endpts: Vec<i64> = output_endpts
        .iter()
        .map(|&y| inverse_lookup_map(map_info, y))
        .collect();
    output_src_endpts.sort();

    let mut input_src_endpts: Vec<i64> = map_endpts
        .iter()
        .map(|&(_y, x)| x)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    input_src_endpts.sort();

    if input_src_endpts[0] > 0 {
        input_src_endpts.insert(0, 0);
        input_src_endpts.insert(1, input_src_endpts[1] - 1);
    }
    if *input_src_endpts.last().unwrap() < i64::MAX {
        input_src_endpts.push(input_src_endpts.last().unwrap() + 1);
        input_src_endpts.push(i64::MAX);
    }

    let mut input_endpts: Vec<i64> = output_src_endpts
        .iter()
        .chain(input_src_endpts.iter())
        .cloned()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    input_endpts.sort();

    input_endpts
}

fn get_ans(mut seeds: Vec<i64>, m: &Vec<Vec<i64>>) -> Vec<i64> {
    for seed in &mut seeds {
        let mut new_seed = 0;
        for m in m {
            if m[1] <= *seed && *seed < m[1] + m[2] {
                new_seed = m[0] - m[1] + *seed;
                break;
            }
        }
        if new_seed == 0 {
            new_seed = *seed;
        }
        *seed = new_seed;
    }
    seeds
}

fn get_ans2(mut seeds: Vec<i64>, m: &Vec<(i64, i64, i64)>) -> Vec<i64> {
    for seed in &mut seeds {
        let mut new_seed = 0;
        for m in m {
            if m.1 <= *seed && *seed < m.1 + m.2 {
                new_seed = m.0 - m.1 + *seed;
                break;
            }
        }
        if new_seed == 0 {
            new_seed = *seed;
        }
        *seed = new_seed;
    }
    seeds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
