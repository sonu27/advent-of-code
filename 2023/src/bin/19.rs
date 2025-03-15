use std::collections::HashMap;
advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u32> {
    // let mut m: HashMap<_, _> = HashMap::new();

    let a = input.lines().for_each(|line| {
        if line.is_empty() {
        } else if line.starts_with('{') {
        } else {
            let mut parts = line.split('{');
            let name = parts.next().unwrap();

            let tmp = parts.next().and_then(|s| s.split('}').nth(0)).unwrap();
            println!("{}: {}", name, tmp);

            // let mut actions: Vec<Vec<_>> = Vec::new();

            tmp.split(',').for_each(|s| {
                s.split(':').for_each(|s| {
                    if s.contains('<') {
                        s.split('<').for_each(|s| {})
                    }
                    println!("s2: {}", s);
                });
            });

            // m.insert(name, p);
        }
    });

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
